use crate::*;

pub fn codegen(program: Program, entry_point: &str) -> Result<Vec<Token>, Error> {
  let entry_macro = Macro {
    identifier: entry_point.to_string(),
  };
  let entry_label = Label {
    identifier: "main".to_string(),
    scope_uid: None,
  };
  let call_macro = Macro {
    identifier: "call".to_string(),
  };
  let hlt_macro = Macro {
    identifier: "hlt".to_string(),
  };

  let tokens: Vec<Token> = std::iter::empty()
    .chain(vec![
      Token::MacroDef(entry_macro),
      Token::LabelRef(entry_label),
      Token::MacroRef(call_macro),
      Token::MacroRef(hlt_macro),
    ])
    .chain(codegen::program(program, &vec![]))
    .collect();

  Ok(tokens)
}

#[allow(dead_code)]
#[derive(Clone, PartialEq, Debug)]
enum StackEntry {
  ProgramBoundary,
  GlobalDeclaration(Object),
  GlobalDefinition(Object),
  FunctionBoundary(Type),
  LoopBoundary(Type),
  BlockBoundary(Type),
  Local(Object),
  Temporary(Type),
}

impl StackEntry {
  #[allow(dead_code)]
  pub fn size(&self) -> usize {
    match self {
      StackEntry::ProgramBoundary => 0,
      StackEntry::GlobalDeclaration(_) => 0,
      StackEntry::GlobalDefinition(_) => 0,
      StackEntry::FunctionBoundary(type_) => type_.size(),
      StackEntry::LoopBoundary(type_) => type_.size(),
      StackEntry::BlockBoundary(type_) => type_.size(),
      StackEntry::Local(Object(type_, _name)) => type_.size(),
      StackEntry::Temporary(type_) => type_.size(),
    }
  }
}

impl Type {
  pub fn size(&self) -> usize {
    match self {
      Type::Void => 0,
      Type::Bool => 1,
      Type::Char => 1,
      Type::Short => 1,    // TODO nonstandard
      Type::Int => 1,      // TODO nonstandard
      Type::Long => 2,     // TODO potentially nonstandard
      Type::LongLong => 4, // TODO potentially nonstandard
      Type::Array(_) => 1,
      Type::Structure(declarators) => declarators
        .iter()
        .map(|Object(type_, _name)| type_.size())
        .sum(),
      Type::Union(declarators) => declarators
        .iter()
        .map(|Object(type_, _name)| type_.size())
        .max()
        .unwrap_or(0),
      Type::Function(_, _) => 1,
      Type::Pointer(_) => 1,
    }
  }
}

macro_rules! with {
  ($stack:expr, $entry:expr) => {
    &$stack.iter().chain(vec![$entry].iter()).cloned().collect()
  };
}

fn program(program: Program, stack: &Vec<StackEntry>) -> Vec<Token> {
  let stack = stack.clone();

  match program {
    Program {
      function_definitions,
    } => function_definitions
      .into_iter()
      .scan(stack, |stack, function_definition| {
        let FunctionDefinition(Object(type_, name), _, _body) = function_definition.clone();
        stack.push(StackEntry::GlobalDeclaration(Object(
          Type::Function(Box::new(type_.clone()), vec![]),
          name.clone(),
        )));
        let old_stack = stack.clone();
        stack.push(StackEntry::GlobalDefinition(Object(
          Type::Function(Box::new(type_.clone()), vec![]),
          name.clone(),
        )));
        Some((old_stack, function_definition))
      })
      .flat_map(|(stack, function_definition)| {
        codegen::function_definition(
          function_definition,
          with![stack, StackEntry::ProgramBoundary],
        )
      })
      .collect(),
  }
}

fn function_definition(
  function_definition: FunctionDefinition,
  stack: &Vec<StackEntry>,
) -> Vec<Token> {
  match function_definition {
    FunctionDefinition(Object(type_, name), _, body) => {
      stack
        .iter()
        .find(|entry| match entry {
          StackEntry::GlobalDefinition(Object(Type::Function(_, _), name_)) => *name_ == name,
          _ => false,
        })
        .is_some()
        .then(|| panic!("Function `{}` already defined", name.clone()));

      std::iter::empty()
        .chain(vec![Token::LabelDef(Label {
          identifier: name.clone(),
          scope_uid: None,
        })])
        .chain(body.into_iter().flat_map(|statement| {
          codegen::statement(
            statement,
            with![stack, StackEntry::FunctionBoundary(type_.clone())],
          )
        }))
        .collect()
    }
  }
}

