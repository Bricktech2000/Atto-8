use crate::*;
use std::rc::Rc;

// utilities

pub fn psi<T, U, V, F: FnOnce(T) -> U + Clone + 'static, G: FnOnce(U, U) -> V + Clone + 'static>(
  f: F,
  g: G,
) -> Rc<dyn Fn(T, T) -> V> {
  Rc::new(move |x, y| g.clone()(f.clone()(x), f.clone()(y)))
}

pub fn bluebird<
  T,
  U,
  V,
  F: FnOnce(T) -> U + Clone + 'static,
  G: FnOnce(U) -> V + Clone + 'static,
>(
  f: F,
  g: G,
) -> Rc<dyn Fn(T) -> V> {
  Rc::new(move |x| g.clone()(f.clone()(x)))
}

#[derive(Clone)]
pub struct Parser<T: Clone + 'static>(pub Rc<dyn Fn(String) -> ParseResult<T>>);
pub type ParseResult<T> = Result<(T, String), Error>;

impl<T: Clone + 'static> Parser<T> {
  pub fn and_then<U: Clone + 'static, F: FnOnce(T) -> Parser<U> + Clone + 'static>(
    self,
    f: F,
  ) -> Parser<U> {
    Parser(Rc::new(move |input: String| {
      self.0(input).and_then(|(match_, input)| f.clone()(match_).0(input))
    }))
  }

  pub fn or_else<F: FnOnce(Error) -> Parser<T> + Clone + 'static>(self, f: F) -> Parser<T> {
    Parser(Rc::new(move |input: String| {
      self.0(input.clone()).or_else(|error| f.clone()(error).0(input))
    }))
  }

  pub fn map<U: Clone + 'static, F: FnOnce(T) -> U + Clone + 'static>(self, f: F) -> Parser<U> {
    Parser(Rc::new(move |input: String| {
      self.0(input).map(|(match_, input)| (f.clone()(match_), input))
    }))
  }

  pub fn return_(value: T) -> Parser<T> {
    Parser(Rc::new(move |input: String| Ok((value.clone(), input))))
  }

  pub fn error(error: Error) -> Parser<T> {
    Parser(Rc::new(move |_input: String| Err(error.clone())))
  }
}

// elementary parsers

pub fn any() -> Parser<char> {
  Parser(Rc::new(|input: String| match &input[..] {
    "" => Err(Error(format!("Unexpected end of input"))),
    _ => Ok((input.chars().next().unwrap(), input[1..].to_string())),
  }))
}

pub fn eof() -> Parser<()> {
  Parser(Rc::new(|input: String| match &input[..] {
    "" => Ok(((), input)),
    _ => Err(Error(format!(
      "Expected end of input, got `{}`",
      input[0..1].to_string()
    ))),
  }))
}

pub fn satisfy<F: Fn(char) -> bool + Clone + 'static>(predicate: F) -> Parser<char> {
  parse::any().and_then(|char| {
    Parser(Rc::new(move |input: String| match predicate(char) {
      true => Ok((char, input)),
      false => Err(Error(format!("Unexpected `{}`", char))),
    }))
  })
}

pub fn char(char: char) -> Parser<()> {
  parse::satisfy(move |c| c == char).map(|_| ())
}

pub fn string(string: &'static str) -> Parser<()> {
  string
    .chars()
    .map(parse::char)
    .reduce(|acc, parser| acc.and_then(|_| parser))
    .unwrap()
}

// parser combinators

pub fn many1<T: Clone + 'static>(parser: Parser<T>) -> Parser<Vec<T>> {
  parser.clone().and_then(|first| {
    parse::many(parser).map(|rest| std::iter::once(first).chain(rest.into_iter()).collect())
  })
}

pub fn many<T: Clone + 'static>(parser: Parser<T>) -> Parser<Vec<T>> {
  parse::many1(parser).or_else(|_| Parser::return_(vec![]))
}

pub fn sepby1<T: Clone + 'static>(parser: Parser<T>, separator: Parser<()>) -> Parser<Vec<T>> {
  parser.clone().and_then(|first| {
    parse::many(separator.and_then(|_| parser))
      .map(|rest| std::iter::once(first).chain(rest.into_iter()).collect())
  })
}

pub fn sepby<T: Clone + 'static>(parser: Parser<T>, separator: Parser<()>) -> Parser<Vec<T>> {
  parse::sepby1(parser, separator).or_else(|_| Parser::return_(vec![]))
}

pub fn binary_operation<T: Clone + 'static>(
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

