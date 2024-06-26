use crate::*;
use std::collections::{BTreeMap, HashMap, HashSet};

pub fn typecheck(program: Program, errors: &mut impl Extend<(Pos, Error)>) -> TypedProgram {
  typecheck::program(program, &mut State::default(), errors)
}

#[derive(Clone, PartialEq, Default, Debug)]
struct State {
  declarations: HashMap<String, Type>, // map from global declaration to its type
  definitions: HashSet<String>,        // set of currently defined globals
  strings: BTreeMap<String, String>,   // map from string literal to its label
  stack: Vec<StackEntry>,              // symbol stack, keeps track of current scopes
  uid: usize,                          // unique identifier for temporary identifiers
}

#[derive(Clone, PartialEq, Debug)]
enum StackEntry {
  MacroBoundary(Type, Vec<Object>),
  FunctionBoundary(Type, Vec<Object>), // parameters in "push" order (reverse of declaration)
  LoopBoundary(String),                // label
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
      Type::Char => Range::U8,
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
}

impl Object {
  pub fn size(&self) -> usize {
    match self {
      Object(r#type, _name) => r#type.size(),
    }
  }
}

fn program(
  program: Program,
  state: &mut State,
  errors: &mut impl Extend<(Pos, Error)>,
) -> TypedProgram {
  match program {
    Program(globals) => {
      let globals = globals
        .into_iter()
        .filter_map(|global| typecheck::global(global, state, errors))
        .collect::<Vec<_>>();
      let strings = state.strings.iter().map(|(value, label)| {
        TypedGlobal::Data(
          label.clone(),
          // TODO assumes C string literals are UTF-8 as a consequence of `.bytes()`
          value.bytes().map(TypedExpression::N8Constant).collect(),
        )
      });
      TypedProgram(strings.chain(globals).collect())
    }
  }
}

fn global(
  global: Global,
  state: &mut State,
  errors: &mut impl Extend<(Pos, Error)>,
) -> Option<TypedGlobal> {
  match global {
    Global::FunctionDeclaration(is_inline, Object(return_type, name), parameters, is_variadic) => {
      let () = typecheck::function_declaration_global(
        is_inline,
        Object(return_type, name),
        parameters,
        is_variadic,
        state,
        errors,
      );
      None
    }

    Global::FunctionDefinition(
      is_inline,
      Object(return_type, name),
      parameters,
      is_variadic,
      body,
    ) => {
      let global = typecheck::function_definition_global(
        is_inline,
        Object(return_type, name),
        parameters,
        is_variadic,
        body,
        state,
        errors,
      );
      Some(global)
    }

    Global::GlobalDeclaration(Object(r#type, name)) => {
      let () = typecheck::global_declaration_global(Object(r#type, name), state, errors);
      None
    }

    Global::GlobalDefinition(Object(r#type, name), value) => {
      let global = typecheck::global_definition_global(Object(r#type, name), value, state, errors);
      Some(global)
    }

    Global::GlobalAssembly(assembly) => {
      let global = typecheck::assembly_global(assembly, state, errors);
      Some(global)
    }
  }
}

fn function_declaration_global(
  is_inline: bool,
  Object(return_type, name): Object,
  parameters: Vec<Object>,
  is_variadic: bool,
  state: &mut State,
  errors: &mut impl Extend<(Pos, Error)>,
) -> () {
  let parameter_types: Vec<Type> = parameters
    .into_iter()
    .map(|Object(r#type, _name)| r#type)
    .collect();

  let func_type = match is_inline {
    true => Type::Macro(
      Box::new(return_type),
      name.clone(),
      parameter_types,
      is_variadic,
    ),
    false => Type::Function(Box::new(return_type), parameter_types, is_variadic),
  };

  state
    .declarations
    .entry(name.clone())
    .and_modify(|r#type| {
      if *r#type != func_type {
        errors.extend([(
          Pos(File("[pos]".into()), 0, 0),
          Error(format!(
            "Function `{}` of type `{}` previously declared with type `{}`",
            name, func_type, r#type
          )),
        )]);
      }
    })
    .or_insert(func_type);
}

fn function_definition_global(
  is_inline: bool,
  Object(return_type, name): Object,
  parameters: Vec<Object>,
  is_variadic: bool,
  body: Statement,
  state: &mut State,
  errors: &mut impl Extend<(Pos, Error)>,
) -> TypedGlobal {
  let () = typecheck::function_declaration_global(
    is_inline,
    Object(return_type.clone(), name.clone()),
    parameters.clone(),
    is_variadic,
    state,
    errors,
  );

  if is_variadic {
    errors.extend([(
      Pos(File("[todo]".into()), 0, 0),
      Error(format!("Variadic function definitions unimplemented")),
    )]);
  }

  if state.definitions.get(&name).is_some() {
    errors.extend([(
      Pos(File("[pos]".into()), 0, 0),
      Error(format!("Redefinition of function `{}`", name)),
    )]);
  }
  state.definitions.insert(name.clone());

  let mut rev_parameters = parameters;
  rev_parameters.reverse();

  state.stack.push(match is_inline {
    true => StackEntry::MacroBoundary(return_type, rev_parameters),
    false => StackEntry::FunctionBoundary(return_type, rev_parameters),
  });

  let global = match is_inline {
    true => TypedGlobal::Macro(
      name,
      typecheck::statement(body, state, errors),
      typecheck::return_statement(None, state, errors),
    ),
    false => TypedGlobal::Function(
      name,
      typecheck::statement(body, state, errors),
      typecheck::return_statement(None, state, errors),
    ),
  };

  state.stack.pop().unwrap();

  global
}

fn global_declaration_global(
  Object(global_type, name): Object,
  state: &mut State,
  errors: &mut impl Extend<(Pos, Error)>,
) -> () {
  state
    .declarations
    .entry(name.clone())
    .and_modify(|r#type| {
      if *r#type != global_type {
        errors.extend([(
          Pos(File("[pos]".into()), 0, 0),
          Error(format!(
            "Global `{}` of type `{}` previously declared with type `{}`",
            name, global_type, r#type
          )),
        )]);
      }
    })
    .or_insert(global_type);
}

fn global_definition_global(
  Object(global_type, name): Object,
  value: Expression,
  state: &mut State,
  errors: &mut impl Extend<(Pos, Error)>,
) -> TypedGlobal {
  let () =
    typecheck::global_declaration_global(Object(global_type.clone(), name.clone()), state, errors);

  let value = typecheck_expression_cast(global_type.clone(), value, state, errors);

  if state.definitions.get(&name).is_some() {
    errors.extend([(
      Pos(File("[pos]".into()), 0, 0),
      Error(format!("Redefinition of global `{}`", name)),
    )]);
  }
  state.definitions.insert(name.clone());

  match value {
    // TODO should this be constant expressions?
    TypedExpression::N0Constant(_) => TypedGlobal::Data(name, vec![value]),
    TypedExpression::N1Constant(_) => TypedGlobal::Data(name, vec![value]),
    TypedExpression::N8Constant(_) => TypedGlobal::Data(name, vec![value]),
    TypedExpression::N8AddrGlobal(_) => TypedGlobal::Data(name, vec![value]),
    _ => {
      errors.extend([(
        Pos(File("[todo]".into()), 0, 0),
        Error(format!(
          "Global initializer umimplemented for type `{}`",
          global_type
        )),
      )]);
      TypedGlobal::Data(name, vec![])
    }
  }
}

fn assembly_global(
  assembly: String,
  _state: &mut State,
  _errors: &mut impl Extend<(Pos, Error)>,
) -> TypedGlobal {
  TypedGlobal::Assembly(assembly)
}

fn statement(
  statement: Statement,
  state: &mut State,
  errors: &mut impl Extend<(Pos, Error)>,
) -> TypedStatement {
  match statement {
    Statement::Expression(expression) => typecheck::expression_statement(expression, state, errors),
    Statement::Compound(statements) => typecheck::compound_statement(statements, state, errors),
    Statement::If(condition, if_body, else_body) => typecheck::if_statement(
      condition,
      *if_body,
      else_body.map(|else_body| *else_body),
      state,
      errors,
    ),
    Statement::While(condition, body, is_do_while) => {
      typecheck::while_statement(condition, *body, is_do_while, state, errors)
    }
    Statement::Declaration(object, value) => {
      typecheck::declaration_statement(object, value, state, errors)
    }
    Statement::Break => typecheck::break_statement(state, errors),
    Statement::Continue => typecheck::continue_statement(state, errors),
    Statement::Return(expression) => typecheck::return_statement(expression, state, errors),
    Statement::Assembly(assembly) => typecheck::assembly_statement(assembly, state, errors),
  }
}

fn expression_statement(
  expression: Option<Expression>,
  state: &mut State,
  errors: &mut impl Extend<(Pos, Error)>,
) -> TypedStatement {
  let expression = match expression {
    Some(expression) => typecheck_expression_cast(Type::Void, expression, state, errors),
    None => TypedExpression::N0Constant(()), // null statement
  };

  TypedStatement::ExpressionN0(expression)
}

fn compound_statement(
  statements: Vec<Statement>,
  state: &mut State,
  errors: &mut impl Extend<(Pos, Error)>,
) -> TypedStatement {
  state.stack.push(StackEntry::BlockBoundary(vec![]));

  let body_statements: Vec<TypedStatement> = statements
    .into_iter()
    .map(|statement| typecheck::statement(statement, state, errors))
    .collect();

  let locals = match state.stack.pop().unwrap() {
    StackEntry::BlockBoundary(locals) => locals,
    _ => panic!("Expected block boundary to be on the stack"),
  };

  let uninit_statements: Vec<TypedStatement> = locals
    .iter()
    .rev()
    .map(|Object(r#type, _name)| match r#type.range() {
      Range::U0 | Range::I0 => TypedStatement::UninitLocalN0,
      Range::U1 | Range::I1 => TypedStatement::UninitLocalN1,
      Range::U8 | Range::I8 => TypedStatement::UninitLocalN8,
      _ => todo!(),
    })
    .collect();

  let statements = body_statements
    .into_iter()
    .chain(uninit_statements.into_iter())
    .collect();

  TypedStatement::Compound(statements)
}

fn while_statement(
  condition: Expression,
  body: Statement,
  is_do_while: bool,
  state: &mut State,
  errors: &mut impl Extend<(Pos, Error)>,
) -> TypedStatement {
  let condition = typecheck_expression_cast(Type::Bool, condition, state, errors);
  let label = format!("while.{}", state.uid);
  state.uid += 1;

  state.stack.push(StackEntry::LoopBoundary(label.clone()));

  let statement = TypedStatement::WhileN1(
    label,
    condition,
    Box::new(typecheck::statement(body, state, errors)),
    is_do_while,
  );

  state.stack.pop().unwrap();

  statement
}

fn if_statement(
  condition: Expression,
  if_body: Statement,
  else_body: Option<Statement>,
  state: &mut State,
  errors: &mut impl Extend<(Pos, Error)>,
) -> TypedStatement {
  let condition = typecheck_expression_cast(Type::Bool, condition, state, errors);
  let label = format!("if.{}", state.uid);
  state.uid += 1;

  let if_body = typecheck::statement(if_body, state, errors);
  let else_body = else_body.map(|else_body| typecheck::statement(else_body, state, errors));

  let statement =
    TypedStatement::IfN1(label, condition, Box::new(if_body), else_body.map(Box::new));

  statement
}

fn declaration_statement(
  object: Object,
  value: Option<Expression>,
  state: &mut State,
  errors: &mut impl Extend<(Pos, Error)>,
) -> TypedStatement {
  let Object(ref object_type, ref object_name) = object;
  let value =
    value.map(|value| typecheck_expression_cast(object_type.clone(), value, state, errors));

  let locals = match state.stack.last_mut().unwrap() {
    StackEntry::BlockBoundary(locals) => locals,
    _ => panic!("Expected block boundary to be on the stack"),
  };

  // remove this check to enable shadowing within a block
  if locals.iter().any(|Object(_, name)| *name == *object_name) {
    errors.extend([(
      Pos(File("[pos]".into()), 0, 0),
      Error(format!("Redefinition of local variable `{}`", object_name)),
    )]);
  }

  locals.push(object.clone());

  match object_type.range() {
    Range::U0 | Range::I0 => TypedStatement::InitLocalN0(value),
    Range::U1 | Range::I1 => TypedStatement::InitLocalN1(value),
    Range::U8 | Range::I8 => TypedStatement::InitLocalN8(value),
    _ => todo!(),
  }
}

fn break_statement(state: &mut State, errors: &mut impl Extend<(Pos, Error)>) -> TypedStatement {
  let mut locals_size = 0;
  let label = state
    .stack
    .iter()
    .rev()
    .find_map(|stack_entry| match stack_entry {
      StackEntry::MacroBoundary(_, _) | StackEntry::FunctionBoundary(_, _) => {
        errors.extend([(
          Pos(File("[pos]".into()), 0, 0),
          Error(format!("Use of `break` not within a loop")),
        )]);
        Some("".to_string())
      }
      StackEntry::LoopBoundary(label) => Some(label.clone()),
      StackEntry::BlockBoundary(locals) => {
        locals_size += locals.iter().map(Object::size).sum::<usize>();
        None
      }
    })
    .unwrap_or_else(|| panic!("Bare `break`"));

  TypedStatement::Break(label, locals_size)
}

fn continue_statement(state: &mut State, errors: &mut impl Extend<(Pos, Error)>) -> TypedStatement {
  let mut locals_size = 0;
  let label = state
    .stack
    .iter()
    .rev()
    .find_map(|stack_entry| match stack_entry {
      StackEntry::MacroBoundary(_, _) | StackEntry::FunctionBoundary(_, _) => {
        errors.extend([(
          Pos(File("[pos]".into()), 0, 0),
          Error(format!("Use of `continue` not within a loop")),
        )]);
        Some("".to_string())
      }
      StackEntry::LoopBoundary(label) => Some(label.clone()),
      StackEntry::BlockBoundary(locals) => {
        locals_size += locals.iter().map(Object::size).sum::<usize>();
        None
      }
    })
    .unwrap_or_else(|| panic!("Bare `continue`"));

  TypedStatement::Continue(label, locals_size)
}

fn return_statement(
  expression: Option<Expression>,
  state: &mut State,
  errors: &mut impl Extend<(Pos, Error)>,
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
      StackEntry::LoopBoundary(_label) => None,
      StackEntry::BlockBoundary(locals) => {
        locals_size += locals.iter().map(Object::size).sum::<usize>();
        None
      }
    })
    .unwrap_or_else(|| panic!("Bare `return`"));

  let expression = expression
    .map(|expression| typecheck_expression_cast(return_type.clone(), expression, state, errors));

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
      errors.extend([(
        Pos(File("[todo]".into()), 0, 0),
        Error(format!("Return unimplemented for type `{}`", return_type)),
      )]);
      TypedStatement::Assembly("".to_string())
    }
  }
}

fn assembly_statement(
  assembly: String,
  _state: &mut State,
  _errors: &mut impl Extend<(Pos, Error)>,
) -> TypedStatement {
  TypedStatement::Assembly(assembly)
}

fn expression(
  expression: Expression,
  state: &mut State,
  errors: &mut impl Extend<(Pos, Error)>,
) -> (Type, TypedExpression) {
  match expression {
    Expression::AddressOf(expression) => {
      typecheck::address_of_expression(*expression, state, errors)
    }

    Expression::Dereference(expression) => {
      let (r#type, expression) = typecheck::expression(*expression, state, errors);

      match r#type {
        Type::Pointer(r#type) => {
          let expression = match r#type.range() {
            Range::U0 | Range::I0 => {
              errors.extend([(
                Pos(File("[pos]".into()), 0, 0),
                Error(format!(
                  "Dereference of value of type `{}`",
                  Type::Pointer(r#type.clone())
                )),
              )]);
              dummy_typed_expression(&r#type)
            }
            Range::U1 | Range::I1 => TypedExpression::N1DereferenceN8(Box::new(expression)),
            Range::U8 | Range::I8 => TypedExpression::N8DereferenceN8(Box::new(expression)),
            _ => todo!(),
          };
          (*r#type, expression)
        }
        _ => {
          errors.extend([(
            Pos(File("[pos]".into()), 0, 0),
            Error(format!("Dereference of value of type `{}`", r#type)),
          )]);
          (r#type, expression)
        }
      }
    }

    Expression::Positive(expression) => {
      let promoted = integer_promotions(*expression, state, errors);
      let (r#type, expression) = typecheck::expression(promoted, state, errors);

      let expression = match r#type.range() {
        Range::U0 | Range::I0 | Range::U1 | Range::I1 => unreachable!(),
        Range::U8 | Range::I8 => expression,
        _ => todo!(),
      };
      (r#type, expression)
    }

    Expression::Negation(expression) => {
      let promoted = integer_promotions(*expression, state, errors);
      let (r#type, expression) = typecheck::expression(promoted, state, errors);

      let expression = match r#type.range() {
        Range::U0 | Range::I0 | Range::U1 | Range::I1 => unreachable!(),
        Range::U8 | Range::I8 => TypedExpression::N8Subtraction(
          Box::new(TypedExpression::N8Constant(0x00)),
          Box::new(expression),
        ),
        _ => todo!(),
      };
      (r#type, expression)
    }

    Expression::LogicalNegation(expression) => {
      let (r#type, expression) = typecheck::expression(*expression, state, errors);

      let expression = match r#type.range() {
        Range::U0 | Range::I0 => {
          errors.extend([(
            Pos(File("[pos]".into()), 0, 0),
            Error(format!("Logical negation of value of type `{}`", r#type)),
          )]);
          dummy_typed_expression(&Type::Bool)
        }
        Range::U1 | Range::I1 => TypedExpression::N1BitwiseComplement(Box::new(expression)),
        Range::U8 | Range::I8 => TypedExpression::N1EqualToN8(
          Box::new(expression),
          Box::new(TypedExpression::N8Constant(0x00)),
        ),
        _ => todo!(),
      };
      (Type::Bool, expression) // TODO logical negation returns `int` in C
    }

    Expression::BitwiseComplement(expression) => {
      let promoted = integer_promotions(*expression, state, errors);
      let (r#type, expression) = typecheck::expression(promoted, state, errors);

      let expression = match r#type.range() {
        Range::U0 | Range::I0 => {
          errors.extend([(
            Pos(File("[pos]".into()), 0, 0),
            Error(format!("Bitwise complement of value of type `{}`", r#type)),
          )]);
          dummy_typed_expression(&Type::Bool)
        }
        Range::U1 | Range::I1 => TypedExpression::N1BitwiseComplement(Box::new(expression)),
        Range::U8 | Range::I8 => TypedExpression::N8BitwiseComplement(Box::new(expression)),
        _ => todo!(),
      };
      (r#type, expression)
    }

    Expression::Addition(expression1, expression2) => {
      let promoted1 = integer_promotions(*expression1, state, errors);
      let promoted2 = integer_promotions(*expression2, state, errors);
      let (r#type, expression1, expression2) =
        typecheck_pointer_arithmetic_conversions((promoted1, promoted2), state, errors);

      let expression = match r#type.range() {
        Range::U0 | Range::I0 | Range::U1 | Range::I1 => unreachable!(),
        Range::U8 | Range::I8 => {
          TypedExpression::N8Addition(Box::new(expression1), Box::new(expression2))
        }
        _ => todo!(),
      };
      (r#type, expression)
    }

    Expression::Subtraction(expression1, expression2) => {
      let promoted1 = integer_promotions(*expression1, state, errors);
      let promoted2 = integer_promotions(*expression2, state, errors);
      let (r#type, expression1, expression2) =
        typecheck_pointer_arithmetic_conversions((promoted1, promoted2), state, errors);

      let expression = match r#type.range() {
        Range::U0 | Range::I0 | Range::U1 | Range::I1 => unreachable!(),
        Range::U8 | Range::I8 => {
          TypedExpression::N8Subtraction(Box::new(expression1), Box::new(expression2))
        }
        _ => todo!(),
      };
      (r#type, expression)
    }

    Expression::Multiplication(expression1, expression2) => {
      let promoted1 = integer_promotions(*expression1, state, errors);
      let promoted2 = integer_promotions(*expression2, state, errors);
      let (r#type, expression1, expression2) =
        typecheck_usual_arithmetic_conversions((promoted1, promoted2), state, errors);

      let expression = match r#type.range() {
        Range::U0 | Range::I0 | Range::U1 | Range::I1 => unreachable!(),
        Range::U8 | Range::I8 => {
          TypedExpression::N8Multiplication(Box::new(expression1), Box::new(expression2))
        }
        _ => todo!(),
      };
      (r#type, expression)
    }

    Expression::Division(expression1, expression2) => {
      let promoted1 = integer_promotions(*expression1, state, errors);
      let promoted2 = integer_promotions(*expression2, state, errors);
      let (r#type, expression1, expression2) =
        typecheck_usual_arithmetic_conversions((promoted1, promoted2), state, errors);

      let expression = match r#type.range() {
        Range::U0 | Range::I0 | Range::U1 | Range::I1 => unreachable!(),
        Range::U8 => TypedExpression::U8Division(Box::new(expression1), Box::new(expression2)),
        Range::I8 => {
          errors.extend([(
            Pos(File("[todo]".into()), 0, 0),
            Error(format!("Signed division unimplemented")),
          )]);
          TypedExpression::U8Division(Box::new(expression1), Box::new(expression2))
        }
        _ => todo!(),
      };
      (r#type, expression)
    }

    Expression::Modulo(expression1, expression2) => {
      let promoted1 = integer_promotions(*expression1, state, errors);
      let promoted2 = integer_promotions(*expression2, state, errors);
      let (r#type, expression1, expression2) =
        typecheck_usual_arithmetic_conversions((promoted1, promoted2), state, errors);

      let expression = match r#type.range() {
        Range::U0 | Range::I0 | Range::U1 | Range::I1 => unreachable!(),
        Range::U8 => TypedExpression::U8Modulo(Box::new(expression1), Box::new(expression2)),
        Range::I8 => {
          errors.extend([(
            Pos(File("[todo]".into()), 0, 0),
            Error(format!("Signed modulo unimplemented")),
          )]);
          TypedExpression::U8Modulo(Box::new(expression1), Box::new(expression2))
        }
        _ => todo!(),
      };
      (r#type, expression)
    }

    Expression::LogicalAnd(_, _) => todo!(),

    Expression::LogicalOr(_, _) => todo!(),

    Expression::BitwiseAnd(expression1, expression2) => {
      let promoted1 = integer_promotions(*expression1, state, errors);
      let promoted2 = integer_promotions(*expression2, state, errors);

      let (r#type, expression1, expression2) =
        typecheck_pointer_arithmetic_conversions((promoted1, promoted2), state, errors);

      let expression = match r#type.range() {
        Range::U0 | Range::I0 | Range::U1 | Range::I1 => unreachable!(),
        Range::U8 | Range::I8 => {
          TypedExpression::N8BitwiseAnd(Box::new(expression1), Box::new(expression2))
        }
        _ => todo!(),
      };
      (r#type, expression)
    }

    Expression::BitwiseInclusiveOr(expression1, expression2) => {
      let promoted1 = integer_promotions(*expression1, state, errors);
      let promoted2 = integer_promotions(*expression2, state, errors);

      let (r#type, expression1, expression2) =
        typecheck_pointer_arithmetic_conversions((promoted1, promoted2), state, errors);

      let expression = match r#type.range() {
        Range::U0 | Range::I0 | Range::U1 | Range::I1 => unreachable!(),
        Range::U8 | Range::I8 => {
          TypedExpression::N8BitwiseInclusiveOr(Box::new(expression1), Box::new(expression2))
        }
        _ => todo!(),
      };
      (r#type, expression)
    }

    Expression::BitwiseExclusiveOr(expression1, expression2) => {
      let promoted1 = integer_promotions(*expression1, state, errors);
      let promoted2 = integer_promotions(*expression2, state, errors);

      let (r#type, expression1, expression2) =
        typecheck_pointer_arithmetic_conversions((promoted1, promoted2), state, errors);

      let expression = match r#type.range() {
        Range::U0 | Range::I0 | Range::U1 | Range::I1 => unreachable!(),
        Range::U8 | Range::I8 => {
          TypedExpression::N8BitwiseExclusiveOr(Box::new(expression1), Box::new(expression2))
        }
        _ => todo!(),
      };
      (r#type, expression)
    }

    Expression::LeftShift(_, _) => todo!(),

    Expression::RightShift(_, _) => todo!(),

    Expression::EqualTo(expression1, expression2) => {
      let promoted1 = integer_promotions(*expression1, state, errors);
      let promoted2 = integer_promotions(*expression2, state, errors);
      let (r#type, expression1, expression2) =
        typecheck_pointer_arithmetic_conversions((promoted1, promoted2), state, errors);

      (
        Type::Bool,
        match r#type.range() {
          Range::U0 | Range::I0 | Range::U1 | Range::I1 => unreachable!(),
          Range::U8 | Range::I8 => {
            TypedExpression::N1EqualToN8(Box::new(expression1), Box::new(expression2))
          }
          _ => todo!(),
        },
      )
    }

    Expression::NotEqualTo(expression1, expression2) => typecheck::expression(
      Expression::LogicalNegation(Box::new(Expression::EqualTo(expression1, expression2))),
      state,
      errors,
    ),

    Expression::LessThan(expression1, expression2) => {
      let promoted1 = integer_promotions(*expression1, state, errors);
      let promoted2 = integer_promotions(*expression2, state, errors);
      let (r#type, expression1, expression2) =
        typecheck_pointer_arithmetic_conversions((promoted1, promoted2), state, errors);

      (
        Type::Bool,
        match r#type.range() {
          Range::U0 | Range::I0 | Range::U1 | Range::I1 => unreachable!(),
          Range::U8 => TypedExpression::N1LessThanU8(Box::new(expression1), Box::new(expression2)),
          Range::I8 => TypedExpression::N1LessThanI8(Box::new(expression1), Box::new(expression2)),
          _ => todo!(),
        },
      )
    }

    Expression::LessThanOrEqualTo(expression1, expression2) => typecheck::expression(
      Expression::LogicalNegation(Box::new(Expression::GreaterThan(expression1, expression2))),
      state,
      errors,
    ),

    Expression::GreaterThan(expression1, expression2) => {
      let promoted1 = integer_promotions(*expression1, state, errors);
      let promoted2 = integer_promotions(*expression2, state, errors);
      let (r#type, expression1, expression2) =
        typecheck_pointer_arithmetic_conversions((promoted1, promoted2), state, errors);

      (
        Type::Bool,
        match r#type.range() {
          Range::U0 | Range::I0 | Range::U1 | Range::I1 => unreachable!(),
          Range::U8 => TypedExpression::N1LessThanU8(Box::new(expression2), Box::new(expression1)),
          Range::I8 => TypedExpression::N1LessThanI8(Box::new(expression2), Box::new(expression1)),
          _ => todo!(),
        },
      )
    }

    Expression::GreaterThanOrEqualTo(expression1, expression2) => typecheck::expression(
      Expression::LogicalNegation(Box::new(Expression::LessThan(expression1, expression2))),
      state,
      errors,
    ),

    Expression::Conditional(_, _, _) => todo!(),

    Expression::Comma(expression1, expression2) => {
      let expression1 = typecheck_expression_cast(Type::Void, *expression1, state, errors);
      let (r#type, expression2) = typecheck::expression(*expression2, state, errors);

      let expression = match r#type.range() {
        Range::U0 | Range::I0 => {
          TypedExpression::N0SecondN0N0(Box::new(expression1), Box::new(expression2))
        }
        Range::U1 | Range::I1 => {
          TypedExpression::N1SecondN0N1(Box::new(expression1), Box::new(expression2))
        }
        Range::U8 | Range::I8 => {
          TypedExpression::N8SecondN0N8(Box::new(expression1), Box::new(expression2))
        }
        _ => todo!(),
      };
      (r#type, expression)
    }

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

    Expression::Subscript(expression1, expression2) => {
      let (r#type, expression) = typecheck::expression(
        Expression::Addition(expression1.clone(), expression2.clone()),
        state,
        &mut vec![],
      );

      match r#type {
        Type::Pointer(_) => typecheck::expression(
          Expression::Dereference(Box::new(Expression::Addition(expression1, expression2))),
          state,
          errors,
        ),
        _ => {
          errors.extend([(
            Pos(File("[pos]".into()), 0, 0),
            Error(format!("Subscript of value of type `{}`", r#type)),
          )]);
          (r#type, expression)
        }
      }
    }

    Expression::FunctionCall(designator, arguments) => {
      typecheck::function_call_expression(*designator, arguments, state, errors)
    }
  }
}

fn address_of_expression(
  expression: Expression,
  state: &mut State,
  errors: &mut impl Extend<(Pos, Error)>,
) -> (Type, TypedExpression) {
  match expression {
    Expression::Dereference(expression) => {
      let (r#type, expression) = typecheck::expression(*expression, state, errors);

      match r#type {
        Type::Pointer(r#type) => (Type::Pointer(r#type), expression),
        _ => {
          errors.extend([(
            Pos(File("[pos]".into()), 0, 0),
            Error(format!("Dereference of value of type `{}`", r#type)),
          )]);
          (r#type, expression)
        }
      }
    }

    Expression::Subscript(expression1, expression2) => {
      let (r#type, expression) = typecheck::expression(
        Expression::Addition(expression1, expression2),
        state,
        errors,
      );

      match r#type {
        Type::Pointer(r#type) => (Type::Pointer(r#type), expression),
        _ => {
          errors.extend([(
            Pos(File("[pos]".into()), 0, 0),
            Error(format!("Subscript of value of type `{}`", r#type)),
          )]);
          (r#type, expression)
        }
      }
    }

    // TODO code duplication with `identifier_expression`
    Expression::Identifier(identifier) => {
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
            params_locals.iter().rev().find_map(|Object(r#type, name)| {
              if *name != identifier {
                offset += r#type.size();
                return None;
              }

              Some((
                r#type.clone(),
                match r#type.range() {
                  Range::U8 | Range::I8 => TypedExpression::N8LoadLocal(offset),
                  _ => todo!(),
                },
              ))
            })
          }

          StackEntry::LoopBoundary(_label) => None,
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

              Type::Macro(_, _, _, _) => {
                errors.extend([(
                  Pos(File("[pos]".into()), 0, 0),
                  Error(format!("Address of macro `{}`", identifier)),
                )]);
                (r#type.clone(), dummy_typed_expression(&Type::Void))
              }

              _ => (
                r#type.clone(),
                match r#type.range() {
                  Range::U8 | Range::I8 => TypedExpression::N8AddrGlobal(identifier.clone()),
                  _ => todo!(),
                },
              ),
            })
        })
        .unwrap_or_else(|| {
          errors.extend([(
            Pos(File("[pos]".into()), 0, 0),
            Error(format!("Address of undeclared identifier `{}`", identifier)),
          )]);
          dummy_type_typed_expression(Type::Void)
        })
    }

    _ => {
      let (r#type, expression) = typecheck::expression(expression, state, errors);

      errors.extend([(
        Pos(File("[pos]".into()), 0, 0),
        Error(format!("Address of value of type `{}`", r#type)),
      )]);
      (r#type, expression)
    }
  }
}

fn cast_expression(
  r#type: Type,
  expression: Expression,
  state: &mut State,
  errors: &mut impl Extend<(Pos, Error)>,
) -> (Type, TypedExpression) {
  fn width(r#type: &Type) -> usize {
    match r#type.range() {
      Range::U0 | Range::I0 => 0,
      Range::U1 | Range::I1 => 1,
      Range::U8 | Range::I8 => 8,
      Range::U16 | Range::I16 => 16,
      Range::U32 | Range::I32 => 32,
    }
  }

  let (type1, expression1) = typecheck::expression(expression, state, errors);

  let expression = match (type1, &r#type) {
    (type1 @ Type::Macro(_, _, _, _), r#type) => {
      errors.extend([(
        Pos(File("[pos]".into()), 0, 0),
        Error(format!("Cast from macro type `{}`, to `{}`", type1, r#type)),
      )]);
      dummy_typed_expression(r#type)
    }

    (type1, r#type @ Type::Macro(_, _, _, _)) => {
      errors.extend([(
        Pos(File("[pos]".into()), 0, 0),
        Error(format!("Cast to macro type `{}`, from `{}`", r#type, type1)),
      )]);
      dummy_typed_expression(r#type)
    }

    (type1, r#type) if type1 == *r#type => expression1,
    (type1, r#type) if width(&type1) == width(&r#type) => expression1,

    (Type::Int, Type::Bool)
    | (Type::UnsignedInt, Type::Bool)
    | (Type::Char, Type::Bool)
    | (Type::SignedChar, Type::Bool)
    | (Type::UnsignedChar, Type::Bool)
    | (Type::Pointer(_), Type::Bool) => {
      TypedExpression::N1BitwiseComplement(Box::new(TypedExpression::N1EqualToN8(
        Box::new(expression1),
        Box::new(TypedExpression::N8Constant(0x00)),
      )))
    }

    (Type::Int, Type::Void)
    | (Type::UnsignedInt, Type::Void)
    | (Type::Char, Type::Void)
    | (Type::SignedChar, Type::Void)
    | (Type::UnsignedChar, Type::Void)
    | (Type::Pointer(_), Type::Void) => TypedExpression::N0CastN8(Box::new(expression1)),

    (Type::Bool, Type::Int)
    | (Type::Bool, Type::UnsignedInt)
    | (Type::Bool, Type::Char)
    | (Type::Bool, Type::SignedChar)
    | (Type::Bool, Type::UnsignedChar)
    | (Type::Bool, Type::Pointer(_)) => TypedExpression::N8CastN1(Box::new(expression1)),

    (Type::Bool, Type::Void) => TypedExpression::N0CastN1(Box::new(expression1)),

    (type1, r#type) => {
      errors.extend([(
        Pos(File("[todo]".into()), 0, 0),
        Error(format!(
          "Type Cast unimplemented from `{}` to `{}`",
          type1, r#type
        )),
      )]);
      dummy_typed_expression(r#type)
    }
  };

  (r#type, expression)
}

fn string_literal_expression(
  value: String,
  state: &mut State,
  _errors: &mut impl Extend<(Pos, Error)>,
) -> (Type, TypedExpression) {
  let name = state.strings.entry(value).or_insert_with(|| {
    let label = format!("str.{}", state.uid);
    state.uid += 1;
    label
  });

  (
    Type::Pointer(Box::new(Type::Char)),
    TypedExpression::N8AddrGlobal(name.clone()),
  )
}

fn identifier_expression(
  identifier: String,
  state: &mut State,
  errors: &mut impl Extend<(Pos, Error)>,
) -> (Type, TypedExpression) {
  // TODO code duplication with `address_of_expression`

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
        params_locals.iter().rev().find_map(|Object(r#type, name)| {
          if *name != identifier {
            offset += r#type.size();
            return None;
          }

          Some((
            r#type.clone(),
            match r#type.range() {
              Range::U8 | Range::I8 => TypedExpression::N8LoadLocal(offset),
              _ => todo!(),
            },
          ))
        })
      }

      StackEntry::LoopBoundary(_label) => None,
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

          Type::Macro(_, _, _, _) => (r#type.clone(), dummy_typed_expression(&Type::Void)),

          _ => (
            r#type.clone(),
            match r#type.range() {
              Range::U8 | Range::I8 => TypedExpression::N8LoadGlobal(identifier.clone()),
              _ => todo!(),
            },
          ),
        })
    })
    .unwrap_or_else(|| {
      errors.extend([(
        Pos(File("[pos]".into()), 0, 0),
        Error(format!(
          "Reference to undeclared identifier `{}`",
          identifier
        )),
      )]);
      dummy_type_typed_expression(Type::Void)
    })
}

fn function_call_expression(
  designator: Expression,
  arguments: Vec<Expression>,
  state: &mut State,
  errors: &mut impl Extend<(Pos, Error)>,
) -> (Type, TypedExpression) {
  let (designator_type, designator) = typecheck::expression(designator, state, errors);

  let designator_type = match designator_type {
    Type::Pointer(r#type) => *r#type,
    r#type => r#type,
  };

  let (inline_name, return_type, parameter_types, is_variadic) = match designator_type {
    Type::Function(ref return_type, ref parameter_types, ref is_variadic) => {
      (None, return_type, parameter_types, is_variadic)
    }
    Type::Macro(ref return_type, ref name, ref parameter_types, ref is_variadic) => {
      (Some(name), return_type, parameter_types, is_variadic)
    }
    _ => {
      errors.extend([(
        Pos(File("[pos]".into()), 0, 0),
        Error(format!(
          "Function call on value of type `{}`",
          designator_type
        )),
      )]);
      return (designator_type, designator);
    }
  };

  if match *is_variadic {
    true => arguments.len() < parameter_types.len(),
    false => arguments.len() != parameter_types.len(),
  } {
    errors.extend([(
      Pos(File("[pos]".into()), 0, 0),
      Error(format!(
        "Function of type `{}` called with {} arguments",
        designator_type,
        arguments.len()
      )),
    )]);
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

  let expression = match (inline_name, return_type.range()) {
    (Some(name), Range::U0 | Range::I0) => TypedExpression::N0MacroCall(name.clone(), arguments),
    (Some(name), Range::U1 | Range::I1) => TypedExpression::N1MacroCall(name.clone(), arguments),
    (Some(name), Range::U8 | Range::I8) => TypedExpression::N8MacroCall(name.clone(), arguments),
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
      errors.extend([(
        Pos(File("[todo]".into()), 0, 0),
        Error(format!(
          "Function call unimplemented for return type `{}`",
          return_type
        )),
      )]);
      dummy_typed_expression(return_type)
    }
  };
  (*return_type.clone(), expression)
}

fn integer_promotions(
  expression: Expression,
  state: &mut State,
  _errors: &mut impl Extend<(Pos, Error)>,
) -> Expression {
  let (r#type, _) = typecheck::expression(expression.clone(), state, &mut vec![]);

  match r#type {
    Type::Bool | Type::SignedChar | Type::Short | Type::Int | Type::Enumeration(_) => {
      Expression::Cast(Type::Int, Box::new(expression))
    }

    Type::UnsignedShort | Type::Char | Type::UnsignedChar | Type::UnsignedInt => {
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
  errors: &mut impl Extend<(Pos, Error)>,
) -> (Expression, Expression) {
  // TODO nonstandard, completely ad-hoc

  let (type1, _) = typecheck::expression(expression1.clone(), state, &mut vec![]);
  let (type2, _) = typecheck::expression(expression2.clone(), state, &mut vec![]);

  let r#type = match (type1, type2) {
    (
      type1 @ Type::Void
      | type1 @ Type::Array(_)
      | type1 @ Type::Structure(_)
      | type1 @ Type::Union(_)
      | type1 @ Type::Macro(_, _, _, _)
      | type1 @ Type::Function(_, _, _)
      | type1 @ Type::Pointer(_),
      type2,
    )
    | (
      type1,
      type2 @ Type::Void
      | type2 @ Type::Array(_)
      | type2 @ Type::Structure(_)
      | type2 @ Type::Union(_)
      | type2 @ Type::Macro(_, _, _, _)
      | type2 @ Type::Function(_, _, _)
      | type2 @ Type::Pointer(_),
    ) => {
      errors.extend([(
        Pos(File("[pos]".into()), 0, 0),
        Error(format!("Invalid operand types `{}` and `{}`", type1, type2)),
      )]);

      Type::Int
    }

    (type1, type2) if type1 == type2 => type1,

    (Type::Char, Type::Int) | (Type::Int, Type::Char) => Type::Int,
    (Type::UnsignedInt, Type::Int) | (Type::Int, Type::UnsignedInt) => Type::UnsignedInt,

    (type1, type2) => {
      errors.extend([(
        Pos(File("[todo]".into()), 0, 0),
        Error(format!(
          "Usual Arithmetic Conversions unimplemented between `{}` and `{}`",
          type1, type2
        )),
      )]);

      type1
    }
  };

  (
    Expression::Cast(r#type.clone(), Box::new(expression1)),
    Expression::Cast(r#type, Box::new(expression2)),
  )
}

fn typecheck_usual_arithmetic_conversions(
  (expression1, expression2): (Expression, Expression),
  state: &mut State,
  errors: &mut impl Extend<(Pos, Error)>,
) -> (Type, TypedExpression, TypedExpression) {
  let (expression1, expression2) =
    usual_arithmetic_conversions((expression1, expression2), state, errors);

  let (type1, expression1) = typecheck::expression(expression1, state, errors);
  let (type2, expression2) = typecheck::expression(expression2, state, errors);

  assert_eq!(type1, type2, "Expected expressions to have identical type`");

  (type1, expression1, expression2)
}

fn pointer_arithmetic_conversions(
  (expression1, expression2): (Expression, Expression),
  state: &mut State,
  errors: &mut impl Extend<(Pos, Error)>,
) -> (Expression, Expression) {
  // TODO nonstandard, completely ad-hoc

  let (type1, _) = typecheck::expression(expression1.clone(), state, &mut vec![]);
  let (type2, _) = typecheck::expression(expression2.clone(), state, &mut vec![]);

  match (type1, type2) {
    (Type::Pointer(_), Type::Pointer(_)) => (
      Expression::Cast(Type::UnsignedInt, Box::new(expression1)),
      Expression::Cast(Type::UnsignedInt, Box::new(expression2)),
    ),

    (Type::Pointer(type1), _) => {
      let size = type1.size() as u8;
      (
        expression1,
        Expression::Cast(
          Type::Pointer(type1),
          Box::new(Expression::Multiplication(
            Box::new(expression2),
            Box::new(Expression::IntegerConstant(size)),
          )),
        ),
      )
    }

    (_, Type::Pointer(type2)) => {
      let size = type2.size() as u8;
      (
        expression2,
        Expression::Cast(
          Type::Pointer(type2),
          Box::new(Expression::Multiplication(
            Box::new(expression1),
            Box::new(Expression::IntegerConstant(size)),
          )),
        ),
      )
    }

    (_, _) => usual_arithmetic_conversions((expression1, expression2), state, errors),
  }
}

fn typecheck_pointer_arithmetic_conversions(
  (expression1, expression2): (Expression, Expression),
  state: &mut State,
  errors: &mut impl Extend<(Pos, Error)>,
) -> (Type, TypedExpression, TypedExpression) {
  let (expression1, expression2) =
    pointer_arithmetic_conversions((expression1, expression2), state, errors);

  let (type1, expression1) = typecheck::expression(expression1, state, errors);
  let (type2, expression2) = typecheck::expression(expression2, state, errors);

  assert_eq!(type1, type2, "Expected expressions to have identical type`");

  (type1, expression1, expression2)
}

fn typecheck_expression_cast(
  r#type: Type,
  expression: Expression,
  state: &mut State,
  errors: &mut impl Extend<(Pos, Error)>,
) -> TypedExpression {
  let (r#type, expression) = typecheck::expression(
    Expression::Cast(r#type, Box::new(expression)),
    state,
    errors,
  );

  assert_eq!(r#type, r#type, "Expected expression to have requested type");

  expression
}

fn dummy_typed_expression(r#type: &Type) -> TypedExpression {
  match r#type.range() {
    Range::U0 | Range::I0 => TypedExpression::N0Constant(()),
    Range::U1 | Range::I1 => TypedExpression::N1Constant(false),
    Range::U8 | Range::I8 => TypedExpression::N8Constant(0x00),
    _ => todo!(),
  }
}

fn dummy_type_typed_expression(r#type: Type) -> (Type, TypedExpression) {
  let expression = dummy_typed_expression(&r#type);
  (r#type, expression)
}
