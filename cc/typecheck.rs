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
      Range::U8 | Range::I8 => 8,
      Range::U16 | Range::I16 => 16,
      Range::U32 | Range::I32 => 32,
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
        Object(return_type.clone(), name.clone()),
        parameters.clone(),
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
        Object(return_type.clone(), name.clone()),
        parameters.clone(),
        is_variadic,
        body,
        state,
        errors,
      );
      Some(global)
    }

    Global::GlobalDeclaration(Object(r#type, name)) => {
      let () =
        typecheck::global_declaration_global(Object(r#type.clone(), name.clone()), state, errors);
      None
    }

    Global::GlobalDefinition(Object(r#type, name), value) => {
      let global = typecheck::global_definition_global(
        Object(r#type.clone(), name.clone()),
        value,
        state,
        errors,
      );
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

  state
    .declarations
    .entry(name.clone())
    .and_modify(|r#type| match (is_inline, r#type) {
      (false, Type::Function(return_type_, parameter_types_, is_variadic_))
      | (true, Type::Macro(return_type_, _, parameter_types_, is_variadic_))
        if return_type == **return_type_
          && parameter_types == *parameter_types_
          && is_variadic == *is_variadic_ => {}
      _ => errors.extend([(
        Pos(File("[pos]".into()), 0, 0),
        Error(format!(
          "Function `{}` previously declared with different prototype",
          name.clone()
        )),
      )]),
    })
    .or_insert(match is_inline {
      true => Type::Macro(Box::new(return_type), name, parameter_types, is_variadic),
      false => Type::Function(Box::new(return_type), parameter_types, is_variadic),
    });
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
  fn control_always_excapes(statement: &Statement) -> bool {
    match statement {
      Statement::Expression(_) => false,
      Statement::Compound(statements) => statements.iter().any(control_always_excapes),
      Statement::If(_, if_body, else_body) => {
        else_body
          .as_deref()
          .map(control_always_excapes)
          .unwrap_or(false)
          && control_always_excapes(if_body)
      }
      Statement::While(_, body) => control_always_excapes(body), // TODO or condition always true
      Statement::Return(_) => true,
      Statement::Declaration(_, _) => false,
      Statement::Assembly(_) => false,
    }
  }

  let body = match control_always_excapes(&body) {
    false => Statement::Compound(vec![body, Statement::Return(None)]),
    true => body,
  };

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
      Error(format!("Variadic function definitions unimplemented",)),
    )])
  }

  state.definitions.get(&name).is_some().then(|| {
    errors.extend([(
      Pos(File("[pos]".into()), 0, 0),
      Error(format!(
        "Function `{}` has already been defined",
        name.clone()
      )),
    )])
  });
  state.definitions.insert(name.clone());

  let mut rev_parameters = parameters;
  rev_parameters.reverse();

  state.stack.push(match is_inline {
    true => StackEntry::MacroBoundary(return_type.clone(), rev_parameters),
    false => StackEntry::FunctionBoundary(return_type.clone(), rev_parameters),
  });

  let statement = match is_inline {
    true => TypedGlobal::Macro(name, typecheck::statement(body, state, errors)),
    false => TypedGlobal::Function(name, typecheck::statement(body, state, errors)),
  };

  state.stack.pop().unwrap();

  statement
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
            "Global `{}` previously declared with different type",
            name.clone()
          )),
        )])
      }
    })
    .or_insert(global_type.clone());
}

