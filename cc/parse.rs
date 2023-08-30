use crate::*;
use std::rc::Rc;

// utilities

fn psi<
  T: 'static,
  U: 'static,
  V: 'static,
  F: FnOnce(T) -> U + 'static + Clone,
  G: FnOnce(U, U) -> V + 'static + Clone,
>(
  f: F,
  g: G,
) -> Rc<dyn Fn(T, T) -> V> {
  Rc::new(move |x, y| g.clone()(f.clone()(x), f.clone()(y)))
}

fn bluebird<
  T: 'static,
  U: 'static,
  V: 'static,
  F: FnOnce(T) -> U + 'static + Clone,
  G: FnOnce(U) -> V + 'static + Clone,
>(
  f: F,
  g: G,
) -> Rc<dyn Fn(T) -> V> {
  Rc::new(move |x| g.clone()(f.clone()(x)))
}

#[derive(Clone)]
struct Parser<T: 'static + Clone>(Rc<dyn Fn(String) -> ParseResult<T>>);
type ParseResult<T> = Result<(T, String), Error>;

impl<T: 'static + Clone> Parser<T> {
  fn and_then<U: 'static + Clone, F: FnOnce(T) -> Parser<U> + 'static + Clone>(
    self,
    f: F,
  ) -> Parser<U> {
    Parser(Rc::new(move |input: String| {
      self.0(input).and_then(|(match_, input)| f.clone()(match_).0(input))
    }))
  }

  fn or_else<F: FnOnce(Error) -> Parser<T> + 'static + Clone>(self, f: F) -> Parser<T> {
    Parser(Rc::new(move |input: String| {
      self.0(input.clone()).or_else(|error| f.clone()(error).0(input))
    }))
  }

  fn map<U: 'static + Clone, F: FnOnce(T) -> U + 'static + Clone>(self, f: F) -> Parser<U> {
    Parser(Rc::new(move |input: String| {
      self.0(input).map(|(match_, input)| (f.clone()(match_), input))
    }))
  }

  fn return_(value: T) -> Parser<T> {
    Parser(Rc::new(move |input: String| Ok((value.clone(), input))))
  }

  fn error(error: Error) -> Parser<T> {
    Parser(Rc::new(move |_input: String| Err(error.clone())))
  }
}

// elementary parsers

fn any() -> Parser<char> {
  Parser(Rc::new(|input: String| match &input[..] {
    "" => Err(Error(format!("Unexpected end of input"))),
    _ => Ok((input.chars().next().unwrap(), input[1..].to_string())),
  }))
}

fn eof() -> Parser<()> {
  Parser(Rc::new(|input: String| match &input[..] {
    "" => Ok(((), input)),
    _ => Err(Error(format!(
      "Expected end of input, got `{}`",
      input[0..1].to_string()
    ))),
  }))
}

fn satisfy<F: Fn(char) -> bool + 'static + Clone>(predicate: F) -> Parser<char> {
  parse::any().and_then(|char| {
    Parser(Rc::new(move |input: String| match predicate(char) {
      true => Ok((char, input)),
      false => Err(Error(format!("Unexpected `{}`", char))),
    }))
  })
}

fn char(char: char) -> Parser<()> {
  parse::satisfy(move |x| x == char).map(|_| ())
}

fn string(string: &'static str) -> Parser<()> {
  string
    .chars()
    .map(parse::char)
    .reduce(|acc, parser| acc.and_then(|_| parser))
    .unwrap()
}

// parser combinators

fn many1<T: 'static + Clone>(parser: Parser<T>) -> Parser<Vec<T>> {
  parser.clone().and_then(|first| {
    parse::many(parser).map(|rest| std::iter::once(first).chain(rest.into_iter()).collect())
  })
}

fn many<T: 'static + Clone>(parser: Parser<T>) -> Parser<Vec<T>> {
  parse::many1(parser).or_else(|_| Parser::return_(vec![]))
}

fn sepby1<T: 'static + Clone>(parser: Parser<T>, separator: Parser<()>) -> Parser<Vec<T>> {
  parser.clone().and_then(|first| {
    parse::many(separator.and_then(|_| parser))
      .map(|rest| std::iter::once(first).chain(rest.into_iter()).collect())
  })
}