fn statement(statement: Statement, stack: &Vec<StackEntry>) -> Vec<Token> {
  match statement {
    Statement::Expression(expression) => codegen::expression_statement(expression, stack),
    Statement::Return(expression) => codegen::return_statement(expression, stack),
    Statement::Asm(expressions, assembly) => codegen::asm_statement(expressions, assembly, stack),
  }
}

fn expression_statement(expression: Expression, stack: &Vec<StackEntry>) -> Vec<Token> {
  let (_type, tokens) =
    codegen::expression(Expression::Cast(Type::Void, Box::new(expression)), stack);

  tokens
}

fn return_statement(expression: Expression, stack: &Vec<StackEntry>) -> Vec<Token> {
  let ret_macro = Macro {
    identifier: "ret".to_string(),
  };

  let type_ = stack
    .iter()
    .rev()
    .find_map(|entry| match entry {
      StackEntry::FunctionBoundary(type_) => Some(type_),
      _ => None,
    })
    .unwrap_or_else(|| panic!("`return` outside of function"));

  let (type_, tokens) =
    codegen::expression(Expression::Cast(type_.clone(), Box::new(expression)), stack);

  match type_ {
    type_ if type_.size() == 1 => std::iter::empty()
      .chain(tokens)
      .chain(vec![Token::Swp, Token::MacroRef(ret_macro)])
      .collect(), // TODO check function boundary
    _ => todo!(),
  }
}

fn asm_statement(
  expressions: Vec<Expression>,
  assembly: String,
  stack: &Vec<StackEntry>,
) -> Vec<Token> {
  let arguments: Vec<Token> = expressions
    .into_iter()
    .map(|expression| codegen::expression(expression, stack))
    .flat_map(|(_type, tokens)| tokens)
    .collect();

  let mnemonics: Vec<Mnemonic> = assembly
    .split_whitespace()
    .map(|mnemonic| Mnemonic(mnemonic.to_string()))
    .collect();

  let tokens: Vec<Token> = mnemonics
    .into_iter()
    .map(|mnemonic| {
      common::mnemonic_to_token(mnemonic.clone())
        .unwrap_or_else(|| panic!("Unknown assembly mnemonic `{}`", mnemonic))
    })
    .collect();

  std::iter::empty().chain(arguments).chain(tokens).collect()
}

fn expression(expression: Expression, stack: &Vec<StackEntry>) -> (Type, Vec<Token>) {
  match expression.clone() {
    Expression::Negation(expression) => codegen::negation_expression(*expression, stack),
    Expression::LogicalNegation(expression) => {
      codegen::logical_negation_expression(*expression, stack)
    }
    Expression::BitwiseComplement(expression) => {
      codegen::bitwise_complement_expression(*expression, stack)
    }

    Expression::Addition(expression1, expression2) => {
      codegen::addition_expression(*expression1, *expression2, stack)
    }
    Expression::Subtraction(expression1, expression2) => {
      codegen::subtraction_expression(*expression1, *expression2, stack)
    }
    Expression::Multiplication(expression1, expression2) => {
      codegen::multiplication_expression(*expression1, *expression2, stack)
    }
    Expression::Division(expression1, expression2) => {
      codegen::division_expression(*expression1, *expression2, stack)
    }
    Expression::Modulo(expression1, expression2) => {
      codegen::modulo_expression(*expression1, *expression2, stack)
    }
    Expression::LogicalAnd(expression1, expression2) => {
      codegen::logical_and_expression(*expression1, *expression2, stack)
    }
    Expression::LogicalOr(expression1, expression2) => {
      codegen::logical_or_expression(*expression1, *expression2, stack)
    }
    Expression::BitwiseAnd(expression1, expression2) => {
      codegen::bitwise_and_expression(*expression1, *expression2, stack)
    }
    Expression::BitwiseInclusiveOr(expression1, expression2) => {
      codegen::bitwise_inclusive_or_expression(*expression1, *expression2, stack)
    }
    Expression::BitwiseExclusiveOr(expression1, expression2) => {
      codegen::bitwise_exclusive_or_expression(*expression1, *expression2, stack)
    }
    Expression::LeftShift(expression1, expression2) => {
      codegen::left_shift_expression(*expression1, *expression2, stack)
    }
    Expression::RightShift(expression1, expression2) => {
      codegen::right_shift_expression(*expression1, *expression2, stack)
    }

    Expression::EqualTo(expression1, expression2) => {
      codegen::equal_to_expression(*expression1, *expression2, stack)
    }
    Expression::NotEqualTo(expression1, expression2) => {
      codegen::not_equal_to_expression(*expression1, *expression2, stack)
    }
    Expression::LessThan(expression1, expression2) => {
      codegen::less_than_expression(*expression1, *expression2, stack)
    }
    Expression::LessThanOrEqualTo(expression1, expression2) => {
      codegen::less_than_or_equal_to_expression(*expression1, *expression2, stack)
    }
    Expression::GreaterThan(expression1, expression2) => {
      codegen::greater_than_expression(*expression1, *expression2, stack)
    }
    Expression::GreaterThanOrEqualTo(expression1, expression2) => {
      codegen::greater_than_or_equal_to_expression(*expression1, *expression2, stack)
    }

    Expression::Conditional(expression1, expression2, expression3) => {
      codegen::conditional_expression(*expression1, *expression2, *expression3, stack)
    }

    Expression::Cast(type_, expression) => codegen::cast_expression(type_, *expression, stack),
    Expression::IntegerConstant(value) => codegen::integer_constant_expression(value, stack),
    Expression::CharacterConstant(value) => codegen::character_constant_expression(value, stack),
    Expression::Identifier(_) => todo!(),
    Expression::FunctionCall(name) => codegen::function_call_expression(name, stack),
  }
}

