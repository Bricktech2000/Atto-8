use crate::*;
use std::rc::Rc;

// utilities

fn psi<T, U, V, F: FnOnce(T) -> U + Clone + 'static, G: FnOnce(U, U) -> V + Clone + 'static>(
  f: F,
  g: G,
) -> Rc<dyn Fn(T, T) -> V> {
  Rc::new(move |x, y| g.clone()(f.clone()(x), f.clone()(y)))
}

fn b<T, U, V, F: FnOnce(T) -> U + Clone + 'static, G: FnOnce(U) -> V + Clone + 'static>(
  f: F,
  g: G,
) -> Rc<dyn Fn(T) -> V> {
  Rc::new(move |x| g.clone()(f.clone()(x)))
}

fn id<T>() -> Rc<dyn Fn(T) -> T> {
  Rc::new(|x| x)
}

#[derive(Clone)]
pub struct Parser<T: Clone + 'static>(pub Rc<dyn Fn(&str) -> ParseResult<T>>);
pub type Expecteds = Vec<String>;
pub enum ParseResult<T> {
  Ok((T, String)),                        // parser succeeded without ever failing
  Err((Expecteds, String)),               // all parsers failed
  Both((T, String), (Expecteds, String)), // parser succeeded but failed with longer match
}

fn merge_expecteds(
  expecteds1: (Expecteds, String),
  expecteds2: (Expecteds, String),
) -> (Expecteds, String) {
  use std::cmp::Ordering;
  use std::hash::Hash;

  fn dedup<T: Clone + Hash + Eq>(expecteds: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    use std::collections::HashSet;
    let mut uniques = HashSet::new();
    expecteds.filter(move |expected| uniques.insert(expected.clone()))
  }

  match expecteds1.1.len().cmp(&expecteds2.1.len()) {
    // prefer the failed parse that has consumed the most input. otherwise, concatenate
    // their diagnostics and deduplicate
    Ordering::Less => expecteds1,
    Ordering::Greater => expecteds2,
    Ordering::Equal => (
      dedup(std::iter::empty().chain(expecteds1.0).chain(expecteds2.0)).collect(),
      expecteds1.1,
    ),
  }
}

