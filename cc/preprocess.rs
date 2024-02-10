use crate::*;
use parse::Parser;
use std::collections::HashMap;

const DUNDER_FILE: &str = "__FILE__";
const DUNDER_LINE: &str = "__LINE__";

pub fn preprocess(
  file: File,
  defines: &mut HashMap<String, TextLine>,
  errors: &mut impl Extend<(Pos, Error)>,
  pos: Option<Pos>,
) -> String {
  // remove comments and resolve includes and defines

  let preprocessor = parse::many(preprocess::whitespace()).and_then(|_| {
    Parser::expected(vec![])
      .or_else(|_| preprocess::include_directive())
      .or_else(|_| preprocess::define_directive())
      .or_else(|_| preprocess::undef_directive())
      .or_else(|_| preprocess::pragma_directive())
      .or_else(|_| preprocess::error_directive())
      .or_else(|_| preprocess::null_directive())
      .or_else(|_| preprocess::text_line_directive())
      .or_else(|_| parse::eof().map(|_| Directive::EOF))
  });

  defines.insert("__STDC_NO_ATOMICS__".to_string(), vec![Ok(1.to_string())]);
  defines.insert("__STDC_NO_COMPLEX__".to_string(), vec![Ok(1.to_string())]);
  defines.insert("__STDC_NO_THREADS__".to_string(), vec![Ok(1.to_string())]);
  defines.insert("__STDC_NO_VLA__".to_string(), vec![Ok(1.to_string())]);

  let last_file = defines.get(DUNDER_FILE).unwrap_or(&vec![]).clone();
  let current_file = vec![Err('"'), Ok(file.0.clone()), Err('"')];
  defines.insert(DUNDER_FILE.to_string(), current_file); // TODO escape quotes in filename
  defines.insert(DUNDER_LINE.to_string(), vec![Ok(0.to_string())]);

  let source = std::fs::read_to_string(&file.0).unwrap_or_else(|_| {
    errors.extend([(
      pos.unwrap_or(Pos(File("[bootstrap]".to_string()), 0, 0)),
      Error(format!("Unable to read file `{}`", file)),
    )]);
    format!("")
  });

  // adjacent string literals are concatenated later by the parser
  let mut preprocessed = "".to_string();
  let mut source = source
    .replace("\\\n", "") // line continuation
    .split("\n")
    .map(|line| line.split("//").next().unwrap_or(line)) // line comments
    .map(|line| line.to_owned() + "\n")
    .collect::<String>()
    .split("*/")
    .map(|item| item.split("/*").next().unwrap_or(item)) // block comments
    .map(|item| item.to_owned() + " ")
    .collect::<String>();

  let preprocessed = loop {
    (preprocessed, source) = match preprocessor.0(&source).into_result() {
      Ok((Directive::Include(filename), input)) => (
        preprocessed
          + &preprocess_include_directive(
            &file,
            filename,
            defines,
            errors,
            Pos(File("[preprocess]".to_string()), 0, 0),
          ),
        input,
      ),

      Ok((Directive::Define(identifier, replacement_list), input)) => {
        defines.insert(identifier.clone(), replacement_list.clone());
        (preprocessed, input)
      }

      Ok((Directive::Undef(identifier), input)) => {
        defines.remove(&identifier);
        (preprocessed, input)
      }

      Ok((Directive::Pragma(arguments), input)) => (
        preprocessed
          + &preprocess_pragma_directive(
            arguments,
            defines,
            errors,
            Pos(File("[preprocess]".to_string()), 0, 0),
          )
          + "\n",
        input,
      ),

      Ok((Directive::Error(message), input)) => (
        preprocessed
          + &preprocess_error_directive(
            message,
            defines,
            errors,
            Pos(File("[preprocess]".to_string()), 0, 0),
          )
          + "\n",
        input,
      ),

      Ok((Directive::Null, input)) => (preprocessed, input),

      Ok((Directive::TextLine(text_line), input)) => (
        preprocessed + &preprocess_text_line_directive(text_line, defines, errors) + "\n",
        input,
      ),

      Ok((Directive::EOF, input)) => {
        break match &input[..] {
          "" => preprocessed,
          _ => panic!("Input not fully parsed"),
        };
      }

      Err(expecteds) => {
        errors.extend([(
          Pos(File("[preprocess]".to_string()), 0, 0),
          Error(parse::format_expecteds(expecteds)),
        )]);
        (preprocessed, "".to_string())
      }
    }
  };

  defines.insert(DUNDER_FILE.to_string(), last_file.clone());

  preprocessed
}

