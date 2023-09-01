use crate::*;
use std::rc::Rc;

// `Parser`s are functions that might be called multiple times, hence `Fn` instead of `FnOnce`.
// nested `Fn` closures cannot move data into themselves, hence the `clone`s and `+ Clone`s everywhere.
// since parsers can be called anytime, closures must borrow their environment for `'static`. therefore,
// with nested `Fn` closures, we need to use `move`s and `let X = X.clone()`s and `+ 'static`s
// everywhere, allowing them to fully own their environment. clasures might hove to refer to `Parser`s
// in their environment, hence `Parser` is `Clone`. `Box<dyn Fn ...>` is not `Clone`,
// hence the use of `Rc<dyn Fn ...>`. "are we GC yet?"

// utilities

fn psi<T, U, V, F: Fn(T) -> U, G: Fn(U, U) -> V>(f: F, g: G) -> impl Fn(T, T) -> V {
  move |x, y| g(f(x), f(y))
}

fn bluebird<T, U, V, F: Fn(T) -> U, G: Fn(U) -> V>(f: F, g: G) -> impl Fn(T) -> V {
  move |x| g(f(x))
}

#[derive(Clone)]
struct Parser<T: Clone + 'static>(Rc<dyn Fn(String) -> ParseResult<T>>);
type ParseResult<T> = Result<(T, String), Error>;

impl<T: Clone + 'static> Parser<T> {
  fn and_then<U: Clone, F: Fn(T) -> Parser<U> + 'static>(self, f: F) -> Parser<U> {
    Parser(Rc::new(move |input: String| {
      self.0(input).and_then(|(match_, input)| f(match_).0(input))
    }))
  }

  fn or_else<F: Fn(Error) -> Parser<T> + 'static>(self, f: F) -> Parser<T> {
    Parser(Rc::new(move |input: String| {
      self.0(input.clone()).or_else(|error| f(error).0(input))
    }))
  }

  fn map<U: Clone, F: Fn(T) -> U + 'static>(self, f: F) -> Parser<U> {
    Parser(Rc::new(move |input: String| {
      self.0(input).map(|(match_, input)| (f(match_), input))
    }))
  }

  fn return_(value: T) -> Parser<T> {
    Parser(Rc::new(move |input: String| {
      Ok((value.clone(), input.clone()))
    }))
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

fn satisfy<F: Fn(char) -> bool + Clone + 'static>(predicate: F) -> Parser<char> {
  parse::any().and_then(move |char| {
    let predicate = predicate.clone();
    Parser(Rc::new(move |input: String| match predicate(char) {
      true => Ok((char, input)),
      false => Err(Error(format!("Unexpected `{}`", char))),
    }))
  })
}

fn char(char: char) -> Parser<()> {
  parse::satisfy(move |x| x == char).map(|_| ())
}

fn string(string: &str) -> Parser<()> {
  string
    .chars()
    .map(parse::char)
    .reduce(|acc, parser| acc.and_then(move |_| parser.clone()))
    .unwrap()
}

// parser combinators

fn many1<T: Clone + 'static>(parser: Parser<T>) -> Parser<Vec<T>> {
  parser.clone().and_then(move |first| {
    parse::many(parser.clone()).map(move |rest| {
      std::iter::once(first.clone())
        .chain(rest.into_iter())
        .collect()
    })
  })
}

fn many<T: Clone + 'static>(parser: Parser<T>) -> Parser<Vec<T>> {
  parse::many1(parser.clone()).or_else(|_| Parser::return_(vec![]))
}

fn sepby1<T: Clone + 'static>(parser: Parser<T>, separator: Parser<()>) -> Parser<Vec<T>> {
  parser.clone().and_then(move |first| {
    let parser = parser.clone();
    parse::many(separator.clone().and_then(move |_| parser.clone())).map(move |rest| {
      std::iter::once(first.clone())
        .chain(rest.into_iter())
        .collect()
    })
  })
}