fn global_definition_global(
  Object(global_type, name): Object,
  value: Expression,
  state: &mut State,
  errors: &mut impl Extend<(Pos, Error)>,
) -> TypedGlobal {
  let () =
    typecheck::global_declaration_global(Object(global_type.clone(), name.clone()), state, errors);

  let (r#type, value) = typecheck::expression(
    Expression::Cast(global_type.clone(), Box::new(value)),
    state,
    errors,
  );

  if global_type != r#type {
    panic!("Expected global type to match initializer type");
  }

  state.definitions.get(&name).is_some().then(|| {
    errors.extend([(
      Pos(File("[pos]".into()), 0, 0),
      Error(format!(
        "Global `{}` has already been defined",
        name.clone()
      )),
    )])
  });
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
          "Global initializer umimplemented for expression `{:?}`",
          value
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
    Statement::While(condition, body) => {
      typecheck::while_statement(condition, *body, state, errors)
    }
    Statement::Declaration(object, value) => {
      typecheck::declaration_statement(object, value, state, errors)
    }
    Statement::Return(expression) => typecheck::return_statement(expression, state, errors),
    Statement::Assembly(assembly) => typecheck::assembly_statement(assembly, state, errors),
  }
}

fn expression_statement(
  expression: Expression,
  state: &mut State,
  errors: &mut impl Extend<(Pos, Error)>,
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
  state: &mut State,
  errors: &mut impl Extend<(Pos, Error)>,
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
  let (r#type, condition) = typecheck::expression(
    Expression::Cast(Type::Bool, Box::new(condition)),
    state,
    errors,
  );

  if r#type != Type::Bool {
    panic!("Expected if condition to have type `bool`");
  }

  let if_body = typecheck::statement(if_body, state, errors);
  let else_body = else_body.map(|else_body| typecheck::statement(else_body, state, errors));

  let statement = TypedStatement::IfN1(
    format!("if.{}", state.uid),
    condition,
    Box::new(if_body),
    else_body.map(Box::new),
  );
  state.uid += 1;

  statement
}

