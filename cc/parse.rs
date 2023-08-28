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
    let (_, input) = parse::char(' ')(input.clone())
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

fn translation_unit(input: String) -> ParseResult<TranslationUnit> {
  // TODO should be `many` instead of `many1`
  let (external_declarations, input) = parse::many1(Rc::new(parse::external_declaration))(input)?;
  let ((), input) = parse::whitespaces_eof()(input)?;

  Ok((
    TranslationUnit::ExternalDeclarations(external_declarations),
    input,
  ))
}

fn external_declaration(input: String) -> ParseResult<ExternalDeclaration> {
  let (function_definition, input) = parse::function_definition(input)?;

  Ok((
    ExternalDeclaration::FunctionDefinition(function_definition),
    input,
  ))
}

fn function_definition(input: String) -> ParseResult<FunctionDefinition> {
  let (_, input) = parse::whitespaces_string("int")(input)?;
  let (_, input) = parse::whitespaces_string("main")(input)?;
  let ((), input) = parse::whitespaces_char('(')(input)?;
  let ((), input) = parse::whitespaces_char(')')(input)?;
  let (compound_statement, input) = parse::compound_statement(input)?;

  Ok((
    FunctionDefinition::NameBody("main".to_string(), compound_statement),
    input,
  ))
}

fn integer_constant(input: String) -> ParseResult<IntegerConstant> {
  let (_, input) = parse::many(parse::whitespace())(input)?;
  let (digits, input) = parse::many1(parse::decimal_digit())(input)?;
  let value = String::from_iter(digits).parse::<u8>().unwrap();

  Ok((IntegerConstant::IntegerConstant(value), input))
}

fn compound_statement(input: String) -> ParseResult<CompoundStatement> {
  let ((), input) = parse::whitespaces_char('{')(input)?;
  let (magic_return, input) = parse::magic_return(input)?;
  let ((), input) = parse::whitespaces_char('}')(input)?;

  Ok((CompoundStatement::MagicReturn(magic_return), input))
}

fn magic_return(input: String) -> ParseResult<MagicReturn> {
  let ((), input) = parse::whitespaces_string("return")(input)?;
  let (additive_expression, input) = parse::additive_expression(input)?;
  let ((), input) = parse::whitespaces_char(';')(input)?;

  Ok((MagicReturn::AdditiveExpression(additive_expression), input))
}

fn additive_expression(input: String) -> ParseResult<AdditiveExpression> {
  fn multiplicative_expression(input: String) -> ParseResult<AdditiveExpression> {
    let (multiplicative_expression, input) = parse::multiplicative_expression(input)?;

    Ok((
      AdditiveExpression::MultiplicativeExpression(multiplicative_expression),
      input,
    ))
  }

  fn additive_operator_multiplicative_expression(
    input: String,
  ) -> Result<((AdditiveOperator, MultiplicativeExpression), String), Error> {
    let (additive_operator, input) = parse::additive_operator(input)?;
    let (multiplicative_expression, input) = parse::multiplicative_expression(input)?;

    Ok(((additive_operator, multiplicative_expression), input))
  }

  let (mut multiplicative_expression, mut input) = multiplicative_expression(input)?;
  loop {
    match additive_operator_multiplicative_expression(input.clone()) {
      Ok(((additive_operator, multiplicative_expression_), input_)) => {
        multiplicative_expression =
          AdditiveExpression::AdditiveExpressionAdditiveOperatorMultiplicativeExpression(
            Box::new(multiplicative_expression),
            additive_operator,
            multiplicative_expression_,
          );
        input = input_;
      }
      Err(_) => break,
    }
  }

  Ok((multiplicative_expression, input))
}

fn multiplicative_expression(input: String) -> ParseResult<MultiplicativeExpression> {
  fn cast_expression(input: String) -> ParseResult<MultiplicativeExpression> {
    let (cast_expression, input) = parse::cast_expression(input)?;

    Ok((
      MultiplicativeExpression::CastExpression(cast_expression),
      input,
    ))
  }

  fn multiplicative_operator_cast_expression(
    input: String,
  ) -> Result<((MultiplicativeOperator, CastExpression), String), Error> {
    let (multiplicative_operator, input) = parse::multiplicative_operator(input)?;
    let (cast_expression, input) = parse::cast_expression(input)?;

    Ok(((multiplicative_operator, cast_expression), input))
  }

  let (mut cast_expression, mut input) = cast_expression(input)?;
  loop {
    match multiplicative_operator_cast_expression(input.clone()) {
      Ok(((multiplicative_operator, cast_expression_), input_)) => {
        cast_expression =
          MultiplicativeExpression::MultiplicativeExpressionMultiplicativeOperatorCastExpression(
            Box::new(cast_expression),
            multiplicative_operator,
            cast_expression_,
          );
        input = input_;
      }
      Err(_) => break,
    }
  }

  Ok((cast_expression, input))
}

