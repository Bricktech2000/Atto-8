use crate::*;
use std::rc::Rc;

// elementary parsers

type ParseResult<T> = Result<(T, String), Error>;
type Parser<T> = Rc<dyn Fn(String) -> ParseResult<T>>;

fn any() -> Parser<char> {
  Rc::new(move |input: String| match &input[..] {
    "" => Err(Error(format!("Unexpected end of input"))),
    _ => Ok((input.chars().next().unwrap(), input[1..].to_string())),
  })
}

fn eof() -> Parser<()> {
  Rc::new(move |input: String| match &input[..] {
    "" => Ok(((), input)),
    _ => Err(Error(format!("Expected end of input, got `{}`", input))),
  })
}

// other parsers

fn satisfy(predicate: Rc<dyn Fn(char) -> bool>) -> Parser<char> {
  Rc::new(move |input: String| match parse::any()(input) {
    Ok((char, input)) => match predicate(char) {
      true => Ok((char, input)),
      false => Err(Error(format!("Unexpected character `{}`", char))),
    },
    Err(e) => Err(e),
  })
}

fn char(char: char) -> Parser<()> {
  Rc::new(
    move |input: String| match parse::satisfy(Rc::new(move |x| x == char))(input) {
      Ok((_, input)) => Ok(((), input)),
      Err(e) => Err(e),
    },
  )
}

fn whitespace() -> Parser<()> {
  Rc::new(move |input: String| {
    let (_, input) = Err(())
      .or_else(|_| parse::char(' ')(input.clone()))
      .or_else(|_| parse::char('\n')(input.clone()))
      .or_else(|_| parse::char('\t')(input.clone()))?;
    Ok(((), input))
  })
}

fn decimal_digit() -> Parser<char> {
  parse::satisfy(Rc::new(move |x| x.is_digit(10)))
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

fn whitespaces_eof() -> Parser<()> {
  Rc::new(move |input: String| {
    let (_, input) = parse::many(parse::whitespace())(input.clone())?;
    let ((), input) = parse::eof()(input)?;

    Ok(((), input))
  })
}

fn whitespaces_char(char: char) -> Parser<()> {
  Rc::new(move |input: String| {
    let (_, input) = parse::many(parse::whitespace())(input.clone())?;
    let ((), input) = parse::char(char)(input)?;

    Ok(((), input))
  })
}

fn whitespaces_string(string: &'static str) -> Parser<()> {
  Rc::new(move |input: String| {
    let (_, input) = parse::many(parse::whitespace())(input.clone())?;
    let ((), input) = parse::string(string)(input)?;

    Ok(((), input))
  })
}

// combinators

fn many<T: 'static>(parser: Parser<T>) -> Parser<Vec<T>> {
  Rc::new(move |input: String| {
    let mut result = vec![];
    let mut input = input;
    loop {
      match parser(input.clone()) {
        Ok((result_, input_)) => {
          result.push(result_);
          input = input_;
        }
        Err(_) => break,
      }
    }
    Ok((result, input))
  })
}

fn many1<T: 'static>(parser: Parser<T>) -> Parser<Vec<T>> {
  Rc::new(move |input: String| {
    let (first, input) = parser(input)?;
    let (rest, input) = parse::many(parser.clone())(input)?;
    let mut result = vec![first];
    result.extend(rest);
    Ok((result, input))
  })
}

#[allow(dead_code)]
fn choice<T: 'static>(parser: Vec<Parser<T>>) -> Parser<T> {
  Rc::new(move |input: String| {
    for parser in parser.iter() {
      match parser(input.clone()) {
        Ok((result, input)) => return Ok((result, input)),
        Err(_) => continue,
      }
    }
    Err(Error(format!("No parser succeeded")))
  })
}

// C99 grammar

pub fn parse(input: String) -> Result<Program, Error> {
  let (programm, input) = parse::translation_unit(input)?;

  match &input[..] {
    "" => Ok(programm),
    _ => panic!("Input not fully parsed"),
  }
}