fn sepby<T: Clone + 'static>(parser: Parser<T>, separator: Parser<()>) -> Parser<Vec<T>> {
  parse::sepby1(parser, separator).or_else(|_| Parser::return_(vec![]))
}

fn binary_operation<T: Clone + 'static>(
  parser: Parser<T>,
  separator: Parser<impl Fn(T, T) -> T + Clone + 'static>,
) -> Parser<T> {
  parser.clone().and_then(move |first| {
    let parser = parser.clone();
    parse::many(separator.clone().and_then(move |constructor| {
      parser
        .clone()
        .map(move |second| (constructor.clone(), second.clone()))
    }))
    .map(move |rest| {
      rest
        .into_iter()
        .fold(first.clone(), |acc, (constructor, second)| {
          constructor(acc, second)
        })
    })
  })
}

// C99 grammar

type EEE = fn(Expression, Expression) -> Expression;

pub fn parse(input: String) -> Result<Program, Error> {
  parse::translation_unit().0(input).map(|(programm, input)| match &input[..] {
    "" => programm,
    _ => panic!("Input not fully parsed"),
  })
}

fn translation_unit() -> Parser<Program> {
  // TODO should be `many` instead of `many1`
  parse::many1(parse::function_definition()).and_then(|function_definitions| {
    parse::whitespaces_eof().map(move |_| Program {
      function_definitions: function_definitions.clone(),
    })
  })
}

fn function_definition() -> Parser<FunctionDefinition> {
  Parser::return_(())
    .and_then(|_| parse::type_name())
    .and_then(|type_name| {
      parse::identifier().and_then(move |identifier| {
        let type_name = type_name.clone();
        Parser::return_(())
          .and_then(|_| parse::whitespaces_char('('))
          .and_then(|_| parse::whitespaces_char(')'))
          .and_then(|_| parse::compound_statement())
          .map(move |statements| {
            FunctionDefinition(
              Object(type_name.clone(), identifier.clone()),
              vec![],
              statements,
            )
          })
      })
    })
}