fn negation_expression(expression: Expression, stack: &Vec<StackEntry>) -> (Type, Vec<Token>) {
  let (type_, tokens) = codegen::expression(expression, stack);

  (
    type_.clone(),
    match type_ {
      Type::Int | Type::Char => std::iter::empty()
        .chain(tokens)
        .chain(vec![Token::Neg])
        .collect(),
      _ => todo!(),
    },
  )
}

fn logical_negation_expression(
  expression: Expression,
  stack: &Vec<StackEntry>,
) -> (Type, Vec<Token>) {
  let (type_, tokens) = codegen::expression(expression, stack);

  (
    type_.clone(),
    match type_ {
      Type::Bool => std::iter::empty()
        .chain(tokens)
        .chain(vec![
          Token::Shr,
          Token::AtDyn,
          Token::Flc,
          Token::Shl,
          Token::AtDyn,
        ])
        .collect(),
      Type::Int | Type::Char => std::iter::empty()
        .chain(tokens)
        .chain(vec![
          Token::Buf,
          Token::AtDyn,
          Token::Pop,
          Token::Flc,
          Token::XXX(0x00),
          Token::Shl,
          Token::AtDyn,
        ])
        .collect(),
      _ => todo!(),
    },
  )
}

fn bitwise_complement_expression(
  expression: Expression,
  stack: &Vec<StackEntry>,
) -> (Type, Vec<Token>) {
  let (type_, tokens) = codegen::expression(expression, stack);

  (
    type_.clone(),
    match type_ {
      Type::Int | Type::Char => std::iter::empty()
        .chain(tokens)
        .chain(vec![Token::Not])
        .collect(),
      _ => todo!(),
    },
  )
}

fn addition_expression(
  expression1: Expression,
  expression2: Expression,
  stack: &Vec<StackEntry>,
) -> (Type, Vec<Token>) {
  let (type_, tokens1, tokens2) =
    codegen::usual_arithmetic_conversion(expression1, expression2, stack);

  (
    type_.clone(),
    match type_ {
      Type::Int | Type::Char => std::iter::empty()
        .chain(tokens1)
        .chain(tokens2)
        .chain(vec![Token::Add])
        .collect(),
      _ => todo!(),
    },
  )
}

fn subtraction_expression(
  expression1: Expression,
  expression2: Expression,
  stack: &Vec<StackEntry>,
) -> (Type, Vec<Token>) {
  let (type_, tokens1, tokens2) =
    codegen::usual_arithmetic_conversion(expression1, expression2, stack);

  (
    type_.clone(),
    match type_ {
      Type::Int | Type::Char => std::iter::empty()
        .chain(tokens1)
        .chain(tokens2)
        .chain(vec![Token::Sub])
        .collect(),
      _ => todo!(),
    },
  )
}

