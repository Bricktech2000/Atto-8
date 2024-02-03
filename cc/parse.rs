use crate::*;
use std::rc::Rc;

// utilities

fn psi<T, U, V, F: FnOnce(T) -> U + Clone + 'static, G: FnOnce(U, U) -> V + Clone + 'static>(
  f: F,
  g: G,
) -> Rc<dyn Fn(T, T) -> V> {
  Rc::new(move |x, y| g.clone()(f.clone()(x), f.clone()(y)))
}

#[derive(Clone)]
pub struct Parser<T: Clone + 'static>(pub Rc<dyn Fn(&str) -> ParseResult<T>>);
pub type ParseResult<T> = Result<(T, String), Error>;

impl<T: Clone + 'static> Parser<T> {
  pub fn and_then<U: Clone + 'static, F: FnOnce(T) -> Parser<U> + Clone + 'static>(
    self,
    f: F,
  ) -> Parser<U> {
    Parser(Rc::new(move |input: &str| {
      self.0(input).and_then(|(r#match, input)| f.clone()(r#match).0(&input))
    }))
  }

  pub fn or_else<F: FnOnce(&Error) -> Parser<T> + Clone + 'static>(self, f: F) -> Parser<T> {
    Parser(Rc::new(move |input: &str| {
      self.0(input).or_else(|error1| {
        f.clone()(&error1).0(input).or_else(|error2| match (error1.0.as_str(), error2.0.as_str()) {
          ("", _) | (_, "") => Err(Error(format!("{}{}", error1, error2))),
          _ => Err(Error(format!("{}; {}", error1, error2))),
        })
      })
    }))
  }

  pub fn map<U: Clone + 'static, F: FnOnce(T) -> U + Clone + 'static>(self, f: F) -> Parser<U> {
    Parser(Rc::new(move |input: &str| {
      self.0(input).map(|(r#match, input)| (f.clone()(r#match), input))
    }))
  }

  pub fn map_err<F: FnOnce(&Error) -> Error + Clone + 'static>(self, f: F) -> Parser<T> {
    Parser(Rc::new(move |input: &str| {
      self.0(input).map_err(|error| f.clone()(&error))
    }))
  }

  pub fn meta(self, meta: String) -> Parser<T> {
    self.map_err(move |error| Error(format!("{}: {}", meta, error)))
  }

  pub fn r#return(value: T) -> Parser<T> {
    Parser(Rc::new(move |input: &str| {
      Ok((value.clone(), input.to_string()))
    }))
  }

  pub fn error(error: Error) -> Parser<T> {
    Parser(Rc::new(move |_input: &str| Err(error.clone())))
  }
}

// elementary parsers

pub fn any() -> Parser<char> {
  Parser(Rc::new(|input: &str| match &input[..] {
    "" => Err(Error(format!("got EOF"))), // to be concatenated
    _ => Ok((input.chars().next().unwrap(), input[1..].to_string())),
  }))
}

pub fn eof() -> Parser<()> {
  Parser(Rc::new(|input: &str| match &input[..] {
    "" => Ok(((), input.to_string())),
    // TODO uses debug formatting
    _ => Err(Error(format!(
      "EOF: got {:?}",
      input[0..std::cmp::min(16, input.len())].to_string() + "..."
    ))), // to be concatenated
  }))
}

pub fn satisfy<F: Fn(char) -> bool + Clone + 'static>(predicate: F) -> Parser<char> {
  parse::any().and_then(|char| {
    Parser(Rc::new(move |input: &str| match predicate(char) {
      true => Ok((char, input.to_string())),
      // TODO uses debug formatting
      false => Err(Error(format!("got {:?}", char))), // to be concatenated
    }))
  })
}

pub fn char(char: char) -> Parser<()> {
  parse::satisfy(move |c| c == char)
    .map(|_| ())
    // TODO uses debug formatting
    .meta(format!("Char {:?}", char))
}

pub fn string(string: &'static str) -> Parser<()> {
  string
    .chars()
    // similar to `parse::char` but without its `meta`, for clearer diagnostics
    .map(|char| parse::satisfy(move |c| c == char).map(|_| ()))
    .reduce(|acc, parser| acc.and_then(|_| parser))
    .unwrap()
    // TODO uses debug formatting
    .meta(format!("String {:?}", string))
}