fn sepby<T: 'static + Clone>(parser: Parser<T>, separator: Parser<()>) -> Parser<Vec<T>> {
  parse::sepby1(parser, separator).or_else(|_| Parser::return_(vec![]))
}

fn binary_operation<T: 'static + Clone>(
  parser: Parser<T>,
  separator: Parser<Rc<dyn Fn(T, T) -> T>>,
) -> Parser<T> {
  parser.clone().and_then(|first| {
    parse::many(separator.and_then(|constructor| parser.map(|second| (constructor, second)))).map(
      |rest| {
        rest
          .into_iter()
          .fold(first, |acc, (constructor, second)| constructor(acc, second))
      },
    )
  })
}

// C99 grammar

pub fn parse(input: String) -> Result<Program, Error> {
  parse::translation_unit().0(input).map(|(programm, input)| match &input[..] {
    "" => programm,
    _ => panic!("Input not fully parsed"),
  })
}

fn translation_unit() -> Parser<Program> {
  // TODO should be `many` instead of `many1`
  parse::many1(parse::function_definition()).and_then(|function_definitions| {
    parse::whitespaces_eof().map(|_| Program {
      function_definitions,
    })
  })
}

fn function_definition() -> Parser<FunctionDefinition> {
  Parser::return_(())
    .and_then(|_| parse::whitespaces_string("int"))
    .and_then(|_| parse::identifier())
    .and_then(|identifier| {
      Parser::return_(())
        .and_then(|_| parse::whitespaces_char('('))
        .and_then(|_| parse::whitespaces_char(')'))
        .and_then(|_| parse::compound_statement())
        .map(|statements| {
          FunctionDefinition(Type::BasicType(BasicType::Int), identifier, statements)
        })
    })
}

fn compound_statement() -> Parser<Vec<Statement>> {
  // TODO should be {<declaration>}* {<statement>}*
  Parser::return_(())
    .and_then(|_| parse::whitespaces_char('{'))
    .and_then(|_| parse::many(parse::statement()))
    .and_then(|statements| parse::whitespaces_char('}').map(|_| statements))
}

fn statement() -> Parser<Statement> {
  // TODO cases missing
  Parser::error(Error("".to_string()))
    .or_else(|_| parse::jump_statement())
    .or_else(|_| parse::expression_statement())
    .or_else(|_| parse::asm_statement()) // TODO does not obey grammar
}

fn jump_statement() -> Parser<Statement> {
  // TODO cases missing
  Parser::return_(())
    .and_then(|_| parse::whitespaces_string("return"))
    .and_then(|_| parse::expression()) // TODO does not obey grammar
    .and_then(|expression| parse::whitespaces_char(';').map(|_| Statement::Return(expression)))
}

fn expression_statement() -> Parser<Statement> {
  // TODO cases missing
  Parser::return_(())
    .and_then(|_| parse::expression())
    .and_then(|expression| parse::whitespaces_char(';').map(|_| Statement::Expression(expression)))
}

fn asm_statement() -> Parser<Statement> {
  Parser::return_(())
    .and_then(|_| parse::whitespaces_string("asm"))
    .and_then(|_| parse::whitespaces_char('('))
    .and_then(|_| parse::sepby(parse::expression(), parse::whitespaces_char(',')))
    .and_then(|expressions| {
      Parser::return_(())
        .and_then(|_| parse::whitespaces_char(')'))
        .and_then(|_| parse::whitespaces_char('{'))
        .and_then(|_| parse::many(parse::satisfy(|c| c != '}')))
        .map(|chars| chars.iter().collect())
        .and_then(|assembly| {
          parse::whitespaces_char('}').map(move |_| Statement::Asm(expressions, assembly))
        })
    })
}

fn expression() -> Parser<Expression> {
  parse::constant_expression() // TODO does not obey grammar
}

fn constant_expression() -> Parser<Expression> {
  parse::conditional_expression()
}