fn multiplication_expression(
  expression1: Expression,
  expression2: Expression,
  stack: &Vec<StackEntry>,
) -> (Type, Vec<Token>) {
  let (type_, tokens1, tokens2) =
    codegen::usual_arithmetic_conversion(expression1, expression2, stack);

  let mul_macro = Macro {
    identifier: "mul".to_string(), // TODO implement operation
  };

  (
    type_.clone(),
    match type_ {
      Type::Int | Type::Char => std::iter::empty()
        .chain(tokens1)
        .chain(tokens2)
        .chain(vec![Token::MacroRef(mul_macro)])
        .collect(),
      _ => todo!(),
    },
  )
}

fn division_expression(
  expression1: Expression,
  expression2: Expression,
  stack: &Vec<StackEntry>,
) -> (Type, Vec<Token>) {
  let (type_, tokens1, tokens2) =
    codegen::usual_arithmetic_conversion(expression1, expression2, stack);

  let div_macro = Macro {
    identifier: "div".to_string(), // TODO implement operation
  };

  (
    type_.clone(),
    match type_ {
      Type::Int | Type::Char => std::iter::empty()
        .chain(tokens1)
        .chain(tokens2)
        .chain(vec![Token::MacroRef(div_macro)])
        .collect(),
      _ => todo!(),
    },
  )
}

fn modulo_expression(
  expression1: Expression,
  expression2: Expression,
  stack: &Vec<StackEntry>,
) -> (Type, Vec<Token>) {
  let (type_, tokens1, tokens2) =
    codegen::usual_arithmetic_conversion(expression1, expression2, stack);

  let mod_macro = Macro {
    identifier: "mod".to_string(), // TODO implement operation
  };

  (
    type_.clone(),
    match type_ {
      Type::Int | Type::Char => std::iter::empty()
        .chain(tokens1)
        .chain(tokens2)
        .chain(vec![Token::MacroRef(mod_macro)])
        .collect(),
      _ => todo!(),
    },
  )
}

fn logical_and_expression(
  _expression1: Expression,
  _expression2: Expression,
  _stack: &Vec<StackEntry>,
) -> (Type, Vec<Token>) {
  todo!()
}

fn logical_or_expression(
  _expression1: Expression,
  _expression2: Expression,
  _stack: &Vec<StackEntry>,
) -> (Type, Vec<Token>) {
  todo!()
}

fn bitwise_and_expression(
  expression1: Expression,
  expression2: Expression,
  stack: &Vec<StackEntry>,
) -> (Type, Vec<Token>) {
  let (type_, tokens1, tokens2) =
    codegen::usual_arithmetic_conversion(expression1, expression2, stack);

  (
    type_.clone(),
    match type_ {
      Type::Int | Type::Char => std::iter::empty()
        .chain(tokens1)
        .chain(tokens2)
        .chain(vec![Token::And])
        .collect(),
      _ => todo!(),
    },
  )
}

fn bitwise_inclusive_or_expression(
  expression1: Expression,
  expression2: Expression,
  stack: &Vec<StackEntry>,
) -> (Type, Vec<Token>) {
  let (type_, tokens1, tokens2) =
    codegen::usual_arithmetic_conversion(expression1, expression2, stack);

  (
    type_.clone(),
    match type_ {
      Type::Int | Type::Char => std::iter::empty()
        .chain(tokens1)
        .chain(tokens2)
        .chain(vec![Token::Orr])
        .collect(),
      _ => todo!(),
    },
  )
}

fn bitwise_exclusive_or_expression(
  expression1: Expression,
  expression2: Expression,
  stack: &Vec<StackEntry>,
) -> (Type, Vec<Token>) {
  let (type_, tokens1, tokens2) =
    codegen::usual_arithmetic_conversion(expression1, expression2, stack);

  (
    type_.clone(),
    match type_ {
      Type::Int | Type::Char => std::iter::empty()
        .chain(tokens1)
        .chain(tokens2)
        .chain(vec![Token::Xor])
        .collect(),
      _ => todo!(),
    },
  )
}

fn left_shift_expression(
  _expression1: Expression,
  _expression2: Expression,
  _stack: &Vec<StackEntry>,
) -> (Type, Vec<Token>) {
  todo!()
}

