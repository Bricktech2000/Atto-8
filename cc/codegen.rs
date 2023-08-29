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

  let tokens: Vec<Token> = vec![
    vec![
      Token::MacroDef(entry_macro),
      Token::LabelRef(entry_label),
      Token::MacroRef(call_macro),
      Token::MacroRef(hlt_macro),
    ],
    codegen::program(program, &vec![]),
  ]
  .into_iter()
  .flatten()
  .collect();

  Ok(tokens)
}

macro_rules! with {
  ($stack:expr, $entry:expr) => {
    &vec![$stack.clone(), vec![$entry]]
      .into_iter()
      .flatten()
      .collect()
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
        let FunctionDefinition(type_, name, _body) = function_definition.clone();
        stack.push(StackEntry::FunctionDeclaration(type_.clone(), name.clone()));
        let old_stack = stack.clone();
        stack.push(StackEntry::FunctionDefinition(type_.clone(), name.clone()));
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
    FunctionDefinition(type_, name, body) => {
      stack
        .iter()
        .find(|entry| match entry {
          StackEntry::FunctionDefinition(_type, name_) => *name_ == name,
          _ => false,
        })
        .is_some()
        .then(|| panic!("Function `{}` already defined", name.clone()));

      vec![
        vec![Token::LabelDef(Label {
          identifier: name.clone(),
          scope_uid: None,
        })],
        body
          .into_iter()
          .flat_map(|statement| {
            codegen::statement(
              statement,
              with![
                stack,
                StackEntry::FunctionBoundary(type_.clone(), name.clone())
              ],
            )
          })
          .collect(),
      ]
    }
  }
  .into_iter()
  .flatten()
  .collect()
}

fn statement(statement: Statement, stack: &Vec<StackEntry>) -> Vec<Token> {
  match statement {
    Statement::Expression(expression) => codegen::expression_statement(expression, stack),
    Statement::Return(expression) => codegen::return_statement(expression, stack),
  }
}

fn expression_statement(expression: Expression, stack: &Vec<StackEntry>) -> Vec<Token> {
  let (type_, tokens) = codegen::expression(expression, stack);

  match type_ {
    type_ if type_.size() == 1 => vec![tokens, vec![Token::Pop]],
    _ => todo!(),
  }
  .into_iter()
  .flatten()
  .collect()
}

fn return_statement(expression: Expression, stack: &Vec<StackEntry>) -> Vec<Token> {
  let ret_macro = Macro {
    identifier: "ret".to_string(),
  };

  let type_ = stack
    .iter()
    .rev()
    .find_map(|entry| match entry {
      StackEntry::FunctionBoundary(type_, _name) => Some(type_),
      _ => None,
    })
    .unwrap_or_else(|| panic!("`return` outside of function"));

  let (type_, tokens) =
    codegen::expression(Expression::Cast(type_.clone(), Box::new(expression)), stack);

  match type_ {
    type_ if type_.size() == 1 => vec![tokens, vec![Token::Swp, Token::MacroRef(ret_macro)]], // TODO check function boundary
    _ => todo!(),
  }
  .into_iter()
  .flatten()
  .collect()
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
    Expression::Identifier(_) => todo!(),
    Expression::FunctionCall(name) => codegen::function_call_expression(name, stack),
  }
}