fn preprocess_text_line_directive(
  text_line: TextLine,
  defines: &mut HashMap<String, TextLine>,
  errors: &mut impl Extend<(Pos, Error)>,
) -> String {
  // resolve defines recursively in text line and return preprocessed text line

  let mut acc = "".to_string();

  for line_item in text_line.iter() {
    acc += &match line_item {
      Ok(identifier) => match defines.remove(identifier) {
        Some(text_line) => {
          // prevents infinite recursion
          let preprocessed = preprocess_text_line_directive(text_line.clone(), defines, errors);
          defines.insert(identifier.clone(), text_line);
          preprocessed
        }
        None => identifier.clone(),
      },
      Err(char) => char.to_string(),
    }
  }

  acc
}

fn preprocess_pragma_directive(
  _arguments: TextLine,
  _defines: &mut HashMap<String, TextLine>,
  _errors: &mut impl Extend<(Pos, Error)>,
  _pos: Pos,
) -> String {
  // silently ignore unsupported pragmas as per standard
  "".to_string()
}

fn preprocess_error_directive(
  message: TextLine,
  defines: &mut HashMap<String, TextLine>,
  errors: &mut impl Extend<(Pos, Error)>,
  pos: Pos,
) -> String {
  let message = preprocess_text_line_directive(message.clone(), defines, errors);

  errors.extend([(pos.clone(), Error(format!("#error {}", message)))]);

  "".to_string()
}

fn preprocess_include_directive(
  file: &File,
  filename: TextLine,
  defines: &mut HashMap<String, TextLine>,
  errors: &mut impl Extend<(Pos, Error)>,
  pos: Pos,
) -> String {
  // resolve defines in include directive and preprocess included file

  use std::path::Path;
  let filename = preprocess_text_line_directive(filename, defines, errors);

  match preprocess::include_directive_filename().parse(&filename) {
    Ok(filename) => {
      let incl = File(
        Path::new(&file.0)
          .parent()
          .unwrap()
          .join(filename)
          .to_str()
          .unwrap()
          .to_string(),
      );
      preprocess(incl, defines, errors, Some(pos))
    }
    Err(error) => {
      errors.extend([(Pos(File("[preprocess]".to_string()), 0, 0), Error(error))]);
      format!("")
    }
  }
}

type TextLine = Vec<Result<String, char>>;

#[derive(Clone, PartialEq, Debug)]
enum Directive {
  Include(TextLine),
  Define(String, TextLine),
  Undef(String),
  Pragma(TextLine),
  Error(TextLine),
  Null,
  TextLine(TextLine),
  EOF,
}

fn any() -> Parser<char> {
  parse::char_not('\n').name(format!("non-newline character"))
}

pub fn whitespace() -> Parser<char> {
  parse::satisfy(|c| c.is_whitespace() && c != '\n').name(format!("non-newline whitespace"))
}

pub fn ws<T: Clone + 'static>(parser: Parser<T>) -> Parser<T> {
  parser.and_then(|r#match| parse::many(preprocess::whitespace()).map(move |_| r#match))
}

fn include_directive() -> Parser<Directive> {
  // TODO does not obey grammar
  Parser::pure(())
    .and_then(|_| preprocess::ws(parse::char('#')))
    .and_then(|_| preprocess::ws(parse::string("include")))
    .and_then(|_| preprocess::text_line())
    .and_then(|filename| parse::newline().map(move |_| Directive::Include(filename)))
}