// similar to `Result`, but reaches in to keep track of the longest failed parse
impl<T> ParseResult<T> {
  pub fn and_then<
    U: Clone + 'static,
    F: FnOnce((T, String)) -> ParseResult<U> + Clone + 'static,
  >(
    self,
    f: F,
  ) -> ParseResult<U> {
    match self {
      ParseResult::Ok(r#match) => f(r#match),
      ParseResult::Err(expecteds) => ParseResult::Err(expecteds),
      ParseResult::Both(r#match, expecteds) => match f(r#match) {
        ParseResult::Ok(f_match) => ParseResult::Both(f_match, expecteds),
        ParseResult::Err(f_expecteds) => ParseResult::Err(merge_expecteds(expecteds, f_expecteds)),
        ParseResult::Both(f_match, f_expecteds) => {
          ParseResult::Both(f_match, merge_expecteds(expecteds, f_expecteds))
        }
      },
    }
  }

  pub fn or_else<F: FnOnce((Expecteds, String)) -> ParseResult<T> + Clone + 'static>(
    self,
    f: F,
  ) -> ParseResult<T> {
    match self {
      ParseResult::Ok(r#match) => ParseResult::Ok(r#match),
      ParseResult::Err(expecteds) => match f(expecteds.clone()) {
        ParseResult::Ok(f_match) => ParseResult::Both(f_match, expecteds),
        ParseResult::Err(f_expecteds) => ParseResult::Err(merge_expecteds(expecteds, f_expecteds)),
        ParseResult::Both(f_match, f_expecteds) => {
          ParseResult::Both(f_match, merge_expecteds(expecteds, f_expecteds))
        }
      },
      ParseResult::Both(r#match, expecteds) => ParseResult::Both(r#match, expecteds),
    }
  }

  pub fn map<U: Clone + 'static, F: FnOnce((T, String)) -> (U, String) + Clone + 'static>(
    self,
    f: F,
  ) -> ParseResult<U> {
    match self {
      ParseResult::Ok(r#match) => ParseResult::Ok(f(r#match)),
      ParseResult::Err(expecteds) => ParseResult::Err(expecteds),
      ParseResult::Both(r#match, expecteds) => ParseResult::Both(f(r#match), expecteds),
    }
  }

  pub fn map_err<F: FnOnce((Expecteds, String)) -> (Expecteds, String) + Clone + 'static>(
    self,
    f: F,
  ) -> ParseResult<T> {
    match self {
      ParseResult::Ok(r#match) => ParseResult::Ok(r#match),
      ParseResult::Err(expecteds) => ParseResult::Err(f(expecteds)),
      ParseResult::Both(r#match, expecteds) => ParseResult::Both(r#match, f(expecteds)),
    }
  }

  pub fn into_result(self) -> Result<(T, String), (Expecteds, String)> {
    match self {
      ParseResult::Ok(r#match) => Ok(r#match),
      ParseResult::Err(expecteds) => Err(expecteds),
      ParseResult::Both(r#match, _expecteds) => Ok(r#match),
    }
  }
}

pub fn format_expecteds((expecteds, input): (Expecteds, String)) -> String {
  let expecteds = match expecteds.len() {
    0 => panic!("No expecteds"),
    1 => format!("{}", expecteds[0]),
    _ => format!(
      "{}, or {}",
      expecteds[..expecteds.len() - 1].join(", "),
      expecteds[expecteds.len() - 1]
    ),
  };

  let got = match input.len() {
    0 => "end of input".to_string(),
    // TODO uses debug formatting
    0..=16 => format!("{:?}", input),
    _ => format!("{:?}...", &input[0..16]),
  };

  format!("Expected {} (got {})", expecteds, got)
}

impl<T: Clone + 'static> Parser<T> {
  pub fn parse(&self, input: &str) -> Result<T, String> {
    match self.0(&input).into_result() {
      Ok((r#match, input)) => match &input[..] {
        "" => Ok(r#match),
        _ => panic!("Input not fully parsed"), // parser must be exhaustive
      },
      Err(expecteds) => Err(format_expecteds(expecteds)),
    }
  }

  pub fn and_then<U: Clone + 'static, F: FnOnce(T) -> Parser<U> + Clone + 'static>(
    self,
    f: F,
  ) -> Parser<U> {
    Parser(Rc::new(move |input: &str| {
      let f = f.clone();
      self.0(input).and_then(move |(r#match, input)| f(r#match).0(&input))
    }))
  }

  pub fn or_else<F: FnOnce(Expecteds) -> Parser<T> + Clone + 'static>(self, f: F) -> Parser<T> {
    Parser(Rc::new(move |input: &str| {
      let f = f.clone();
      let input = input.to_string();
      self.0(&input).or_else(move |(expecteds, _)| f(expecteds).0(&input))
    }))
  }

  pub fn map<U: Clone + 'static, F: FnOnce(T) -> U + Clone + 'static>(self, f: F) -> Parser<U> {
    Parser(Rc::new(move |input: &str| {
      let f = f.clone();
      self.0(input).map(move |(r#match, input)| (f(r#match), input))
    }))
  }

  pub fn map_err<F: FnOnce(Expecteds) -> Expecteds + Clone + 'static>(self, f: F) -> Parser<T> {
    Parser(Rc::new(move |input: &str| {
      let f = f.clone();
      self.0(input).map_err(move |(expecteds, input)| (f(expecteds), input))
    }))
  }

  pub fn pure(r#match: T) -> Parser<T> {
    Parser(Rc::new(move |input: &str| {
      ParseResult::Ok((r#match.clone(), input.to_string()))
    }))
  }

  pub fn expected(expecteds: Expecteds) -> Parser<T> {
    Parser(Rc::new(move |input: &str| {
      ParseResult::Err((expecteds.clone(), input.to_string()))
    }))
  }
}

// elementary parsers

pub fn any() -> Parser<char> {
  Parser(Rc::new(|input: &str| match &input[..] {
    "" => ParseResult::Err((vec![format!("any character")], input.to_string())),
    _ => ParseResult::Ok((input.chars().next().unwrap(), input[1..].to_string())),
  }))
}

pub fn eof() -> Parser<()> {
  Parser(Rc::new(|input: &str| match &input[..] {
    "" => ParseResult::Ok(((), input.to_string())),
    _ => ParseResult::Err((vec![format!("end of input")], input.to_string())),
  }))
}

pub fn satisfy<F: Fn(char) -> bool + Clone + 'static>(predicate: F) -> Parser<char> {
  parse::any().and_then(|char| {
    Parser(Rc::new(move |input: &str| match predicate(char) {
      true => ParseResult::Ok((char, input.to_string())),
      false => ParseResult::Err((vec![], char.to_string() + input)),
    }))
  })
}

pub fn char(char: char) -> Parser<()> {
  parse::group(
    // TODO uses debug formatting
    format!("{:?}", char),
    parse::satisfy(move |c| c == char).map(|_| ()),
  )
}

pub fn string(string: &'static str) -> Parser<()> {
  parse::group(
    // TODO uses debug formatting
    format!("{:?}", string),
    string
      .chars()
      .map(|char| parse::char(char).map(|_| ()))
      .reduce(|acc, parser| acc.and_then(|_| parser))
      .unwrap(),
  )
}

pub fn group<T: Clone + 'static>(name: String, parser: Parser<T>) -> Parser<T> {
  // try `parser`. if it fails without consuming any input, error out with `name` instead.
  // that is, if we tried to parse `X` and failed without ever consuming any input, we should
  // report that we expected `X` and not that we expected the specifics of how to parse `X`
  Parser(Rc::new(move |input: &str| match parser.0(input) {
    ParseResult::Ok(r#match) => ParseResult::Ok(r#match),
    ParseResult::Err((_expecteds, new_input)) if input == new_input => {
      ParseResult::Err((vec![name.clone()], input.to_string()))
    }
    ParseResult::Err(expecteds) => ParseResult::Err(expecteds),
    ParseResult::Both(r#match, expecteds) => ParseResult::Both(r#match, expecteds),
  }))
}

pub fn digit(radix: u32) -> Parser<char> {
  parse::group(
    match radix {
      2 => format!("binary digit"),
      8 => format!("octal digit"),
      10 => format!("decimal digit"),
      16 => format!("hexadecimal digit"),
      _ => format!("base-{} digit", radix),
    },
    parse::satisfy(move |c| c.is_digit(radix)),
  )
}

pub fn alphabetic() -> Parser<char> {
  parse::group(
    format!("alphabetic character"),
    parse::satisfy(|c| c.is_alphabetic()),
  )
}

pub fn whitespace() -> Parser<char> {
  parse::group(
    format!("whitespace character"),
    parse::satisfy(|c| c.is_whitespace()),
  )
}

pub fn whitespaces_group<T: Clone + 'static>(expected: String, parser: Parser<T>) -> Parser<T> {
  parse::many(parse::whitespace()).and_then(|_| parse::group(expected, parser))
}

pub fn whitespaces_eof() -> Parser<()> {
  parse::many(parse::whitespace()).and_then(|_| parse::eof())
}

pub fn whitespaces_char(char: char) -> Parser<()> {
  parse::many(parse::whitespace()).and_then(move |_| parse::char(char))
}

pub fn whitespaces_string(string: &'static str) -> Parser<()> {
  parse::many(parse::whitespace()).and_then(move |_| parse::string(string))
}

// parser combinators

pub fn maybe<T: Clone + 'static>(parser: Parser<T>) -> Parser<Option<T>> {
  parser
    .clone()
    .map(|r#match| Some(r#match))
    .or_else(|_| Parser::pure(None))
}

pub fn many1<T: Clone + 'static>(parser: Parser<T>) -> Parser<Vec<T>> {
  parser
    .clone()
    .and_then(|first| parse::many(parser).map(|rest| std::iter::once(first).chain(rest).collect()))
}

pub fn many<T: Clone + 'static>(parser: Parser<T>) -> Parser<Vec<T>> {
  // causes occasional stack overflow
  parse::many1(parser).or_else(|_| Parser::pure(vec![]))
}

#[allow(dead_code)]
pub fn many1_and_then<T: Clone + 'static, U: Clone + 'static>(
  parser: Parser<T>,
  r#final: Parser<U>,
) -> Parser<(Vec<T>, U)> {
  parse::many1(parser.clone()).and_then(move |matches| {
    parser
      // include `parser`'s error message if `final` fails
      .map(|_| unreachable!())
      .or_else(move |_| r#final.clone())
      .map(move |r#final| (matches, r#final))
  })
}

pub fn sepby1<T: Clone + 'static>(parser: Parser<T>, separator: Parser<()>) -> Parser<Vec<T>> {
  parser.clone().and_then(|first| {
    parse::many(separator.and_then(|_| parser))
      .map(|rest| std::iter::once(first).chain(rest).collect())
  })
}

pub fn sepby<T: Clone + 'static>(parser: Parser<T>, separator: Parser<()>) -> Parser<Vec<T>> {
  parse::sepby1(parser, separator).or_else(|_| Parser::pure(vec![]))
}

pub fn unop<T: Clone + 'static>(
  parser: Parser<T>,
  operator: Parser<Rc<dyn Fn(T) -> T>>,
) -> Parser<T> {
  operator
    .map_err(|_| vec![format!("unary operator")])
    .and_then(|constructor| parser.map(move |operand| constructor(operand)))
}

pub fn binop<T: Clone + 'static>(
  parser: Parser<T>,
  operator: Parser<Rc<dyn Fn(T, T) -> T>>,
) -> Parser<T> {
  parser.clone().and_then(|first| {
    parse::many(
      operator
        .map_err(|_| vec![format!("binary operator")])
        .and_then(|constructor| parser.map(|second| (constructor, second))),
    )
    .map(|rest| {
      rest
        .into_iter()
        .fold(first, |acc, (constructor, second)| constructor(acc, second)) // left-associative
    })
  })
}

// C99 grammar

pub fn parse(input: String, errors: &mut impl Extend<(Pos, Error)>) -> Program {
  parse::translation_unit()
    .parse(&input)
    .unwrap_or_else(|error| {
      errors.extend([(Pos(File("[parse]".to_string()), 0, 0), Error(error))]);
      Program(vec![])
    })
}

fn translation_unit() -> Parser<Program> {
  parse::group(
    format!("translation unit"),
    parse::many(
      Parser::expected(vec![])
        .or_else(|_| parse::function_declaration_global())
        .or_else(|_| parse::function_definition_global())
        .or_else(|_| parse::assembly_global()),
    )
    .and_then(|globals| parse::whitespaces_eof().map(move |_| globals))
    .map(|globals| Program(globals)),
  )
}

fn function_declaration_global() -> Parser<Global> {
  // TODO does not obey grammar
  parse::whitespaces_group(
    format!("function declaration"),
    Parser::pure(())
      .and_then(|_| parse::maybe(parse::whitespaces_string("inline")))
      .and_then(|is_inline| {
        parse::type_name().and_then(move |type_name| {
          parse::identifier().and_then(move |identifier| {
            Parser::pure(())
              .and_then(|_| {
                parse::whitespaces_char('(')
                  .map_err(|_| vec![format!("'(' to begin parameter list")])
              })
              .and_then(|_| parse::parameter_list())
              .and_then(move |parameters| {
                parse::maybe(
                  parse::whitespaces_char(',')
                    .map_err(|_| vec![format!("',' then \"...\" for variadic parameter")])
                    .and_then(|_| parse::whitespaces_string("...")),
                )
                .and_then(move |is_variadic| {
                  Parser::pure(())
                    .and_then(|_| {
                      parse::whitespaces_char(')')
                        .map_err(|_| vec![format!("')' to end parameter list")])
                    })
                    .and_then(|_| {
                      parse::whitespaces_char(';')
                        .map_err(|_| vec![format!("';' to end declaration")])
                    })
                    .map(move |_| {
                      Global::FunctionDeclaration(
                        is_inline.is_some(),
                        Object(type_name, identifier),
                        parameters,
                        is_variadic.is_some(),
                      )
                    })
                })
              })
          })
        })
      }),
  )
}

fn function_definition_global() -> Parser<Global> {
  // TODO does not obey grammar
  parse::whitespaces_group(
    format!("function definition"),
    Parser::pure(())
      .and_then(|_| parse::maybe(parse::whitespaces_string("inline")))
      .and_then(|is_inline| {
        parse::type_name().and_then(move |type_name| {
          parse::identifier().and_then(move |identifier| {
            Parser::pure(())
              .and_then(|_| {
                parse::whitespaces_char('(')
                  .map_err(|_| vec![format!("'(' to begin parameter list")])
              })
              .and_then(|_| parse::parameter_list())
              .and_then(move |parameters| {
                parse::maybe(
                  parse::whitespaces_char(',')
                    .map_err(|_| vec![format!("',' then \"...\" for variadic parameter")])
                    .and_then(|_| parse::whitespaces_string("...")),
                )
                .and_then(move |is_variadic| {
                  Parser::pure(())
                    .and_then(|_| {
                      parse::whitespaces_char(')')
                        .map_err(|_| vec![format!("')' to end parameter list")])
                    })
                    .and_then(|_| parse::statement())
                    .map(move |statement| {
                      Global::FunctionDefinition(
                        is_inline.is_some(),
                        Object(type_name, identifier),
                        parameters,
                        is_variadic.is_some(),
                        statement,
                      )
                    })
                })
              })
          })
        })
      }),
  )
}

fn parameter_list() -> Parser<Vec<Object>> {
  // TODO does not obey grammar
  parse::whitespaces_group(
    format!("parameter list"),
    parse::sepby(
      parse::type_name().and_then(|type_name| {
        parse::identifier()
          .or_else(|_| Parser::pure("".to_string()))
          .map(|identifier| Object(type_name, identifier))
      }),
      parse::whitespaces_char(',').map_err(|_| vec![format!("',' to continue parameter list")]),
    ),
  )
}

fn type_name() -> Parser<Type> {
  // TODO does not obey grammar
  parse::whitespaces_group(
    format!("type name"),
    Parser::pure(())
      .and_then(|_| parse::maybe(parse::whitespaces_string("const")))
      .and_then(|_const| {
        Parser::expected(vec![])
          .or_else(|_| {
            parse::whitespaces_string("long long int")
              .or_else(|_| parse::whitespaces_string("long long"))
              .map(|_| Type::LongLong)
          })
          .or_else(|_| {
            parse::whitespaces_string("unsigned long long int")
              .or_else(|_| parse::whitespaces_string("unsigned long long"))
              .map(|_| Type::UnsignedLongLong)
          })
          .or_else(|_| {
            parse::whitespaces_string("long int")
              .or_else(|_| parse::whitespaces_string("long"))
              .map(|_| Type::Long)
          })
          .or_else(|_| {
            parse::whitespaces_string("unsigned long int")
              .or_else(|_| parse::whitespaces_string("unsigned long"))
              .map(|_| Type::UnsignedLong)
          })
          .or_else(|_| parse::whitespaces_string("int").map(|_| Type::Int))
          .or_else(|_| {
            parse::whitespaces_string("unsigned int")
              .or_else(|_| parse::whitespaces_string("unsigned"))
              .map(|_| Type::UnsignedInt)
          })
          .or_else(|_| {
            parse::whitespaces_string("short int")
              .or_else(|_| parse::whitespaces_string("short"))
              .map(|_| Type::Short)
          })
          .or_else(|_| {
            parse::whitespaces_string("unsigned short int")
              .or_else(|_| parse::whitespaces_string("unsigned short"))
              .map(|_| Type::UnsignedShort)
          })
          .or_else(|_| parse::whitespaces_string("char").map(|_| Type::Char))
          .or_else(|_| parse::whitespaces_string("bool").map(|_| Type::Bool))
          .or_else(|_| parse::whitespaces_string("void").map(|_| Type::Void))
      })
      // TODO implement proper pointer types
      .and_then(|r#type| {
        let type1 = r#type.clone();
        parse::whitespaces_char('*')
          .map(|_| Type::Pointer(Box::new(r#type)))
          .or_else(|_| Parser::pure(type1))
      }),
  )
}

fn assembly_global() -> Parser<Global> {
  parse::whitespaces_group(
    format!("assembly global"),
    parse::assembly_statement().map(|statement| match statement {
      Statement::Assembly(global_assembly) => Global::GlobalAssembly(global_assembly),
      _ => panic!("`assembly_statement` did not return `Statement::Assembly`"),
    }),
  )
}

fn compound_statement() -> Parser<Statement> {
  // TODO should be {<declaration>}* {<statement>}*
  parse::whitespaces_group(
    format!("compound statement"),
    Parser::pure(())
      .and_then(|_| parse::whitespaces_char('{').map_err(|_| vec![format!("'{{' to begin block")]))
      .and_then(|_| {
        parse::many(parse::statement()).and_then(|statements| {
          parse::whitespaces_char('}')
            .map_err(|_| vec![format!("'}}' to end block")])
            .map(move |_| statements)
        })
      })
      .map(|statements| Statement::Compound(statements)),
  )
}

fn statement() -> Parser<Statement> {
  // TODO cases missing
  parse::whitespaces_group(
    format!("statement"),
    Parser::expected(vec![])
      .or_else(|_| parse::jump_statement())
      .or_else(|_| parse::iteration_statement())
      .or_else(|_| parse::compound_statement())
      .or_else(|_| parse::selection_statement())
      .or_else(|_| parse::expression_statement())
      .or_else(|_| parse::assembly_statement()), // TODO does not obey grammar
  )
}

fn jump_statement() -> Parser<Statement> {
  // TODO cases missing
  Parser::pure(())
    .and_then(|_| parse::whitespaces_string("return"))
    .and_then(|_| {
      parse::maybe(parse::expression()).and_then(|expression| {
        parse::whitespaces_char(';')
          .map_err(|_| vec![format!("';' to end statement")])
          .map(|_| expression)
      })
    }) // TODO does not obey grammar
    .map(|expression| Statement::Return(expression))
}

fn selection_statement() -> Parser<Statement> {
  // TODO cases missing
  Parser::expected(vec![])
    .or_else(|_| parse::if_statement())
    .or_else(|_| parse::if_else_statement())
    .or_else(|_| parse::switch_statement())
}

fn if_statement() -> Parser<Statement> {
  Parser::pure(())
    .and_then(|_| parse::whitespaces_string("if"))
    .and_then(|_| parse::whitespaces_char('(').map_err(|_| vec![format!("'(' to begin condition")]))
    .and_then(|_| parse::expression())
    .and_then(|expression| {
      parse::whitespaces_char(')')
        .map_err(|_| vec![format!("')' to end condition")])
        .and_then(|_| parse::statement())
        .map(|statement| Statement::If(expression, Box::new(statement), None))
    })
}

fn if_else_statement() -> Parser<Statement> {
  Parser::pure(())
    .and_then(|_| parse::whitespaces_string("if"))
    .and_then(|_| parse::whitespaces_char('(').map_err(|_| vec![format!("'(' to begin condition")]))
    .and_then(|_| parse::expression())
    .and_then(|expression| {
      parse::whitespaces_char(')')
        .map_err(|_| vec![format!("')' to end condition")])
        .and_then(|_| parse::statement())
        .and_then(|statement1| {
          parse::whitespaces_string("else")
            .and_then(|_| parse::statement())
            .map(move |statement2| {
              Statement::If(expression, Box::new(statement1), Some(Box::new(statement2)))
            })
        })
    })
}

fn switch_statement() -> Parser<Statement> {
  // TODO switch statement
  Parser::expected(vec![])
}

fn iteration_statement() -> Parser<Statement> {
  // TODO cases missing
  Parser::expected(vec![])
    .or_else(|_| parse::while_statement())
    .or_else(|_| parse::do_while_statement())
    .or_else(|_| parse::for_statement())
}

fn while_statement() -> Parser<Statement> {
  Parser::pure(())
    .and_then(|_| parse::whitespaces_string("while"))
    .and_then(|_| parse::whitespaces_char('(').map_err(|_| vec![format!("'(' to begin condition")]))
    .and_then(|_| parse::expression())
    .and_then(|expression| {
      parse::whitespaces_char(')')
        .map_err(|_| vec![format!("')' to end condition")])
        .and_then(|_| parse::compound_statement())
        .map(|statements| Statement::While(expression, Box::new(statements)))
    })
}

fn do_while_statement() -> Parser<Statement> {
  // TODO do while statement
  Parser::expected(vec![])
}

fn for_statement() -> Parser<Statement> {
  // TODO for statement
  Parser::expected(vec![])
}

fn expression_statement() -> Parser<Statement> {
  // TODO cases missing
  Parser::pure(())
    .and_then(|_| parse::expression())
    .and_then(|expression| {
      parse::whitespaces_char(';')
        .map_err(|_| vec![format!("';' to end statement")])
        .map(|_| Statement::Expression(expression))
    })
}

fn assembly_statement() -> Parser<Statement> {
  Parser::pure(())
    .and_then(|_| parse::whitespaces_string("asm"))
    .and_then(|_| {
      parse::whitespaces_char('{').map_err(|_| vec![format!("'{{' to begin assembly statement")])
    })
    .and_then(|_| {
      parse::many(
        parse::satisfy(|c| c != '}')
          .map_err(|_| vec![format!("non-'}}' to continue assembly statement")]),
      )
    })
    .map(|chars| chars.iter().collect::<String>().trim().to_string())
    .and_then(|assembly| {
      parse::whitespaces_char('}')
        .map_err(|_| vec![format!("'}}' to end assembly statement")])
        .map(move |_| Statement::Assembly(assembly))
    })
}

fn expression() -> Parser<Expression> {
  parse::whitespaces_group(
    format!("expression"),
    parse::constant_expression(), // TODO does not obey grammar
  )
}

fn constant_expression() -> Parser<Expression> {
  parse::conditional_expression()
}

fn conditional_expression() -> Parser<Expression> {
  parse::logical_or_expression().and_then(|expression1| {
    let expression = expression1.clone();
    Parser::pure(())
      .and_then(|_| {
        parse::whitespaces_char('?').map_err(|_| vec![format!("'?' to begin ternary operator")])
      })
      .and_then(|_| parse::expression())
      .and_then(|expression2| {
        parse::whitespaces_char(':')
          .map_err(|_| vec![format!("':' then expression to end ternary operator")])
          .and_then(|_| parse::conditional_expression())
          .map(|expression3| {
            Expression::Conditional(
              Box::new(expression1),
              Box::new(expression2),
              Box::new(expression3),
            )
          })
      })
      .or_else(|_| Parser::pure(expression))
  })
}

fn logical_or_expression() -> Parser<Expression> {
  parse::binop(
    parse::logical_and_expression(),
    parse::whitespaces_string("||").map(|_| psi(Box::new, Expression::LogicalOr)),
  )
}

fn logical_and_expression() -> Parser<Expression> {
  parse::binop(
    parse::bitwise_inclusive_or_expression(),
    parse::whitespaces_string("&&").map(|_| psi(Box::new, Expression::LogicalAnd)),
  )
}

fn bitwise_inclusive_or_expression() -> Parser<Expression> {
  parse::binop(
    parse::bitwise_exclusive_or_expression(),
    parse::whitespaces_char('|').map(|_| psi(Box::new, Expression::BitwiseInclusiveOr)),
  )
}

fn bitwise_exclusive_or_expression() -> Parser<Expression> {
  parse::binop(
    parse::bitwise_and_expression(),
    parse::whitespaces_char('^').map(|_| psi(Box::new, Expression::BitwiseExclusiveOr)),
  )
}

fn bitwise_and_expression() -> Parser<Expression> {
  parse::binop(
    parse::equality_expression(),
    parse::whitespaces_char('&').map(|_| psi(Box::new, Expression::BitwiseAnd)),
  )
}

fn equality_expression() -> Parser<Expression> {
  parse::binop(
    parse::relational_expression(),
    Parser::expected(vec![])
      .or_else(|_| parse::whitespaces_string("==").map(|_| psi(Box::new, Expression::EqualTo)))
      .or_else(|_| parse::whitespaces_string("!=").map(|_| psi(Box::new, Expression::NotEqualTo))),
  )
}

fn relational_expression() -> Parser<Expression> {
  parse::binop(
    parse::shift_expression(),
    Parser::expected(vec![])
      .or_else(|_| {
        parse::whitespaces_string(">=").map(|_| psi(Box::new, Expression::GreaterThanOrEqualTo))
      })
      .or_else(|_| parse::whitespaces_char('>').map(|_| psi(Box::new, Expression::GreaterThan)))
      .or_else(|_| {
        parse::whitespaces_string("<=").map(|_| psi(Box::new, Expression::LessThanOrEqualTo))
      })
      .or_else(|_| parse::whitespaces_char('<').map(|_| psi(Box::new, Expression::LessThan))),
  )
}

fn shift_expression() -> Parser<Expression> {
  parse::binop(
    parse::additive_expression(),
    Parser::expected(vec![])
      .or_else(|_| parse::whitespaces_string("<<").map(|_| psi(Box::new, Expression::LeftShift)))
      .or_else(|_| parse::whitespaces_string(">>").map(|_| psi(Box::new, Expression::RightShift))),
  )
}

fn additive_expression() -> Parser<Expression> {
  parse::binop(
    parse::multiplicative_expression(),
    Parser::expected(vec![])
      .or_else(|_| parse::whitespaces_char('+').map(|_| psi(Box::new, Expression::Addition)))
      .or_else(|_| parse::whitespaces_char('-').map(|_| psi(Box::new, Expression::Subtraction))),
  )
}

fn multiplicative_expression() -> Parser<Expression> {
  parse::binop(
    parse::cast_expression(),
    Parser::expected(vec![])
      .or_else(|_| parse::whitespaces_char('*').map(|_| psi(Box::new, Expression::Multiplication)))
      .or_else(|_| parse::whitespaces_char('/').map(|_| psi(Box::new, Expression::Division)))
      .or_else(|_| parse::whitespaces_char('%').map(|_| psi(Box::new, Expression::Modulo))),
  )
}

fn cast_expression() -> Parser<Expression> {
  parse::whitespaces_char('(')
    .map_err(|_| vec![format!("'(' to begin cast")])
    .and_then(|_| parse::type_name())
    .and_then(|type_name| {
      parse::whitespaces_char(')')
        .map_err(|_| vec![format!("')' then expression to end cast")])
        .and_then(|_| parse::cast_expression())
        .map(|cast_expression| Expression::Cast(type_name, Box::new(cast_expression)))
    })
    .or_else(|_| parse::unary_expression())
}

fn unary_expression() -> Parser<Expression> {
  Parser::expected(vec![])
    .or_else(|_| {
      parse::unop(
        parse::unary_expression(),
        parse::whitespaces_char('-').map(|_| b(Box::new, Expression::Negation)),
      )
    })
    .or_else(|_| {
      parse::unop(
        parse::unary_expression(),
        parse::whitespaces_char('+').map(|_| id()),
      )
    })
    .or_else(|_| {
      parse::unop(
        parse::unary_expression(),
        parse::whitespaces_char('~').map(|_| b(Box::new, Expression::BitwiseComplement)),
      )
    })
    .or_else(|_| {
      parse::unop(
        parse::unary_expression(),
        parse::whitespaces_char('!').map(|_| b(Box::new, Expression::LogicalNegation)),
      )
    })
    .or_else(|_| {
      Parser::pure(())
        .and_then(|_| {
          parse::whitespaces_char('(').map_err(|_| vec![format!("'(' to begin expression")])
        })
        .and_then(|_| parse::expression()) // TODO does not obey grammar
        .and_then(|expression| {
          parse::whitespaces_char(')')
            .map_err(|_| vec![format!("')' to end expression")])
            .map(|_| expression)
        })
    })
    // TODO does not obey grammar
    .or_else(|_| parse::integer_constant())
    // TODO does not obey grammar
    .or_else(|_| parse::character_constant())
    // TODO does not obey grammar
    .or_else(|_| parse::string_literal())
    // TODO does not obey grammar
    .or_else(|_| parse::identifier().map(|identifier| Expression::Identifier(identifier)))
    // TODO does not obey grammar
    .and_then(|expression| {
      let expression1 = expression.clone();
      Parser::expected(vec![])
        .or_else(|_| {
          Parser::pure(())
            .and_then(|_| {
              parse::whitespaces_char('(').map_err(|_| vec![format!("'(' to begin argument list")])
            })
            .and_then(|_| {
              parse::sepby(
                parse::expression(),
                parse::whitespaces_char(',')
                  .map_err(|_| vec![format!("',' to continue argument list")]),
              )
            })
            .and_then(|arguments| {
              parse::whitespaces_char(')')
                .map_err(|_| vec![format!("')' to end argument list")])
                .map(|_| Expression::FunctionCall(Box::new(expression1), arguments))
            })
        })
        .or_else(|_| Parser::pure(expression))
    })
}

fn identifier() -> Parser<String> {
  parse::whitespaces_group(
    format!("identifier"),
    parse::many(parse::whitespace())
      .and_then(|_| parse::alphabetic().or_else(|_| parse::char('_').map(|_| '_')))
      .and_then(|first| {
        parse::many(
          Parser::expected(vec![])
            .or_else(|_| parse::digit(10))
            .or_else(|_| parse::alphabetic())
            .or_else(|_| parse::char('_').map(|_| '_')),
        )
        .map(move |rest| std::iter::once(first).chain(rest).collect())
      }),
  )
}

fn integer_constant() -> Parser<Expression> {
  // TODO does not obey grammar
  parse::whitespaces_group(
    format!("integer constant"),
    Parser::expected(vec![])
      .or_else(|_| {
        parse::whitespaces_string("0x")
          .and_then(|_| parse::many1(parse::digit(0x10)))
          .map(|digits| digits.into_iter().collect::<String>())
          .map(|digits| u8::from_str_radix(&digits, 0x10))
      })
      .or_else(|_| {
        parse::whitespaces_string("0b")
          .and_then(|_| parse::many1(parse::digit(0b10)))
          .map(|digits| digits.into_iter().collect::<String>())
          .map(|digits| u8::from_str_radix(&digits, 0b10))
      })
      .or_else(|_| {
        parse::many(parse::whitespace())
          .and_then(|_| parse::many1(parse::digit(10)))
          .map(|digits| digits.into_iter().collect::<String>())
          .map(|digits| u8::from_str_radix(&digits, 10))
      })
      .map(|digits| digits.unwrap_or_else(|_| panic!("Could not parse integer constant")))
      .map(|value| Expression::IntegerConstant(value)),
  )
}

fn character_constant() -> Parser<Expression> {
  // TODO currently only parsing <simple-escape-sequence>s
  parse::whitespaces_group(
    format!("character constant"),
    Parser::pure(())
      .and_then(|_| parse::char('\''))
      .and_then(|_| {
        parse::satisfy(|c| !"\'\\\n".contains(c))
          .map_err(|_| vec![format!("character constant character")])
          .or_else(|_| parse::escape_sequence())
      })
      .and_then(|char| parse::char('\'').map(move |_| Expression::CharacterConstant(char))),
  )
}

fn string_literal() -> Parser<Expression> {
  parse::whitespaces_group(
    format!("string literal"),
    Parser::pure(())
      .and_then(|_| parse::char('"'))
      .and_then(|_| {
        parse::many(
          parse::satisfy(|c| !"\"\\\n".contains(c))
            .map_err(|_| vec![format!("string literal character")])
            .or_else(|_| parse::escape_sequence()),
        )
        .map(|chars| chars.into_iter().collect::<String>())
      })
      .and_then(|string| parse::char('"').map(move |_| Expression::StringLiteral(string))),
  )
}

fn escape_sequence() -> Parser<char> {
  parse::group(
    format!("escape sequence"),
    Parser::expected(vec![])
      .or_else(|_| parse::string("\\\'").map(|_| '\''))
      .or_else(|_| parse::string("\\\"").map(|_| '\"'))
      .or_else(|_| parse::string("\\?").map(|_| '?'))
      .or_else(|_| parse::string("\\\\").map(|_| '\\'))
      .or_else(|_| parse::string("\\a").map(|_| '\x07'))
      .or_else(|_| parse::string("\\b").map(|_| '\x08'))
      .or_else(|_| parse::string("\\f").map(|_| '\x0C'))
      .or_else(|_| parse::string("\\n").map(|_| '\n'))
      .or_else(|_| parse::string("\\r").map(|_| '\r'))
      .or_else(|_| parse::string("\\t").map(|_| '\t'))
      .or_else(|_| parse::string("\\0").map(|_| '\0')), // TODO should be <octal-escape-sequence>
  )
}
