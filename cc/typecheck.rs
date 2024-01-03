use crate::*;
use std::collections::{BTreeMap, HashMap, HashSet};

pub fn typecheck(program: Program, errors: &mut Vec<(Pos, Error)>) -> TypedProgram {
  typecheck::program(program, &mut State::default(), errors)
}

#[derive(Clone, PartialEq, Default, Debug)]
struct State {
  declarations: HashMap<String, Type>, // map from global declaration to its type
  definitions: HashSet<String>,        // set of currently defined globals
  strings: BTreeMap<String, String>,   // map from string literal to its label
  stack: Vec<StackEntry>,              // current nesting scope stack
  uid: usize,                          // unique identifier for temporary identifiers
}

#[derive(Clone, PartialEq, Debug)]
enum StackEntry {
  MacroBoundary(Type, Vec<Object>),
  FunctionBoundary(Type, Vec<Object>),
  LoopBoundary,
  BlockBoundary(Vec<Object>),
}

#[allow(dead_code)]
#[derive(Clone, PartialEq, Debug)]
enum Range {
  U0,
  I0,
  U1,
  I1,
  U8,
  I8,
  U16,
  I16,
  U32,
  I32,
}

impl Type {
  fn size(&self) -> usize {
    match self {
      Type::Void => 0,
      Type::Bool => 1,
      Type::Char => 1,
      Type::SignedChar => 1,
      Type::UnsignedChar => 1,
      Type::Short => 1,            // TODO nonstandard
      Type::UnsignedShort => 1,    // TODO nonstandard
      Type::Int => 1,              // TODO nonstandard
      Type::UnsignedInt => 1,      // TODO nonstandard
      Type::Long => 2,             // TODO potentially nonstandard
      Type::UnsignedLong => 2,     // TODO potentially nonstandard
      Type::LongLong => 4,         // TODO potentially nonstandard
      Type::UnsignedLongLong => 2, // TODO potentially nonstandard
      Type::Array(_) => todo!(),
      Type::Structure(declarators) => declarators.iter().map(Object::size).sum(),
      Type::Union(declarators) => declarators.iter().map(Object::size).max().unwrap_or(0),
      Type::Enumeration(_) => 1,
      Type::Function(_, _, _) => todo!(),
      Type::Macro(_, _, _, _) => todo!(),
      Type::Pointer(_) => 1,
    }
  }

  fn range(&self) -> Range {
    match self {
      Type::Void => Range::U0,
      Type::Bool => Range::U1,
      Type::Char => Range::I8,
      Type::SignedChar => Range::I8,
      Type::UnsignedChar => Range::U8,
      Type::Short => Range::I8,             // TODO nonstandard
      Type::UnsignedShort => Range::U8,     // TODO nonstandard
      Type::Int => Range::I8,               // TODO nonstandard
      Type::UnsignedInt => Range::U8,       // TODO nonstandard
      Type::Long => Range::I16,             // TODO potentially nonstandard
      Type::UnsignedLong => Range::U16,     // TODO potentially nonstandard
      Type::LongLong => Range::I32,         // TODO potentially nonstandard
      Type::UnsignedLongLong => Range::U32, // TODO potentially nonstandard
      Type::Array(_) => unreachable!(),
      Type::Structure(_) => unreachable!(),
      Type::Union(_) => unreachable!(),
      Type::Enumeration(_) => Range::I8,
      Type::Function(_, _, _) => unreachable!(),
      Type::Macro(_, _, _, _) => unreachable!(),
      Type::Pointer(_) => Range::U8,
    }
  }

  fn width(&self) -> usize {
    match self.range() {
      Range::U0 | Range::I0 => 0,
      Range::U1 | Range::I1 => 1,
      Range::U8 | Range::I8 => 5,
      Range::U16 | Range::I16 => 6,
      Range::U32 | Range::I32 => 7,
    }
  }
}

