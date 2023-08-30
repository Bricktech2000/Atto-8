use crate::*;
use std::rc::Rc;

// elementary parsers

type ParseResult<T> = Result<(T, String), Error>;
type Parser<T> = Rc<dyn Fn(String) -> ParseResult<T>>;

fn any(input: String) -> ParseResult<char> {
  match &input[..] {
    "" => Err(Error(format!("Unexpected end of input"))),
    _ => Ok((input.chars().next().unwrap(), input[1..].to_string())),
  }
}

fn eof(input: String) -> ParseResult<()> {
  match &input[..] {
    "" => Ok(((), input)),
    _ => Err(Error(format!("Expected end of input, got `{}`", input))),
  }
}

// parser combinators

fn satisfy(predicate: Rc<dyn Fn(char) -> bool>) -> Parser<char> {
  Rc::new(move |input: String| {
    parse::any(input).and_then(|(char, input)| match predicate(char) {
      true => Ok((char, input)),
      false => Err(Error(format!("Unexpected character `{}`", char))),
    })
  })
}

fn char(char: char) -> Parser<()> {
  Rc::new(move |input: String| {
    parse::satisfy(Rc::new(move |x| x == char))(input).map(|(_, input)| ((), input))
  })
}

fn string(string: &'static str) -> Parser<()> {
  Rc::new(move |input: String| {
    string
      .chars()
      .map(parse::char)
      .fold(Ok(((), input)), |acc, parser| {
        acc.and_then(|((), input)| parser(input))
      })
  })
}

fn many1<T: 'static>(parser: Parser<T>) -> Parser<Vec<T>> {
  Rc::new(move |input: String| {
    let (first, input) = parser(input)?;
    let mut input = input;
    let mut results = vec![first];
    loop {
      match parser(input.clone()) {
        Ok((match_, input_)) => {
          results.push(match_);
          input = input_;
        }
        Err(_) => break,
      }
    }
    Ok((results, input))
  })
}

fn many<T: 'static>(parser: Parser<T>) -> Parser<Vec<T>> {
  Rc::new(move |input: String| {
    parse::many1(parser.clone())(input.clone()).or_else(|_| Ok((vec![], input)))
  })
}

fn sepby1<T: 'static, S: 'static>(parser: Parser<T>, separator: Parser<S>) -> Parser<Vec<T>> {
  Rc::new(move |input: String| {
    let parser = parser.clone();
    let separator = separator.clone();
    let (first, input) = parser(input)?;
    let (rest, input) = parse::many(Rc::new(move |input: String| {
      let (_, input) = separator(input)?;
      let (result, input) = parser(input)?;
      Ok((result, input))
    }))(input)?;
    let mut results = vec![first];
    results.extend(rest);
    Ok((results, input))
  })
}

fn sepby<T: 'static, S: 'static>(parser: Parser<T>, separator: Parser<S>) -> Parser<Vec<T>> {
  Rc::new(move |input: String| {
    parse::sepby1(parser.clone(), separator.clone())(input.clone()).or_else(|_| Ok((vec![], input)))
  })
}

fn binary_operation<T: 'static + Clone>(
  parser: Parser<T>,
  operators_constructors: Vec<(Parser<()>, fn(Box<T>, Box<T>) -> T)>,
) -> Parser<T> {
  Rc::new(move |input: String| {
    let (mut expression1, mut input) = parser(input)?;

    loop {
      let result = Ok(((), input.clone())).and_then(|((), input)| {
        operators_constructors.iter().fold(
          Err(Error(format!("No matching operator"))),
          |acc, (operator, constructor)| {
            acc.or_else(|_| {
              operator(input.clone()).and_then(|((), input)| {
                parser(input).map(|(expression2, input)| {
                  (
                    constructor(Box::new(expression1.clone()), Box::new(expression2)),
                    input,
                  )
                })
              })
            })
          },
        )
      });

      match result {
        Ok((expression, input_)) => {
          expression1 = expression;
          input = input_;
        }
        Err(_) => break Ok((expression1, input)),
      }
    }
  })
}

// C99 grammar

pub fn parse(input: String) -> Result<Program, Error> {
  Ok(((), input))
    .and_then(|((), input)| parse::translation_unit(input))
    .map(|(programm, input)| match &input[..] {
      "" => programm,
      _ => panic!("Input not fully parsed"),
    })
}