fn translation_unit(input: String) -> ParseResult<Program> {
  // TODO should be `many` instead of `many1`
  let (function_definitions, input) = parse::many1(Rc::new(parse::external_declaration))(input)?;
  let ((), input) = parse::whitespaces_eof()(input)?;

  Ok((
    Program {
      function_definitions,
    },
    input,
  ))
}

fn external_declaration(input: String) -> ParseResult<FunctionDefinition> {
  let (function_definition, input) = parse::function_definition(input)?;

  Ok((function_definition, input))
}

fn function_definition(input: String) -> ParseResult<FunctionDefinition> {
  let name = "main";

  let ((), input) = parse::whitespaces_string("int")(input)?;
  let ((), input) = parse::whitespaces_string(name)(input)?;
  let ((), input) = parse::whitespaces_char('(')(input)?;
  let ((), input) = parse::whitespaces_char(')')(input)?;
  let (statements, input) = parse::compound_statement(input)?;

  Ok((
    FunctionDefinition {
      name: name.to_string(),
      body: statements,
    },
    input,
  ))
}

fn integer_constant(input: String) -> ParseResult<Expression> {
  let (_, input) = parse::many(parse::whitespace())(input)?;
  let (digits, input) = parse::many1(parse::decimal_digit())(input)?;
  let value = String::from_iter(digits.clone()).parse().map_err(|_| {
    Error(format!(
      "Invalid integer constant `{}`",
      digits.iter().collect::<String>()
    ))
  })?;

  Ok((Expression::IntegerConstant(value), input))
}

fn compound_statement(input: String) -> ParseResult<Vec<Statement>> {
  let ((), input) = parse::whitespaces_char('{')(input)?;
  let (statement, input) = parse::magic_return(input)?;
  let ((), input) = parse::whitespaces_char('}')(input)?;

  Ok((vec![statement], input))
}

fn magic_return(input: String) -> ParseResult<Statement> {
  let ((), input) = parse::whitespaces_string("return")(input)?;
  let (expression, input) = parse::constant_expression(input)?; // TODO does not obey grammar
  let ((), input) = parse::whitespaces_char(';')(input)?;

  Ok((Statement::MagicReturn(expression), input)) // TODO does not obey grammar
}

fn constant_expression(input: String) -> ParseResult<Expression> {
  let (expression, input) = parse::conditional_expression(input)?;

  Ok((expression, input))
}

fn conditional_expression(input: String) -> ParseResult<Expression> {
  let result = Err(())
    .or_else(|_| {
      parse::logical_or_expression(input.clone()).and_then(|(expression, input)| {
        parse::whitespaces_char('?')(input.clone()).and_then(|((), input)| {
          parse::expression(input.clone()).and_then(|(expression_, input)| {
            parse::whitespaces_char(':')(input.clone()).and_then(|((), input)| {
              parse::conditional_expression(input.clone()).map(|(expression__, input)| {
                (
                  Expression::Conditional(
                    Box::new(expression.clone()),
                    Box::new(expression_),
                    Box::new(expression__),
                  ),
                  input,
                )
              })
            })
          })
        })
      })
    })
    .or_else(|_| parse::logical_or_expression(input.clone()))
    .or_else(|_| Err(Error(format!("Could not parse conditional expression"))));

  let (expression, input) = result?;

  Ok((expression, input))
}

fn logical_or_expression(input: String) -> ParseResult<Expression> {
  let (mut expression, mut input) = parse::logical_and_expression(input.clone())?;

  loop {
    let result = Err(())
      .or_else(|_| {
        parse::whitespaces_string("||")(input.clone()).and_then(|((), input)| {
          parse::logical_and_expression(input.clone()).map(|(expression_, input)| {
            (
              Expression::LogicalOr(Box::new(expression.clone()), Box::new(expression_)),
              input,
            )
          })
        })
      })
      .or_else(|_| parse::logical_and_expression(input.clone()))
      .or_else(|_| Err(Error(format!("Could not parse logical or expression"))));

    match result {
      Ok((expression_, input_)) => {
        expression = expression_;
        input = input_;
      }
      Err(_) => break,
    }
  }

  Ok((expression, input))
}

