use crate::*;
use parse::Parser;
use std::collections::HashMap;

pub fn preprocess(file: File, defines: &mut HashMap<String, TextLine>) -> Result<String, Error> {
  // remove comments and resolve includes and defines

  let preprocessor = Parser::error(Error(format!("")))
    .or_else(|_| preprocess::include_directive())
    .or_else(|_| preprocess::define_directive())
    .or_else(|_| preprocess::text_line_directive())
    .or_else(|_| parse::eof().map(|_| Directive::EOF));

  let source = std::fs::read_to_string(&file.0).unwrap_or_else(|_| {
    panic!("Unable to read file `{}`", file.0);
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
        preprocessed + &preprocess_include_directive(text_line, &file, defines)?,
        input,
      ),

      Ok((Directive::Define(identifier, value), input)) => {
        defines.insert(identifier.clone(), value.clone());
        (preprocessed, input)
      }

      Ok((Directive::TextLine(text_line), input)) => (
        preprocessed + &preprocess_text_line_directive(text_line, defines)? + "\n",
        input,
      ),

      Ok((Directive::EOF, input)) => {
        break match &input[..] {
          "" => Ok(preprocessed),
          _ => panic!("Input not fully parsed"),
        };
      }

      Err(_) => unreachable!(),
    }
  }
}

fn preprocess_text_line_directive(
  text_line: TextLine,
  defines: &mut HashMap<String, TextLine>,
) -> Result<String, Error> {
  // resolve defines recursively in text line and return preprocessed text line

  let mut acc = "".to_string();

  for line_item in text_line.iter() {
    acc += &match line_item {
      Ok(identifier) => match defines.remove(identifier) {
        Some(text_line) => {
          // prevents infinite recursion
          let preprocessed = preprocess_text_line_directive(text_line.clone(), defines)?;
          defines.insert(identifier.clone(), text_line);
          preprocessed
        }
        None => identifier.clone(),
      },
      Err(char) => char.to_string(),
    }
  }

  Ok(acc)
}

fn preprocess_include_directive(
  text_line: TextLine,
  file: &File,
  defines: &mut HashMap<String, TextLine>,
) -> Result<String, Error> {
  // resolve defines in include directive and preprocess included file

  use std::path::Path;
  let text_line = preprocess_text_line_directive(text_line, defines)?;
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
      ),
      _ => Err(Error(format!(
        "Trailing characters in include directive filename: `{}`",
        text_line
      ))),
    },
    Err(e) => Err(e),
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
  parse::whitespaces_string("#include")
    .and_then(|_| parse::many1(preprocess::non_newline_whitespace()))
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
  parse::whitespaces_string("#define")
    .and_then(|_| parse::many1(preprocess::non_newline_whitespace()))
    .and_then(|_| parse::identifier())
    .and_then(|identifier| {
      let identifier2 = identifier.clone();
      parse::many1(preprocess::non_newline_whitespace())
        .and_then(|_| {
          preprocess::text_line_directive().map(|directive| match directive {
            Directive::TextLine(text_line) => Directive::Define(identifier, text_line),
            _ => panic!("`text_line` did not return `Directive::TextLine`"),
          })
        })
        .or_else(|_| Parser::return_(Directive::Define(identifier2, vec![])))
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
  .map(|(line_items, _)| Directive::TextLine(line_items))
  .meta(format!("Text Line Directive"))
}

fn identifier() -> Parser<String> {
  // TODO does not obey grammar
  parse::many1(
    Parser::error(Error(format!("")))
      .or_else(|_| parse::digit_10())
      .or_else(|_| parse::alphabetic())
      .or_else(|_| parse::satisfy(|c| c == '_')),
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