fn right_shift_expression(
  _expression1: Expression,
  _expression2: Expression,
  _stack: &Vec<StackEntry>,
) -> (Type, Vec<Token>) {
  todo!()
}

fn equal_to_expression(
  expression1: Expression,
  expression2: Expression,
  stack: &Vec<StackEntry>,
) -> (Type, Vec<Token>) {
  let (type_, tokens1, tokens2) =
    codegen::usual_arithmetic_conversion(expression1, expression2, stack);

  (
    Type::Bool,
    match type_ {
      Type::Bool => std::iter::empty()
        .chain(tokens1)
        .chain(tokens2)
        .chain(vec![Token::Xor, Token::Clc])
        .collect(),
      Type::Int | Type::Char => std::iter::empty()
        .chain(tokens1)
        .chain(tokens2)
        .chain(vec![
          Token::Xor,
          Token::AtDyn,
          Token::Pop,
          Token::XXX(0x00),
          Token::Shl,
          Token::AtDyn,
        ])
        .collect(),
      _ => todo!(),
    },
  )
}

fn not_equal_to_expression(
  expression1: Expression,
  expression2: Expression,
  stack: &Vec<StackEntry>,
) -> (Type, Vec<Token>) {
  let (type_, tokens1, tokens2) =
    codegen::usual_arithmetic_conversion(expression1, expression2, stack);

  (
    Type::Bool,
    match type_ {
      Type::Bool => std::iter::empty()
        .chain(tokens1)
        .chain(tokens2)
        .chain(vec![Token::Xor, Token::XXX(0x01), Token::Xor, Token::Clc])
        .collect(),
      Type::Int | Type::Char => std::iter::empty()
        .chain(tokens1)
        .chain(tokens2)
        .chain(vec![
          Token::Xor,
          Token::AtDyn,
          Token::Pop,
          Token::Flc,
          Token::XXX(0x00),
          Token::Shl,
          Token::AtDyn,
        ])
        .collect(),
      _ => todo!(),
    },
  )
}

fn less_than_expression(
  expression1: Expression,
  expression2: Expression,
  stack: &Vec<StackEntry>,
) -> (Type, Vec<Token>) {
  let (type_, tokens1, tokens2) =
    codegen::usual_arithmetic_conversion(expression1, expression2, stack);

  (
    Type::Bool,
    match type_ {
      Type::Int | Type::Char => std::iter::empty()
        .chain(tokens1)
        .chain(tokens2)
        .chain(vec![
          Token::Sub,
          Token::AtDyn,
          Token::Pop,
          Token::XXX(0x00),
          Token::Shl,
          Token::AtDyn,
        ])
        .collect(),
      _ => todo!(),
    },
  )
}

fn less_than_or_equal_to_expression(
  expression1: Expression,
  expression2: Expression,
  stack: &Vec<StackEntry>,
) -> (Type, Vec<Token>) {
  let (type_, tokens1, tokens2) =
    codegen::usual_arithmetic_conversion(expression1, expression2, stack);

  (
    Type::Bool,
    match type_ {
      Type::Int | Type::Char => std::iter::empty()
        .chain(tokens1)
        .chain(tokens2)
        .chain(vec![
          Token::Sub,
          Token::AtDyn,
          Token::Pop,
          Token::Flc,
          Token::XXX(0x00),
          Token::Shl,
          Token::AtDyn,
        ])
        .collect(),
      _ => todo!(),
    },
  )
}

fn greater_than_expression(
  expression1: Expression,
  expression2: Expression,
  stack: &Vec<StackEntry>,
) -> (Type, Vec<Token>) {
  let (type_, tokens1, tokens2) =
    codegen::usual_arithmetic_conversion(expression1, expression2, stack);

  (
    Type::Bool,
    match type_ {
      Type::Int | Type::Char => std::iter::empty()
        .chain(tokens1)
        .chain(tokens2)
        .chain(vec![
          Token::Sub,
          Token::AtDyn,
          Token::Pop,
          Token::XXX(0x00),
          Token::Shl,
          Token::AtDyn,
        ])
        .collect(),
      _ => todo!(),
    },
  )
}