fn conditional_expression() -> Parser<Expression> {
  parse::logical_or_expression()
    .and_then(|expression1| {
      parse::whitespaces_char('?')
        .and_then(|_| parse::expression())
        .and_then(|expression2| {
          parse::whitespaces_char(':')
            .and_then(|_| parse::conditional_expression())
            .map(move |expression3| {
              Expression::Conditional(
                Box::new(expression1),
                Box::new(expression2),
                Box::new(expression3),
              )
            })
        })
    })
    .or_else(|_| parse::logical_or_expression())
}

fn logical_or_expression() -> Parser<Expression> {
  parse::binary_operation(
    parse::logical_and_expression(),
    parse::whitespaces_string("||")
      .and_then(|_| Parser::return_(psi(Box::new, Expression::LogicalOr))),
  )
}

fn logical_and_expression() -> Parser<Expression> {
  parse::binary_operation(
    parse::bitwise_inclusive_or_expression(),
    parse::whitespaces_string("&&")
      .and_then(|_| Parser::return_(psi(Box::new, Expression::LogicalAnd))),
  )
}

fn bitwise_inclusive_or_expression() -> Parser<Expression> {
  parse::binary_operation(
    parse::bitwise_exclusive_or_expression(),
    parse::whitespaces_char('|')
      .and_then(|_| Parser::return_(psi(Box::new, Expression::BitwiseInclusiveOr))),
  )
}

fn bitwise_exclusive_or_expression() -> Parser<Expression> {
  parse::binary_operation(
    parse::bitwise_and_expression(),
    parse::whitespaces_char('^')
      .and_then(|_| Parser::return_(psi(Box::new, Expression::BitwiseExclusiveOr))),
  )
}

fn bitwise_and_expression() -> Parser<Expression> {
  parse::binary_operation(
    parse::equality_expression(),
    parse::whitespaces_char('&')
      .and_then(|_| Parser::return_(psi(Box::new, Expression::BitwiseAnd))),
  )
}

fn equality_expression() -> Parser<Expression> {
  parse::binary_operation(
    parse::relational_expression(),
    Parser::error(Error("".to_string()))
      .or_else(|_| {
        parse::whitespaces_string("==")
          .and_then(|_| Parser::return_(psi(Box::new, Expression::EqualTo)))
      })
      .or_else(|_| {
        parse::whitespaces_string("!=")
          .and_then(|_| Parser::return_(psi(Box::new, Expression::NotEqualTo)))
      }),
  )
}

fn relational_expression() -> Parser<Expression> {
  parse::binary_operation(
    parse::shift_expression(),
    Parser::error(Error("".to_string()))
      .or_else(|_| {
        parse::whitespaces_string(">=")
          .and_then(|_| Parser::return_(psi(Box::new, Expression::GreaterThanOrEqualTo)))
      })
      .or_else(|_| {
        parse::whitespaces_char('>')
          .and_then(|_| Parser::return_(psi(Box::new, Expression::GreaterThan)))
      })
      .or_else(|_| {
        parse::whitespaces_string("<=")
          .and_then(|_| Parser::return_(psi(Box::new, Expression::LessThanOrEqualTo)))
      })
      .or_else(|_| {
        parse::whitespaces_char('<')
          .and_then(|_| Parser::return_(psi(Box::new, Expression::LessThan)))
      }),
  )
}

fn shift_expression() -> Parser<Expression> {
  parse::binary_operation(
    parse::additive_expression(),
    Parser::error(Error("".to_string()))
      .or_else(|_| {
        parse::whitespaces_string("<<")
          .and_then(|_| Parser::return_(psi(Box::new, Expression::LeftShift)))
      })
      .or_else(|_| {
        parse::whitespaces_string(">>")
          .and_then(|_| Parser::return_(psi(Box::new, Expression::RightShift)))
      }),
  )
}

fn additive_expression() -> Parser<Expression> {
  parse::binary_operation(
    parse::multiplicative_expression(),
    Parser::error(Error("".to_string()))
      .or_else(|_| {
        parse::whitespaces_char('+')
          .and_then(|_| Parser::return_(psi(Box::new, Expression::Addition)))
      })
      .or_else(|_| {
        parse::whitespaces_char('-')
          .and_then(|_| Parser::return_(psi(Box::new, Expression::Subtraction)))
      }),
  )
}