pub fn translation_unit() -> Parser<Program> {
  // TODO should be `many` instead of `many1`
  parse::many1(
    Parser::error(Error("".to_string()))
      .or_else(|_| {
        parse::function_definition()
          .map(|function_definition| Global::FunctionDefinition(function_definition))
      })
      .or_else(|_| {
        parse::asm_statement().map(|statement| match statement {
          Statement::Asm(asm_statement) => Global::AsmStatement(asm_statement),
          // TODO `Global::AsmStatement(statement)` in parameter instead?
          _ => panic!("`asm_statement` did not return `Statement::Asm`"),
        })
      }),
  )
  .and_then(|globals| parse::whitespaces_eof().map(|_| Program(globals)))
}

pub fn function_definition() -> Parser<FunctionDefinition> {
  Parser::return_(())
    .and_then(|_| parse::type_name())
    .and_then(|type_name| {
      parse::identifier().and_then(|identifier| {
        Parser::return_(())
          .and_then(|_| parse::whitespaces_char('('))
          .and_then(|_| parse::whitespaces_char(')'))
          .and_then(|_| parse::compound_statement())
          .map(|statements| FunctionDefinition(Object(type_name, identifier), vec![], statements))
      })
    })
}

pub fn type_name() -> Parser<Type> {
  // TODO does not obey grammar
  Parser::error(Error("".to_string()))
    .or_else(|_| {
      parse::whitespaces_string("long long int")
        .or_else(|_| parse::whitespaces_string("long long"))
        .map(|_| Type::LongLong)
    })
    .or_else(|_| {
      parse::whitespaces_string("long int")
        .or_else(|_| parse::whitespaces_string("long"))
        .map(|_| Type::Long)
    })
    .or_else(|_| parse::whitespaces_string("int").map(|_| Type::Int))
    .or_else(|_| {
      parse::whitespaces_string("short int")
        .or_else(|_| parse::whitespaces_string("short"))
        .map(|_| Type::Short)
    })
    .or_else(|_| parse::whitespaces_string("char").map(|_| Type::Char))
    .or_else(|_| parse::whitespaces_string("bool").map(|_| Type::Bool))
    .or_else(|_| parse::whitespaces_string("void").map(|_| Type::Void))
}

pub fn compound_statement() -> Parser<Vec<Statement>> {
  // TODO should be {<declaration>}* {<statement>}*
  Parser::return_(())
    .and_then(|_| parse::whitespaces_char('{'))
    .and_then(|_| parse::many(parse::statement()))
    .and_then(|statements| parse::whitespaces_char('}').map(|_| statements))
}

pub fn statement() -> Parser<Statement> {
  // TODO cases missing
  Parser::error(Error("".to_string()))
    .or_else(|_| parse::jump_statement())
    .or_else(|_| parse::expression_statement())
    .or_else(|_| parse::asm_statement()) // TODO does not obey grammar
}

pub fn jump_statement() -> Parser<Statement> {
  // TODO cases missing
  Parser::return_(())
    .and_then(|_| parse::whitespaces_string("return"))
    .and_then(|_| parse::expression()) // TODO does not obey grammar
    .and_then(|expression| parse::whitespaces_char(';').map(|_| Statement::Return(expression)))
}

pub fn expression_statement() -> Parser<Statement> {
  // TODO cases missing
  Parser::return_(())
    .and_then(|_| parse::expression())
    .and_then(|expression| parse::whitespaces_char(';').map(|_| Statement::Expression(expression)))
}

pub fn asm_statement() -> Parser<Statement> {
  Parser::return_(())
    .and_then(|_| parse::whitespaces_string("asm"))
    .and_then(|_| parse::whitespaces_char('{'))
    .and_then(|_| parse::many(parse::satisfy(|c| c != '}')))
    .map(|chars| chars.iter().collect::<String>())
    .and_then(|assembly| {
      parse::whitespaces_char('}').map(move |_| {
        Statement::Asm(
          assembly
            .clone()
            .lines()
            .map(|line| line.strip_suffix("#").unwrap_or(line))
            .map(|line| line.split("# ").next().unwrap_or(line))
            .map(|line| line.to_string())
            .collect::<Vec<String>>()
            .join("\n"),
        )
      })
    })
}

pub fn expression() -> Parser<Expression> {
  parse::constant_expression() // TODO does not obey grammar
}

pub fn constant_expression() -> Parser<Expression> {
  parse::conditional_expression()
}

