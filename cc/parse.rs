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

fn digit() -> Parser<char> {
  parse::satisfy(Rc::new(move |x| x.is_digit(10)))
}

fn nondigit() -> Parser<char> {
  parse::satisfy(Rc::new(move |x| x.is_alphabetic() || x == '_'))
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
  let ((), input) = parse::whitespaces_string("int")(input)?;
  let (identifier, input) = parse::identifier(input)?;
  let ((), input) = parse::whitespaces_char('(')(input)?;
  let ((), input) = parse::whitespaces_char(')')(input)?;
  let (statements, input) = parse::compound_statement(input)?;

  Ok((
    FunctionDefinition(Type::BasicType(BasicType::Int), identifier, statements),
    input,
  ))
}

fn integer_constant(input: String) -> ParseResult<Expression> {
  let (_, input) = parse::many(parse::whitespace())(input)?;
  let (digits, input) = parse::many1(parse::digit())(input)?;
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
  let (statements, input) = parse::many(Rc::new(parse::statement))(input)?;
  let ((), input) = parse::whitespaces_char('}')(input)?;

  Ok((statements, input)) // TODO should be {<declaration>}* {<statement>}*
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
    parse::whitespaces_string("return")(input.clone()).and_then(|((), input)| {
      parse::constant_expression(input) // TODO does not obey grammar
        .and_then(|(expression, input)| {
          parse::whitespaces_char(';')(input)
            .map(|(_, input)| (Statement::Return(expression), input))
        })
    })
  })
}

fn expression_statement(input: String) -> ParseResult<Statement> {
  // TODO cases missing
  Err(()).or_else(|_| {
    parse::expression(input.clone()).and_then(|(expression, input)| {
      parse::whitespaces_char(';')(input)
        .map(|(_, input)| (Statement::Expression(expression), input))
    })
  })
}

fn asm_statement(input: String) -> ParseResult<Statement> {
  parse::whitespaces_string("asm")(input.clone()).and_then(|((), input)| {
    parse::whitespaces_char('(')(input.clone())
      .and_then(|((), input)| {
        parse::sepby(Rc::new(parse::expression), parse::whitespaces_char(','))(input.clone())
      })
      .and_then(|(expressions, input)| {
        parse::whitespaces_char(')')(input.clone()).and_then(|((), input)| {
          parse::whitespaces_char('{')(input.clone()).and_then(|((), input)| {
            parse::many(parse::satisfy(Rc::new(move |c| c != '}')))(input.clone())
              .map(|(chars, input)| (chars.iter().collect(), input))
              .and_then(|(assembly, input)| {
                parse::whitespaces_char('}')(input.clone())
                  .map(|((), input)| (Statement::Asm(expressions, assembly), input))
              })
          })
        })
      })
  })
}

fn constant_expression(input: String) -> ParseResult<Expression> {
  parse::conditional_expression(input)
}