fn greater_than_or_equal_to_expression(
  expression1: Expression,
  expression2: Expression,
  stack: &Vec<StackEntry>,
) -> (Type, Vec<Token>) {
  let (type_, tokens1, tokens2) =
    codegen::usual_arithmetic_conversion(expression1, expression2, stack);

  (
    Type::Bool,
    match type_ {
      Type::Int | Type::Char => std::iter::empty()
        .chain(tokens1)
        .chain(tokens2)
        .chain(vec![
          Token::Sub,
          Token::AtDyn,
          Token::Pop,
          Token::Flc,
          Token::XXX(0x00),
          Token::Shl,
          Token::AtDyn,
        ])
        .collect(),
      _ => todo!(),
    },
  )
}

fn conditional_expression(
  expression1: Expression,
  expression2: Expression,
  expression3: Expression,
  stack: &Vec<StackEntry>,
) -> (Type, Vec<Token>) {
  let (type1, tokens1) = codegen::expression(expression1, stack);
  let (type2, tokens2, tokens3) =
    codegen::usual_arithmetic_conversion(expression2, expression3, stack);

  (
    type2.clone(),
    match type1 {
      Type::Int | Type::Char => std::iter::empty()
        .chain(tokens3)
        .chain(tokens1)
        .chain(tokens2)
        .chain(vec![Token::Buf, Token::AtDyn, Token::Pop, Token::Iff])
        .collect(),
      _ => todo!(),
    },
  )
}

fn cast_expression(
  type_: Type,
  expression: Expression,
  stack: &Vec<StackEntry>,
) -> (Type, Vec<Token>) {
  let (type1, tokens1) = codegen::expression(expression, stack);

  (
    type_.clone(),
    match (type1.clone(), type_.clone()) {
      (type1, type2) if type1 == type2 => tokens1,

      (Type::Bool, Type::Int)
      | (Type::Bool, Type::Char)
      | (Type::Int, Type::Char)
      | (Type::Char, Type::Int) => tokens1,

      (Type::Int, Type::Bool) | (Type::Char, Type::Bool) => std::iter::empty()
        .chain(tokens1)
        .chain(vec![
          Token::Buf,
          Token::AtDyn,
          Token::Pop,
          Token::Flc,
          Token::XXX(0x00),
          Token::Shl,
          Token::AtDyn,
        ])
        .collect(),

      (type1, Type::Void) => std::iter::empty()
        .chain(tokens1)
        .chain(std::iter::repeat(Token::Pop).take(type1.size()))
        .collect(),

      _ => panic!(
        "Unimplemented type cast from `{:?}` to `{:?}`",
        type1, type_
      ),
    },
  )
}

fn integer_constant_expression(value: u8, _stack: &Vec<StackEntry>) -> (Type, Vec<Token>) {
  (
    Type::Int, // TODO assumes all integer literals are ints
    vec![Token::XXX(value)],
  )
}

fn character_constant_expression(value: char, _stack: &Vec<StackEntry>) -> (Type, Vec<Token>) {
  (
    Type::Char, // TODO character constants are `int`s in C
    vec![Token::XXX(value as u8)],
  )
}

fn function_call_expression(name: String, stack: &Vec<StackEntry>) -> (Type, Vec<Token>) {
  let call_macro = Macro {
    identifier: "call".to_string(),
  };

  let type_ = stack
    .iter()
    .find_map(|entry| match entry {
      StackEntry::GlobalDeclaration(Object(Type::Function(type_, _), name_)) if *name_ == name => {
        Some(*type_.clone())
      }
      _ => None,
    })
    .unwrap_or_else(|| panic!("Function `{}` not found", name));

  (
    type_.clone(),
    vec![
      Token::LabelRef(Label {
        identifier: name.clone(),
        scope_uid: None,
      }),
      Token::MacroRef(call_macro),
    ],
  )
}

fn usual_arithmetic_conversion(
  expression1: Expression,
  expression2: Expression,
  stack: &Vec<StackEntry>,
) -> (Type, Vec<Token>, Vec<Token>) {
  let (type1, tokens1) = codegen::expression(expression1, stack);
  let (type2, tokens2) = codegen::expression(expression2, stack);
  match (type1.clone(), type2.clone()) {
    (type1, type2) if type1 == type2 => (type1, tokens1, tokens2),

    (Type::Char, Type::Int) | (Type::Int, Type::Char) => (Type::Int, tokens1, tokens2),

    _ => panic!(
      "Unimplemented usual arithmetic conversion between `{:?}` and `{:?}`",
      type1, type2
    ),
  }
}
