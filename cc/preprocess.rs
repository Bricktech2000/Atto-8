use crate::*;
use parse::Parser;
use std::collections::HashMap;
use std::rc::Rc;

const DUNDER_FILE: &str = "__FILE__";
const DUNDER_LINE: &str = "__LINE__";

pub fn preprocess(
  file: File,
  defines: &mut HashMap<String, TextLine>,
  errors: &mut Vec<(Pos, Error)>,
  scope: Option<&str>,
) -> String {
  // remove comments and resolve includes and defines

  let preprocessor = Parser::error(Error(format!("")))
    .or_else(|_| preprocess::include_directive())
    .or_else(|_| preprocess::define_directive())
    .or_else(|_| preprocess::undef_directive())
    .or_else(|_| preprocess::pragma_directive())
    .or_else(|_| preprocess::error_directive())
    .or_else(|_| preprocess::null_directive())
    .or_else(|_| preprocess::text_line_directive())
    .or_else(|_| parse::eof().map(|_| Directive::EOF));

  defines.insert("__STDC_NO_ATOMICS__".to_string(), vec![Ok(1.to_string())]);
  defines.insert("__STDC_NO_COMPLEX__".to_string(), vec![Ok(1.to_string())]);
  defines.insert("__STDC_NO_THREADS__".to_string(), vec![Ok(1.to_string())]);
  defines.insert("__STDC_NO_VLA__".to_string(), vec![Ok(1.to_string())]);

  let last_file = defines.get(DUNDER_FILE).unwrap_or(&vec![]).clone();
  let current_file = vec![Err('"'), Ok(file.0.clone()), Err('"')];
  defines.insert(DUNDER_FILE.to_string(), current_file); // TODO escape quotes in filename
  defines.insert(DUNDER_LINE.to_string(), vec![Ok(0.to_string())]);

  let source = std::fs::read_to_string(&file.0).unwrap_or_else(|_| {
    errors.push((
      Pos(scope.unwrap_or("[bootstrap]").to_string(), 0),
      Error(format!("Unable to read file `{}`", file)),
    ));
    format!("")
  });

  let mut preprocessed = "".to_string();
  let mut source = source
    .replace("\\\n", "") // line continuation
    .split("\n")
    .map(|line| line.split("//").next().unwrap_or(line)) // line comments
    .map(|line| line.to_owned() + "\n")
    .collect::<String>()
    .split("*/")
    .map(|item| item.split("/*").next().unwrap_or(item)) // block comments
    .collect::<String>();

  let preprocessed = loop {
    (preprocessed, source) = match preprocessor.0(&source) {
      Ok((Directive::Include(filename), input)) => (
        preprocessed + &preprocess_include_directive(filename, &file, defines, errors),
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
        preprocessed + &preprocess_pragma_directive(arguments, defines, errors) + "\n",
        input,
      ),

      Ok((Directive::Error(message), input)) => (
        preprocessed + &preprocess_error_directive(message, &file, defines, errors) + "\n",
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

      Err(error) => {
        errors.push((
          Pos(format!("{}", file), 0),
          Error(format!("Could not preprocess: {}", error)),
        ));
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
  errors: &mut Vec<(Pos, Error)>,
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
  _errors: &mut Vec<(Pos, Error)>,
) -> String {
  // silently ignore unsupported pragmas as per standard
  "".to_string()
}

fn preprocess_error_directive(
  message: TextLine,
  file: &File,
  defines: &mut HashMap<String, TextLine>,
  errors: &mut Vec<(Pos, Error)>,
) -> String {
  let message = preprocess_text_line_directive(message.clone(), defines, errors);

  errors.push((
    Pos(format!("{}", file), 0),
    Error(format!("Error directive: {}", message)),
  ));

  "".to_string()
}

fn preprocess_include_directive(
  filename: TextLine,
  file: &File,
  defines: &mut HashMap<String, TextLine>,
  errors: &mut Vec<(Pos, Error)>,
) -> String {
  // resolve defines in include directive and preprocess included file

  use std::path::Path;
  let filename = preprocess_text_line_directive(filename, defines, errors);

  match preprocess::include_directive_filename().0(&filename) {
    Ok((filename, trailing)) => match &trailing[..] {
      "" => preprocess(
        File(
          Path::new(&file.0)
            .parent()
            .unwrap()
            .join(filename)
            .to_str()
            .unwrap()
            .to_string(),
        ),
        defines,
        errors,
        Some(&format!("{}", file)),
      ),
      _ => {
        errors.push((
          Pos(format!("{}", file), 0),
          Error(format!(
            "Trailing characters in include directive filename: `{}`",
            filename
          )),
        ));
        format!("")
      }
    },

    Err(error) => {
      errors.push((
        Pos(format!("{}", file), 0),
        Error(format!("Could not parse: {}", error)),
      ));
      format!("")
    }
  }
}

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

type TextLine = Vec<Result<String, char>>;

fn include_directive() -> Parser<Directive> {
  // TODO does not obey grammar
  Parser::r#return(())
    .and_then(|_| preprocess::whitespaces_char('#'))
    .and_then(|_| preprocess::whitespaces_string("include"))
    .and_then(|_| preprocess::text_line())
    .and_then(|filename| {
      preprocess::whitespaces_char('\n').map(move |_| Directive::Include(filename))
    })
    .meta(format!("Include Directive"))
}

fn define_directive() -> Parser<Directive> {
  // TODO does not obey grammar
  Parser::r#return(())
    .and_then(|_| preprocess::whitespaces_char('#'))
    .and_then(|_| preprocess::whitespaces_string("define"))
    .and_then(|_| parse::identifier())
    .and_then(|identifier| {
      let identifier2 = identifier.clone();
      preprocess::text_line()
        .and_then(|replacement_list| {
          preprocess::whitespaces_char('\n')
            .map(move |_| Directive::Define(identifier, replacement_list))
        })
        .or_else(|_| Parser::r#return(Directive::Define(identifier2, vec![])))
    })
    .meta(format!("Define Directive"))
}

fn undef_directive() -> Parser<Directive> {
  // TODO does not obey grammar
  Parser::r#return(())
    .and_then(|_| preprocess::whitespaces_char('#'))
    .and_then(|_| preprocess::whitespaces_string("undef"))
    .and_then(|_| parse::identifier())
    .and_then(|identifier| {
      preprocess::whitespaces_char('\n').map(move |_| Directive::Undef(identifier))
    })
    .meta(format!("Undef Directive"))
}

fn pragma_directive() -> Parser<Directive> {
  // TODO does not obey grammar
  Parser::r#return(())
    .and_then(|_| preprocess::whitespaces_char('#'))
    .and_then(|_| preprocess::whitespaces_string("pragma"))
    .and_then(|_| preprocess::text_line())
    .and_then(|arguments| {
      preprocess::whitespaces_char('\n').map(move |_| Directive::Pragma(arguments))
    })
    .meta(format!("Pragma Directive"))
}

fn error_directive() -> Parser<Directive> {
  // TODO does not obey grammar
  Parser::r#return(())
    .and_then(|_| preprocess::whitespaces_char('#'))
    .and_then(|_| preprocess::whitespaces_string("error"))
    .and_then(|_| preprocess::text_line())
    .and_then(|message| preprocess::whitespaces_char('\n').map(move |_| Directive::Error(message)))
    .meta(format!("Error Directive"))
}

fn null_directive() -> Parser<Directive> {
  // TODO does not obey grammar
  Parser::r#return(())
    .and_then(|_| preprocess::whitespaces_char('#'))
    .and_then(|_| preprocess::whitespaces_char('\n').map(|_| Directive::Null))
    .meta(format!("Null Directive"))
}

fn text_line_directive() -> Parser<Directive> {
  // TODO does not obey grammar
  preprocess::text_line()
    .and_then(|text_line| {
      // if first non-whitespace character is `#`, assume it was a misparsed directive
      // and error out here in the preprocessor rather than in later stages
      Parser(Rc::new(move |input: &str| {
        match text_line
          .iter()
          .find(|item| !item.as_ref().err().map_or(false, |c| c.is_whitespace()))
        {
          Some(Err('#')) => Err(Error(format!("got directive"))),
          _ => Ok((text_line.clone(), input.to_string())),
        }
      }))
    })
    .and_then(|text_line| {
      preprocess::whitespaces_char('\n').map(move |_| Directive::TextLine(text_line))
    })
    .meta(format!("Text Line Directive"))
}

fn text_line() -> Parser<TextLine> {
  // TODO does not obey grammar
  parse::many(preprocess::whitespace())
    .and_then(|_| {
      parse::many(
        Parser::error(Error(format!("")))
          .or_else(|_| preprocess::identifier().map(|identifier| Ok(identifier)))
          .or_else(|_| preprocess::non_newline().map(|character| Err(character))),
      )
    })
    .meta(format!("Text Line"))
}

fn identifier() -> Parser<String> {
  // TODO does not obey grammar
  parse::many1(
    Parser::error(Error(format!("")))
      .or_else(|_| parse::digit(10))
      .or_else(|_| parse::alphabetic())
      .or_else(|_| parse::char('_').map(|_| '_')),
  )
  .map(|chars| chars.iter().collect::<String>())
  .meta(format!("Identifier"))
}

fn non_newline() -> Parser<char> {
  parse::satisfy(|c| c != '\n').meta(format!("Non-Newline"))
}

fn whitespace() -> Parser<char> {
  parse::satisfy(|c| c.is_whitespace() && c != '\n').meta(format!("Whitespace"))
}

#[allow(dead_code)]
pub fn whitespaces_eof() -> Parser<()> {
  parse::many(preprocess::whitespace()).and_then(|_| parse::eof())
}

pub fn whitespaces_char(char: char) -> Parser<()> {
  parse::many(preprocess::whitespace()).and_then(move |_| parse::char(char))
}

pub fn whitespaces_string(string: &'static str) -> Parser<()> {
  parse::many(preprocess::whitespace()).and_then(move |_| parse::string(string))
}

fn include_directive_filename() -> Parser<String> {
  Parser::error(Error(format!("")))
    .or_else(|_| {
      parse::char('"')
        .and_then(|_| parse::many(parse::satisfy(|c| c != '"')))
        .and_then(|chars| parse::char('"').map(|_| chars))
    })
    .or_else(|_| {
      parse::char('<')
        .and_then(|_| parse::many(parse::satisfy(|c| c != '>')))
        .and_then(|chars| parse::char('>').map(|_| chars))
    })
    .map(|chars| chars.into_iter().collect::<String>())
    .meta(format!("Include Directive Filename"))
}