impl Object {
  pub fn size(&self) -> usize {
    match self {
      Object(r#type, _name) => r#type.size(),
    }
  }
}

fn program(program: Program, state: &mut State, errors: &mut Vec<(Pos, Error)>) -> TypedProgram {
  match program {
    Program(globals) => TypedProgram(
      std::iter::empty()
        .chain(
          globals
            .into_iter()
            .filter_map(|global| typecheck::global(global, state, errors)),
        )
        .collect::<Vec<_>>()
        .into_iter()
        .chain(
          state
            .strings
            .iter()
            .map(|(value, label)| TypedGlobal::String(label.clone(), value.clone())),
        )
        .collect(),
    ),
  }
}

fn global(
  global: Global,
  state: &mut State,
  errors: &mut Vec<(Pos, Error)>,
) -> Option<TypedGlobal> {
  match global {
    Global::FunctionDeclaration(function_declaration) => {
      let () = typecheck::function_declaration_global(function_declaration, state, errors);
      None
    }

    Global::FunctionDefinition(function_definition) => Some(typecheck::function_definition_global(
      function_definition,
      state,
      errors,
    )),

    Global::GlobalAssembly(assembly) => Some(typecheck::assembly_global(assembly, state, errors)),
  }
}

fn function_declaration_global(
  function_declaration: FunctionDeclaration,
  state: &mut State,
  errors: &mut Vec<(Pos, Error)>,
) -> () {
  match function_declaration {
    FunctionDeclaration(is_inline, Object(return_type, name), parameters, is_variadic) => {
      let parameter_types = match parameters[..] {
        [Object(Type::Void, _)] => vec![], // for `T func(void)`-style declarations
        _ => parameters
          .iter()
          .map(|Object(r#type, _name)| r#type.clone())
          .collect::<Vec<Type>>(),
      };

      state
        .declarations
        .entry(name.clone())
        .and_modify(|r#type| match (is_inline, r#type) {
          (false, Type::Function(return_type_, parameter_types_, is_variadic_))
          | (true, Type::Macro(return_type_, _, parameter_types_, is_variadic_))
            if return_type == **return_type_
              && parameter_types == *parameter_types_
              && is_variadic == *is_variadic_ => {}
          _ => errors.push((
            Pos("pos".to_string(), 0),
            Error(format!(
              "Function `{}` previously declared with different prototype",
              name.clone()
            )),
          )),
        })
        .or_insert(match is_inline {
          true => Type::Macro(Box::new(return_type), name, parameter_types, is_variadic),
          false => Type::Function(Box::new(return_type), parameter_types, is_variadic),
        });
    }
  }
}

fn function_definition_global(
  function_definition: FunctionDefinition,
  state: &mut State,
  errors: &mut Vec<(Pos, Error)>,
) -> TypedGlobal {
  match function_definition {
    FunctionDefinition(is_inline, Object(return_type, name), parameters, is_variadic, body) => {
      // TODO optimize: only do so if function does not end with return
      let body = Statement::Compound(vec![body, Statement::Return(None)]);

      let () = typecheck::function_declaration_global(
        FunctionDeclaration(
          is_inline,
          Object(return_type.clone(), name.clone()),
          parameters.clone(),
          is_variadic,
        ),
        state,
        errors,
      );

      if is_variadic {
        errors.push((
          Pos("[todo]".to_string(), 0),
          Error(format!("Variadic function definitions unimplemented",)),
        ))
      }

      state.definitions.get(&name).is_some().then(|| {
        errors.push((
          Pos("pos".to_string(), 0),
          Error(format!(
            "Function `{}` has already been defined",
            name.clone()
          )),
        ))
      });
      state.definitions.insert(name.clone());

      state.stack.push(match is_inline {
        true => StackEntry::MacroBoundary(return_type.clone(), parameters.clone()),
        false => StackEntry::FunctionBoundary(return_type.clone(), parameters.clone()),
      });

      let statement = match is_inline {
        true => TypedGlobal::Macro(name, typecheck::statement(body, state, errors)),
        false => TypedGlobal::Function(name, typecheck::statement(body, state, errors)),
      };

      state.stack.pop();

      statement
    }
  }
}

fn assembly_global(
  assembly: String,
  _state: &mut State,
  _errors: &mut Vec<(Pos, Error)>,
) -> TypedGlobal {
  TypedGlobal::Assembly(assembly)
}

fn statement(
  statement: Statement,
  state: &mut State,
  errors: &mut Vec<(Pos, Error)>,
) -> TypedStatement {
  match statement {
    Statement::Expression(expression) => typecheck::expression_statement(expression, state, errors),
    Statement::Compound(statements) => typecheck::compound_statement(statements, state, errors),
    Statement::While(condition, body) => {
      typecheck::while_statement(condition, *body, state, errors)
    }
    Statement::Return(expression) => typecheck::return_statement(expression, state, errors),
    Statement::Assembly(assembly) => typecheck::assembly_statement(assembly, state, errors),
  }
}

fn expression_statement(
  expression: Expression,
  state: &mut State,
  errors: &mut Vec<(Pos, Error)>,
) -> TypedStatement {
  let (r#type, statement) = typecheck::expression(
    Expression::Cast(Type::Void, Box::new(expression)),
    state,
    errors,
  );

  if r#type != Type::Void {
    panic!("Expected expression statement to have type `void`");
  }

  TypedStatement::ExpressionN0(statement)
}

fn compound_statement(
  statements: Vec<Statement>,
  state: &mut State,
  errors: &mut Vec<(Pos, Error)>,
) -> TypedStatement {
  state.stack.push(StackEntry::BlockBoundary(vec![]));

  let statements = statements
    .into_iter()
    .map(|statement| typecheck::statement(statement, state, errors))
    .collect();

  state.stack.pop();

  TypedStatement::Compound(statements)
}

fn while_statement(
  condition: Expression,
  body: Statement,
  state: &mut State,
  errors: &mut Vec<(Pos, Error)>,
) -> TypedStatement {
  let (r#type, condition) = typecheck::expression(
    Expression::Cast(Type::Bool, Box::new(condition)),
    state,
    errors,
  );

  if r#type != Type::Bool {
    panic!("Expected while condition to have type `bool`");
  }

  state.stack.push(StackEntry::LoopBoundary);

  let statement = TypedStatement::WhileN1(
    format!("while.{}", state.uid),
    condition,
    Box::new(typecheck::statement(body, state, errors)),
  );
  state.uid += 1;

  state.stack.pop();

  statement
}

fn return_statement(
  expression: Option<Expression>,
  state: &mut State,
  errors: &mut Vec<(Pos, Error)>,
) -> TypedStatement {
  let mut locals_size = 0;
  let (is_inline, return_type, parameters_size) = state
    .stack
    .iter()
    .rev()
    .find_map(|stack_entry| match stack_entry {
      StackEntry::MacroBoundary(return_type, parameters) => Some((
        true,
        return_type.clone(),
        parameters.iter().map(Object::size).sum(),
      )),
      StackEntry::FunctionBoundary(return_type, parameters) => Some((
        false,
        return_type.clone(),
        parameters.iter().map(Object::size).sum(),
      )),
      StackEntry::LoopBoundary => None,
      StackEntry::BlockBoundary(locals) => {
        locals_size += locals.iter().map(Object::size).sum::<usize>();
        None
      }
    })
    .unwrap_or_else(|| {
      errors.push((
        Pos("pos".to_string(), 0),
        Error(format!("`return` encountered outside of function")),
      ));
      (false, Type::Void, 0)
    });

  let expression = expression.map(|expression| {
    let (r#type, expression) = typecheck::expression(
      Expression::Cast(return_type.clone(), Box::new(expression)),
      state,
      errors,
    );

    if r#type != return_type {
      panic!("Expected return type to match function return type");
    }

    expression
  });

  match (is_inline, return_type.range()) {
    (true, Range::U0 | Range::I0) => {
      TypedStatement::MacroReturnN0(parameters_size, locals_size, expression)
    }
    (true, Range::U1 | Range::I1) => {
      TypedStatement::MacroReturnN1(parameters_size, locals_size, expression)
    }
    (true, Range::U8 | Range::I8) => {
      TypedStatement::MacroReturnN8(parameters_size, locals_size, expression)
    }
    (false, Range::U0 | Range::I0) => {
      TypedStatement::FunctionReturnN0(parameters_size, locals_size, expression)
    }
    (false, Range::U1 | Range::I1) => {
      TypedStatement::FunctionReturnN1(parameters_size, locals_size, expression)
    }
    (false, Range::U8 | Range::I8) => {
      TypedStatement::FunctionReturnN8(parameters_size, locals_size, expression)
    }
    _ => {
      errors.push((
        Pos("[todo]".to_string(), 0),
        Error(format!("Return unimplemented for type `{:?}`", return_type)),
      ));
      TypedStatement::Assembly("".to_string())
    }
  }
}

fn assembly_statement(
  assembly: String,
  _state: &mut State,
  _errors: &mut Vec<(Pos, Error)>,
) -> TypedStatement {
  TypedStatement::Assembly(assembly)
}