fn logical_and_expression(input: String) -> ParseResult<Expression> {
  let (mut expression, mut input) = parse::inclusive_or_expression(input.clone())?;

  loop {
    let result = Err(())
      .or_else(|_| {
        parse::whitespaces_string("&&")(input.clone()).and_then(|((), input)| {
          parse::inclusive_or_expression(input.clone()).map(|(expression_, input)| {
            (
              Expression::LogicalAnd(Box::new(expression.clone()), Box::new(expression_)),
              input,
            )
          })
        })
      })
      .or_else(|_| parse::inclusive_or_expression(input.clone()))
      .or_else(|_| Err(Error(format!("Could not parse logical and expression"))));

    match result {
      Ok((expression_, input_)) => {
        expression = expression_;
        input = input_;
      }
      Err(_) => break,
    }
  }

  Ok((expression, input))
}

fn inclusive_or_expression(input: String) -> ParseResult<Expression> {
  let (mut expression, mut input) = parse::exclusive_or_expression(input.clone())?;

  loop {
    let result = Err(())
      .or_else(|_| {
        parse::whitespaces_char('|')(input.clone()).and_then(|((), input)| {
          parse::exclusive_or_expression(input.clone()).map(|(expression_, input)| {
            (
              Expression::BitwiseInclusiveOr(Box::new(expression.clone()), Box::new(expression_)),
              input,
            )
          })
        })
      })
      .or_else(|_| parse::exclusive_or_expression(input.clone()))
      .or_else(|_| Err(Error(format!("Could not parse inclusive or expression"))));

    match result {
      Ok((expression_, input_)) => {
        expression = expression_;
        input = input_;
      }
      Err(_) => break,
    }
  }

  Ok((expression, input))
}

fn exclusive_or_expression(input: String) -> ParseResult<Expression> {
  let (mut expression, mut input) = parse::and_expression(input.clone())?;

  loop {
    let result = Err(())
      .or_else(|_| {
        parse::whitespaces_char('^')(input.clone()).and_then(|((), input)| {
          parse::and_expression(input.clone()).map(|(expression_, input)| {
            (
              Expression::BitwiseExclusiveOr(Box::new(expression.clone()), Box::new(expression_)),
              input,
            )
          })
        })
      })
      .or_else(|_| parse::and_expression(input.clone()))
      .or_else(|_| Err(Error(format!("Could not parse exclusive or expression"))));

    match result {
      Ok((expression_, input_)) => {
        expression = expression_;
        input = input_;
      }
      Err(_) => break,
    }
  }

  Ok((expression, input))
}

fn and_expression(input: String) -> ParseResult<Expression> {
  let (mut expression, mut input) = parse::equality_expression(input.clone())?;

  loop {
    let result = Err(())
      .or_else(|_| {
        parse::whitespaces_char('&')(input.clone()).and_then(|((), input)| {
          parse::equality_expression(input.clone()).map(|(expression_, input)| {
            (
              Expression::BitwiseAnd(Box::new(expression.clone()), Box::new(expression_)),
              input,
            )
          })
        })
      })
      .or_else(|_| parse::equality_expression(input.clone()))
      .or_else(|_| Err(Error(format!("Could not parse and expression"))));

    match result {
      Ok((expression_, input_)) => {
        expression = expression_;
        input = input_;
      }
      Err(_) => break,
    }
  }

  Ok((expression, input))
}