fn cast_expression(input: String) -> ParseResult<CastExpression> {
  let (unary_expression, input) = parse::unary_expression(input)?;

  Ok((CastExpression::UnaryExpression(unary_expression), input))
}

fn unary_expression(input: String) -> ParseResult<UnaryExpression> {
  fn unary_operator_cast_expression(input: String) -> ParseResult<UnaryExpression> {
    let (unary_operator, input) = parse::unary_operator(input)?;
    let (cast_expression, input) = parse::cast_expression(input)?;

    Ok((
      UnaryExpression::UnaryOperatorCastExpression(unary_operator, Box::new(cast_expression)),
      input,
    ))
  }

  fn paren_additive_expression_paren(input: String) -> ParseResult<UnaryExpression> {
    let ((), input) = parse::whitespaces_char('(')(input)?;
    let (additive_expression, input) = parse::additive_expression(input)?;
    let ((), input) = parse::whitespaces_char(')')(input)?;

    Ok((
      UnaryExpression::ParenAdditiveExpressionParen(Box::new(additive_expression)),
      input,
    ))
  }

  fn integer_constant(input: String) -> ParseResult<UnaryExpression> {
    let (integer_constant, input) = parse::integer_constant(input)?;

    Ok((UnaryExpression::IntegerConstant(integer_constant), input))
  }

  unary_operator_cast_expression(input.clone())
    .or_else(|_| paren_additive_expression_paren(input.clone()))
    .or_else(|_| integer_constant(input.clone()))
}

fn unary_operator(input: String) -> ParseResult<UnaryOperator> {
  fn negation(input: String) -> ParseResult<UnaryOperator> {
    let ((), input) = parse::whitespaces_char('-')(input)?;

    Ok((UnaryOperator::Negation, input))
  }

  fn bitwise_complement(input: String) -> ParseResult<UnaryOperator> {
    let ((), input) = parse::whitespaces_char('~')(input)?;

    Ok((UnaryOperator::BitwiseComplement, input))
  }

  fn logical_complement(input: String) -> ParseResult<UnaryOperator> {
    let ((), input) = parse::whitespaces_char('!')(input)?;

    Ok((UnaryOperator::LogicalNegation, input))
  }

  negation(input.clone())
    .or_else(|_| bitwise_complement(input.clone()))
    .or_else(|_| logical_complement(input.clone()))
}

fn additive_operator(input: String) -> ParseResult<AdditiveOperator> {
  fn addition(input: String) -> ParseResult<AdditiveOperator> {
    let ((), input) = parse::whitespaces_char('+')(input)?;

    Ok((AdditiveOperator::Addition, input))
  }

  fn subtraction(input: String) -> ParseResult<AdditiveOperator> {
    let ((), input) = parse::whitespaces_char('-')(input)?;

    Ok((AdditiveOperator::Subtraction, input))
  }

  addition(input.clone()).or_else(|_| subtraction(input.clone()))
}

fn multiplicative_operator(input: String) -> ParseResult<MultiplicativeOperator> {
  fn multiplication(input: String) -> ParseResult<MultiplicativeOperator> {
    let ((), input) = parse::whitespaces_char('*')(input)?;

    Ok((MultiplicativeOperator::Multiplication, input))
  }

  fn division(input: String) -> ParseResult<MultiplicativeOperator> {
    let ((), input) = parse::whitespaces_char('/')(input)?;

    Ok((MultiplicativeOperator::Division, input))
  }

  fn modulo(input: String) -> ParseResult<MultiplicativeOperator> {
    let ((), input) = parse::whitespaces_char('%')(input)?;

    Ok((MultiplicativeOperator::Modulo, input))
  }

  multiplication(input.clone())
    .or_else(|_| division(input.clone()))
    .or_else(|_| modulo(input.clone()))
}

pub fn parse(input: String) -> Result<TranslationUnit, Error> {
  let (translation_unit, input) = parse::translation_unit(input)?;

  match &input[..] {
    "" => Ok(translation_unit),
    _ => panic!("Input not fully parsed"),
  }
}