fn declaration_statement(
  object: Object,
  value: Option<Expression>,
  state: &mut State,
  errors: &mut impl Extend<(Pos, Error)>,
) -> TypedStatement {
  let Object(object_type, object_name) = object.clone();
  let value = value.map(|value| {
    let (r#type, value) = typecheck::expression(
      Expression::Cast(object_type.clone(), Box::new(value)),
      state,
      errors,
    );

    if r#type != object_type {
      panic!("Expected declaration type to match initializer type");
    }

    value
  });

  let locals = state
    .stack
    .iter_mut()
    .rev()
    .find_map(|stack_entry| match stack_entry {
      StackEntry::BlockBoundary(locals) => Some(locals),
      _ => None,
    })
    .unwrap_or_else(|| {
      panic!("Expected block boundary to be on the stack");
    });

  // removing this check enables shadowing
  if locals.iter().any(|Object(_, name)| *name == object_name) {
    errors.extend([(
      Pos(File("[pos]".into()), 0, 0),
      Error(format!("Redeclaration of variable `{}`", object_name)),
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
      StackEntry::LoopBoundary => None,
      StackEntry::BlockBoundary(locals) => {
        locals_size += locals.iter().map(Object::size).sum::<usize>();
        None
      }
    })
    .unwrap_or_else(|| {
      errors.extend([(
        Pos(File("[pos]".into()), 0, 0),
        Error(format!("`return` encountered outside of function")),
      )]);
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
      errors.extend([(
        Pos(File("[todo]".into()), 0, 0),
        Error(format!("Return unimplemented for type `{:?}`", return_type)),
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
  match expression.clone() {
    // Expression::AddressOf(expression) => match expression.clone() {
    //   typecheck::address_of_expression(*expression, state, errors)
    // },
    //
    Expression::Dereference(expression) => {
      let (r#type, expression) = typecheck::expression(*expression, state, errors);

      match r#type {
        Type::Pointer(r#type) => (
          *r#type.clone(),
          match r#type.range() {
            Range::U0 | Range::I0 => TypedExpression::N0Dereference(Box::new(expression)),
            Range::U1 | Range::I1 => TypedExpression::N1Dereference(Box::new(expression)),
            Range::U8 | Range::I8 => TypedExpression::N8Dereference(Box::new(expression)),
            _ => todo!(),
          },
        ),
        _ => {
          errors.extend([(
            // TODO uses debug formatting
            Pos(File("[pos]".into()), 0, 0),
            Error(format!("Dereference of type `{:?}`", r#type)),
          )]);
          (Type::Void, TypedExpression::N0Constant(()))
        }
      }
    }

    Expression::Positive(expression) => {
      let promoted = integer_promotions(*expression, state, errors);
      let (r#type, expression) = typecheck::expression(promoted, state, errors);

      (
        r#type.clone(),
        match r#type.range() {
          Range::U0 | Range::I0 | Range::U1 | Range::I1 => unreachable!(),
          Range::U8 | Range::I8 => expression,
          _ => todo!(),
        },
      )
    }

    Expression::Negation(expression) => {
      let promoted = integer_promotions(*expression, state, errors);
      let (r#type, expression) = typecheck::expression(promoted, state, errors);

      (
        r#type.clone(),
        match r#type.range() {
          Range::U0 | Range::I0 | Range::U1 | Range::I1 => unreachable!(),
          Range::U8 | Range::I8 => TypedExpression::N8Subtraction(
            Box::new(TypedExpression::N8Constant(0x00)),
            Box::new(expression),
          ),
          _ => todo!(),
        },
      )
    }

    Expression::LogicalNegation(expression) => {
      let (r#type, expression) = typecheck::expression(*expression, state, errors);

      (
        Type::Bool, // TODO logical negation returns `int` in C
        match r#type.range() {
          Range::U0 | Range::I0 => {
            errors.extend([(
              // TODO uses debug formatting
              Pos(File("[pos]".into()), 0, 0),
              Error(format!("Logical negation of type `{:?}`", r#type)),
            )]);
            TypedExpression::N1Constant(false)
          }
          Range::U1 | Range::I1 => TypedExpression::N1BitwiseComplement(Box::new(expression)),
          Range::U8 | Range::I8 => TypedExpression::N1EqualToN8(
            Box::new(expression),
            Box::new(TypedExpression::N8Constant(0x00)),
          ),
          _ => todo!(),
        },
      )
    }

    Expression::BitwiseComplement(expression) => {
      let promoted = integer_promotions(*expression, state, errors);
      let (r#type, expression) = typecheck::expression(promoted, state, errors);

      (
        r#type.clone(),
        match r#type.range() {
          Range::U0 | Range::I0 => {
            errors.extend([(
              // TODO uses debug formatting
              Pos(File("[pos]".into()), 0, 0),
              Error(format!("Bitwise complement of type `{:?}`", r#type)),
            )]);
            TypedExpression::N0Constant(())
          }
          Range::U1 | Range::I1 => TypedExpression::N1BitwiseComplement(Box::new(expression)),
          Range::U8 | Range::I8 => TypedExpression::N8BitwiseComplement(Box::new(expression)),
          _ => todo!(),
        },
      )
    }

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
            errors.extend([(
              Pos(File("[todo]".into()), 0, 0),
              Error(format!("Signed multiplication unimplemented",)),
            )]);
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
            errors.extend([(
              Pos(File("[todo]".into()), 0, 0),
              Error(format!("Signed division unimplemented",)),
            )]);
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
            errors.extend([(
              Pos(File("[todo]".into()), 0, 0),
              Error(format!("Signed modulo unimplemented",)),
            )]);
            TypedExpression::U8Modulo(Box::new(expression1), Box::new(expression2))
          }
          _ => todo!(),
        },
      )
    }

    //   Expression::LogicalAnd(expression1, expression2) => {
    //     typecheck::logical_and_expression(*expression1, *expression2, state, errors)
    //   }
    //
    //   Expression::LogicalOr(expression1, expression2) => {
    //     typecheck::logical_or_expression(*expression1, *expression2, state, errors)
    //   }
    //
    //   Expression::BitwiseAnd(expression1, expression2) => {
    //     typecheck::bitwise_and_expression(*expression1, *expression2, state, errors)
    //   }
    //
    //   Expression::BitwiseInclusiveOr(expression1, expression2) => {
    //     typecheck::bitwise_inclusive_or_expression(*expression1, *expression2, state, errors)
    //   }
    //
    //   Expression::BitwiseExclusiveOr(expression1, expression2) => {
    //     typecheck::bitwise_exclusive_or_expression(*expression1, *expression2, state, errors)
    //   }
    //
    //   Expression::LeftShift(expression1, expression2) => {
    //     typecheck::left_shift_expression(*expression1, *expression2, state, errors)
    //   }
    //
    //   Expression::RightShift(expression1, expression2) => {
    //     typecheck::right_shift_expression(*expression1, *expression2, state, errors)
    //   }
    //
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
          Range::I8 => {
            errors.extend([(
              Pos(File("[todo]".into()), 0, 0),
              Error(format!("Signed less than unimplemented",)),
            )]);
            TypedExpression::N1LessThanU8(Box::new(expression1), Box::new(expression2))
          }
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
          Range::I8 => {
            errors.extend([(
              Pos(File("[todo]".into()), 0, 0),
              Error(format!("Signed greater than unimplemented",)),
            )]);
            TypedExpression::N1LessThanU8(Box::new(expression2), Box::new(expression1))
          }
          _ => todo!(),
        },
      )
    }

    Expression::GreaterThanOrEqualTo(expression1, expression2) => typecheck::expression(
      Expression::LogicalNegation(Box::new(Expression::LessThan(expression1, expression2))),
      state,
      errors,
    ),

    //   Expression::Conditional(expression1, expression2, expression3) => {
    //     typecheck::conditional_expression(*expression1, *expression2, *expression3, state, errors)
    //   }
    //
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

// fn logical_and_expression(
//   _expression1: Expression,
//   _expression2: Expression,
//   _state: &mut State,
//   errors: &mut impl Extend<(Pos, Error)>,
// ) -> (Type, Vec<Result<Token, String>>) {
//   // TODO implement
//   errors.extend([(
//     Pos(File("[todo]".into()), 0, 0),
//     Error(format!("Logical AND unimplemented")),
//   )]);
//
//   (Type::Bool, vec![])
// }
//
// fn logical_or_expression(
//   _expression1: Expression,
//   _expression2: Expression,
//   _state: &mut State,
//   errors: &mut impl Extend<(Pos, Error)>,
// ) -> (Type, Vec<Result<Token, String>>) {
//   // TODO implement
//   errors.extend([(
//     Pos(File("[todo]".into()), 0, 0),
//     Error(format!("Logical OR unimplemented")),
//   )]);
//
//   (Type::Bool, vec![])
// }
//
// fn bitwise_and_expression(
//   expression1: Expression,
//   expression2: Expression,
//   state: &mut State,
//   errors: &mut impl Extend<(Pos, Error)>,
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
//         .chain([Ok(Token::And)])
//         .collect(),
//       _ => {
//         // TODO implement
//         errors.extend([(
//           Pos(File("[todo]".into()), 0, 0),
//           Error(format!("Bitwise AND unimplemented for type `{:?}`", r#type)),
//         )]);
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
//   errors: &mut impl Extend<(Pos, Error)>,
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
//         .chain([Ok(Token::Orr)])
//         .collect(),
//       _ => {
//         // TODO implement
//         errors.extend([(
//           Pos(File("[todo]".into()), 0, 0),
//           Error(format!(
//             "Bitwise Inclusive OR unimplemented for type `{:?}`",
//             r#type
//           )),
//         )]);
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
//   errors: &mut impl Extend<(Pos, Error)>,
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
//         .chain([Ok(Token::Xor)])
//         .collect(),
//       _ => {
//         // TODO implement
//         errors.extend([(
//           Pos(File("[todo]".into()), 0, 0),
//           Error(format!(
//             "Bitwise Exclusive OR unimplemented for type `{:?}`",
//             r#type
//           )),
//         )]);
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
//   errors: &mut impl Extend<(Pos, Error)>,
// ) -> (Type, Vec<Result<Token, String>>) {
//   // TODO implement
//   errors.extend([(
//     Pos(File("[todo]".into()), 0, 0),
//     Error(format!("Left Shift unimplemented")),
//   )]);
//
//   (Type::Int, vec![])
// }
//
// fn right_shift_expression(
//   _expression1: Expression,
//   _expression2: Expression,
//   _state: &mut State,
//   errors: &mut impl Extend<(Pos, Error)>,
// ) -> (Type, Vec<Result<Token, String>>) {
//   // TODO implement
//   errors.extend([(
//     Pos(File("[todo]".into()), 0, 0),
//     Error(format!("Right Shift unimplemented")),
//   )]);
//
//   (Type::Int, vec![])
// }
//
// fn less_than_expression(
//   expression1: Expression,
//   expression2: Expression,
//   state: &mut State,
//   errors: &mut impl Extend<(Pos, Error)>,
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
//         .chain([
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
//         errors.extend([(
//           Pos(File("[todo]".into()), 0, 0),
//           Error(format!("Less Than unimplemented for type `{:?}`", r#type)),
//         )]);
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
//   errors: &mut impl Extend<(Pos, Error)>,
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
//         .chain([
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
//         errors.extend([(
//           Pos(File("[todo]".into()), 0, 0),
//           Error(format!(
//             "Less Than Or Equal To unimplemented for type `{:?}`",
//             r#type
//           )),
//         )]);
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
//   errors: &mut impl Extend<(Pos, Error)>,
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
//         .chain([
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
//         errors.extend([(
//           Pos(File("[todo]".into()), 0, 0),
//           Error(format!(
//             "Greater Than unimplemented for type `{:?}`",
//             r#type
//           )),
//         )]);
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
//   errors: &mut impl Extend<(Pos, Error)>,
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
//         .chain([
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
//         errors.extend([(
//           Pos(File("[todo]".into()), 0, 0),
//           Error(format!(
//             "Greater Than Or Equal To unimplemented for type `{:?}`",
//             r#type
//           )),
//         )]);
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
//   errors: &mut impl Extend<(Pos, Error)>,
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
//         .chain([
//           Ok(Token::Buf),
//           Ok(Token::AtDyn),
//           Ok(Token::Pop),
//           Ok(Token::Iff),
//         ])
//         .collect(),
//       _ => {
//         // TODO implement
//         errors.extend([(
//           Pos(File("[todo]".into()), 0, 0),
//           Error(format!(
//             "Conditional unimplemented for types `{:?}, {:?}`",
//             type1, type2
//           )),
//         )]);
//         vec![]
//       }
//     },
//   )
// }

fn cast_expression(
  r#type: Type,
  expression: Expression,
  state: &mut State,
  errors: &mut impl Extend<(Pos, Error)>,
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

      (Type::Bool, Type::Void) => TypedExpression::N0CastN1(Box::new(expression1)),

      _ => {
        errors.extend([(
          Pos(File("[todo]".into()), 0, 0),
          Error(format!(
            "Type Cast unimplemented from `{:?}` to `{:?}`",
            type1, r#type
          )),
        )]);
        match r#type.range() {
          Range::U0 | Range::I0 => TypedExpression::N0Constant(()),
          Range::U1 | Range::I1 => TypedExpression::N1Constant(false),
          Range::U8 | Range::I8 => TypedExpression::N8Constant(0x00),
          _ => todo!(),
        }
      }
    },
  )
}

fn string_literal_expression(
  value: String,
  state: &mut State,
  _errors: &mut impl Extend<(Pos, Error)>,
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
  errors: &mut impl Extend<(Pos, Error)>,
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
        params_locals.iter().rev().find_map(|Object(r#type, name)| {
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
                errors.extend([(
                  Pos(File("[todo]".into()), 0, 0),
                  Error(format!("Identifier unimplemented for type `{:?}`", r#type)),
                )]);
                TypedExpression::N0Constant(())
              }
            },
          ),
        })
    })
    .unwrap_or_else(|| {
      errors.extend([(
        Pos(File("[pos]".into()), 0, 0),
        Error(format!(
          "Reference to undefined identifier `{}`",
          identifier
        )),
      )]);
      (Type::Void, TypedExpression::N0Constant(()))
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
    Type::Function(return_type, parameter_types, is_variadic) => {
      (None, return_type, parameter_types, is_variadic)
    }
    Type::Macro(return_type, name, parameter_types, is_variadic) => {
      (Some(name), return_type, parameter_types, is_variadic)
    }
    _ => {
      // TODO uses debug formatting
      errors.extend([(
        Pos(File("[pos]".into()), 0, 0),
        Error(format!("`{:?}` is not a function", designator)),
      )]);
      return (Type::Void, TypedExpression::N0Constant(()));
    }
  };

  if is_variadic && arguments.len() < parameter_types.len() {
    errors.extend([(
      Pos(File("[pos]".into()), 0, 0),
      // TODO uses debug formatting
      Error(format!(
        "Expected at least {} arguments to variadic function `{:?}`, got {}",
        parameter_types.len(),
        designator,
        arguments.len()
      )),
    )]);
  }

  if !is_variadic && arguments.len() != parameter_types.len() {
    errors.extend([(
      Pos(File("[pos]".into()), 0, 0),
      // TODO uses debug formatting
      Error(format!(
        "Expected {} arguments to function `{:?}`, got {}",
        parameter_types.len(),
        designator,
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

  (
    *return_type.clone(),
    match (inline_name, return_type.range()) {
      (Some(name), Range::U0 | Range::I0) => TypedExpression::N0MacroCall(name, arguments),
      (Some(name), Range::U1 | Range::I1) => TypedExpression::N1MacroCall(name, arguments),
      (Some(name), Range::U8 | Range::I8) => TypedExpression::N8MacroCall(name, arguments),
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
            "Function call unimplemented for type `{:?}`",
            return_type
          )),
        )]);
        TypedExpression::N0Constant(())
      }
    },
  )
}

fn integer_promotions(
  expression: Expression,
  state: &mut State,
  errors: &mut impl Extend<(Pos, Error)>,
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
  errors: &mut impl Extend<(Pos, Error)>,
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
      errors.extend([(
        Pos(File("[pos]".into()), 0, 0),
        Error(format!(
          "Invalid operand types `{:?}` and `{:?}`",
          type1, type2
        )),
      )]);

      Type::Int
    }

    (type1, type2) if type1 == type2 => type1,

    (Type::Char, Type::Int) | (Type::Int, Type::Char) => Type::Int,
    (Type::UnsignedInt, Type::Int) | (Type::Int, Type::UnsignedInt) => Type::UnsignedInt,

    _ => {
      errors.extend([(
        Pos(File("[todo]".into()), 0, 0),
        Error(format!(
          // TODO uses debug formatting
          "Usual Arithmetic Conversions unimplemented between `{:?}` and `{:?}`",
          type1, type2
        )),
      )]);

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
  errors: &mut impl Extend<(Pos, Error)>,
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
  errors: &mut impl Extend<(Pos, Error)>,
) -> (Expression, Expression) {
  // TODO nonstandard, completely ad-hoc

  let (type1, _) = typecheck::expression(expression1.clone(), state, errors);
  let (type2, _) = typecheck::expression(expression2.clone(), state, errors);

  match (&type1, &type2) {
    (Type::Pointer(_), Type::Pointer(_)) => (
      Expression::Cast(Type::UnsignedInt, Box::new(expression1)),
      Expression::Cast(Type::UnsignedInt, Box::new(expression2)),
    ),

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
  errors: &mut impl Extend<(Pos, Error)>,
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