fn translation_unit(input: String) -> ParseResult<Program> {
  // TODO should be `many` instead of `many1`
  Ok(((), input))
    .and_then(|((), input)| parse::many1(Rc::new(parse::external_declaration))(input))
    .and_then(|(function_definitions, input)| {
      Ok(((), input))
        .and_then(|((), input)| parse::whitespaces_eof(input))
        .map(|((), input)| {
          (
            Program {
              function_definitions,
            },
            input,
          )
        })
    })
}

fn external_declaration(input: String) -> ParseResult<FunctionDefinition> {
  Err(()).or_else(|_| parse::function_definition(input.clone()))
}

fn function_definition(input: String) -> ParseResult<FunctionDefinition> {
  Ok(((), input))
    .and_then(|((), input)| parse::whitespaces_string("int")(input))
    .and_then(|((), input)| parse::identifier(input))
    .and_then(|(identifier, input)| {
      Ok(((), input))
        .and_then(|((), input)| parse::whitespaces_char('(')(input))
        .and_then(|((), input)| parse::whitespaces_char(')')(input))
        .and_then(|((), input)| parse::compound_statement(input))
        .map(|(statements, input)| {
          (
            FunctionDefinition(Type::BasicType(BasicType::Int), identifier, statements),
            input,
          )
        })
    })
}

fn integer_constant(input: String) -> ParseResult<Expression> {
  Ok(((), input))
    .and_then(|((), input)| parse::many(Rc::new(parse::whitespace))(input))
    .and_then(|(_, input)| parse::many1(Rc::new(parse::digit))(input))
    .and_then(|(digits, input)| {
      String::from_iter(digits.clone())
        .parse()
        .map(|value| (value, input))
        .map_err(|_| {
          Error(format!(
            "Invalid integer constant `{}`",
            digits.iter().collect::<String>()
          ))
        })
    })
    .map(|(value, input)| (Expression::IntegerConstant(value), input))
}

fn compound_statement(input: String) -> ParseResult<Vec<Statement>> {
  // TODO should be {<declaration>}* {<statement>}*
  Ok(((), input))
    .and_then(|((), input)| parse::whitespaces_char('{')(input))
    .and_then(|((), input)| parse::many(Rc::new(parse::statement))(input))
    .and_then(|(statements, input)| {
      Ok(((), input))
        .and_then(|((), input)| parse::whitespaces_char('}')(input))
        .map(|((), input)| (statements, input))
    })
}

fn statement(input: String) -> ParseResult<Statement> {
  // TODO cases missing
  Err(())
    .or_else(|_| parse::jump_statement(input.clone()))
    .or_else(|_| parse::expression_statement(input.clone()))
    .or_else(|_| parse::asm_statement(input.clone())) // TODO does not obey grammar
}

fn jump_statement(input: String) -> ParseResult<Statement> {
  // TODO cases missing
  Err(()).or_else(|_| {
    Ok(((), input))
      .and_then(|((), input)| parse::whitespaces_string("return")(input))
      .and_then(|((), input)| parse::constant_expression(input)) // TODO does not obey grammar
      .and_then(|(expression, input)| {
        Ok(((), input))
          .and_then(|((), input)| parse::whitespaces_char(';')(input))
          .map(|(_, input)| (Statement::Return(expression), input))
      })
  })
}

fn expression_statement(input: String) -> ParseResult<Statement> {
  // TODO cases missing
  Err(()).or_else(|_| {
    Ok(((), input))
      .and_then(|((), input)| parse::expression(input.clone()))
      .and_then(|(expression, input)| {
        Ok(((), input))
          .and_then(|((), input)| parse::whitespaces_char(';')(input))
          .map(|(_, input)| (Statement::Expression(expression), input))
      })
  })
}

fn asm_statement(input: String) -> ParseResult<Statement> {
  Ok(((), input))
    .and_then(|((), input)| parse::whitespaces_string("asm")(input))
    .and_then(|((), input)| parse::whitespaces_char('(')(input))
    .and_then(|((), input)| {
      parse::sepby(Rc::new(parse::expression), parse::whitespaces_char(','))(input)
    })
    .and_then(|(expressions, input)| {
      Ok(((), input))
        .and_then(|((), input)| parse::whitespaces_char(')')(input))
        .and_then(|((), input)| parse::whitespaces_char('{')(input))
        .and_then(|((), input)| parse::many(parse::satisfy(Rc::new(move |c| c != '}')))(input))
        .map(|(chars, input)| (chars.iter().collect(), input))
        .and_then(|(assembly, input)| {
          Ok(((), input))
            .and_then(|((), input)| parse::whitespaces_char('}')(input))
            .map(|((), input)| (Statement::Asm(expressions, assembly), input))
        })
    })
}

