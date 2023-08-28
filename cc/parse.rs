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

fn char(c: char) -> Parser<()> {
  Rc::new(
    move |input: String| match parse::satisfy(Rc::new(move |x| x == c))(input) {
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

fn whitespaces() -> Parser<()> {
  Rc::new(move |input: String| {
    let (_, input) = parse::many(parse::whitespace())(input)?;
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

fn end_of_file(input: String) -> ParseResult<()> {
  let ((), input) = parse::whitespaces()(input)?;
  let ((), input) = parse::eof()(input)?;

  Ok(((), input))
}

fn translation_unit(input: String) -> ParseResult<TranslationUnit> {
  // TODO should be `many` instead of `many1`
  let (external_declarations, input) = parse::many1(Rc::new(parse::external_declaration))(input)?;
  let ((), input) = parse::end_of_file(input)?;

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
  let ((), input) = parse::whitespaces()(input)?;
  let (_, input) = parse::string("int")(input)?;
  let ((), input) = parse::whitespaces()(input)?;
  let (_, input) = parse::string("main")(input)?;
  let ((), input) = parse::whitespaces()(input)?;
  let ((), input) = parse::char('(')(input)?;
  let ((), input) = parse::whitespaces()(input)?;
  let ((), input) = parse::char(')')(input)?;
  let (compound_statement, input) = parse::compound_statement(input)?;

  Ok((
    FunctionDefinition::NameBody("main".to_string(), compound_statement),
    input,
  ))
}

fn integer_constant(input: String) -> ParseResult<IntegerConstant> {
  let ((), input) = parse::whitespaces()(input)?;
  let (digits, input) = parse::many1(parse::decimal_digit())(input)?;
  let value = String::from_iter(digits).parse::<u8>().unwrap();

  Ok((IntegerConstant::IntegerConstant(value), input))
}

fn compound_statement(input: String) -> ParseResult<CompoundStatement> {
  let ((), input) = parse::whitespaces()(input)?;
  let ((), input) = parse::char('{')(input)?;
  let (magic_return, input) = parse::magic_return(input)?;
  let ((), input) = parse::whitespaces()(input)?;
  let ((), input) = parse::char('}')(input)?;

  Ok((CompoundStatement::MagicReturn(magic_return), input))
}

fn magic_return(input: String) -> ParseResult<MagicReturn> {
  let ((), input) = parse::whitespaces()(input)?;
  let ((), input) = parse::string("return")(input)?;
  let ((), input) = parse::whitespaces()(input)?;
  let (unary_expression, input) = parse::unary_expression(input)?;
  let ((), input) = parse::whitespaces()(input)?;
  let ((), input) = parse::char(';')(input)?;

  Ok((MagicReturn::UnaryExpression(unary_expression), input))
}

fn unary_expression(input: String) -> ParseResult<UnaryExpression> {
  fn unary_operator_cast_expression(input: String) -> ParseResult<UnaryExpression> {
    let (unary_operator, input) = parse::unary_operator(input)?;
    let (cast_expression, input) = parse::cast_expression(input)?;

    Ok((
      UnaryExpression::UnaryOperatorCastExpression(unary_operator, cast_expression),
      input,
    ))
  }

  fn integer_constant(input: String) -> ParseResult<UnaryExpression> {
    let (integer_constant, input) = parse::integer_constant(input)?;

    Ok((UnaryExpression::IntegerConstant(integer_constant), input))
  }

  Ok(unary_operator_cast_expression(input.clone()).or_else(|_| integer_constant(input.clone()))?)
}

fn unary_operator(input: String) -> ParseResult<UnaryOperator> {
  fn negation(input: String) -> ParseResult<UnaryOperator> {
    let ((), input) = parse::whitespaces()(input)?;
    let ((), input) = parse::char('-')(input)?;

    Ok((UnaryOperator::Negation, input))
  }

  fn bitwise_complement(input: String) -> ParseResult<UnaryOperator> {
    let ((), input) = parse::whitespaces()(input)?;
    let ((), input) = parse::char('~')(input)?;

    Ok((UnaryOperator::BitwiseComplement, input))
  }

  fn logical_complement(input: String) -> ParseResult<UnaryOperator> {
    let ((), input) = parse::whitespaces()(input)?;
    let ((), input) = parse::char('!')(input)?;

    Ok((UnaryOperator::LogicalNegation, input))
  }

  Ok(
    negation(input.clone())
      .or_else(|_| bitwise_complement(input.clone()))
      .or_else(|_| logical_complement(input.clone()))?,
  )
}

fn cast_expression(input: String) -> ParseResult<CastExpression> {
  let (unary_expression, input) = parse::unary_expression(input)?;

  Ok((
    CastExpression::UnaryExpression(Box::new(unary_expression)),
    input,
  ))
}

pub fn parse(input: String) -> Result<TranslationUnit, Error> {
  let (translation_unit, input) = parse::translation_unit(input)?;

  match &input[..] {
    "" => Ok(translation_unit),
    _ => panic!("Input not fully parsed"),
  }
}