fn equality_expression(input: String) -> ParseResult<Expression> {
  let (mut expression, mut input) = parse::relational_expression(input.clone())?;

  loop {
    let result = Err(())
      .or_else(|_| {
        parse::whitespaces_string("==")(input.clone()).and_then(|((), input)| {
          parse::relational_expression(input.clone()).map(|(expression_, input)| {
            (
              Expression::EqualTo(Box::new(expression.clone()), Box::new(expression_)),
              input,
            )
          })
        })
      })
      .or_else(|_| {
        parse::whitespaces_string("!=")(input.clone()).and_then(|((), input)| {
          parse::relational_expression(input.clone()).map(|(expression_, input)| {
            (
              Expression::NotEqualTo(Box::new(expression.clone()), Box::new(expression_)),
              input,
            )
          })
        })
      })
      .or_else(|_| parse::relational_expression(input.clone()))
      .or_else(|_| Err(Error(format!("Could not parse equality expression"))));

    match result {
      Ok((expression_, input_)) => {
        expression = expression_;
        input = input_;
      }
      Err(_) => break,
    }
  }

  Ok((expression, input))
}

fn relational_expression(input: String) -> ParseResult<Expression> {
  let (mut expression, mut input) = parse::shift_expression(input.clone())?;

  loop {
    let result = Err(())
      .or_else(|_| {
        parse::whitespaces_string("<=")(input.clone()).and_then(|((), input)| {
          parse::shift_expression(input.clone()).map(|(expression_, input)| {
            (
              Expression::LessThanOrEqualTo(Box::new(expression.clone()), Box::new(expression_)),
              input,
            )
          })
        })
      })
      .or_else(|_| {
        parse::whitespaces_string(">=")(input.clone()).and_then(|((), input)| {
          parse::shift_expression(input.clone()).map(|(expression_, input)| {
            (
              Expression::GreaterThanOrEqualTo(Box::new(expression.clone()), Box::new(expression_)),
              input,
            )
          })
        })
      })
      .or_else(|_| {
        parse::whitespaces_char('<')(input.clone()).and_then(|((), input)| {
          parse::shift_expression(input.clone()).map(|(expression_, input)| {
            (
              Expression::LessThan(Box::new(expression.clone()), Box::new(expression_)),
              input,
            )
          })
        })
      })
      .or_else(|_| {
        parse::whitespaces_char('>')(input.clone()).and_then(|((), input)| {
          parse::shift_expression(input.clone()).map(|(expression_, input)| {
            (
              Expression::GreaterThan(Box::new(expression.clone()), Box::new(expression_)),
              input,
            )
          })
        })
      })
      .or_else(|_| parse::shift_expression(input.clone()))
      .or_else(|_| Err(Error(format!("Could not parse relational expression"))));

    match result {
      Ok((expression_, input_)) => {
        expression = expression_;
        input = input_;
      }
      Err(_) => break,
    }
  }

  Ok((expression, input))
}

fn shift_expression(input: String) -> ParseResult<Expression> {
  let (mut expression, mut input) = parse::additive_expression(input.clone())?;

  loop {
    let result = Err(())
      .or_else(|_| {
        parse::whitespaces_string("<<")(input.clone()).and_then(|((), input)| {
          parse::additive_expression(input.clone()).map(|(expression_, input)| {
            (
              Expression::LeftShift(Box::new(expression.clone()), Box::new(expression_)),
              input,
            )
          })
        })
      })
      .or_else(|_| {
        parse::whitespaces_string(">>")(input.clone()).and_then(|((), input)| {
          parse::additive_expression(input.clone()).map(|(expression_, input)| {
            (
              Expression::RightShift(Box::new(expression.clone()), Box::new(expression_)),
              input,
            )
          })
        })
      })
      .or_else(|_| parse::additive_expression(input.clone()))
      .or_else(|_| Err(Error(format!("Could not parse shift expression"))));

    match result {
      Ok((expression_, input_)) => {
        expression = expression_;
        input = input_;
      }
      Err(_) => break,
    }
  }

  Ok((expression, input))
}

fn expression(input: String) -> ParseResult<Expression> {
  let (expression, input) = parse::constant_expression(input.clone())?; // TODO does not obey grammar

  Ok((expression, input))
}