pub fn conditional_expression() -> Parser<Expression> {
  parse::logical_or_expression()
    .and_then(|expression1| {
      parse::whitespaces_char('?')
        .and_then(|_| parse::expression())
        .and_then(|expression2| {
          parse::whitespaces_char(':')
            .and_then(|_| parse::conditional_expression())
            .map(|expression3| {
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

pub fn logical_or_expression() -> Parser<Expression> {
  parse::binary_operation(
    parse::logical_and_expression(),
    parse::whitespaces_string("||")
      .and_then(|_| Parser::return_(psi(Box::new, Expression::LogicalOr))),
  )
}

pub fn logical_and_expression() -> Parser<Expression> {
  parse::binary_operation(
    parse::bitwise_inclusive_or_expression(),
    parse::whitespaces_string("&&")
      .and_then(|_| Parser::return_(psi(Box::new, Expression::LogicalAnd))),
  )
}

pub fn bitwise_inclusive_or_expression() -> Parser<Expression> {
  parse::binary_operation(
    parse::bitwise_exclusive_or_expression(),
    parse::whitespaces_char('|')
      .and_then(|_| Parser::return_(psi(Box::new, Expression::BitwiseInclusiveOr))),
  )
}

pub fn bitwise_exclusive_or_expression() -> Parser<Expression> {
  parse::binary_operation(
    parse::bitwise_and_expression(),
    parse::whitespaces_char('^')
      .and_then(|_| Parser::return_(psi(Box::new, Expression::BitwiseExclusiveOr))),
  )
}

pub fn bitwise_and_expression() -> Parser<Expression> {
  parse::binary_operation(
    parse::equality_expression(),
    parse::whitespaces_char('&')
      .and_then(|_| Parser::return_(psi(Box::new, Expression::BitwiseAnd))),
  )
}

pub fn equality_expression() -> Parser<Expression> {
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

pub fn relational_expression() -> Parser<Expression> {
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

pub fn shift_expression() -> Parser<Expression> {
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

pub fn additive_expression() -> Parser<Expression> {
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

pub fn multiplicative_expression() -> Parser<Expression> {
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

pub fn cast_expression() -> Parser<Expression> {
  // (type) identifier
  parse::whitespaces_char('(')
    .and_then(|_| parse::type_name())
    .and_then(|type_name| {
      parse::whitespaces_char(')')
        .and_then(|_| parse::cast_expression())
        .map(|cast_expression| Expression::Cast(type_name, Box::new(cast_expression)))
    })
    .or_else(|_| parse::unary_expression())
}

pub fn unary_expression() -> Parser<Expression> {
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
        .and_then(|expression| parse::whitespaces_char(')').map(|_| expression))
    })
    .or_else(|_| {
      parse::identifier().and_then(|identifier| {
        Parser::return_(())
          .and_then(|_| parse::whitespaces_char('('))
          .and_then(|_| parse::whitespaces_char(')'))
          .map(|_| Expression::FunctionCall(identifier))
      })
    })
    .or_else(|_| parse::integer_constant())
    .or_else(|_| parse::character_constant())
}

pub fn identifier() -> Parser<String> {
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

pub fn integer_constant() -> Parser<Expression> {
  // TODO does not ebey grammar
  parse::many(parse::whitespace())
    .and_then(|_| parse::many1(parse::digit()))
    .map(|digits| digits.into_iter().collect::<String>())
    .and_then(|digits| {
      Parser(Rc::new(move |input: String| {
        digits
          .parse()
          .map(|value| (Expression::IntegerConstant(value), input))
          .map_err(|_| Error(format!("Invalid integer constant `{}`", digits)))
      }))
    })
}

pub fn character_constant() -> Parser<Expression> {
  // TODO currently only parsing <simple-escape-sequence>s
  parse::many(parse::whitespace())
    .and_then(|_| parse::char('\''))
    .and_then(|_| {
      Parser::error(Error("".to_string()))
        .or_else(|_| parse::satisfy(|c| !"\'\\\n".contains(c)))
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
        .or_else(|_| parse::string("\\0").map(|_| '\0')) // TODO should be <octal-escape-sequence>
    })
    .and_then(|char| parse::char('\'').map(move |_| Expression::CharacterConstant(char)))
}

pub fn whitespace() -> Parser<()> {
  Parser::error(Error("".to_string()))
    .or_else(|_| parse::char(' '))
    .or_else(|_| parse::char('\r'))
    .or_else(|_| parse::char('\n'))
}

pub fn digit() -> Parser<char> {
  parse::satisfy(|c| c.is_digit(10))
}

pub fn alphabetic() -> Parser<char> {
  parse::satisfy(|c| c.is_alphabetic() || c == '_')
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
