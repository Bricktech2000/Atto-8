use crate::*;
use parse::Parser;
use std::collections::HashMap;
use std::rc::Rc;

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
    .or_else(|_| preprocess::text_line_directive())
    .or_else(|_| parse::eof().map(|_| Directive::EOF));

  let source = std::fs::read_to_string(&file.0).unwrap_or_else(|_| {
    errors.push((
      Pos(scope.unwrap_or("[bootstrap]").to_string(), 0),
      Error(format!("Unable to read file `{}`", file)),
    ));
    format!("")
  });

  let mut preprocessed = "".to_string();
  let mut source = source
    .lines()
    .map(|line| line.split("//").next().unwrap_or(line))
    .map(|line| line.to_string() + "\n")
    .collect::<String>();

  loop {
    (preprocessed, source) = match preprocessor.0(&source) {
      Ok((Directive::Include(text_line), input)) => (
        preprocessed + &preprocess_include_directive(text_line, &file, defines, errors),
        input,
      ),

      Ok((Directive::Define(identifier, value), input)) => {
        defines.insert(identifier.clone(), value.clone());
        (preprocessed, input)
      }

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
          Pos("[preprocess]".to_string(), 0),
          Error(format!("Could not preprocess: {}", error)),
        ));
        (preprocessed, "".to_string())
      }
    }
  }
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

fn preprocess_include_directive(
  text_line: TextLine,
  file: &File,
  defines: &mut HashMap<String, TextLine>,
  errors: &mut Vec<(Pos, Error)>,
) -> String {
  // resolve defines in include directive and preprocess included file

  use std::path::Path;
  let text_line = preprocess_text_line_directive(text_line, defines, errors);

  match preprocess::include_directive_filename().0(&text_line) {
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
          Pos(file.0.clone(), 0),
          Error(format!(
            "Trailing characters in include directive filename: `{}`",
            text_line
          )),
        ));
        format!("")
      }
    },

    Err(error) => {
      errors.push((
        Pos(file.0.clone(), 0),
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
  TextLine(TextLine),
  EOF,
}

type TextLine = Vec<Result<String, char>>;

fn include_directive() -> Parser<Directive> {
  // TODO does not obey grammar
  parse::whitespaces_char('#')
    .and_then(|_| parse::many(preprocess::non_newline_whitespace()))
    .and_then(|_| parse::string("include"))
    .and_then(|_| parse::many(preprocess::non_newline_whitespace()))
    .and_then(|_| {
      preprocess::text_line_directive().map(|directive| match directive {
        Directive::TextLine(text_line) => Directive::Include(text_line),
        _ => panic!("`text_line` did not return `Directive::TextLine`"),
      })
    })
    .meta(format!("Include Directive"))
}

fn define_directive() -> Parser<Directive> {
  // TODO does not obey grammar
  parse::whitespaces_char('#')
    .and_then(|_| parse::many(preprocess::non_newline_whitespace()))
    .and_then(|_| parse::string("define"))
    .and_then(|_| parse::many(preprocess::non_newline_whitespace()))
    .and_then(|_| parse::identifier())
    .and_then(|identifier| {
      let identifier2 = identifier.clone();
      parse::many(preprocess::non_newline_whitespace()).and_then(|_| {
        preprocess::text_line_directive()
          .map(|directive| match directive {
            Directive::TextLine(text_line) => Directive::Define(identifier, text_line),
            _ => panic!("`text_line` did not return `Directive::TextLine`"),
          })
          .or_else(|_| Parser::return_(Directive::Define(identifier2, vec![])))
      })
    })
    .meta(format!("Define Directive"))
}

fn text_line_directive() -> Parser<Directive> {
  // TODO does not obey grammar
  parse::many_and_then(
    Parser::error(Error(format!("")))
      .or_else(|_| preprocess::identifier().map(|identifier| Ok(identifier)))
      .or_else(|_| preprocess::non_newline().map(|character| Err(character))),
    parse::char('\n'),
  )
  .and_then(|(line_items, _)| {
    // if first non-whitespace character is `#`, assume it was a misparsed directive
    // and error out here in the preprocessor rather than in later stages
    Parser(Rc::new(move |input: &str| {
      match line_items
        .iter()
        .find(|item| !item.as_ref().err().map_or(false, |c| c.is_whitespace()))
      {
        Some(Err('#')) => Err(Error(format!("got preprocessor directive"))),
        _ => Ok((line_items.clone(), input.to_string())),
      }
    }))
  })
  .map(|line_items| Directive::TextLine(line_items))
  .meta(format!("Text Line Directive"))
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

fn non_newline_whitespace() -> Parser<char> {
  parse::satisfy(|c| c.is_whitespace() && c != '\n').meta(format!("Non-Newline Whitespace"))
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