fn constant_expression(input: String) -> ParseResult<Expression> {
  Err(()).or_else(|_| parse::conditional_expression(input.clone()))
}

fn conditional_expression(input: String) -> ParseResult<Expression> {
  Err(())
    .or_else(|_| {
      Ok(((), input.clone()))
        .and_then(|((), input)| parse::logical_or_expression(input))
        .and_then(|(expression1, input)| {
          Ok(((), input))
            .and_then(|((), input)| parse::whitespaces_char('?')(input))
            .and_then(|((), input)| parse::expression(input))
            .and_then(|(expression2, input)| {
              Ok(((), input))
                .and_then(|((), input)| parse::whitespaces_char(':')(input))
                .and_then(|((), input)| parse::conditional_expression(input))
                .map(|(expression3, input)| {
                  (
                    Expression::Conditional(
                      Box::new(expression1),
                      Box::new(expression2),
                      Box::new(expression3),
                    ),
                    input,
                  )
                })
            })
        })
    })
    .or_else(|_| parse::logical_or_expression(input.clone()))
    .or_else(|_| Err(Error(format!("Could not parse conditional expression"))))
}

fn expression(input: String) -> ParseResult<Expression> {
  parse::constant_expression(input) // TODO does not obey grammar
}

fn logical_or_expression(input: String) -> ParseResult<Expression> {
  binary_operation(
    Rc::new(parse::logical_and_expression),
    vec![(parse::whitespaces_string("||"), Expression::LogicalOr)],
  )(input)
}

fn logical_and_expression(input: String) -> ParseResult<Expression> {
  binary_operation(
    Rc::new(parse::inclusive_or_expression),
    vec![(parse::whitespaces_string("&&"), Expression::LogicalAnd)],
  )(input)
}

fn inclusive_or_expression(input: String) -> ParseResult<Expression> {
  binary_operation(
    Rc::new(parse::exclusive_or_expression),
    vec![(parse::whitespaces_char('|'), Expression::BitwiseInclusiveOr)],
  )(input)
}

fn exclusive_or_expression(input: String) -> ParseResult<Expression> {
  binary_operation(
    Rc::new(parse::and_expression),
    vec![(parse::whitespaces_char('^'), Expression::BitwiseExclusiveOr)],
  )(input)
}

fn and_expression(input: String) -> ParseResult<Expression> {
  binary_operation(
    Rc::new(parse::equality_expression),
    vec![(parse::whitespaces_char('&'), Expression::BitwiseAnd)],
  )(input)
}

fn equality_expression(input: String) -> ParseResult<Expression> {
  binary_operation(
    Rc::new(parse::relational_expression),
    vec![
      (parse::whitespaces_string("=="), Expression::EqualTo),
      (parse::whitespaces_string("!="), Expression::NotEqualTo),
    ],
  )(input)
}

fn relational_expression(input: String) -> ParseResult<Expression> {
  binary_operation(
    Rc::new(parse::shift_expression),
    vec![
      (parse::whitespaces_char('>'), Expression::GreaterThan),
      (
        parse::whitespaces_string(">="),
        Expression::GreaterThanOrEqualTo,
      ),
      (parse::whitespaces_char('<'), Expression::LessThan),
      (
        parse::whitespaces_string("<="),
        Expression::LessThanOrEqualTo,
      ),
    ],
  )(input)
}

fn shift_expression(input: String) -> ParseResult<Expression> {
  binary_operation(
    Rc::new(parse::additive_expression),
    vec![
      (parse::whitespaces_string("<<"), Expression::LeftShift),
      (parse::whitespaces_string(">>"), Expression::RightShift),
    ],
  )(input)
}

fn additive_expression(input: String) -> ParseResult<Expression> {
  binary_operation(
    Rc::new(parse::multiplicative_expression),
    vec![
      (parse::whitespaces_char('+'), Expression::Addition),
      (parse::whitespaces_char('-'), Expression::Subtraction),
    ],
  )(input)
}

