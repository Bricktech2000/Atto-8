use crate::*;
use parse::Parser;
use std::collections::HashMap;

pub fn preprocess(file: File, defines: &mut HashMap<String, String>) -> Result<String, Error> {
  // remove comments and resolve includes

  let mut preprocessed = "".to_string();
  let mut source = std::fs::read_to_string(&file.0).unwrap_or_else(|_| {
    panic!("Unable to read file `{}`", file.0);
  });

  let preprocessor = Parser::error(Error(format!("")))
    .or_else(|_| preprocess::comment_directive())
    .or_else(|_| preprocess::include_directive())
    .or_else(|_| preprocess::define_directive())
    .or_else(|_| preprocess::identifier_directive())
    .or_else(|_| parse::any().map(|c| Directive::Char(c)))
    .or_else(|_| parse::eof().map(|_| Directive::EOF));

  loop {
    (preprocessed, source) = match preprocessor.0(source.clone()) {
      Ok((Directive::Comment(_), input)) => (preprocessed, input),
      Ok((Directive::Include(filename), input)) => {
        use std::path::Path;
        // feed back into preprocessor
        (
          preprocessed,
          preprocess(
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
          )? + &input,
        )
      }
      Ok((Directive::Define(identifier, value), input)) => {
        defines.insert(identifier.clone(), value.clone());
        (preprocessed, input)
      }
      Ok((Directive::Identifier(identifier), input)) => {
        match defines.get(&identifier) {
          // feed back into preprocessor
          // TODO a `#define` resolving to itself will cause infinite recursion
          Some(value) => (preprocessed, value.clone() + &input),
          // do not feed back into preprocessor
          None => (preprocessed + &identifier, input),
        }
      }
      Ok((Directive::Char(c), input)) => {
        // do not feed back into preprocessor
        (preprocessed + &c.to_string(), input)
      }
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

#[derive(Clone, PartialEq, Debug)]
enum Directive {
  Comment(String),
  Include(String),
  Define(String, String),
  Identifier(String),
  Char(char),
  EOF,
}

fn comment_directive() -> Parser<Directive> {
  parse::whitespaces_string("//")
    .and_then(|_| parse::many(parse::satisfy(|c| c != '\n')))
    .map(|chars| chars.iter().collect::<String>())
    .map(|comment| Directive::Comment(comment))
}

fn include_directive() -> Parser<Directive> {
  // TODO does not obey grammar
  parse::whitespaces_string("#include")
    .and_then(|_| {
      Parser::error(Error(format!("")))
        .or_else(|_| {
          parse::whitespaces_char('"')
            .and_then(|_| parse::many(parse::satisfy(|c| c != '"')))
            .and_then(|chars| parse::char('"').map(|_| chars))
        })
        .or_else(|_| {
          parse::whitespaces_char('<')
            .and_then(|_| parse::many(parse::satisfy(|c| c != '>')))
            .and_then(|chars| parse::char('>').map(|_| chars))
        })
    })
    .map(|chars| chars.iter().collect::<String>())
    .map(|filename| Directive::Include(filename))
}

fn define_directive() -> Parser<Directive> {
  // TODO does not obey grammar
  parse::whitespaces_string("#define")
    .and_then(|_| parse::identifier())
    .and_then(|identifier| {
      parse::many(parse::whitespace()).and_then(|_| {
        parse::many(parse::satisfy(|c| c != '\n'))
          .map(|chars| chars.iter().collect::<String>())
          .map(|value| Directive::Define(identifier, value))
      })
    })
}

fn identifier_directive() -> Parser<Directive> {
  // TODO does not obey grammar
  parse::many1(
    Parser::error(Error(format!("")))
      .or_else(|_| parse::digit_10())
      .or_else(|_| parse::alphabetic())
      .or_else(|_| parse::satisfy(|c| c == '_')),
  )
  .map(|chars| chars.iter().collect::<String>())
  .map(|identifier| Directive::Identifier(identifier))
}
