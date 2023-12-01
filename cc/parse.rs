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
pub struct Parser<T: Clone + 'static>(pub Rc<dyn Fn(&str) -> ParseResult<T>>);
pub type ParseResult<T> = Result<(T, String), Error>;

impl<T: Clone + 'static> Parser<T> {
  pub fn and_then<U: Clone + 'static, F: FnOnce(T) -> Parser<U> + Clone + 'static>(
    self,
    f: F,
  ) -> Parser<U> {
    Parser(Rc::new(move |input: &str| {
      self.0(input).and_then(|(match_, input)| f.clone()(match_).0(&input))
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
      self.0(input).map(|(match_, input)| (f.clone()(match_), input))
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

  pub fn return_(value: T) -> Parser<T> {
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

// parser combinators

pub fn maybe<T: Clone + 'static>(parser: Parser<T>) -> Parser<Option<T>> {
  parser
    .clone()
    .map(|match_| Some(match_))
    .or_else(|_| Parser::return_(None))
}

pub fn many1<T: Clone + 'static>(parser: Parser<T>) -> Parser<Vec<T>> {
  parser
    .clone()
    .and_then(|first| parse::many(parser).map(|rest| std::iter::once(first).chain(rest).collect()))
}

pub fn many<T: Clone + 'static>(parser: Parser<T>) -> Parser<Vec<T>> {
  // causes occasional stack overflow
  // parse::many1(parser).or_else(|_| Parser::return_(vec![]))

  Parser(Rc::new(move |input: &str| {
    let mut input = input.to_string();
    let mut matches = vec![];
    while let Ok((match_, input_)) = parser.0(&input) {
      matches.push(match_);
      input = input_;
    }
    Ok((matches, input))
  }))
}

#[allow(dead_code)]
pub fn many1_and_then<T: Clone + 'static, U: Clone + 'static>(
  parser: Parser<T>,
  final_: Parser<U>,
) -> Parser<(Vec<T>, U)> {
  parse::many1(parser.clone()).and_then(move |matches| {
    parser
      // include `parser`'s error message if `final_` fails
      .map(|_| unreachable!())
      .or_else(move |_| final_.clone())
      .map(move |final_| (matches, final_))
  })
}

pub fn many_and_then<T: Clone + 'static, U: Clone + 'static>(
  parser: Parser<T>,
  final_: Parser<U>,
) -> Parser<(Vec<T>, U)> {
  parse::many(parser.clone()).and_then(move |matches| {
    parser
      // include `parser`'s error message if `final_` fails
      .map(|_| unreachable!())
      .or_else(move |_| final_.clone())
      .map(move |final_| (matches, final_))
  })
}

#[allow(dead_code)]
pub fn sepby1<T: Clone + 'static>(parser: Parser<T>, separator: Parser<()>) -> Parser<Vec<T>> {
  parser.clone().and_then(|first| {
    parse::many(separator.and_then(|_| parser))
      .map(|rest| std::iter::once(first).chain(rest).collect())
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
          .fold(first, |acc, (constructor, second)| constructor(acc, second)) // left-associative
      },
    )
  })
}

// C99 grammar

pub fn parse(input: String, errors: &mut Vec<(Pos, Error)>) -> Program {
  let program = parse::translation_unit().0(&input).map(|(programm, input)| match &input[..] {
    "" => programm,
    _ => panic!("Input not fully parsed"),
  });

  match program {
    Ok(program) => program,
    Err(error) => {
      errors.push((
        Pos("[parse]".to_string(), 0),
        Error(format!("Could not parse: {}", error)),
      ));
      Program(vec![])
    }
  }
}

pub fn translation_unit() -> Parser<Program> {
  parse::many_and_then(
    Parser::error(Error(format!("")))
      .or_else(|_| {
        parse::function_declaration()
          .map(|function_declaration| Global::FunctionDeclaration(function_declaration))
      })
      .or_else(|_| {
        parse::function_definition()
          .map(|function_definition| Global::FunctionDefinition(function_definition))
      })
      .or_else(|_| {
        parse::asm_statement().map(|statement| match statement {
          Statement::Asm(asm_statement) => Global::AsmStatement(asm_statement),
          _ => panic!("`asm_statement` did not return `Statement::Asm`"),
        })
      }),
    parse::whitespaces_eof(),
  )
  .map(|(globals, _)| Program(globals))
}