fn additive_expression(input: String) -> ParseResult<Expression> {
  let (mut expression, mut input) = parse::multiplicative_expression(input.clone())?;

  loop {
    let result = Err(())
      .or_else(|_| {
        parse::whitespaces_char('+')(input.clone()).and_then(|((), input)| {
          parse::multiplicative_expression(input.clone()).map(|(expression_, input)| {
            (
              Expression::Addition(Box::new(expression.clone()), Box::new(expression_)),
              input,
            )
          })
        })
      })
      .or_else(|_| {
        parse::whitespaces_char('-')(input.clone()).and_then(|((), input)| {
          parse::multiplicative_expression(input.clone()).map(|(expression_, input)| {
            (
              Expression::Subtraction(Box::new(expression.clone()), Box::new(expression_)),
              input,
            )
          })
        })
      })
      .or_else(|_| Err(Error(format!("Could not parse additive expression"))));

    match result {
      Ok((expression_, input_)) => {
        expression = expression_;
        input = input_;
      }
      Err(_) => break,
    }
  }

  Ok((expression, input))
}

fn multiplicative_expression(input: String) -> ParseResult<Expression> {
  let (mut expression, mut input) = parse::cast_expression(input.clone())?;

  loop {
    let result = Err(())
      .or_else(|_| {
        parse::whitespaces_char('*')(input.clone()).and_then(|((), input)| {
          parse::cast_expression(input.clone()).map(|(expression_, input)| {
            (
              Expression::Multiplication(Box::new(expression.clone()), Box::new(expression_)),
              input,
            )
          })
        })
      })
      .or_else(|_| {
        parse::whitespaces_char('/')(input.clone()).and_then(|((), input)| {
          parse::cast_expression(input.clone()).map(|(expression_, input)| {
            (
              Expression::Division(Box::new(expression.clone()), Box::new(expression_)),
              input,
            )
          })
        })
      })
      .or_else(|_| {
        parse::whitespaces_char('%')(input.clone()).and_then(|((), input)| {
          parse::cast_expression(input.clone()).map(|(expression_, input)| {
            (
              Expression::Modulo(Box::new(expression.clone()), Box::new(expression_)),
              input,
            )
          })
        })
      })
      .or_else(|_| Err(Error(format!("Could not parse multiplicative expression"))));

    match result {
      Ok((expression_, input_)) => {
        expression = expression_;
        input = input_;
      }
      Err(_) => break,
    }
  }

  Ok((expression, input))
}

fn cast_expression(input: String) -> ParseResult<Expression> {
  let (expression, input) = parse::unary_expression(input)?;

  Ok((expression, input))
}

fn unary_expression(input: String) -> ParseResult<Expression> {
  let result = Err(())
    .or_else(|_| {
      parse::whitespaces_char('-')(input.clone())
        .and_then(|((), input)| parse::cast_expression(input.clone()))
        .map(|(expression, input)| (Expression::Negation(Box::new(expression)), input))
    })
    .or_else(|_| {
      parse::whitespaces_char('~')(input.clone())
        .and_then(|((), input)| parse::cast_expression(input.clone()))
        .map(|(expression, input)| (Expression::BitwiseComplement(Box::new(expression)), input))
    })
    .or_else(|_| {
      parse::whitespaces_char('!')(input.clone())
        .and_then(|((), input)| parse::cast_expression(input.clone()))
        .map(|(expression, input)| (Expression::LogicalNegation(Box::new(expression)), input))
    })
    .or_else(|_| {
      parse::whitespaces_char('(')(input.clone()).and_then(|((), input)| {
        parse::additive_expression(input.clone()).and_then(|(expression, input)| {
          parse::whitespaces_char(')')(input.clone()).map(|((), input)| (expression, input))
        })
      })
    })
    .or_else(|_| parse::integer_constant(input.clone()))
    .or_else(|_| Err(Error(format!("Could not parse unary expression"))));

  let (expression, input) = result?;

  Ok((expression, input))
}