fn conditional_expression(input: String) -> ParseResult<Expression> {
  Err(())
    .or_else(|_| {
      parse::logical_or_expression(input.clone()).and_then(|(expression1, input)| {
        parse::whitespaces_char('?')(input.clone()).and_then(|((), input)| {
          parse::expression(input.clone()).and_then(|(expression2, input)| {
            parse::whitespaces_char(':')(input.clone()).and_then(|((), input)| {
              parse::conditional_expression(input.clone()).map(|(expression3, input)| {
                (
                  Expression::Conditional(
                    Box::new(expression1.clone()),
                    Box::new(expression2),
                    Box::new(expression3),
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
    .or_else(|_| Err(Error(format!("Could not parse conditional expression"))))
}

fn logical_or_expression(input: String) -> ParseResult<Expression> {
  let (mut expression1, mut input) = parse::logical_and_expression(input.clone())?;

  loop {
    let result = Err(())
      .or_else(|_| {
        parse::whitespaces_string("||")(input.clone()).and_then(|((), input)| {
          parse::logical_and_expression(input.clone()).map(|(expression2, input)| {
            (
              Expression::LogicalOr(Box::new(expression1.clone()), Box::new(expression2)),
              input,
            )
          })
        })
      })
      .or_else(|_| parse::logical_and_expression(input.clone()))
      .or_else(|_| Err(Error(format!("Could not parse logical or expression"))));

    match result {
      Ok((expression, input_)) => {
        expression1 = expression;
        input = input_;
      }
      Err(_) => break Ok((expression1, input)),
    }
  }
}

fn logical_and_expression(input: String) -> ParseResult<Expression> {
  let (mut expression1, mut input) = parse::inclusive_or_expression(input.clone())?;

  loop {
    let result = Err(())
      .or_else(|_| {
        parse::whitespaces_string("&&")(input.clone()).and_then(|((), input)| {
          parse::inclusive_or_expression(input.clone()).map(|(expression2, input)| {
            (
              Expression::LogicalAnd(Box::new(expression1.clone()), Box::new(expression2)),
              input,
            )
          })
        })
      })
      .or_else(|_| parse::inclusive_or_expression(input.clone()))
      .or_else(|_| Err(Error(format!("Could not parse logical and expression"))));

    match result {
      Ok((expression, input_)) => {
        expression1 = expression;
        input = input_;
      }
      Err(_) => break Ok((expression1, input)),
    }
  }
}

fn inclusive_or_expression(input: String) -> ParseResult<Expression> {
  let (mut expression1, mut input) = parse::exclusive_or_expression(input.clone())?;

  loop {
    let result = Err(())
      .or_else(|_| {
        parse::whitespaces_char('|')(input.clone()).and_then(|((), input)| {
          parse::exclusive_or_expression(input.clone()).map(|(expression2, input)| {
            (
              Expression::BitwiseInclusiveOr(Box::new(expression1.clone()), Box::new(expression2)),
              input,
            )
          })
        })
      })
      .or_else(|_| parse::exclusive_or_expression(input.clone()))
      .or_else(|_| Err(Error(format!("Could not parse inclusive or expression"))));

    match result {
      Ok((expression, input_)) => {
        expression1 = expression;
        input = input_;
      }
      Err(_) => break Ok((expression1, input)),
    }
  }
}

fn exclusive_or_expression(input: String) -> ParseResult<Expression> {
  let (mut expression1, mut input) = parse::and_expression(input.clone())?;

  loop {
    let result = Err(())
      .or_else(|_| {
        parse::whitespaces_char('^')(input.clone()).and_then(|((), input)| {
          parse::and_expression(input.clone()).map(|(expression2, input)| {
            (
              Expression::BitwiseExclusiveOr(Box::new(expression1.clone()), Box::new(expression2)),
              input,
            )
          })
        })
      })
      .or_else(|_| parse::and_expression(input.clone()))
      .or_else(|_| Err(Error(format!("Could not parse exclusive or expression"))));

    match result {
      Ok((expression, input_)) => {
        expression1 = expression;
        input = input_;
      }
      Err(_) => break Ok((expression1, input)),
    }
  }
}

fn and_expression(input: String) -> ParseResult<Expression> {
  let (mut expression1, mut input) = parse::equality_expression(input.clone())?;

  loop {
    let result = Err(())
      .or_else(|_| {
        parse::whitespaces_char('&')(input.clone()).and_then(|((), input)| {
          parse::equality_expression(input.clone()).map(|(expression2, input)| {
            (
              Expression::BitwiseAnd(Box::new(expression1.clone()), Box::new(expression2)),
              input,
            )
          })
        })
      })
      .or_else(|_| parse::equality_expression(input.clone()))
      .or_else(|_| Err(Error(format!("Could not parse and expression"))));

    match result {
      Ok((expression, input_)) => {
        expression1 = expression;
        input = input_;
      }
      Err(_) => break Ok((expression1, input)),
    }
  }
}

fn equality_expression(input: String) -> ParseResult<Expression> {
  let (mut expression1, mut input) = parse::relational_expression(input.clone())?;

  loop {
    let result = Err(())
      .or_else(|_| {
        parse::whitespaces_string("==")(input.clone()).and_then(|((), input)| {
          parse::relational_expression(input.clone()).map(|(expression2, input)| {
            (
              Expression::EqualTo(Box::new(expression1.clone()), Box::new(expression2)),
              input,
            )
          })
        })
      })
      .or_else(|_| {
        parse::whitespaces_string("!=")(input.clone()).and_then(|((), input)| {
          parse::relational_expression(input.clone()).map(|(expression2, input)| {
            (
              Expression::NotEqualTo(Box::new(expression1.clone()), Box::new(expression2)),
              input,
            )
          })
        })
      })
      .or_else(|_| parse::relational_expression(input.clone()))
      .or_else(|_| Err(Error(format!("Could not parse equality expression"))));

    match result {
      Ok((expression, input_)) => {
        expression1 = expression;
        input = input_;
      }
      Err(_) => break Ok((expression1, input)),
    }
  }
}

fn relational_expression(input: String) -> ParseResult<Expression> {
  let (mut expression1, mut input) = parse::shift_expression(input.clone())?;

  loop {
    let result = Err(())
      .or_else(|_| {
        parse::whitespaces_string("<=")(input.clone()).and_then(|((), input)| {
          parse::shift_expression(input.clone()).map(|(expression2, input)| {
            (
              Expression::LessThanOrEqualTo(Box::new(expression1.clone()), Box::new(expression2)),
              input,
            )
          })
        })
      })
      .or_else(|_| {
        parse::whitespaces_string(">=")(input.clone()).and_then(|((), input)| {
          parse::shift_expression(input.clone()).map(|(expression2, input)| {
            (
              Expression::GreaterThanOrEqualTo(
                Box::new(expression1.clone()),
                Box::new(expression2),
              ),
              input,
            )
          })
        })
      })
      .or_else(|_| {
        parse::whitespaces_char('<')(input.clone()).and_then(|((), input)| {
          parse::shift_expression(input.clone()).map(|(expression2, input)| {
            (
              Expression::LessThan(Box::new(expression1.clone()), Box::new(expression2)),
              input,
            )
          })
        })
      })
      .or_else(|_| {
        parse::whitespaces_char('>')(input.clone()).and_then(|((), input)| {
          parse::shift_expression(input.clone()).map(|(expression2, input)| {
            (
              Expression::GreaterThan(Box::new(expression1.clone()), Box::new(expression2)),
              input,
            )
          })
        })
      })
      .or_else(|_| parse::shift_expression(input.clone()))
      .or_else(|_| Err(Error(format!("Could not parse relational expression"))));

    match result {
      Ok((expression, input_)) => {
        expression1 = expression;
        input = input_;
      }
      Err(_) => break Ok((expression1, input)),
    }
  }
}

fn shift_expression(input: String) -> ParseResult<Expression> {
  let (mut expression1, mut input) = parse::additive_expression(input.clone())?;

  loop {
    let result = Err(())
      .or_else(|_| {
        parse::whitespaces_string("<<")(input.clone()).and_then(|((), input)| {
          parse::additive_expression(input.clone()).map(|(expression2, input)| {
            (
              Expression::LeftShift(Box::new(expression1.clone()), Box::new(expression2)),
              input,
            )
          })
        })
      })
      .or_else(|_| {
        parse::whitespaces_string(">>")(input.clone()).and_then(|((), input)| {
          parse::additive_expression(input.clone()).map(|(expression2, input)| {
            (
              Expression::RightShift(Box::new(expression1.clone()), Box::new(expression2)),
              input,
            )
          })
        })
      })
      .or_else(|_| parse::additive_expression(input.clone()))
      .or_else(|_| Err(Error(format!("Could not parse shift expression"))));

    match result {
      Ok((expression, input_)) => {
        expression1 = expression;
        input = input_;
      }
      Err(_) => break Ok((expression1, input)),
    }
  }
}

fn expression(input: String) -> ParseResult<Expression> {
  let (expression, input) = parse::constant_expression(input.clone())?; // TODO does not obey grammar

  Ok((expression, input))
}

fn additive_expression(input: String) -> ParseResult<Expression> {
  let (mut expression1, mut input) = parse::multiplicative_expression(input.clone())?;

  loop {
    let result = Err(())
      .or_else(|_| {
        parse::whitespaces_char('+')(input.clone()).and_then(|((), input)| {
          parse::multiplicative_expression(input.clone()).map(|(expression2, input)| {
            (
              Expression::Addition(Box::new(expression1.clone()), Box::new(expression2)),
              input,
            )
          })
        })
      })
      .or_else(|_| {
        parse::whitespaces_char('-')(input.clone()).and_then(|((), input)| {
          parse::multiplicative_expression(input.clone()).map(|(expression2, input)| {
            (
              Expression::Subtraction(Box::new(expression1.clone()), Box::new(expression2)),
              input,
            )
          })
        })
      })
      .or_else(|_| Err(Error(format!("Could not parse additive expression"))));

    match result {
      Ok((expression, input_)) => {
        expression1 = expression;
        input = input_;
      }
      Err(_) => break Ok((expression1, input)),
    }
  }
}

fn multiplicative_expression(input: String) -> ParseResult<Expression> {
  let (mut expression1, mut input) = parse::cast_expression(input.clone())?;

  loop {
    let result = Err(())
      .or_else(|_| {
        parse::whitespaces_char('*')(input.clone()).and_then(|((), input)| {
          parse::cast_expression(input.clone()).map(|(expression2, input)| {
            (
              Expression::Multiplication(Box::new(expression1.clone()), Box::new(expression2)),
              input,
            )
          })
        })
      })
      .or_else(|_| {
        parse::whitespaces_char('/')(input.clone()).and_then(|((), input)| {
          parse::cast_expression(input.clone()).map(|(expression2, input)| {
            (
              Expression::Division(Box::new(expression1.clone()), Box::new(expression2)),
              input,
            )
          })
        })
      })
      .or_else(|_| {
        parse::whitespaces_char('%')(input.clone()).and_then(|((), input)| {
          parse::cast_expression(input.clone()).map(|(expression2, input)| {
            (
              Expression::Modulo(Box::new(expression1.clone()), Box::new(expression2)),
              input,
            )
          })
        })
      })
      .or_else(|_| Err(Error(format!("Could not parse multiplicative expression"))));

    match result {
      Ok((expression, input_)) => {
        expression1 = expression;
        input = input_;
      }
      Err(_) => break Ok((expression1, input)),
    }
  }
}

fn cast_expression(input: String) -> ParseResult<Expression> {
  let (expression, input) = parse::unary_expression(input)?;

  Ok((expression, input))
}

fn unary_expression(input: String) -> ParseResult<Expression> {
  Err(())
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
    .or_else(|_| {
      parse::identifier(input.clone()).and_then(|(identifier, input)| {
        parse::whitespaces_char('(')(input.clone()).and_then(|((), input)| {
          parse::whitespaces_char(')')(input.clone())
            .map(|((), input)| (Expression::FunctionCall(identifier), input)) // TODO does not obey grammar
        })
      })
    })
    .or_else(|_| parse::integer_constant(input.clone()))
    .or_else(|_| Err(Error(format!("Could not parse unary expression"))))
}

fn identifier(input: String) -> ParseResult<String> {
  parse::many(parse::whitespace())(input.clone()).and_then(|(_, input)| {
    parse::nondigit()(input.clone()).and_then(|(first, input)| {
      parse::many(Rc::new(|input| {
        Err(())
          .or_else(|_| parse::digit()(input.clone()))
          .or_else(|_| parse::nondigit()(input.clone()))
          .or_else(|_| Err(Error(format!("Could not parse identifier"))))
          .map(|(character, input_)| (character, input_))
      }))(input.clone())
      .map(|(rest, input)| (vec![vec![first], rest].iter().flatten().collect(), input))
    })
  })
}