pub fn whitespace() -> Parser<char> {
  parse::satisfy(move |c| c.is_whitespace()).meta(format!("Whitespace"))
}

pub fn digit(radix: u32) -> Parser<char> {
  parse::satisfy(move |c| c.is_digit(radix)).meta(format!("Digit"))
}

pub fn alphabetic() -> Parser<char> {
  parse::satisfy(|c| c.is_alphabetic()).meta(format!("Alphabetic"))
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
    .or_else(|_| Parser::r#return(None))
}

pub fn many1<T: Clone + 'static>(parser: Parser<T>) -> Parser<Vec<T>> {
  parser
    .clone()
    .and_then(|first| parse::many(parser).map(|rest| std::iter::once(first).chain(rest).collect()))
}

pub fn many<T: Clone + 'static>(parser: Parser<T>) -> Parser<Vec<T>> {
  // causes occasional stack overflow
  // parse::many1(parser).or_else(|_| Parser::r#return(vec![]))

  Parser(Rc::new(move |input: &str| {
    let mut input = input.to_string();
    let mut matches = vec![];
    while let Ok((r#match, new_input)) = parser.0(&input) {
      matches.push(r#match);
      input = new_input;
    }
    Ok((matches, input))
  }))
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

pub fn many_and_then<T: Clone + 'static, U: Clone + 'static>(
  parser: Parser<T>,
  r#final: Parser<U>,
) -> Parser<(Vec<T>, U)> {
  parse::many(parser.clone()).and_then(move |matches| {
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
  parse::sepby1(parser, separator).or_else(|_| Parser::r#return(vec![]))
}

pub fn binop<T: Clone + 'static>(
  parser: Parser<T>,
  separator: Parser<Rc<dyn Fn(T, T) -> T>>,
) -> Parser<T> {
  parser.clone().and_then(|first| {
    parse::many(separator.and_then(|constructor| parser.map(|second| (constructor, second)))).map(
      |rest| {
        rest
          .into_iter()
          .fold(first, |acc, (constructor, second)| constructor(acc, second)) // left-associative
      },
    )
  })
}

// C99 grammar

pub fn parse(input: String, errors: &mut impl Extend<(Pos, Error)>) -> Program {
  let program = parse::translation_unit().0(&input).map(|(programm, input)| match &input[..] {
    "" => programm,
    _ => panic!("Input not fully parsed"),
  });

  match program {
    Ok(program) => program,
    Err(error) => {
      errors.extend([(
        Pos(File("[parse]".to_string()), 0, 0),
        Error(format!("Could not parse: {}", error)),
      )]);
      Program(vec![])
    }
  }
}

fn translation_unit() -> Parser<Program> {
  parse::many_and_then(
    Parser::error(Error(format!("")))
      .or_else(|_| parse::function_declaration_global())
      .or_else(|_| parse::function_definition_global())
      .or_else(|_| parse::assembly_global()),
    parse::whitespaces_eof(),
  )
  .map(|(globals, _)| Program(globals))
}

fn function_declaration_global() -> Parser<Global> {
  // TODO does not obey grammar
  Parser::r#return(())
    .and_then(|_| parse::maybe(parse::whitespaces_string("inline")))
    .and_then(|is_inline| {
      parse::type_name().and_then(move |type_name| {
        parse::identifier().and_then(move |identifier| {
          Parser::r#return(())
            .and_then(|_| parse::whitespaces_char('('))
            .and_then(|_| parse::parameter_list())
            .and_then(move |parameters| {
              parse::maybe(
                parse::whitespaces_char(',').and_then(|_| parse::whitespaces_string("...")),
              )
              .and_then(move |is_variadic| {
                Parser::r#return(())
                  .and_then(|_| parse::whitespaces_char(')'))
                  .and_then(|_| parse::whitespaces_char(';'))
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
    })
    .meta(format!("Function Declaration"))
}

fn function_definition_global() -> Parser<Global> {
  // TODO does not obey grammar
  Parser::r#return(())
    .and_then(|_| parse::maybe(parse::whitespaces_string("inline")))
    .and_then(|is_inline| {
      parse::type_name().and_then(move |type_name| {
        parse::identifier().and_then(move |identifier| {
          Parser::r#return(())
            .and_then(|_| parse::whitespaces_char('('))
            .and_then(|_| parse::parameter_list())
            .and_then(move |parameters| {
              parse::maybe(
                parse::whitespaces_char(',').and_then(|_| parse::whitespaces_string("...")),
              )
              .and_then(move |is_variadic| {
                Parser::r#return(())
                  .and_then(|_| parse::whitespaces_char(')'))
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
    })
    .meta(format!("Function Definition"))
}

fn parameter_list() -> Parser<Vec<Object>> {
  // TODO does not obey grammar
  parse::sepby(
    parse::type_name().and_then(|type_name| {
      parse::identifier()
        .or_else(|_| Parser::r#return("".to_string()))
        .map(|identifier| Object(type_name, identifier))
    }),
    parse::whitespaces_char(','),
  )
}

fn type_name() -> Parser<Type> {
  // TODO does not obey grammar
  Parser::r#return(())
    .and_then(|_| parse::maybe(parse::whitespaces_string("const")))
    .and_then(|_const| {
      Parser::error(Error(format!("")))
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
      parse::whitespaces_string("*")
        .map(|_| Type::Pointer(Box::new(r#type)))
        .or_else(|_| Parser::r#return(type1))
    })
    .meta(format!("Type Name"))
}

fn assembly_global() -> Parser<Global> {
  parse::assembly_statement().map(|statement| match statement {
    Statement::Assembly(global_assembly) => Global::GlobalAssembly(global_assembly),
    _ => panic!("`assembly_statement` did not return `Statement::Assembly`"),
  })
}

fn compound_statement() -> Parser<Statement> {
  // TODO should be {<declaration>}* {<statement>}*
  Parser::r#return(())
    .and_then(|_| parse::whitespaces_char('{'))
    .and_then(|_| parse::many_and_then(parse::statement(), parse::whitespaces_char('}')))
    .map(|(statements, _)| Statement::Compound(statements))
    .meta(format!("Compound Statement"))
}

fn statement() -> Parser<Statement> {
  // TODO cases missing
  Parser::error(Error(format!("")))
    .or_else(|_| parse::jump_statement())
    .or_else(|_| parse::iteration_statement())
    .or_else(|_| parse::compound_statement())
    .or_else(|_| parse::selection_statement())
    .or_else(|_| parse::expression_statement())
    .or_else(|_| parse::assembly_statement()) // TODO does not obey grammar
}

fn jump_statement() -> Parser<Statement> {
  // TODO cases missing
  Parser::r#return(())
    .and_then(|_| parse::whitespaces_string("return"))
    .and_then(|_| {
      parse::maybe(parse::expression())
        .and_then(|expression| parse::whitespaces_char(';').map(|_| expression))
    }) // TODO does not obey grammar
    .map(|expression| Statement::Return(expression))
    .meta(format!("Jump Statement"))
}

fn selection_statement() -> Parser<Statement> {
  // TODO cases missing
  Parser::error(Error(format!("")))
    .or_else(|_| parse::if_statement())
    .or_else(|_| parse::if_else_statement())
    .or_else(|_| parse::switch_statement())
}

fn if_statement() -> Parser<Statement> {
  Parser::r#return(())
    .and_then(|_| parse::whitespaces_string("if"))
    .and_then(|_| parse::whitespaces_char('('))
    .and_then(|_| parse::expression())
    .and_then(|expression| {
      parse::whitespaces_char(')')
        .and_then(|_| parse::statement())
        .map(|statement| Statement::If(expression, Box::new(statement), None))
    })
    .meta(format!("If Statement"))
}

fn if_else_statement() -> Parser<Statement> {
  Parser::r#return(())
    .and_then(|_| parse::whitespaces_string("if"))
    .and_then(|_| parse::whitespaces_char('('))
    .and_then(|_| parse::expression())
    .and_then(|expression| {
      parse::whitespaces_char(')')
        .and_then(|_| parse::statement())
        .and_then(|statement1| {
          parse::whitespaces_string("else")
            .and_then(|_| parse::statement())
            .map(move |statement2| {
              Statement::If(expression, Box::new(statement1), Some(Box::new(statement2)))
            })
        })
    })
    .meta(format!("If Else Statement"))
}

fn switch_statement() -> Parser<Statement> {
  // TODO switch statement
  Parser::error(Error(format!("")))
}

fn iteration_statement() -> Parser<Statement> {
  // TODO cases missing
  Parser::error(Error(format!("")))
    .or_else(|_| parse::while_statement())
    .or_else(|_| parse::do_while_statement())
    .or_else(|_| parse::for_statement())
}

fn while_statement() -> Parser<Statement> {
  Parser::r#return(())
    .and_then(|_| parse::whitespaces_string("while"))
    .and_then(|_| parse::whitespaces_char('('))
    .and_then(|_| parse::expression())
    .and_then(|expression| {
      parse::whitespaces_char(')')
        .and_then(|_| parse::compound_statement())
        .map(|statements| Statement::While(expression, Box::new(statements)))
    })
    .meta(format!("Iteration Statement"))
}

fn do_while_statement() -> Parser<Statement> {
  // TODO do while statement
  Parser::error(Error(format!("")))
}

fn for_statement() -> Parser<Statement> {
  // TODO for statement
  Parser::error(Error(format!("")))
}

fn expression_statement() -> Parser<Statement> {
  // TODO cases missing
  Parser::r#return(())
    .and_then(|_| parse::expression())
    .and_then(|expression| parse::whitespaces_char(';').map(|_| Statement::Expression(expression)))
    .meta(format!("Expression Statement"))
}

fn assembly_statement() -> Parser<Statement> {
  Parser::r#return(())
    .and_then(|_| parse::whitespaces_string("asm"))
    .and_then(|_| parse::whitespaces_char('{'))
    .and_then(|_| parse::many(parse::satisfy(|c| c != '}')))
    .map(|chars| chars.iter().collect::<String>().trim().to_string())
    .and_then(|assembly| parse::whitespaces_char('}').map(move |_| Statement::Assembly(assembly)))
    .meta(format!("Assembly Statement"))
}

fn expression() -> Parser<Expression> {
  parse::constant_expression() // TODO does not obey grammar
}

fn constant_expression() -> Parser<Expression> {
  parse::conditional_expression()
}

fn conditional_expression() -> Parser<Expression> {
  parse::logical_or_expression().and_then(|expression1| {
    let expression = expression1.clone();
    Parser::r#return(())
      .and_then(|_| parse::whitespaces_char('?'))
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
      .or_else(|_| Parser::r#return(expression))
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
    Parser::error(Error(format!("")))
      .or_else(|_| parse::whitespaces_string("==").map(|_| psi(Box::new, Expression::EqualTo)))
      .or_else(|_| parse::whitespaces_string("!=").map(|_| psi(Box::new, Expression::NotEqualTo))),
  )
}

fn relational_expression() -> Parser<Expression> {
  parse::binop(
    parse::shift_expression(),
    Parser::error(Error(format!("")))
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
    Parser::error(Error(format!("")))
      .or_else(|_| parse::whitespaces_string("<<").map(|_| psi(Box::new, Expression::LeftShift)))
      .or_else(|_| parse::whitespaces_string(">>").map(|_| psi(Box::new, Expression::RightShift))),
  )
}

fn additive_expression() -> Parser<Expression> {
  parse::binop(
    parse::multiplicative_expression(),
    Parser::error(Error(format!("")))
      .or_else(|_| parse::whitespaces_char('+').map(|_| psi(Box::new, Expression::Addition)))
      .or_else(|_| parse::whitespaces_char('-').map(|_| psi(Box::new, Expression::Subtraction))),
  )
}

fn multiplicative_expression() -> Parser<Expression> {
  parse::binop(
    parse::cast_expression(),
    Parser::error(Error(format!("")))
      .or_else(|_| parse::whitespaces_char('*').map(|_| psi(Box::new, Expression::Multiplication)))
      .or_else(|_| parse::whitespaces_char('/').map(|_| psi(Box::new, Expression::Division)))
      .or_else(|_| parse::whitespaces_char('%').map(|_| psi(Box::new, Expression::Modulo))),
  )
}

fn cast_expression() -> Parser<Expression> {
  parse::whitespaces_char('(')
    .and_then(|_| parse::type_name())
    .and_then(|type_name| {
      parse::whitespaces_char(')')
        .and_then(|_| parse::cast_expression())
        .map(|cast_expression| Expression::Cast(type_name, Box::new(cast_expression)))
    })
    .or_else(|_| parse::unary_expression())
}

fn unary_expression() -> Parser<Expression> {
  Parser::error(Error(format!("")))
    .or_else(|_| {
      Parser::r#return(())
        .and_then(|_| parse::whitespaces_char('-'))
        .and_then(|_| parse::unary_expression())
        .map(|expression| Expression::Negation(Box::new(expression)))
    })
    .or_else(|_| {
      Parser::r#return(())
        .and_then(|_| parse::whitespaces_char('+'))
        .and_then(|_| parse::unary_expression())
    })
    .or_else(|_| {
      Parser::r#return(())
        .and_then(|_| parse::whitespaces_char('~'))
        .and_then(|_| parse::unary_expression())
        .map(|expression| Expression::BitwiseComplement(Box::new(expression)))
    })
    .or_else(|_| {
      Parser::r#return(())
        .and_then(|_| parse::whitespaces_char('!'))
        .and_then(|_| parse::unary_expression())
        .map(|expression| Expression::LogicalNegation(Box::new(expression)))
    })
    .or_else(|_| {
      Parser::r#return(())
        .and_then(|_| parse::whitespaces_char('('))
        .and_then(|_| parse::expression()) // TODO does not obey grammar
        .and_then(|expression| parse::whitespaces_char(')').map(|_| expression))
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
      Parser::error(Error(format!("")))
        .or_else(|_| {
          Parser::r#return(())
            .and_then(|_| parse::whitespaces_char('('))
            .and_then(|_| parse::sepby(parse::expression(), parse::whitespaces_char(',')))
            .and_then(|arguments| {
              parse::whitespaces_char(')')
                .map(|_| Expression::FunctionCall(Box::new(expression1), arguments))
            })
        })
        .or_else(|_| Parser::r#return(expression))
    })
}

fn identifier() -> Parser<String> {
  parse::many(parse::whitespace())
    .and_then(|_| parse::alphabetic().or_else(|_| parse::char('_').map(|_| '_')))
    .and_then(|first| {
      parse::many(
        Parser::error(Error(format!("")))
          .or_else(|_| parse::digit(10))
          .or_else(|_| parse::alphabetic())
          .or_else(|_| parse::char('_').map(|_| '_')),
      )
      .map(move |rest| std::iter::once(first).chain(rest).collect())
    })
    .meta(format!("Identifier"))
}

fn integer_constant() -> Parser<Expression> {
  // TODO does not obey grammar
  Parser::error(Error(format!("")))
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
    .map(|value| Expression::IntegerConstant(value))
    .meta(format!("Integer Constant"))
}

fn character_constant() -> Parser<Expression> {
  // TODO currently only parsing <simple-escape-sequence>s
  parse::many(parse::whitespace())
    .and_then(|_| parse::char('\''))
    .and_then(|_| parse::satisfy(|c| !"\'\\\n".contains(c)).or_else(|_| parse::escape_sequence()))
    .and_then(|char| parse::char('\'').map(move |_| Expression::CharacterConstant(char)))
    .meta(format!("Character Constant"))
}

fn string_literal() -> Parser<Expression> {
  parse::many(parse::whitespace())
    .and_then(|_| parse::char('"'))
    .and_then(|_| {
      parse::many(parse::satisfy(|c| !"\"\\\n".contains(c)).or_else(|_| parse::escape_sequence()))
        .map(|chars| chars.into_iter().collect::<String>())
    })
    .and_then(|string| parse::char('"').map(move |_| Expression::StringLiteral(string)))
}

fn escape_sequence() -> Parser<char> {
  Parser::error(Error(format!("")))
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
}