fn multiplicative_expression() -> Parser<Expression> {
  parse::binary_operation(
    parse::cast_expression(),
    Parser::error(Error("".to_string()))
      .or_else(|_| {
        parse::whitespaces_char('*')
          .and_then(|_| Parser::return_(psi(Box::new, Expression::Multiplication)))
      })
      .or_else(|_| {
        parse::whitespaces_char('/')
          .and_then(|_| Parser::return_(psi(Box::new, Expression::Division)))
      })
      .or_else(|_| {
        parse::whitespaces_char('%')
          .and_then(|_| Parser::return_(psi(Box::new, Expression::Modulo)))
      }),
  )
}

fn cast_expression() -> Parser<Expression> {
  parse::unary_expression() // TODO does not obey grammar
}

fn unary_expression() -> Parser<Expression> {
  Parser::error(Error("".to_string()))
    .or_else(|_| {
      Parser::return_(())
        .and_then(|_| parse::whitespaces_char('-'))
        .and_then(|_| parse::unary_expression())
        .map(|x| bluebird(Box::new, Expression::Negation)(x))
    })
    .or_else(|_| {
      Parser::return_(())
        .and_then(|_| parse::whitespaces_char('+'))
        .and_then(|_| parse::unary_expression())
    })
    .or_else(|_| {
      Parser::return_(())
        .and_then(|_| parse::whitespaces_char('~'))
        .and_then(|_| parse::unary_expression())
        .map(|x| bluebird(Box::new, Expression::BitwiseComplement)(x))
    })
    .or_else(|_| {
      Parser::return_(())
        .and_then(|_| parse::whitespaces_char('!'))
        .and_then(|_| parse::unary_expression())
        .map(|x| bluebird(Box::new, Expression::LogicalNegation)(x))
    })
    .or_else(|_| {
      Parser::return_(())
        .and_then(|_| parse::whitespaces_char('('))
        .and_then(|_| parse::expression()) // TODO does not obey grammar
        .and_then(|expression| parse::whitespaces_char(')').map(move |_| expression))
    })
    .or_else(|_| {
      parse::identifier().and_then(|identifier| {
        Parser::return_(())
          .and_then(|_| parse::whitespaces_char('('))
          .and_then(|_| parse::whitespaces_char(')'))
          .map(move |_| Expression::FunctionCall(identifier))
      })
    })
    .or_else(|_| parse::integer_constant())
}

fn identifier() -> Parser<String> {
  parse::many(parse::whitespace())
    .and_then(|_| parse::alphabetic())
    .and_then(|first| {
      parse::many(
        parse::digit()
          .or_else(|_| parse::alphabetic())
          .or_else(|_| Parser::error(Error(format!("Could not parse identifier")))),
      )
      .map(move |rest| std::iter::once(first).chain(rest).collect())
    })
}

fn integer_constant() -> Parser<Expression> {
  parse::many(parse::whitespace())
    .and_then(|_| parse::many1(parse::digit()))
    .map(|digits| digits.into_iter().collect())
    .and_then(|digits: String| {
      Parser(Rc::new(move |input| {
        digits
          .parse()
          .map(|value| (Expression::IntegerConstant(value), input))
          .map_err(|_| Error(format!("Invalid integer constant `{}`", digits)))
      }))
    })
}

fn whitespace() -> Parser<()> {
  Parser::error(Error("".to_string()))
    .or_else(|_| parse::char(' '))
    .or_else(|_| parse::char('\r'))
    .or_else(|_| parse::char('\n'))
}

fn digit() -> Parser<char> {
  parse::satisfy(|x| x.is_digit(10))
}

fn alphabetic() -> Parser<char> {
  parse::satisfy(|x| x.is_alphabetic() || x == '_')
}

fn whitespaces_eof() -> Parser<()> {
  parse::many(parse::whitespace()).and_then(|_| parse::eof())
}

fn whitespaces_char(char: char) -> Parser<()> {
  parse::many(parse::whitespace()).and_then(move |_| parse::char(char))
}

fn whitespaces_string(string: &'static str) -> Parser<()> {
  parse::many(parse::whitespace()).and_then(move |_| parse::string(string))
}