fn expression(
  expression: Expression,
  state: &mut State,
  errors: &mut Vec<(Pos, Error)>,
) -> (Type, TypedExpression) {
  match expression.clone() {
    //   Expression::Negation(expression) => typecheck::negation_expression(*expression, state, errors),
    //   Expression::LogicalNegation(expression) => {
    //     typecheck::logical_negation_expression(*expression, state, errors)
    //   }
    //   Expression::BitwiseComplement(expression) => {
    //     typecheck::bitwise_complement_expression(*expression, state, errors)
    //   }
    Expression::Addition(expression1, expression2) => {
      let promoted1 = integer_promotions(*expression1, state, errors);
      let promoted2 = integer_promotions(*expression2, state, errors);
      let (r#type, expression1, expression2) =
        typecheck_pointer_arithmetic_conversions((promoted1, promoted2), state, errors);

      (
        r#type.clone(),
        match r#type.range() {
          Range::U0 | Range::I0 | Range::U1 | Range::I1 => unreachable!(),
          Range::U8 | Range::I8 => {
            TypedExpression::N8Addition(Box::new(expression1), Box::new(expression2))
          }
          _ => todo!(),
        },
      )
    }

    Expression::Subtraction(expression1, expression2) => {
      let promoted1 = integer_promotions(*expression1, state, errors);
      let promoted2 = integer_promotions(*expression2, state, errors);
      let (r#type, expression1, expression2) =
        typecheck_pointer_arithmetic_conversions((promoted1, promoted2), state, errors);

      (
        r#type.clone(),
        match r#type.range() {
          Range::U0 | Range::I0 | Range::U1 | Range::I1 => unreachable!(),
          Range::U8 | Range::I8 => {
            TypedExpression::N8Subtraction(Box::new(expression1), Box::new(expression2))
          }
          _ => todo!(),
        },
      )
    }

    Expression::Multiplication(expression1, expression2) => {
      let promoted1 = integer_promotions(*expression1, state, errors);
      let promoted2 = integer_promotions(*expression2, state, errors);
      let (r#type, expression1, expression2) =
        typecheck_usual_arithmetic_conversions((promoted1, promoted2), state, errors);

      (
        r#type.clone(),
        match r#type.range() {
          Range::U0 | Range::I0 | Range::U1 | Range::I1 => unreachable!(),
          Range::U8 => {
            TypedExpression::U8Multiplication(Box::new(expression1), Box::new(expression2))
          }
          Range::I8 => {
            errors.push((
              Pos("[todo]".to_string(), 0),
              Error(format!("Signed multiplication unimplemented",)),
            ));
            TypedExpression::U8Multiplication(Box::new(expression1), Box::new(expression2))
          }
          _ => todo!(),
        },
      )
    }

    Expression::Division(expression1, expression2) => {
      let promoted1 = integer_promotions(*expression1, state, errors);
      let promoted2 = integer_promotions(*expression2, state, errors);
      let (r#type, expression1, expression2) =
        typecheck_usual_arithmetic_conversions((promoted1, promoted2), state, errors);

      (
        r#type.clone(),
        match r#type.range() {
          Range::U0 | Range::I0 | Range::U1 | Range::I1 => unreachable!(),
          Range::U8 => TypedExpression::U8Division(Box::new(expression1), Box::new(expression2)),
          Range::I8 => {
            errors.push((
              Pos("[todo]".to_string(), 0),
              Error(format!("Signed division unimplemented",)),
            ));
            TypedExpression::U8Division(Box::new(expression1), Box::new(expression2))
          }
          _ => todo!(),
        },
      )
    }

    Expression::Modulo(expression1, expression2) => {
      let promoted1 = integer_promotions(*expression1, state, errors);
      let promoted2 = integer_promotions(*expression2, state, errors);
      let (r#type, expression1, expression2) =
        typecheck_usual_arithmetic_conversions((promoted1, promoted2), state, errors);

      (
        r#type.clone(),
        match r#type.range() {
          Range::U0 | Range::I0 | Range::U1 | Range::I1 => unreachable!(),
          Range::U8 => TypedExpression::U8Modulo(Box::new(expression1), Box::new(expression2)),
          Range::I8 => {
            errors.push((
              Pos("[todo]".to_string(), 0),
              Error(format!("Signed modulo unimplemented",)),
            ));
            TypedExpression::U8Modulo(Box::new(expression1), Box::new(expression2))
          }
          _ => todo!(),
        },
      )
    }

    //   Expression::LogicalAnd(expression1, expression2) => {
    //     typecheck::logical_and_expression(*expression1, *expression2, state, errors)
    //   }
    //   Expression::LogicalOr(expression1, expression2) => {
    //     typecheck::logical_or_expression(*expression1, *expression2, state, errors)
    //   }
    //   Expression::BitwiseAnd(expression1, expression2) => {
    //     typecheck::bitwise_and_expression(*expression1, *expression2, state, errors)
    //   }
    //   Expression::BitwiseInclusiveOr(expression1, expression2) => {
    //     typecheck::bitwise_inclusive_or_expression(*expression1, *expression2, state, errors)
    //   }
    //   Expression::BitwiseExclusiveOr(expression1, expression2) => {
    //     typecheck::bitwise_exclusive_or_expression(*expression1, *expression2, state, errors)
    //   }
    //   Expression::LeftShift(expression1, expression2) => {
    //     typecheck::left_shift_expression(*expression1, *expression2, state, errors)
    //   }
    //   Expression::RightShift(expression1, expression2) => {
    //     typecheck::right_shift_expression(*expression1, *expression2, state, errors)
    //   }

    //   Expression::EqualTo(expression1, expression2) => {
    //     typecheck::equal_to_expression(*expression1, *expression2, state, errors)
    //   }
    //   Expression::NotEqualTo(expression1, expression2) => {
    //     typecheck::not_equal_to_expression(*expression1, *expression2, state, errors)
    //   }
    //   Expression::LessThan(expression1, expression2) => {
    //     typecheck::less_than_expression(*expression1, *expression2, state, errors)
    //   }
    //   Expression::LessThanOrEqualTo(expression1, expression2) => {
    //     typecheck::less_than_or_equal_to_expression(*expression1, *expression2, state, errors)
    //   }
    //   Expression::GreaterThan(expression1, expression2) => {
    //     typecheck::greater_than_expression(*expression1, *expression2, state, errors)
    //   }
    //   Expression::GreaterThanOrEqualTo(expression1, expression2) => {
    //     typecheck::greater_than_or_equal_to_expression(*expression1, *expression2, state, errors)
    //   }

    //   Expression::Conditional(expression1, expression2, expression3) => {
    //     typecheck::conditional_expression(*expression1, *expression2, *expression3, state, errors)
    //   }
    Expression::Cast(r#type, expression) => {
      typecheck::cast_expression(r#type, *expression, state, errors)
    }
    Expression::IntegerConstant(value) => (Type::Int, TypedExpression::N8Constant(value)),
    // TODO character constants are `int`s in C
    Expression::CharacterConstant(value) => (Type::Char, TypedExpression::N8Constant(value as u8)),
    Expression::StringLiteral(value) => typecheck::string_literal_expression(value, state, errors),
    Expression::Identifier(identifier) => {
      typecheck::identifier_expression(identifier, state, errors)
    }
    Expression::FunctionCall(designator, arguments) => {
      typecheck::function_call_expression(*designator, arguments, state, errors)
    }
    _ => todo!(), // TODO
  }
}

// fn negation_expression(
//   expression: Expression,
//   state: &mut State,
//   errors: &mut Vec<(Pos, Error)>,
// ) -> (Type, Vec<Result<Token, String>>) {
//   let (r#type, tokens) = typecheck::expression(expression, state, errors);
//
//   (
//     r#type.clone(),
//     match r#type {
//       Type::Int | Type::Char => std::iter::empty()
//         .chain(tokens)
//         .chain(vec![Ok(Token::Neg)])
//         .collect(),
//       _ => {
//         // TODO implement
//         errors.push((
//           Pos("[todo]".to_string(), 0),
//           Error(format!("Negation unimplemented for type `{:?}`", r#type)),
//         ));
//         vec![]
//       }
//     },
//   )
// }
//
// fn logical_negation_expression(
//   expression: Expression,
//   state: &mut State,
//   errors: &mut Vec<(Pos, Error)>,
// ) -> (Type, Vec<Result<Token, String>>) {
//   let (r#type, tokens) = typecheck::expression(expression, state, errors);
//
//   (
//     r#type.clone(),
//     match r#type {
//       Type::Bool => std::iter::empty()
//         .chain(tokens)
//         .chain(vec![
//           Ok(Token::Shr),
//           Ok(Token::AtDyn),
//           Ok(Token::Flc),
//           Ok(Token::Shl),
//           Ok(Token::AtDyn),
//         ])
//         .collect(),
//       Type::Int | Type::Char => std::iter::empty()
//         .chain(tokens)
//         .chain(vec![
//           Ok(Token::Buf),
//           Ok(Token::AtDyn),
//           Ok(Token::Pop),
//           Ok(Token::Flc),
//           Ok(Token::XXX(0x00)),
//           Ok(Token::Shl),
//           Ok(Token::AtDyn),
//         ])
//         .collect(),
//       _ => {
//         // TODO implement
//         errors.push((
//           Pos("[todo]".to_string(), 0),
//           Error(format!(
//             "Logical Negation unimplemented for type `{:?}`",
//             r#type
//           )),
//         ));
//         vec![]
//       }
//     },
//   )
// }
//
// fn bitwise_complement_expression(
//   expression: Expression,
//   state: &mut State,
//   errors: &mut Vec<(Pos, Error)>,
// ) -> (Type, Vec<Result<Token, String>>) {
//   let (r#type, tokens) = typecheck::expression(expression, state, errors);
//
//   (
//     r#type.clone(),
//     match r#type {
//       Type::Int | Type::Char => std::iter::empty()
//         .chain(tokens)
//         .chain(vec![Ok(Token::Not)])
//         .collect(),
//       _ => {
//         // TODO implement
//         errors.push((
//           Pos("[todo]".to_string(), 0),
//           Error(format!(
//             "Bitwise Complement unimplemented for type `{:?}`",
//             r#type
//           )),
//         ));
//         vec![]
//       }
//     },
//   )
// }
//
// fn multiplication_expression(
//   expression1: Expression,
//   expression2: Expression,
//   state: &mut State,
//   errors: &mut Vec<(Pos, Error)>,
// ) -> (Type, Vec<Result<Token, String>>) {
//   let (r#type, tokens1, tokens2) =
//     typecheck::usual_arithmetic_conversion(expression1, expression2, state, errors);
//
//   let mul_macro = Macro("mul".to_string());
//
//   (
//     r#type.clone(),
//     match r#type {
//       Type::Int | Type::Char => std::iter::empty()
//         .chain(tokens1)
//         .chain(tokens2)
//         .chain(vec![Ok(Token::MacroRef(mul_macro))])
//         .collect(),
//       _ => {
//         // TODO implement
//         errors.push((
//           Pos("[todo]".to_string(), 0),
//           Error(format!(
//             "Multiplication unimplemented for type `{:?}`",
//             r#type
//           )),
//         ));
//         vec![]
//       }
//     },
//   )
// }
//
// fn division_expression(
//   expression1: Expression,
//   expression2: Expression,
//   state: &mut State,
//   errors: &mut Vec<(Pos, Error)>,
// ) -> (Type, Vec<Result<Token, String>>) {
//   let (r#type, tokens1, tokens2) =
//     typecheck::usual_arithmetic_conversion(expression1, expression2, state, errors);
//
//   let div_macro = Macro("div".to_string());
//
//   (
//     r#type.clone(),
//     match r#type {
//       Type::Int | Type::Char => std::iter::empty()
//         .chain(tokens1)
//         .chain(tokens2)
//         .chain(vec![Ok(Token::MacroRef(div_macro))])
//         .collect(),
//       _ => {
//         // TODO implement
//         errors.push((
//           Pos("[todo]".to_string(), 0),
//           Error(format!("Division unimplemented for type `{:?}`", r#type)),
//         ));
//         vec![]
//       }
//     },
//   )
// }
//
// fn modulo_expression(
//   expression1: Expression,
//   expression2: Expression,
//   state: &mut State,
//   errors: &mut Vec<(Pos, Error)>,
// ) -> (Type, Vec<Result<Token, String>>) {
//   let (r#type, tokens1, tokens2) =
//     typecheck::usual_arithmetic_conversion(expression1, expression2, state, errors);
//
//   let mod_macro = Macro("mod".to_string());
//
//   (
//     r#type.clone(),
//     match r#type {
//       Type::Int | Type::Char => std::iter::empty()
//         .chain(tokens1)
//         .chain(tokens2)
//         .chain(vec![Ok(Token::MacroRef(mod_macro))])
//         .collect(),
//       _ => {
//         // TODO implement
//         errors.push((
//           Pos("[todo]".to_string(), 0),
//           Error(format!("Modulo unimplemented for type `{:?}`", r#type)),
//         ));
//         vec![]
//       }
//     },
//   )
// }
//
// fn logical_and_expression(
//   _expression1: Expression,
//   _expression2: Expression,
//   _state: &mut State,
//   errors: &mut Vec<(Pos, Error)>,
// ) -> (Type, Vec<Result<Token, String>>) {
//   // TODO implement
//   errors.push((
//     Pos("[todo]".to_string(), 0),
//     Error(format!("Logical AND unimplemented")),
//   ));
//
//   (Type::Bool, vec![])
// }
//
// fn logical_or_expression(
//   _expression1: Expression,
//   _expression2: Expression,
//   _state: &mut State,
//   errors: &mut Vec<(Pos, Error)>,
// ) -> (Type, Vec<Result<Token, String>>) {
//   // TODO implement
//   errors.push((
//     Pos("[todo]".to_string(), 0),
//     Error(format!("Logical OR unimplemented")),
//   ));
//
//   (Type::Bool, vec![])
// }
//
// fn bitwise_and_expression(
//   expression1: Expression,
//   expression2: Expression,
//   state: &mut State,
//   errors: &mut Vec<(Pos, Error)>,
// ) -> (Type, Vec<Result<Token, String>>) {
//   let (r#type, tokens1, tokens2) =
//     typecheck::usual_arithmetic_conversion(expression1, expression2, state, errors);
//
//   (
//     r#type.clone(),
//     match r#type {
//       Type::Int | Type::Char => std::iter::empty()
//         .chain(tokens1)
//         .chain(tokens2)
//         .chain(vec![Ok(Token::And)])
//         .collect(),
//       _ => {
//         // TODO implement
//         errors.push((
//           Pos("[todo]".to_string(), 0),
//           Error(format!("Bitwise AND unimplemented for type `{:?}`", r#type)),
//         ));
//         vec![]
//       }
//     },
//   )
// }
//
// fn bitwise_inclusive_or_expression(
//   expression1: Expression,
//   expression2: Expression,
//   state: &mut State,
//   errors: &mut Vec<(Pos, Error)>,
// ) -> (Type, Vec<Result<Token, String>>) {
//   let (r#type, tokens1, tokens2) =
//     typecheck::usual_arithmetic_conversion(expression1, expression2, state, errors);
//
//   (
//     r#type.clone(),
//     match r#type {
//       Type::Int | Type::Char => std::iter::empty()
//         .chain(tokens1)
//         .chain(tokens2)
//         .chain(vec![Ok(Token::Orr)])
//         .collect(),
//       _ => {
//         // TODO implement
//         errors.push((
//           Pos("[todo]".to_string(), 0),
//           Error(format!(
//             "Bitwise Inclusive OR unimplemented for type `{:?}`",
//             r#type
//           )),
//         ));
//         vec![]
//       }
//     },
//   )
// }
//
// fn bitwise_exclusive_or_expression(
//   expression1: Expression,
//   expression2: Expression,
//   state: &mut State,
//   errors: &mut Vec<(Pos, Error)>,
// ) -> (Type, Vec<Result<Token, String>>) {
//   let (r#type, tokens1, tokens2) =
//     typecheck::usual_arithmetic_conversion(expression1, expression2, state, errors);
//
//   (
//     r#type.clone(),
//     match r#type {
//       Type::Int | Type::Char => std::iter::empty()
//         .chain(tokens1)
//         .chain(tokens2)
//         .chain(vec![Ok(Token::Xor)])
//         .collect(),
//       _ => {
//         // TODO implement
//         errors.push((
//           Pos("[todo]".to_string(), 0),
//           Error(format!(
//             "Bitwise Exclusive OR unimplemented for type `{:?}`",
//             r#type
//           )),
//         ));
//         vec![]
//       }
//     },
//   )
// }
//
// fn left_shift_expression(
//   _expression1: Expression,
//   _expression2: Expression,
//   _state: &mut State,
//   errors: &mut Vec<(Pos, Error)>,
// ) -> (Type, Vec<Result<Token, String>>) {
//   // TODO implement
//   errors.push((
//     Pos("[todo]".to_string(), 0),
//     Error(format!("Left Shift unimplemented")),
//   ));
//
//   (Type::Int, vec![])
// }
//
// fn right_shift_expression(
//   _expression1: Expression,
//   _expression2: Expression,
//   _state: &mut State,
//   errors: &mut Vec<(Pos, Error)>,
// ) -> (Type, Vec<Result<Token, String>>) {
//   // TODO implement
//   errors.push((
//     Pos("[todo]".to_string(), 0),
//     Error(format!("Right Shift unimplemented")),
//   ));
//
//   (Type::Int, vec![])
// }
//
// fn equal_to_expression(
//   expression1: Expression,
//   expression2: Expression,
//   state: &mut State,
//   errors: &mut Vec<(Pos, Error)>,
// ) -> (Type, Vec<Result<Token, String>>) {
//   let (r#type, tokens1, tokens2) =
//     typecheck::usual_arithmetic_conversion(expression1, expression2, state, errors);
//
//   (
//     Type::Bool,
//     match r#type {
//       Type::Bool => std::iter::empty()
//         .chain(tokens1)
//         .chain(tokens2)
//         .chain(vec![Ok(Token::Xor), Ok(Token::Clc)])
//         .collect(),
//       Type::Int | Type::Char => std::iter::empty()
//         .chain(tokens1)
//         .chain(tokens2)
//         .chain(vec![
//           Ok(Token::Xor),
//           Ok(Token::AtDyn),
//           Ok(Token::Pop),
//           Ok(Token::XXX(0x00)),
//           Ok(Token::Shl),
//           Ok(Token::AtDyn),
//         ])
//         .collect(),
//       _ => {
//         // TODO implement
//         errors.push((
//           Pos("[todo]".to_string(), 0),
//           Error(format!("Equal To unimplemented for type `{:?}`", r#type)),
//         ));
//         vec![]
//       }
//     },
//   )
// }
//
// fn not_equal_to_expression(
//   expression1: Expression,
//   expression2: Expression,
//   state: &mut State,
//   errors: &mut Vec<(Pos, Error)>,
// ) -> (Type, Vec<Result<Token, String>>) {
//   let (r#type, tokens1, tokens2) =
//     typecheck::usual_arithmetic_conversion(expression1, expression2, state, errors);
//
//   (
//     Type::Bool,
//     match r#type {
//       Type::Bool => std::iter::empty()
//         .chain(tokens1)
//         .chain(tokens2)
//         .chain(vec![
//           Ok(Token::Xor),
//           Ok(Token::XXX(0x01)),
//           Ok(Token::Xor),
//           Ok(Token::Clc),
//         ])
//         .collect(),
//       Type::Int | Type::Char => std::iter::empty()
//         .chain(tokens1)
//         .chain(tokens2)
//         .chain(vec![
//           Ok(Token::Xor),
//           Ok(Token::AtDyn),
//           Ok(Token::Pop),
//           Ok(Token::Flc),
//           Ok(Token::XXX(0x00)),
//           Ok(Token::Shl),
//           Ok(Token::AtDyn),
//         ])
//         .collect(),
//       _ => {
//         // TODO implement
//         errors.push((
//           Pos("[todo]".to_string(), 0),
//           Error(format!(
//             "Not Equal To unimplemented for type `{:?}`",
//             r#type
//           )),
//         ));
//         vec![]
//       }
//     },
//   )
// }
//
// fn less_than_expression(
//   expression1: Expression,
//   expression2: Expression,
//   state: &mut State,
//   errors: &mut Vec<(Pos, Error)>,
// ) -> (Type, Vec<Result<Token, String>>) {
//   let (r#type, tokens1, tokens2) =
//     typecheck::usual_arithmetic_conversion(expression1, expression2, state, errors);
//
//   (
//     Type::Bool,
//     match r#type {
//       Type::Int | Type::Char => std::iter::empty()
//         .chain(tokens1)
//         .chain(tokens2)
//         .chain(vec![
//           Ok(Token::Sub),
//           Ok(Token::AtDyn),
//           Ok(Token::Pop),
//           Ok(Token::XXX(0x00)),
//           Ok(Token::Shl),
//           Ok(Token::AtDyn),
//         ])
//         .collect(),
//       _ => {
//         // TODO implement
//         errors.push((
//           Pos("[todo]".to_string(), 0),
//           Error(format!("Less Than unimplemented for type `{:?}`", r#type)),
//         ));
//         vec![]
//       }
//     },
//   )
// }
//
// fn less_than_or_equal_to_expression(
//   expression1: Expression,
//   expression2: Expression,
//   state: &mut State,
//   errors: &mut Vec<(Pos, Error)>,
// ) -> (Type, Vec<Result<Token, String>>) {
//   let (r#type, tokens1, tokens2) =
//     typecheck::usual_arithmetic_conversion(expression1, expression2, state, errors);
//
//   (
//     Type::Bool,
//     match r#type {
//       Type::Int | Type::Char => std::iter::empty()
//         .chain(tokens1)
//         .chain(tokens2)
//         .chain(vec![
//           Ok(Token::Sub),
//           Ok(Token::AtDyn),
//           Ok(Token::Pop),
//           Ok(Token::Flc),
//           Ok(Token::XXX(0x00)),
//           Ok(Token::Shl),
//           Ok(Token::AtDyn),
//         ])
//         .collect(),
//       _ => {
//         // TODO implement
//         errors.push((
//           Pos("[todo]".to_string(), 0),
//           Error(format!(
//             "Less Than Or Equal To unimplemented for type `{:?}`",
//             r#type
//           )),
//         ));
//         vec![]
//       }
//     },
//   )
// }
//
// fn greater_than_expression(
//   expression1: Expression,
//   expression2: Expression,
//   state: &mut State,
//   errors: &mut Vec<(Pos, Error)>,
// ) -> (Type, Vec<Result<Token, String>>) {
//   let (r#type, tokens1, tokens2) =
//     typecheck::usual_arithmetic_conversion(expression1, expression2, state, errors);
//
//   (
//     Type::Bool,
//     match r#type {
//       Type::Int | Type::Char => std::iter::empty()
//         .chain(tokens1)
//         .chain(tokens2)
//         .chain(vec![
//           Ok(Token::Sub),
//           Ok(Token::AtDyn),
//           Ok(Token::Pop),
//           Ok(Token::XXX(0x00)),
//           Ok(Token::Shl),
//           Ok(Token::AtDyn),
//         ])
//         .collect(),
//       _ => {
//         // TODO implement
//         errors.push((
//           Pos("[todo]".to_string(), 0),
//           Error(format!(
//             "Greater Than unimplemented for type `{:?}`",
//             r#type
//           )),
//         ));
//         vec![]
//       }
//     },
//   )
// }
//
// fn greater_than_or_equal_to_expression(
//   expression1: Expression,
//   expression2: Expression,
//   state: &mut State,
//   errors: &mut Vec<(Pos, Error)>,
// ) -> (Type, Vec<Result<Token, String>>) {
//   let (r#type, tokens1, tokens2) =
//     typecheck::usual_arithmetic_conversion(expression1, expression2, state, errors);
//
//   (
//     Type::Bool,
//     match r#type {
//       Type::Int | Type::Char => std::iter::empty()
//         .chain(tokens1)
//         .chain(tokens2)
//         .chain(vec![
//           Ok(Token::Sub),
//           Ok(Token::AtDyn),
//           Ok(Token::Pop),
//           Ok(Token::Flc),
//           Ok(Token::XXX(0x00)),
//           Ok(Token::Shl),
//           Ok(Token::AtDyn),
//         ])
//         .collect(),
//       _ => {
//         // TODO implement
//         errors.push((
//           Pos("[todo]".to_string(), 0),
//           Error(format!(
//             "Greater Than Or Equal To unimplemented for type `{:?}`",
//             r#type
//           )),
//         ));
//         vec![]
//       }
//     },
//   )
// }
//
// fn conditional_expression(
//   expression1: Expression,
//   expression2: Expression,
//   expression3: Expression,
//   state: &mut State,
//   errors: &mut Vec<(Pos, Error)>,
// ) -> (Type, Vec<Result<Token, String>>) {
//   let (type1, tokens1) = typecheck::expression(expression1, state, errors);
//   let (type2, tokens2, tokens3) =
//     typecheck::usual_arithmetic_conversion(expression2, expression3, state, errors);
//
//   (
//     type2.clone(),
//     match type1 {
//       Type::Int | Type::Char => std::iter::empty()
//         .chain(tokens3)
//         .chain(tokens1)
//         .chain(tokens2)
//         .chain(vec![
//           Ok(Token::Buf),
//           Ok(Token::AtDyn),
//           Ok(Token::Pop),
//           Ok(Token::Iff),
//         ])
//         .collect(),
//       _ => {
//         // TODO implement
//         errors.push((
//           Pos("[todo]".to_string(), 0),
//           Error(format!(
//             "Conditional unimplemented for types `{:?}, {:?}`",
//             type1, type2
//           )),
//         ));
//         vec![]
//       }
//     },
//   )
// }

fn cast_expression(
  r#type: Type,
  expression: Expression,
  state: &mut State,
  errors: &mut Vec<(Pos, Error)>,
) -> (Type, TypedExpression) {
  let (type1, expression1) = typecheck::expression(expression, state, errors);

  (
    r#type.clone(),
    match (type1.clone(), r#type.clone()) {
      (type1, type2) if type1 == type2 => expression1,
      (type1, type2) if type1.width() == type2.width() => expression1,

      (Type::Int, Type::Bool)
      | (Type::UnsignedInt, Type::Bool)
      | (Type::Char, Type::Bool)
      | (Type::SignedChar, Type::Bool)
      | (Type::UnsignedChar, Type::Bool) => TypedExpression::N1IsZeroN8(Box::new(expression1)),

      (Type::Int, Type::Void)
      | (Type::UnsignedInt, Type::Void)
      | (Type::Char, Type::Void)
      | (Type::SignedChar, Type::Void)
      | (Type::UnsignedChar, Type::Void) => TypedExpression::N0CastN8(Box::new(expression1)),

      (Type::Bool, Type::Void) => TypedExpression::N0CastN1(Box::new(expression1)),

      _ => {
        errors.push((
          Pos("[todo]".to_string(), 0),
          Error(format!(
            "Type Cast unimplemented from `{:?}` to `{:?}`",
            type1, r#type
          )),
        ));
        TypedExpression::N0Constant(())
      }
    },
  )
}

fn string_literal_expression(
  value: String,
  state: &mut State,
  _errors: &mut Vec<(Pos, Error)>,
) -> (Type, TypedExpression) {
  let name = state
    .strings
    .entry(value.clone())
    .or_insert(format!("str.{}", state.uid));
  state.uid += 1;

  (
    Type::Pointer(Box::new(Type::Char)),
    TypedExpression::N8AddrGlobal(name.clone()),
  )
}

fn identifier_expression(
  identifier: String,
  state: &mut State,
  errors: &mut Vec<(Pos, Error)>,
) -> (Type, TypedExpression) {
  let mut offset = 0;

  state
    .stack
    .iter()
    .rev()
    .find_map(|stack_entry| match stack_entry {
      StackEntry::MacroBoundary(_, params_locals)
      | StackEntry::FunctionBoundary(_, params_locals)
      | StackEntry::BlockBoundary(params_locals) => {
        if let StackEntry::FunctionBoundary(_, _) = stack_entry {
          offset += 1; // return address
        }
        params_locals.iter().find_map(|Object(r#type, name)| {
          if *name != identifier {
            offset += r#type.size();
            return None;
          }

          Some(match r#type {
            Type::Function(_, _, _) => (
              Type::Pointer(Box::new(r#type.clone())),
              TypedExpression::N8AddrLocal(offset),
            ),

            Type::Macro(_, _, _, _) => panic!("Macro in local scope"),

            _ => (
              r#type.clone(),
              match r#type.range() {
                Range::U8 | Range::I8 => TypedExpression::N8GetLocal(offset),
                _ => todo!(),
              },
            ),
          })
        })
      }

      StackEntry::LoopBoundary => None,
    })
    .or_else(|| {
      state
        .declarations
        .get(&identifier)
        .map(|r#type| match r#type {
          Type::Function(_, _, _) => (
            Type::Pointer(Box::new(r#type.clone())),
            TypedExpression::N8AddrGlobal(identifier.clone()),
          ),

          Type::Macro(_, _, _, _) => (r#type.clone(), TypedExpression::N0Constant(())),

          _ => (
            r#type.clone(),
            match r#type.range() {
              Range::U8 | Range::I8 => TypedExpression::N8GetGlobal(identifier.clone()),
              _ => {
                errors.push((
                  Pos("[todo]".to_string(), 0),
                  Error(format!("Identifier unimplemented for type `{:?}`", r#type)),
                ));
                TypedExpression::N0Constant(())
              }
            },
          ),
        })
    })
    .unwrap_or_else(|| {
      errors.push((
        Pos("pos".to_string(), 0),
        Error(format!("Identifier `{}` not found", identifier)),
      ));
      (Type::Void, TypedExpression::N0Constant(()))
    })
}

fn function_call_expression(
  designator: Expression,
  arguments: Vec<Expression>,
  state: &mut State,
  errors: &mut Vec<(Pos, Error)>,
) -> (Type, TypedExpression) {
  let (designator_type, designator) = typecheck::expression(designator, state, errors);

  let designator_type = match designator_type {
    Type::Pointer(r#type) => *r#type,
    r#type => r#type,
  };

  let (inline_designator, return_type, parameter_types, is_variadic) = match designator_type {
    Type::Function(return_type, parameter_types, is_variadic) => {
      (None, return_type, parameter_types, is_variadic)
    }
    Type::Macro(return_type, name, parameter_types, is_variadic) => {
      (Some(name), return_type, parameter_types, is_variadic)
    }
    _ => {
      // TODO uses debug formatting
      errors.push((
        Pos("pos".to_string(), 0),
        Error(format!("`{:?}` is not a function", designator)),
      ));
      return (Type::Void, TypedExpression::N0Constant(()));
    }
  };

  if is_variadic && arguments.len() < parameter_types.len() {
    errors.push((
      Pos("pos".to_string(), 0),
      // TODO uses debug formatting
      Error(format!(
        "Expected at least {} arguments to variadic function `{:?}`, got {}",
        parameter_types.len(),
        designator,
        arguments.len()
      )),
    ));
  }

  if !is_variadic && arguments.len() != parameter_types.len() {
    errors.push((
      Pos("pos".to_string(), 0),
      // TODO uses debug formatting
      Error(format!(
        "Expected {} arguments to function `{:?}`, got {}",
        parameter_types.len(),
        designator,
        arguments.len()
      )),
    ));
  }

  let arguments =
    std::iter::empty()
      .chain(match is_variadic {
        true => arguments
          .iter()
          .skip(parameter_types.len())
          .rev()
          .map(|argument| {
            let (_type, expression) = typecheck::expression(argument.clone(), state, errors);
            expression
          })
          .collect(),
        false => vec![],
      })
      .chain(arguments.iter().zip(parameter_types.iter()).rev().map(
        |(argument, parameter_type)| {
          let (_type, expression) = typecheck::expression(
            Expression::Cast(parameter_type.clone(), Box::new(argument.clone())),
            state,
            errors,
          );
          expression
        },
      ))
      .collect();

  (
    *return_type.clone(),
    match (inline_designator, return_type.range()) {
      (Some(designator), Range::U0 | Range::I0) => {
        TypedExpression::N0MacroCall(designator, arguments)
      }
      (Some(designator), Range::U1 | Range::I1) => {
        TypedExpression::N1MacroCall(designator, arguments)
      }
      (Some(designator), Range::U8 | Range::I8) => {
        TypedExpression::N8MacroCall(designator, arguments)
      }
      (None, Range::U0 | Range::I0) => {
        TypedExpression::N0FunctionCall(Box::new(designator), arguments)
      }
      (None, Range::U1 | Range::I1) => {
        TypedExpression::N1FunctionCall(Box::new(designator), arguments)
      }
      (None, Range::U8 | Range::I8) => {
        TypedExpression::N8FunctionCall(Box::new(designator), arguments)
      }
      _ => {
        errors.push((
          Pos("[todo]".to_string(), 0),
          Error(format!(
            "Function call unimplemented for type `{:?}`",
            return_type
          )),
        ));
        TypedExpression::N0Constant(())
      }
    },
  )
}

fn integer_promotions(
  expression: Expression,
  state: &mut State,
  errors: &mut Vec<(Pos, Error)>,
) -> Expression {
  let (r#type, _) = typecheck::expression(expression.clone(), state, errors);

  match r#type {
    Type::Bool | Type::Char | Type::SignedChar | Type::Short | Type::Int | Type::Enumeration(_) => {
      Expression::Cast(Type::Int, Box::new(expression))
    }

    Type::UnsignedShort | Type::UnsignedChar | Type::UnsignedInt => {
      Expression::Cast(Type::UnsignedInt, Box::new(expression))
    }

    Type::Long | Type::UnsignedLong | Type::LongLong | Type::UnsignedLongLong => expression,

    Type::Void
    | Type::Array(_)
    | Type::Structure(_)
    | Type::Union(_)
    | Type::Macro(_, _, _, _)
    | Type::Function(_, _, _)
    | Type::Pointer(_) => expression,
  }
}

fn usual_arithmetic_conversions(
  (expression1, expression2): (Expression, Expression),
  state: &mut State,
  errors: &mut Vec<(Pos, Error)>,
) -> (Expression, Expression) {
  // TODO nonstandard, completely ad-hoc

  let (type1, _) = typecheck::expression(expression1.clone(), state, errors);
  let (type2, _) = typecheck::expression(expression2.clone(), state, errors);

  let r#type = match (type1.clone(), type2.clone()) {
    (
      Type::Void
      | Type::Array(_)
      | Type::Structure(_)
      | Type::Union(_)
      | Type::Macro(_, _, _, _)
      | Type::Function(_, _, _)
      | Type::Pointer(_),
      _,
    )
    | (
      _,
      Type::Void
      | Type::Array(_)
      | Type::Structure(_)
      | Type::Union(_)
      | Type::Macro(_, _, _, _)
      | Type::Function(_, _, _)
      | Type::Pointer(_),
    ) => {
      errors.push((
        Pos("pos".to_string(), 0),
        Error(format!(
          "Invalid operand types `{:?}` and `{:?}`",
          type1, type2
        )),
      ));

      Type::Int
    }

    (type1, type2) if type1 == type2 => type1,

    (Type::Char, Type::Int) | (Type::Int, Type::Char) => Type::Int,
    (Type::UnsignedInt, Type::Int) | (Type::Int, Type::UnsignedInt) => Type::UnsignedInt,

    _ => {
      errors.push((
        Pos("[todo]".to_string(), 0),
        Error(format!(
          // TODO uses debug formatting
          "Usual Arithmetic Conversions unimplemented between `{:?}` and `{:?}`",
          type1, type2
        )),
      ));

      type1
    }
  };

  (
    Expression::Cast(r#type.clone(), Box::new(expression1)),
    Expression::Cast(r#type.clone(), Box::new(expression2)),
  )
}

fn typecheck_usual_arithmetic_conversions(
  (expression1, expression2): (Expression, Expression),
  state: &mut State,
  errors: &mut Vec<(Pos, Error)>,
) -> (Type, TypedExpression, TypedExpression) {
  let (expression1, expression2) =
    usual_arithmetic_conversions((expression1, expression2), state, errors);

  let (type1, expression1) = typecheck::expression(expression1, state, errors);
  let (type2, expression2) = typecheck::expression(expression2, state, errors);

  if type1 != type2 {
    panic!("Expected expressions to have identical type`");
  }

  (type1.clone(), expression1, expression2)
}

fn pointer_arithmetic_conversions(
  (expression1, expression2): (Expression, Expression),
  state: &mut State,
  errors: &mut Vec<(Pos, Error)>,
) -> (Expression, Expression) {
  // TODO nonstandard, completely ad-hoc

  let (type1, _) = typecheck::expression(expression1.clone(), state, errors);
  let (type2, _) = typecheck::expression(expression2.clone(), state, errors);

  match (&type1, &type2) {
    (Type::Pointer(_) | Type::Array(_), Type::Pointer(_) | Type::Array(_)) => {
      errors.push((
        Pos("pos".to_string(), 0),
        Error(format!(
          "Invalid operand types `{:?}` and `{:?}`",
          type1, type2
        )),
      ));

      (
        Expression::IntegerConstant(0),
        Expression::IntegerConstant(0),
      )
    }

    (Type::Pointer(type1), _) => (
      expression1,
      Expression::Cast(
        Type::Pointer(type1.clone()),
        Box::new(Expression::Multiplication(
          Box::new(expression2),
          Box::new(Expression::IntegerConstant(type1.size() as u8)),
        )),
      ),
    ),

    (_, Type::Pointer(type2)) => (
      expression2,
      Expression::Cast(
        Type::Pointer(type2.clone()),
        Box::new(Expression::Multiplication(
          Box::new(expression1),
          Box::new(Expression::IntegerConstant(type2.size() as u8)),
        )),
      ),
    ),

    (_, _) => usual_arithmetic_conversions((expression1, expression2), state, errors),
  }
}

fn typecheck_pointer_arithmetic_conversions(
  (expression1, expression2): (Expression, Expression),
  state: &mut State,
  errors: &mut Vec<(Pos, Error)>,
) -> (Type, TypedExpression, TypedExpression) {
  let (expression1, expression2) =
    pointer_arithmetic_conversions((expression1, expression2), state, errors);

  let (type1, expression1) = typecheck::expression(expression1, state, errors);
  let (type2, expression2) = typecheck::expression(expression2, state, errors);

  if type1 != type2 {
    panic!("Expected expressions to have identical type`");
  }

  (type1.clone(), expression1, expression2)
}