fn define_directive() -> Parser<Directive> {
  // TODO does not obey grammar
  Parser::pure(())
    .and_then(|_| preprocess::ws(parse::char('#')))
    .and_then(|_| preprocess::ws(parse::string("define")))
    .and_then(|_| preprocess::ws(preprocess::identifier()))
    .and_then(|identifier| {
      let identifier2 = identifier.clone();
      preprocess::text_line()
        .and_then(|replacement_list| {
          parse::newline().map(move |_| Directive::Define(identifier, replacement_list))
        })
        .or_else(|_| Parser::pure(Directive::Define(identifier2, vec![])))
    })
}

fn undef_directive() -> Parser<Directive> {
  // TODO does not obey grammar
  Parser::pure(())
    .and_then(|_| preprocess::ws(parse::char('#')))
    .and_then(|_| preprocess::ws(parse::string("undef")))
    .and_then(|_| preprocess::ws(preprocess::identifier()))
    .and_then(|identifier| parse::newline().map(move |_| Directive::Undef(identifier)))
}

fn pragma_directive() -> Parser<Directive> {
  // TODO does not obey grammar
  Parser::pure(())
    .and_then(|_| preprocess::ws(parse::char('#')))
    .and_then(|_| preprocess::ws(parse::string("pragma")))
    .and_then(|_| preprocess::text_line())
    .and_then(|arguments| parse::newline().map(move |_| Directive::Pragma(arguments)))
}

fn error_directive() -> Parser<Directive> {
  // TODO does not obey grammar
  Parser::pure(())
    .and_then(|_| preprocess::ws(parse::char('#')))
    .and_then(|_| preprocess::ws(parse::string("error")))
    .and_then(|_| preprocess::text_line())
    .and_then(|message| parse::newline().map(move |_| Directive::Error(message)))
}

fn null_directive() -> Parser<Directive> {
  // TODO does not obey grammar
  Parser::pure(())
    .and_then(|_| preprocess::ws(parse::char('#')))
    .and_then(|_| parse::newline().map(|_| Directive::Null))
}

fn text_line_directive() -> Parser<Directive> {
  // TODO does not obey grammar

  // if first non-whitespace character is `'#'`, assume misparsed directive
  // and error out here in the preprocessor rather than later in the parser
  preprocess::ws(parse::char('#'))
    // using `"include"` as sentinel is hacky but works
    .and_then(|_| Parser::expected(vec!["\"include\"".to_string()]))
    .or_else(|expecteds| {
      if expecteds.contains(&"\"include\"".to_string()) {
        Parser::expected(vec![])
      } else {
        preprocess::text_line()
          .and_then(|text_line| parse::newline().map(move |_| Directive::TextLine(text_line)))
      }
    })
}

fn text_line() -> Parser<TextLine> {
  // TODO does not obey grammar
  parse::many(
    Parser::expected(vec![])
      .or_else(|_| preprocess::whitespace().map(|character| Err(character)))
      .or_else(|_| preprocess::identifier().map(|identifier| Ok(identifier)))
      .or_else(|_| preprocess::any().map(|character| Err(character))),
  )
}

fn identifier() -> Parser<String> {
  // TODO does not obey grammar

  // note: does not consume trailing whitespace
  parse::many1(
    Parser::expected(vec![])
      .or_else(|_| parse::digit(10))
      .or_else(|_| parse::alphabetic())
      .or_else(|_| parse::char('_').map(|_| '_')),
  )
  .map(|chars| chars.iter().collect::<String>())
  .name(format!("preprocessor identifier"))
}

fn include_directive_filename() -> Parser<String> {
  Parser::expected(vec![])
    .or_else(|_| {
      parse::char('"')
        .info("to begin a quoted filename")
        .and_then(|_| parse::many(parse::char_not('"').info("to continue a quoted filename")))
        .and_then(|chars| {
          parse::char('"')
            .info("to end a quoted filename")
            .map(|_| chars)
        })
        .and_then(|chars| parse::eof().map(|_| chars))
    })
    .or_else(|_| {
      parse::char('<')
        .info("to begin a bracketed filename")
        .and_then(|_| parse::many(parse::char_not('>').info("to continue a bracketed filename")))
        .and_then(|chars| {
          parse::char('>')
            .info("to end a bracketed filename")
            .map(|_| chars)
        })
        .and_then(|chars| parse::eof().map(|_| chars))
    })
    .map(|chars| chars.into_iter().collect::<String>())
}
