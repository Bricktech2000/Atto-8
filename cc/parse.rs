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

  pub fn info(self, info: &'static str) -> Parser<T> {
    self.map_err(move |expecteds| {
      expecteds
        .into_iter()
        .map(|expected| format!("{} {}", expected, info))
        .collect()
    })
  }

  pub fn name(self, name: String) -> Parser<T> {
    // try `self`. if it fails without consuming any input, error out with `name` instead.
    // that is, if we tried to parse `X` and failed without ever consuming any input, we should
    // report that we expected `X` and not that we expected the specifics of how to parse `X`
    Parser(Rc::new(move |input: &str| match self.0(input) {
      ParseResult::Ok(r#match) => ParseResult::Ok(r#match),
      ParseResult::Err((_expecteds, new_input)) if input == new_input => {
        ParseResult::Err((vec![name.clone()], input.to_string()))
      }
      ParseResult::Err(expecteds) => ParseResult::Err(expecteds),
      ParseResult::Both(r#match, expecteds) => ParseResult::Both(r#match, expecteds),
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
  parse::satisfy(move |c| c == char)
    .map(|_| ())
    // TODO uses debug formatting
    .name(format!("{:?}", char))
}

pub fn string(string: &'static str) -> Parser<()> {
  string
    .chars()
    .map(|char| parse::char(char))
    .reduce(|acc, parser| acc.and_then(|_| parser))
    .unwrap()
    // TODO uses debug formatting
    .name(format!("{:?}", string))
}

pub fn char_not(char: char) -> Parser<char> {
  // TODO uses debug formatting
  parse::satisfy(move |c| c != char).name(format!("non-{:?}", char))
}

pub fn char_none_of(chars: &'static str) -> Parser<char> {
  parse::satisfy(move |c| !chars.contains(c)).name(format!("none of `{}`", chars))
}

pub fn digit(radix: u32) -> Parser<char> {
  parse::satisfy(move |c| c.is_digit(radix)).name(match radix {
    0b10 => format!("binary digit"),
    0o10 => format!("octal digit"),
    10 => format!("decimal digit"),
    0x10 => format!("hexadecimal digit"),
    _ => format!("base-{} digit", radix),
  })
}

pub fn alphabetic() -> Parser<char> {
  parse::satisfy(|c| c.is_alphabetic()).name(format!("alphabetic character"))
}

pub fn newline() -> Parser<()> {
  parse::char('\n').name(format!("newline character"))
}

pub fn whitespace() -> Parser<char> {
  parse::satisfy(|c| c.is_whitespace()).name(format!("whitespace character"))
}

pub fn ws<T: Clone + 'static>(parser: Parser<T>) -> Parser<T> {
  parser.and_then(|r#match| parse::many(parse::whitespace()).map(move |_| r#match))
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
    .name(format!("unary operator"))
    .and_then(|constructor| parser.map(move |operand| constructor(operand)))
}

pub fn binop<T: Clone + 'static>(
  parser: Parser<T>,
  operator: Parser<Rc<dyn Fn(T, T) -> T>>,
) -> Parser<T> {
  parser.clone().and_then(|first| {
    parse::many(
      operator
        .name(format!("binary operator"))
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
  parse::many(parse::whitespace())
    .and_then(|_| parse::translation_unit())
    .parse(&input)
    .unwrap_or_else(|error| {
      errors.extend([(Pos(File("[parse]".to_string()), 0, 0), Error(error))]);
      Program(vec![])
    })
}

fn translation_unit() -> Parser<Program> {
  parse::many(
    Parser::expected(vec![])
      .or_else(|_| parse::function_declaration_global())
      .or_else(|_| parse::function_definition_global())
      .or_else(|_| parse::global_declaration_global())
      .or_else(|_| parse::global_definition_global())
      .or_else(|_| parse::assembly_global()),
  )
  .and_then(|globals| parse::eof().map(move |_| globals))
  .map(|globals| Program(globals))
}

fn function_declaration_global() -> Parser<Global> {
  // TODO does not obey grammar
  Parser::pure(())
    .and_then(|_| parse::maybe(parse::ws(parse::string("inline"))))
    .and_then(|is_inline| {
      parse::type_name().and_then(move |type_name| {
        parse::identifier().and_then(move |identifier| {
          Parser::pure(())
            .and_then(|_| parse::ws(parse::char('(').info("to begin parameter list")))
            .and_then(|_| parse::parameter_list())
            .and_then(move |parameters| {
              parse::maybe(
                parse::ws(parse::char(',').info("then ellipsis for variadic parameter"))
                  .and_then(|_| parse::ws(parse::string("..."))),
              )
              .and_then(move |is_variadic| {
                Parser::pure(())
                  .and_then(|_| parse::ws(parse::char(')').info("to end parameter list")))
                  .and_then(|_| parse::ws(parse::char(';').info("to end declaration")))
                  .map(move |_| {
                    Global::FunctionDeclaration(
                      is_inline.is_some(),
                      Object(type_name, identifier),
                      match parameters[..] {
                        [Object(Type::Void, _)] => vec![], // for `T func(void)`-style declarations
                        _ => parameters,
                      },
                      is_variadic.is_some(),
                    )
                  })
              })
            })
        })
      })
    })
    .name(format!("function declaration"))
}

fn function_definition_global() -> Parser<Global> {
  // TODO does not obey grammar
  Parser::pure(())
    .and_then(|_| parse::maybe(parse::ws(parse::string("inline"))))
    .and_then(|is_inline| {
      parse::type_name().and_then(move |type_name| {
        parse::identifier().and_then(move |identifier| {
          Parser::pure(())
            .and_then(|_| parse::ws(parse::char('(').info("to begin parameter list")))
            .and_then(|_| parse::parameter_list())
            .and_then(move |parameters| {
              parse::maybe(
                parse::ws(parse::char(',').info("then ellipsis for variadic parameter"))
                  .and_then(|_| parse::ws(parse::string("..."))),
              )
              .and_then(move |is_variadic| {
                Parser::pure(())
                  .and_then(|_| parse::ws(parse::char(')').info("to end parameter list")))
                  .and_then(|_| {
                    parse::statement().name(format!("statement to begin function body"))
                  })
                  .map(move |statement| {
                    Global::FunctionDefinition(
                      is_inline.is_some(),
                      Object(type_name, identifier),
                      match parameters[..] {
                        [Object(Type::Void, _)] => vec![], // for `T func(void)`-style declarations
                        _ => parameters,
                      },
                      is_variadic.is_some(),
                      statement,
                    )
                  })
              })
            })
        })
      })
    })
    .name(format!("function definition"))
}

fn parameter_list() -> Parser<Vec<Object>> {
  // TODO does not obey grammar
  parse::sepby(
    parse::type_name().and_then(|type_name| {
      parse::identifier()
        .or_else(|_| Parser::pure("".to_string()))
        .map(|identifier| Object(type_name, identifier))
    }),
    parse::ws(parse::char(',').info("to continue parameter list")),
  )
  .name(format!("parameter list"))
}

fn global_declaration_global() -> Parser<Global> {
  // TODO does not obey grammar
  parse::type_name()
    .and_then(move |type_name| {
      parse::identifier().and_then(move |identifier| {
        parse::ws(parse::char(';').info("to end declaration"))
          .map(move |_| Global::GlobalDeclaration(Object(type_name, identifier)))
      })
    })
    .name(format!("global declaration"))
}

fn global_definition_global() -> Parser<Global> {
  // TODO does not obey grammar
  parse::type_name()
    .and_then(move |type_name| {
      parse::identifier().and_then(move |identifier| {
        parse::ws(parse::char('=').info("to begin initializer")).and_then(move |_| {
          parse::expression().and_then(move |expression| {
            parse::ws(parse::char(';').info("to end declaration"))
              .map(move |_| Global::GlobalDefinition(Object(type_name, identifier), expression))
          })
        })
      })
    })
    .name(format!("global definition"))
}

fn type_name() -> Parser<Type> {
  // TODO does not obey grammar
  Parser::pure(())
    .and_then(|_| parse::maybe(parse::ws(parse::string("const"))))
    .and_then(|_is_const| {
      Parser::expected(vec![])
        .or_else(|_| {
          parse::ws(parse::string("long long int").or_else(|_| parse::string("long long")))
            .map(|_| Type::LongLong)
        })
        .or_else(|_| {
          parse::ws(
            parse::string("unsigned long long int")
              .or_else(|_| parse::string("unsigned long long")),
          )
          .map(|_| Type::UnsignedLongLong)
        })
        .or_else(|_| {
          parse::ws(parse::string("long int").or_else(|_| parse::string("long")))
            .map(|_| Type::Long)
        })
        .or_else(|_| {
          parse::ws(parse::string("unsigned long int").or_else(|_| parse::string("unsigned long")))
            .map(|_| Type::UnsignedLong)
        })
        .or_else(|_| parse::ws(parse::string("int")).map(|_| Type::Int))
        .or_else(|_| {
          parse::ws(parse::string("unsigned int").or_else(|_| parse::string("unsigned")))
            .map(|_| Type::UnsignedInt)
        })
        .or_else(|_| {
          parse::ws(parse::string("short int").or_else(|_| parse::string("short")))
            .map(|_| Type::Short)
        })
        .or_else(|_| {
          parse::ws(
            parse::string("unsigned short int").or_else(|_| parse::string("unsigned short")),
          )
          .map(|_| Type::UnsignedShort)
        })
        .or_else(|_| parse::ws(parse::string("char")).map(|_| Type::Char))
        .or_else(|_| parse::ws(parse::string("_Bool")).map(|_| Type::Bool))
        .or_else(|_| parse::ws(parse::string("void")).map(|_| Type::Void))
    })
    // TODO implement proper pointer types
    .and_then(|r#type| {
      let type1 = r#type.clone();
      parse::ws(parse::char('*'))
        .info("for pointer type")
        .map(|_| Type::Pointer(Box::new(r#type)))
        .or_else(|_| Parser::pure(type1))
    })
    .name(format!("type name"))
}

fn assembly_global() -> Parser<Global> {
  parse::assembly_statement()
    .map(|statement| match statement {
      Statement::Assembly(global_assembly) => Global::GlobalAssembly(global_assembly),
      _ => panic!("`assembly_statement` did not return `Statement::Assembly`"),
    })
    .name(format!("assembly global"))
}

fn compound_statement() -> Parser<Statement> {
  // TODO should be {<declaration>}* {<statement>}*
  Parser::pure(())
    .and_then(|_| parse::ws(parse::char('{').info("to begin block")))
    .and_then(|_| {
      parse::many(parse::statement()).and_then(|statements| {
        parse::ws(parse::char('}').info("to end block")).map(move |_| statements)
      })
    })
    .map(|statements| Statement::Compound(statements))
    .name(format!("compound statement"))
}

fn statement() -> Parser<Statement> {
  // TODO cases missing
  Parser::expected(vec![])
    .or_else(|_| parse::jump_statement())
    .or_else(|_| parse::iteration_statement())
    .or_else(|_| parse::compound_statement())
    .or_else(|_| parse::selection_statement())
    .or_else(|_| parse::expression_statement())
    .or_else(|_| parse::assembly_statement()) // TODO nonstandard
    .name(format!("statement"))
}

fn jump_statement() -> Parser<Statement> {
  // TODO cases missing
  Parser::pure(())
    .and_then(|_| parse::ws(parse::string("return")))
    .and_then(|_| {
      parse::maybe(parse::expression()).and_then(|expression| {
        parse::ws(parse::char(';').info("to end statement")).map(|_| expression)
      })
    }) // TODO does not obey grammar
    .map(|expression| Statement::Return(expression))
}

fn selection_statement() -> Parser<Statement> {
  // TODO cases missing
  Parser::expected(vec![])
    .or_else(|_| parse::if_else_statement())
    .or_else(|_| parse::if_statement())
    .or_else(|_| parse::switch_statement())
}

fn if_statement() -> Parser<Statement> {
  Parser::pure(())
    .and_then(|_| parse::ws(parse::string("if")))
    .and_then(|_| parse::ws(parse::char('(').info("to begin condition")))
    .and_then(|_| parse::expression())
    .and_then(|expression| {
      parse::ws(parse::char(')').info("to end condition"))
        .and_then(|_| parse::statement())
        .map(|statement| Statement::If(expression, Box::new(statement), None))
    })
}

fn if_else_statement() -> Parser<Statement> {
  Parser::pure(())
    .and_then(|_| parse::ws(parse::string("if")))
    .and_then(|_| parse::ws(parse::char('(').info("to begin condition")))
    .and_then(|_| parse::expression())
    .and_then(|expression| {
      parse::ws(parse::char(')').info("to end condition"))
        .and_then(|_| parse::statement())
        .and_then(|statement1| {
          parse::ws(parse::string("else"))
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
    .and_then(|_| parse::ws(parse::string("while")))
    .and_then(|_| parse::ws(parse::char('(').info("to begin condition")))
    .and_then(|_| parse::expression())
    .and_then(|expression| {
      parse::ws(parse::char(')').info("to end condition"))
        .and_then(|_| parse::statement())
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
      parse::ws(parse::char(';').info("to end statement"))
        .map(|_| Statement::Expression(expression))
    })
}

fn assembly_statement() -> Parser<Statement> {
  Parser::pure(())
    .and_then(|_| parse::ws(parse::string("asm")))
    .and_then(|_| parse::ws(parse::char('{').info("to begin assembly statement")))
    .and_then(|_| parse::many(parse::char_not('}').info("to continue assembly statement")))
    .map(|chars| chars.iter().collect::<String>().trim().to_string())
    .and_then(|assembly| {
      parse::ws(parse::char('}').info("to end assembly statement"))
        .map(move |_| Statement::Assembly(assembly))
    })
}

fn expression() -> Parser<Expression> {
  parse::constant_expression() // TODO does not obey grammar
    .name(format!("expression"))
}

fn constant_expression() -> Parser<Expression> {
  parse::conditional_expression()
}

fn conditional_expression() -> Parser<Expression> {
  parse::logical_or_expression().and_then(|expression1| {
    let expression = expression1.clone();
    Parser::pure(())
      .and_then(|_| parse::ws(parse::char('?').info("to begin conditional operator")))
      .and_then(|_| parse::expression())
      .and_then(|expression2| {
        parse::ws(parse::char(':').info("then expression to end conditional operator"))
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
    parse::ws(parse::string("||")).map(|_| psi(Box::new, Expression::LogicalOr)),
  )
}

fn logical_and_expression() -> Parser<Expression> {
  parse::binop(
    parse::bitwise_inclusive_or_expression(),
    parse::ws(parse::string("&&")).map(|_| psi(Box::new, Expression::LogicalAnd)),
  )
}

fn bitwise_inclusive_or_expression() -> Parser<Expression> {
  parse::binop(
    parse::bitwise_exclusive_or_expression(),
    parse::ws(parse::char('|')).map(|_| psi(Box::new, Expression::BitwiseInclusiveOr)),
  )
}

fn bitwise_exclusive_or_expression() -> Parser<Expression> {
  parse::binop(
    parse::bitwise_and_expression(),
    parse::ws(parse::char('^')).map(|_| psi(Box::new, Expression::BitwiseExclusiveOr)),
  )
}

fn bitwise_and_expression() -> Parser<Expression> {
  parse::binop(
    parse::equality_expression(),
    parse::ws(parse::char('&')).map(|_| psi(Box::new, Expression::BitwiseAnd)),
  )
}

fn equality_expression() -> Parser<Expression> {
  parse::binop(
    parse::relational_expression(),
    Parser::expected(vec![])
      .or_else(|_| parse::ws(parse::string("==")).map(|_| psi(Box::new, Expression::EqualTo)))
      .or_else(|_| parse::ws(parse::string("!=")).map(|_| psi(Box::new, Expression::NotEqualTo))),
  )
}

fn relational_expression() -> Parser<Expression> {
  parse::binop(
    parse::shift_expression(),
    Parser::expected(vec![])
      .or_else(|_| {
        parse::ws(parse::string(">=")).map(|_| psi(Box::new, Expression::GreaterThanOrEqualTo))
      })
      .or_else(|_| parse::ws(parse::char('>')).map(|_| psi(Box::new, Expression::GreaterThan)))
      .or_else(|_| {
        parse::ws(parse::string("<=")).map(|_| psi(Box::new, Expression::LessThanOrEqualTo))
      })
      .or_else(|_| parse::ws(parse::char('<')).map(|_| psi(Box::new, Expression::LessThan))),
  )
}

fn shift_expression() -> Parser<Expression> {
  parse::binop(
    parse::additive_expression(),
    Parser::expected(vec![])
      .or_else(|_| parse::ws(parse::string("<<")).map(|_| psi(Box::new, Expression::LeftShift)))
      .or_else(|_| parse::ws(parse::string(">>")).map(|_| psi(Box::new, Expression::RightShift))),
  )
}

fn additive_expression() -> Parser<Expression> {
  parse::binop(
    parse::multiplicative_expression(),
    Parser::expected(vec![])
      .or_else(|_| parse::ws(parse::char('+')).map(|_| psi(Box::new, Expression::Addition)))
      .or_else(|_| parse::ws(parse::char('-')).map(|_| psi(Box::new, Expression::Subtraction))),
  )
}

fn multiplicative_expression() -> Parser<Expression> {
  parse::binop(
    parse::cast_expression(),
    Parser::expected(vec![])
      .or_else(|_| parse::ws(parse::char('*')).map(|_| psi(Box::new, Expression::Multiplication)))
      .or_else(|_| parse::ws(parse::char('/')).map(|_| psi(Box::new, Expression::Division)))
      .or_else(|_| parse::ws(parse::char('%')).map(|_| psi(Box::new, Expression::Modulo))),
  )
}

fn cast_expression() -> Parser<Expression> {
  parse::ws(parse::char('(').info("to begin expression"))
    .and_then(|_| parse::type_name())
    .and_then(|type_name| {
      parse::ws(parse::char(')').info("to end expression"))
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
        parse::ws(parse::char('-')).map(|_| b(Box::new, Expression::Negation)),
      )
    })
    .or_else(|_| {
      parse::unop(
        parse::unary_expression(),
        parse::ws(parse::char('+')).map(|_| b(Box::new, Expression::Positive)),
      )
    })
    .or_else(|_| {
      parse::unop(
        parse::unary_expression(),
        parse::ws(parse::char('~')).map(|_| b(Box::new, Expression::BitwiseComplement)),
      )
    })
    .or_else(|_| {
      parse::unop(
        parse::unary_expression(),
        parse::ws(parse::char('!')).map(|_| b(Box::new, Expression::LogicalNegation)),
      )
    })
    .or_else(|_| {
      Parser::pure(())
        .and_then(|_| parse::ws(parse::char('(').info("to begin expression")))
        .and_then(|_| parse::expression()) // TODO does not obey grammar
        .and_then(|expression| {
          parse::ws(parse::char(')').info("to end expression")).map(|_| expression)
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
            .and_then(|_| parse::ws(parse::char('(').info("to begin argument list")))
            .and_then(|_| {
              parse::sepby(
                parse::expression(),
                parse::ws(parse::char(',').info("to continue argument list")),
              )
            })
            .and_then(|arguments| {
              parse::ws(parse::char(')').info("to end argument list"))
                .map(|_| Expression::FunctionCall(Box::new(expression1), arguments))
            })
        })
        .or_else(|_| Parser::pure(expression))
    })
}

fn identifier() -> Parser<String> {
  Parser::pure(())
    .and_then(|_| parse::alphabetic().or_else(|_| parse::char('_').map(|_| '_')))
    .and_then(|first| {
      parse::ws(parse::many(
        Parser::expected(vec![])
          .or_else(|_| parse::digit(10))
          .or_else(|_| parse::alphabetic())
          .or_else(|_| parse::char('_').map(|_| '_'))
          .name(format!("identifier character")),
      ))
      .map(move |rest| std::iter::once(first).chain(rest).collect())
    })
    .name(format!("identifier"))
}

fn integer_constant() -> Parser<Expression> {
  // TODO does not support suffixes
  Parser::expected(vec![])
    .or_else(|_| {
      // TODO nonstandard <binary-constant>
      parse::string("0b")
        .or_else(|_| parse::string("0B"))
        .and_then(|_| parse::ws(parse::many1(parse::digit(0b10))))
        .map(|digits| digits.into_iter().collect::<String>())
        .map(|digits| u8::from_str_radix(&digits, 0b10))
    })
    .or_else(|_| {
      // <hexadecimal-constant>
      parse::string("0x")
        .or_else(|_| parse::string("0X"))
        .and_then(|_| parse::ws(parse::many1(parse::digit(0x10))))
        .map(|digits| digits.into_iter().collect::<String>())
        .map(|digits| u8::from_str_radix(&digits, 0x10))
    })
    .or_else(|_| {
      // <octal-constant>
      parse::char('0')
        .and_then(|_| parse::ws(parse::many(parse::digit(0o10))))
        .map(|digits| std::iter::once('0').chain(digits).collect::<String>())
        .map(|digits| u8::from_str_radix(&digits, 0o10))
    })
    .or_else(|_| {
      // <decimal-constant>
      parse::satisfy(|c| c.is_digit(10) && c != '0')
        .and_then(|first| parse::ws(parse::many(parse::digit(10))).map(move |rest| (first, rest)))
        .map(|(first, rest)| std::iter::once(first).chain(rest).collect::<String>())
        .map(|digits| u8::from_str_radix(&digits, 10))
    })
    .map(|digits| digits.unwrap_or_else(|_| panic!("Could not parse integer constant")))
    .map(|value| Expression::IntegerConstant(value))
    .name(format!("integer constant"))
}

fn character_constant() -> Parser<Expression> {
  Parser::pure(())
    .and_then(|_| parse::char('\''))
    .and_then(|_| {
      parse::char_none_of("\'\\\n")
        .name(format!("character constant character"))
        .or_else(|_| parse::escape_sequence())
    })
    .and_then(|char| {
      parse::ws(parse::char('\'').info("to end character constant"))
        .map(move |_| Expression::CharacterConstant(char))
    })
    .name(format!("character constant"))
}

fn string_literal() -> Parser<Expression> {
  // also concatenates adjacent string literals
  parse::many1(
    Parser::pure(())
      .and_then(|_| parse::char('"'))
      .and_then(|_| {
        parse::many(
          parse::char_none_of("\"\\\n")
            .name(format!("string literal character"))
            .or_else(|_| parse::escape_sequence()),
        )
      })
      .and_then(|chars| {
        parse::ws(parse::char('"').info("to end string literal")).map(move |_| chars)
      }),
  )
  .map(|strings| Expression::StringLiteral(strings.into_iter().flatten().chain(['\0']).collect()))
  .name(format!("string literal"))
}

fn escape_sequence() -> Parser<char> {
  parse::char('\\')
    .and_then(|_| {
      Parser::expected(vec![])
        .or_else(|_| {
          // <simple-escape-sequence>
          Parser::expected(vec![])
            .or_else(|_| parse::char('\'').map(|_| '\''))
            .or_else(|_| parse::char('"').map(|_| '"'))
            .or_else(|_| parse::char('?').map(|_| '?'))
            .or_else(|_| parse::char('\\').map(|_| '\\'))
            .or_else(|_| parse::char('a').map(|_| '\x07'))
            .or_else(|_| parse::char('b').map(|_| '\x08'))
            .or_else(|_| parse::char('f').map(|_| '\x0C'))
            .or_else(|_| parse::char('n').map(|_| '\n'))
            .or_else(|_| parse::char('r').map(|_| '\r'))
            .or_else(|_| parse::char('t').map(|_| '\t'))
            .or_else(|_| parse::char('v').map(|_| '\x0B'))
            .name(format!("one of `'\"?\\abfnrtv`"))
        })
        .or_else(|_| {
          // <octal-escape-sequence>
          parse::digit(0o10).and_then(|first| {
            parse::maybe(parse::digit(0o10)).and_then(move |second| {
              parse::maybe(parse::digit(0o10)).map(move |third| {
                let digits = std::iter::once(first)
                  .chain(second.into_iter())
                  .chain(third.into_iter());
                let digits = digits.collect::<String>();
                u8::from_str_radix(&digits, 0o10)
                  .unwrap_or_else(|_| panic!("Could not parse escape sequence"))
                  as char
              })
            })
          })
        })
        .or_else(|_| {
          // <hexadecimal-escape-sequence>
          parse::char('x')
            .info("then hexadecimal digit")
            .and_then(|_| parse::digit(0x10))
            .and_then(|first| {
              parse::maybe(parse::digit(0x10)).map(move |second| {
                let digits = std::iter::once(first).chain(second.into_iter());
                let digits = digits.collect::<String>();
                u8::from_str_radix(&digits, 0x10)
                  .unwrap_or_else(|_| panic!("Could not parse escape sequence"))
                  as char
              })
            })
        })
    })
    .name(format!("escape sequence"))
}