fn type_name() -> Parser<Type> {
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

fn compound_statement() -> Parser<Vec<Statement>> {
  // TODO should be {<declaration>}* {<statement>}*
  Parser::return_(())
    .and_then(|_| parse::whitespaces_char('{'))
    .and_then(|_| parse::many(parse::statement()))
    .and_then(|statements| parse::whitespaces_char('}').map(move |_| statements.clone()))
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
    .and_then(|expression| {
      parse::whitespaces_char(';').map(move |_| Statement::Return(expression.clone()))
    })
}

fn expression_statement() -> Parser<Statement> {
  // TODO cases missing
  Parser::return_(())
    .and_then(|_| parse::expression())
    .and_then(|expression| {
      parse::whitespaces_char(';').map(move |_| Statement::Expression(expression.clone()))
    })
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
        .and_then(move |assembly: String| {
          let expressions = expressions.clone();
          parse::whitespaces_char('}')
            .map(move |_| Statement::Asm(expressions.clone(), assembly.clone()))
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
        .and_then(move |expression2| {
          let expression1 = expression1.clone();
          parse::whitespaces_char(':')
            .and_then(|_| parse::conditional_expression())
            .map(move |expression3| {
              Expression::Conditional(
                Box::new(expression1.clone()),
                Box::new(expression2.clone()),
                Box::new(expression3),
              )
            })
        })
    })
    .or_else(|_| parse::logical_or_expression())
}

fn logical_or_expression() -> Parser<Expression> {
  fn make_logical_or(expression1: Expression, expression2: Expression) -> Expression {
    psi(Box::new, Expression::LogicalOr)(expression1, expression2)
  }

  parse::binary_operation(
    parse::logical_and_expression(),
    parse::whitespaces_string("||").and_then(|_| Parser::return_(make_logical_or as EEE)),
  )
}

fn logical_and_expression() -> Parser<Expression> {
  fn make_logical_and(expression1: Expression, expression2: Expression) -> Expression {
    psi(Box::new, Expression::LogicalAnd)(expression1, expression2)
  }

  parse::binary_operation(
    parse::bitwise_inclusive_or_expression(),
    parse::whitespaces_string("&&").and_then(|_| Parser::return_(make_logical_and as EEE)),
  )
}

fn bitwise_inclusive_or_expression() -> Parser<Expression> {
  fn make_bitwise_inclusive_or(expression1: Expression, expression2: Expression) -> Expression {
    psi(Box::new, Expression::BitwiseInclusiveOr)(expression1, expression2)
  }

  parse::binary_operation(
    parse::bitwise_exclusive_or_expression(),
    parse::whitespaces_char('|').and_then(|_| Parser::return_(make_bitwise_inclusive_or as EEE)),
  )
}

fn bitwise_exclusive_or_expression() -> Parser<Expression> {
  fn make_bitwise_exclusive_or(expression1: Expression, expression2: Expression) -> Expression {
    psi(Box::new, Expression::BitwiseExclusiveOr)(expression1, expression2)
  }

  parse::binary_operation(
    parse::bitwise_and_expression(),
    parse::whitespaces_char('^').and_then(|_| Parser::return_(make_bitwise_exclusive_or as EEE)),
  )
}

fn bitwise_and_expression() -> Parser<Expression> {
  fn make_bitwise_and(expression1: Expression, expression2: Expression) -> Expression {
    psi(Box::new, Expression::BitwiseAnd)(expression1, expression2)
  }

  parse::binary_operation(
    parse::equality_expression(),
    parse::whitespaces_char('&').and_then(|_| Parser::return_(make_bitwise_and as EEE)),
  )
}

fn equality_expression() -> Parser<Expression> {
  fn make_equal_to(expression1: Expression, expression2: Expression) -> Expression {
    psi(Box::new, Expression::EqualTo)(expression1, expression2)
  }

  fn make_not_equal_to(expression1: Expression, expression2: Expression) -> Expression {
    psi(Box::new, Expression::NotEqualTo)(expression1, expression2)
  }

  parse::binary_operation(
    parse::relational_expression(),
    Parser::error(Error("".to_string()))
      .or_else(|_| {
        parse::whitespaces_string("==").and_then(|_| Parser::return_(make_equal_to as EEE))
      })
      .or_else(|_| {
        parse::whitespaces_string("!=").and_then(|_| Parser::return_(make_not_equal_to as EEE))
      }),
  )
}

fn relational_expression() -> Parser<Expression> {
  fn make_greater_than_or_equal_to(expression1: Expression, expression2: Expression) -> Expression {
    psi(Box::new, Expression::GreaterThanOrEqualTo)(expression1, expression2)
  }

  fn make_greater_than(expression1: Expression, expression2: Expression) -> Expression {
    psi(Box::new, Expression::GreaterThan)(expression1, expression2)
  }

  fn make_less_than_or_equal_to(expression1: Expression, expression2: Expression) -> Expression {
    psi(Box::new, Expression::LessThanOrEqualTo)(expression1, expression2)
  }

  fn make_less_than(expression1: Expression, expression2: Expression) -> Expression {
    psi(Box::new, Expression::LessThan)(expression1, expression2)
  }

  parse::binary_operation(
    parse::shift_expression(),
    Parser::error(Error("".to_string()))
      .or_else(|_| {
        parse::whitespaces_string(">=")
          .and_then(|_| Parser::return_(make_greater_than_or_equal_to as EEE))
      })
      .or_else(|_| {
        parse::whitespaces_char('>').and_then(|_| Parser::return_(make_greater_than as EEE))
      })
      .or_else(|_| {
        parse::whitespaces_string("<=")
          .and_then(|_| Parser::return_(make_less_than_or_equal_to as EEE))
      })
      .or_else(|_| {
        parse::whitespaces_char('<').and_then(|_| Parser::return_(make_less_than as EEE))
      }),
  )
}

fn shift_expression() -> Parser<Expression> {
  fn make_left_shift(expression1: Expression, expression2: Expression) -> Expression {
    psi(Box::new, Expression::LeftShift)(expression1, expression2)
  }

  fn make_right_shift(expression1: Expression, expression2: Expression) -> Expression {
    psi(Box::new, Expression::RightShift)(expression1, expression2)
  }

  parse::binary_operation(
    parse::additive_expression(),
    Parser::error(Error("".to_string()))
      .or_else(|_| {
        parse::whitespaces_string("<<").and_then(|_| Parser::return_(make_left_shift as EEE))
      })
      .or_else(|_| {
        parse::whitespaces_string(">>").and_then(|_| Parser::return_(make_right_shift as EEE))
      }),
  )
}

fn additive_expression() -> Parser<Expression> {
  fn make_addition(expression1: Expression, expression2: Expression) -> Expression {
    psi(Box::new, Expression::Addition)(expression1, expression2)
  }

  fn make_subtraction(expression1: Expression, expression2: Expression) -> Expression {
    psi(Box::new, Expression::Subtraction)(expression1, expression2)
  }

  parse::binary_operation(
    parse::multiplicative_expression(),
    Parser::error(Error("".to_string()))
      .or_else(|_| parse::whitespaces_char('+').and_then(|_| Parser::return_(make_addition as EEE)))
      .or_else(|_| {
        parse::whitespaces_char('-').and_then(|_| Parser::return_(make_subtraction as EEE))
      }),
  )
}

fn multiplicative_expression() -> Parser<Expression> {
  fn make_multiplication(expression1: Expression, expression2: Expression) -> Expression {
    psi(Box::new, Expression::Multiplication)(expression1, expression2)
  }

  fn make_division(expression1: Expression, expression2: Expression) -> Expression {
    psi(Box::new, Expression::Division)(expression1, expression2)
  }

  fn make_modulo(expression1: Expression, expression2: Expression) -> Expression {
    psi(Box::new, Expression::Modulo)(expression1, expression2)
  }

  parse::binary_operation(
    parse::cast_expression(),
    Parser::error(Error("".to_string()))
      .or_else(|_| {
        parse::whitespaces_char('*').and_then(|_| Parser::return_(make_multiplication as EEE))
      })
      .or_else(|_| parse::whitespaces_char('/').and_then(|_| Parser::return_(make_division as EEE)))
      .or_else(|_| parse::whitespaces_char('%').and_then(|_| Parser::return_(make_modulo as EEE))),
  )
}

fn cast_expression() -> Parser<Expression> {
  parse::whitespaces_char('(')
    .and_then(|_| parse::type_name())
    .and_then(|type_name| {
      parse::whitespaces_char(')')
        .and_then(|_| parse::cast_expression())
        .map(move |cast_expression| Expression::Cast(type_name.clone(), Box::new(cast_expression)))
    })
    .or_else(|_| parse::unary_expression())
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
        .and_then(|expression| parse::whitespaces_char(')').map(move |_| expression.clone()))
    })
    .or_else(|_| {
      parse::identifier().and_then(|identifier| {
        Parser::return_(())
          .and_then(|_| parse::whitespaces_char('('))
          .and_then(|_| parse::whitespaces_char(')'))
          .map(move |_| Expression::FunctionCall(identifier.clone()))
      })
    })
    .or_else(|_| parse::integer_constant())
    .or_else(|_| parse::character_constant())
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
  // TODO does not ebey grammar
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

fn character_constant() -> Parser<Expression> {
  // TODO currently only parsing <simple-escape-sequence>s
  parse::many(parse::whitespace())
    .and_then(|_| parse::char('\''))
    .and_then(|_| {
      Parser::error(Error("".to_string()))
        .or_else(|_| parse::satisfy(|x| !"\'\\\n".contains(x)))
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