fn negation_expression(expression: Expression, stack: &Vec<StackEntry>) -> (Type, Vec<Token>) {
  let (type_, tokens) = codegen::expression(expression, stack);

  (
    type_.clone(),
    match type_ {
      Type::BasicType(BasicType::Int) => vec![tokens, vec![Token::Neg]],
      _ => todo!(),
    }
    .into_iter()
    .flatten()
    .collect(),
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
      Type::BasicType(BasicType::Bool) => vec![
        tokens,
        vec![
          Token::Shr,
          Token::AtDyn,
          Token::Flc,
          Token::Shl,
          Token::AtDyn,
        ],
      ],
      Type::BasicType(BasicType::Int) => vec![
        tokens,
        vec![
          Token::Buf,
          Token::AtDyn,
          Token::Pop,
          Token::Flc,
          Token::XXX(0x00),
          Token::Shl,
          Token::AtDyn,
        ],
      ],
      _ => todo!(),
    }
    .into_iter()
    .flatten()
    .collect(),
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
      Type::BasicType(BasicType::Int) => vec![tokens, vec![Token::Not]],
      _ => todo!(),
    }
    .into_iter()
    .flatten()
    .collect(),
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
      Type::BasicType(BasicType::Int) => vec![tokens1, tokens2, vec![Token::Add]],
      _ => todo!(),
    }
    .into_iter()
    .flatten()
    .collect(),
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
      Type::BasicType(BasicType::Int) => vec![tokens1, tokens2, vec![Token::Sub]],
      _ => todo!(),
    }
    .into_iter()
    .flatten()
    .collect(),
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
      Type::BasicType(BasicType::Int) => vec![tokens1, tokens2, vec![Token::MacroRef(mul_macro)]],
      _ => todo!(),
    }
    .into_iter()
    .flatten()
    .collect(),
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
      Type::BasicType(BasicType::Int) => vec![tokens1, tokens2, vec![Token::MacroRef(div_macro)]],
      _ => todo!(),
    }
    .into_iter()
    .flatten()
    .collect(),
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
      Type::BasicType(BasicType::Int) => vec![tokens1, tokens2, vec![Token::MacroRef(mod_macro)]],
      _ => todo!(),
    }
    .into_iter()
    .flatten()
    .collect(),
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
      Type::BasicType(BasicType::Int) => vec![tokens1, tokens2, vec![Token::And]],
      _ => todo!(),
    }
    .into_iter()
    .flatten()
    .collect(),
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
      Type::BasicType(BasicType::Int) => vec![tokens1, tokens2, vec![Token::Orr]],
      _ => todo!(),
    }
    .into_iter()
    .flatten()
    .collect(),
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
      Type::BasicType(BasicType::Int) => vec![tokens1, tokens2, vec![Token::Xor]],
      _ => todo!(),
    }
    .into_iter()
    .flatten()
    .collect(),
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
    Type::BasicType(BasicType::Bool),
    match type_ {
      Type::BasicType(BasicType::Bool) => {
        vec![tokens1, tokens2, vec![Token::Xor, Token::Clc]]
      }
      Type::BasicType(BasicType::Int) => {
        vec![
          tokens1,
          tokens2,
          vec![
            Token::Xor,
            Token::AtDyn,
            Token::Pop,
            Token::XXX(0x00),
            Token::Shl,
            Token::AtDyn,
          ],
        ]
      }
      _ => todo!(),
    }
    .into_iter()
    .flatten()
    .collect(),
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
    Type::BasicType(BasicType::Bool),
    match type_ {
      Type::BasicType(BasicType::Bool) => {
        vec![
          tokens1,
          tokens2,
          vec![Token::Xor, Token::XXX(0x01), Token::Xor, Token::Clc],
        ]
      }
      Type::BasicType(BasicType::Int) => {
        vec![
          tokens1,
          tokens2,
          vec![
            Token::Xor,
            Token::AtDyn,
            Token::Pop,
            Token::Flc,
            Token::XXX(0x00),
            Token::Shl,
            Token::AtDyn,
          ],
        ]
      }
      _ => todo!(),
    }
    .into_iter()
    .flatten()
    .collect(),
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
    Type::BasicType(BasicType::Bool),
    match type_ {
      Type::BasicType(BasicType::Int) => {
        vec![
          tokens1,
          tokens2,
          vec![
            Token::Sub,
            Token::AtDyn,
            Token::Pop,
            Token::XXX(0x00),
            Token::Shl,
            Token::AtDyn,
          ],
        ]
      }
      _ => todo!(),
    }
    .into_iter()
    .flatten()
    .collect(),
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
    Type::BasicType(BasicType::Bool),
    match type_ {
      Type::BasicType(BasicType::Int) => {
        vec![
          tokens1,
          tokens2,
          vec![
            Token::Sub,
            Token::AtDyn,
            Token::Pop,
            Token::Flc,
            Token::XXX(0x00),
            Token::Shl,
            Token::AtDyn,
          ],
        ]
      }
      _ => todo!(),
    }
    .into_iter()
    .flatten()
    .collect(),
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
    Type::BasicType(BasicType::Bool),
    match type_ {
      Type::BasicType(BasicType::Int) => {
        vec![
          tokens1,
          tokens2,
          vec![
            Token::Sub,
            Token::AtDyn,
            Token::Pop,
            Token::XXX(0x00),
            Token::Shl,
            Token::AtDyn,
          ],
        ]
      }
      _ => todo!(),
    }
    .into_iter()
    .flatten()
    .collect(),
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
    Type::BasicType(BasicType::Bool),
    match type_ {
      Type::BasicType(BasicType::Int) => {
        vec![
          tokens1,
          tokens2,
          vec![
            Token::Sub,
            Token::AtDyn,
            Token::Pop,
            Token::Flc,
            Token::XXX(0x00),
            Token::Shl,
            Token::AtDyn,
          ],
        ]
      }
      _ => todo!(),
    }
    .into_iter()
    .flatten()
    .collect(),
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
      Type::BasicType(BasicType::Int) => {
        vec![
          tokens3,
          tokens1,
          tokens2,
          vec![Token::Buf, Token::AtDyn, Token::Pop, Token::Iff],
        ]
      }
      _ => todo!(),
    }
    .into_iter()
    .flatten()
    .collect(),
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
    match (type1, type_) {
      (Type::BasicType(BasicType::Int), Type::BasicType(BasicType::Int))
      | (Type::BasicType(BasicType::Bool), Type::BasicType(BasicType::Bool))
      | (Type::BasicType(BasicType::Bool), Type::BasicType(BasicType::Int)) => vec![tokens1],
      (Type::BasicType(BasicType::Int), Type::BasicType(BasicType::Bool)) => {
        vec![
          tokens1,
          vec![
            Token::Buf,
            Token::AtDyn,
            Token::Pop,
            Token::Flc,
            Token::XXX(0x00),
            Token::Shl,
            Token::AtDyn,
          ],
        ]
      }
      _ => panic!("Unimplemented type cast"),
    }
    .into_iter()
    .flatten()
    .collect(),
  )
}

fn integer_constant_expression(value: u8, _stack: &Vec<StackEntry>) -> (Type, Vec<Token>) {
  (
    Type::BasicType(BasicType::Int), // TODO assumes all integer literals are ints
    vec![Token::XXX(value)],
  )
}

fn function_call_expression(name: String, stack: &Vec<StackEntry>) -> (Type, Vec<Token>) {
  let call_macro = Macro {
    identifier: "call".to_string(),
  };

  let type_ = stack
    .iter()
    .find_map(|entry| match entry {
      StackEntry::FunctionDeclaration(type_, name_) if *name_ == name => Some(type_.clone()),
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
    (Type::BasicType(BasicType::Int), Type::BasicType(BasicType::Int)) => {
      (Type::BasicType(BasicType::Int), tokens1, tokens2)
    }
    (Type::BasicType(BasicType::Bool), Type::BasicType(BasicType::Bool)) => {
      (Type::BasicType(BasicType::Bool), tokens1, tokens2)
    }
    _ => panic!(
      "Unimplemented usual arithmetic conversion from `{:?}` to `{:?}`",
      type1, type2
    ),
  }
}