pub fn function_declaration() -> Parser<FunctionDeclaration> {
  // TODO does not obey grammar
  Parser::return_(())
    .and_then(|_| parse::maybe(parse::whitespaces_string("inline")))
    .and_then(|inline| {
      parse::type_name().and_then(move |type_name| {
        parse::identifier().and_then(move |identifier| {
          Parser::return_(())
            .and_then(|_| parse::whitespaces_char('('))
            .and_then(|_| parse::parameter_list())
            .and_then(move |parameters| {
              Parser::return_(())
                .and_then(|_| parse::whitespaces_char(')'))
                .and_then(|_| parse::whitespaces_char(';'))
                .map(move |_| {
                  FunctionDeclaration(inline.is_some(), Object(type_name, identifier), parameters)
                })
            })
        })
      })
    })
    .meta(format!("Function Declaration"))
}

pub fn function_definition() -> Parser<FunctionDefinition> {
  // TODO does not obey grammar
  Parser::return_(())
    .and_then(|_| parse::maybe(parse::whitespaces_string("inline")))
    .and_then(|inline| {
      parse::type_name().and_then(move |type_name| {
        parse::identifier().and_then(move |identifier| {
          Parser::return_(())
            .and_then(|_| parse::whitespaces_char('('))
            .and_then(|_| parse::parameter_list())
            .and_then(move |parameters| {
              Parser::return_(())
                .and_then(|_| parse::whitespaces_char(')'))
                .and_then(|_| parse::statement())
                .map(move |statement| {
                  FunctionDefinition(
                    inline.is_some(),
                    Object(type_name, identifier),
                    parameters,
                    statement,
                  )
                })
            })
        })
      })
    })
    .meta(format!("Function Definition"))
}

pub fn parameter_list() -> Parser<Vec<Object>> {
  // TODO does not obey grammar
  parse::sepby(
    parse::type_name().and_then(|type_name| {
      parse::identifier()
        .or_else(|_| Parser::return_("".to_string()))
        .map(|identifier| Object(type_name, identifier))
    }),
    parse::whitespaces_char(','),
  )
}

pub fn type_name() -> Parser<Type> {
  // TODO does not obey grammar
  Parser::error(Error(format!("")))
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
    .meta(format!("Type Name"))
}

pub fn compound_statement() -> Parser<Statement> {
  // TODO should be {<declaration>}* {<statement>}*
  Parser::return_(())
    .and_then(|_| parse::whitespaces_char('{'))
    .and_then(|_| parse::many_and_then(parse::statement(), parse::whitespaces_char('}')))
    .map(|(statements, _)| Statement::Compound(statements))
    .meta(format!("Compound Statement"))
}

pub fn statement() -> Parser<Statement> {
  // TODO cases missing
  Parser::error(Error(format!("")))
    .or_else(|_| parse::jump_statement())
    .or_else(|_| parse::iteration_statement())
    .or_else(|_| parse::compound_statement())
    .or_else(|_| parse::expression_statement())
    .or_else(|_| parse::asm_statement()) // TODO does not obey grammar
}

pub fn jump_statement() -> Parser<Statement> {
  // TODO cases missing
  Parser::return_(())
    .and_then(|_| parse::whitespaces_string("return"))
    .and_then(|_| {
      parse::maybe(parse::expression())
        .and_then(|expression| parse::whitespaces_char(';').map(|_| expression))
    }) // TODO does not obey grammar
    .map(|expression| Statement::Return(expression))
    .meta(format!("Jump Statement"))
}

pub fn iteration_statement() -> Parser<Statement> {
  // TODO cases missing
  Parser::error(Error(format!("")))
    .or_else(|_| parse::while_statement())
    .or_else(|_| parse::do_while_statement())
    .or_else(|_| parse::for_statement())
}