fn multiplicative_expression(input: String) -> ParseResult<Expression> {
  binary_operation(
    Rc::new(parse::cast_expression),
    vec![
      (parse::whitespaces_char('*'), Expression::Multiplication),
      (parse::whitespaces_char('/'), Expression::Division),
      (parse::whitespaces_char('%'), Expression::Modulo),
    ],
  )(input)
}

fn cast_expression(input: String) -> ParseResult<Expression> {
  parse::unary_expression(input) // TODO does not obey grammar
}

fn unary_expression(input: String) -> ParseResult<Expression> {
  Err(())
    .or_else(|_| {
      Ok(((), input.clone()))
        .and_then(|((), input)| parse::whitespaces_char('-')(input))
        .and_then(|((), input)| parse::cast_expression(input))
        .map(|(expression, input)| (Expression::Negation(Box::new(expression)), input))
    })
    .or_else(|_| {
      Ok(((), input.clone()))
        .and_then(|((), input)| parse::whitespaces_char('~')(input))
        .and_then(|((), input)| parse::cast_expression(input))
        .map(|(expression, input)| (Expression::BitwiseComplement(Box::new(expression)), input))
    })
    .or_else(|_| {
      Ok(((), input.clone()))
        .and_then(|((), input)| parse::whitespaces_char('!')(input))
        .and_then(|((), input)| parse::cast_expression(input))
        .map(|(expression, input)| (Expression::LogicalNegation(Box::new(expression)), input))
    })
    .or_else(|_| {
      Ok(((), input.clone()))
        .and_then(|((), input)| parse::whitespaces_char('(')(input))
        .and_then(|((), input)| parse::additive_expression(input))
        .and_then(|(expression, input)| {
          Ok(((), input))
            .and_then(|(_, input)| parse::whitespaces_char(')')(input))
            .map(|(_, input)| (expression, input))
        })
    })
    .or_else(|_| {
      Ok(((), input.clone()))
        .and_then(|((), input)| parse::identifier(input))
        .and_then(|(identifier, input)| {
          Ok(((), input))
            .and_then(|(_, input)| parse::whitespaces_char('(')(input))
            .and_then(|((), input)| parse::whitespaces_char(')')(input))
            .map(|((), input)| (Expression::FunctionCall(identifier), input)) // TODO does not obey grammar
        })
    })
    .or_else(|_| parse::integer_constant(input.clone()))
    .or_else(|_| Err(Error(format!("Could not parse unary expression"))))
}

fn identifier(input: String) -> ParseResult<String> {
  Ok(((), input))
    .and_then(|((), input)| parse::many(Rc::new(whitespace))(input))
    .and_then(|(_, input)| parse::nondigit(input))
    .and_then(|(first, input)| {
      parse::many(Rc::new(move |input| {
        Err(())
          .or_else(|_| parse::digit(input.clone()))
          .or_else(|_| parse::nondigit(input.clone()))
          .or_else(|_| Err(Error(format!("Could not parse identifier"))))
      }))(input)
      .map(|(rest, input)| (std::iter::once(first).chain(rest).collect(), input))
    })
}

fn whitespace(input: String) -> ParseResult<()> {
  Err(())
    .or_else(|_| parse::char(' ')(input.clone()))
    .or_else(|_| parse::char('\n')(input.clone()))
    .or_else(|_| parse::char('\t')(input.clone()))
}

fn digit(input: String) -> ParseResult<char> {
  parse::satisfy(Rc::new(move |x| x.is_digit(10)))(input)
}

fn nondigit(input: String) -> ParseResult<char> {
  parse::satisfy(Rc::new(move |x| x.is_alphabetic() || x == '_'))(input)
}

fn whitespaces_eof(input: String) -> ParseResult<()> {
  Ok(((), input))
    .and_then(|((), input)| parse::many(Rc::new(parse::whitespace))(input))
    .and_then(|(_, input)| parse::eof(input))
}

fn whitespaces_char(char: char) -> Parser<()> {
  Rc::new(move |input: String| {
    Ok(((), input))
      .and_then(|((), input)| parse::many(Rc::new(parse::whitespace))(input))
      .and_then(|(_, input)| parse::char(char)(input))
  })
}

fn whitespaces_string(string: &'static str) -> Parser<()> {
  Rc::new(move |input: String| {
    Ok(((), input))
      .and_then(|((), input)| parse::many(Rc::new(parse::whitespace))(input))
      .and_then(|(_, input)| parse::string(string)(input))
  })
}