pub fn while_statement() -> Parser<Statement> {
  Parser::return_(())
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

pub fn do_while_statement() -> Parser<Statement> {
  // TODO do while statement
  Parser::error(Error(format!("")))
}

pub fn for_statement() -> Parser<Statement> {
  // TODO for statement
  Parser::error(Error(format!("")))
}

pub fn expression_statement() -> Parser<Statement> {
  // TODO cases missing
  Parser::return_(())
    .and_then(|_| parse::expression())
    .and_then(|expression| parse::whitespaces_char(';').map(|_| Statement::Expression(expression)))
    .meta(format!("Expression Statement"))
}

pub fn asm_statement() -> Parser<Statement> {
  Parser::return_(())
    .and_then(|_| parse::whitespaces_string("asm"))
    .and_then(|_| parse::whitespaces_char('{'))
    .and_then(|_| parse::many(parse::satisfy(|c| c != '}')))
    .map(|chars| chars.iter().collect::<String>().trim().to_string())
    .and_then(|assembly| parse::whitespaces_char('}').map(move |_| Statement::Asm(assembly)))
    .meta(format!("Asm Statement"))
}

pub fn expression() -> Parser<Expression> {
  parse::constant_expression() // TODO does not obey grammar
}

pub fn constant_expression() -> Parser<Expression> {
  parse::conditional_expression()
}

pub fn conditional_expression() -> Parser<Expression> {
  parse::logical_or_expression().and_then(|expression1| {
    let expression = expression1.clone();
    Parser::return_(())
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
      .or_else(|_| Parser::return_(expression))
  })
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
    Parser::error(Error(format!("")))
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
    Parser::error(Error(format!("")))
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
    Parser::error(Error(format!("")))
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
    Parser::error(Error(format!("")))
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
    Parser::error(Error(format!("")))
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
  Parser::error(Error(format!("")))
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
      // TODO does not obey grammar
      parse::identifier().and_then(|identifier| {
        Parser::return_(())
          .and_then(|_| parse::whitespaces_char('('))
          .and_then(|_| parse::sepby(parse::expression(), parse::whitespaces_char(',')))
          .and_then(|arguments| {
            parse::whitespaces_char(')').map(|_| Expression::FunctionCall(identifier, arguments))
          })
      })
    })
    // TODO does not obey grammar
    .or_else(|_| parse::integer_constant())
    .or_else(|_| parse::character_constant())
    .or_else(|_| parse::string_literal())
}

pub fn identifier() -> Parser<String> {
  parse::many(parse::whitespace())
    .and_then(|_| parse::alphabetic().or_else(|_| parse::char('_').map(|_| '_')))
    .and_then(|first| {
      parse::many(
        parse::digit(10)
          .or_else(|_| parse::alphabetic().or_else(|_| parse::char('_').map(|_| '_'))),
      )
      .map(move |rest| std::iter::once(first).chain(rest).collect())
    })
    .meta(format!("Identifier"))
}

pub fn integer_constant() -> Parser<Expression> {
  // TODO does not obey grammar
  Parser::error(Error(format!("")))
    .or_else(|_| {
      parse::whitespaces_string("0x")
        .and_then(|_| parse::many1(parse::digit(0x10)))
        .map(|digits| u8::from_str_radix(&digits.into_iter().collect::<String>(), 0x10))
    })
    .or_else(|_| {
      parse::whitespaces_string("0b")
        .and_then(|_| parse::many1(parse::digit(0b10)))
        .map(|digits| u8::from_str_radix(&digits.into_iter().collect::<String>(), 0b10))
    })
    .or_else(|_| {
      parse::many(parse::whitespace())
        .and_then(|_| parse::many1(parse::digit(10)))
        .map(|digits| u8::from_str_radix(&digits.into_iter().collect::<String>(), 10))
    })
    .map(|digits| digits.unwrap_or_else(|_| panic!("Could not parse integer constant")))
    .map(|value| Expression::IntegerConstant(value))
    .meta(format!("Integer Constant"))
}

pub fn character_constant() -> Parser<Expression> {
  // TODO currently only parsing <simple-escape-sequence>s
  parse::many(parse::whitespace())
    .and_then(|_| parse::char('\''))
    .and_then(|_| parse::satisfy(|c| !"\'\\\n".contains(c)).or_else(|_| parse::escape_sequence()))
    .and_then(|char| parse::char('\'').map(move |_| Expression::CharacterConstant(char)))
    .meta(format!("Character Constant"))
}

pub fn string_literal() -> Parser<Expression> {
  parse::many(parse::whitespace())
    .and_then(|_| parse::char('"'))
    .and_then(|_| {
      parse::many(parse::satisfy(|c| !"\"\\\n".contains(c)).or_else(|_| parse::escape_sequence()))
        .map(|chars| chars.into_iter().collect::<String>())
    })
    .and_then(|string| parse::char('"').map(move |_| Expression::StringLiteral(string)))
}

pub fn escape_sequence() -> Parser<char> {
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

pub fn whitespace() -> Parser<()> {
  Parser::error(Error(format!("")))
    .or_else(|_| parse::char(' '))
    .or_else(|_| parse::char('\r'))
    .or_else(|_| parse::char('\n'))
    .or_else(|_| parse::char('\t'))
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
