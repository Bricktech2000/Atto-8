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
    codegen::program(program),
  ]
  .into_iter()
  .flatten()
  .collect();

  Ok(tokens)
}

fn program(program: Program) -> Vec<Token> {
  match program {
    Program {
      function_definitions,
    } => function_definitions
      .into_iter()
      .flat_map(codegen::function_definition)
      .collect(),
  }
}

fn function_definition(function_definition: FunctionDefinition) -> Vec<Token> {
  let ret_macro = Macro {
    identifier: "ret".to_string(),
  };

  match function_definition {
    FunctionDefinition { name, body } => vec![
      vec![Token::LabelDef(Label {
        identifier: name.clone(),
        scope_uid: None,
      })],
      body.into_iter().flat_map(codegen::statement).collect(),
      vec![Token::Swp, Token::MacroRef(ret_macro)],
    ],
  }
  .into_iter()
  .flatten()
  .collect()
}

fn statement(statement: Statement) -> Vec<Token> {
  match statement {
    Statement::MagicReturn(expression) => vec![codegen::expression(expression)],
    Statement::Expression(expression) => vec![
      codegen::expression(expression),
      vec![Token::Pop], // TODO size must match
    ],
  }
  .into_iter()
  .flatten()
  .collect()
}

fn expression(expression: Expression) -> Vec<Token> {
  let mul_macro = Macro {
    identifier: "mul".to_string(), // TODO implement operation
  };
  let div_macro = Macro {
    identifier: "div".to_string(), // TODO implement operation
  };
  let mod_macro = Macro {
    identifier: "mod".to_string(), // TODO implement operation
  };

  match expression {
    Expression::Addition(expression, expression_) => vec![
      codegen::expression(*expression),
      codegen::expression(*expression_),
      vec![Token::Add],
    ],
    Expression::Subtraction(expression, expression_) => vec![
      codegen::expression(*expression),
      codegen::expression(*expression_),
      vec![Token::Sub],
    ],
    Expression::Multiplication(expression, expression_) => vec![
      codegen::expression(*expression),
      codegen::expression(*expression_),
      vec![Token::MacroRef(mul_macro)],
    ],
    Expression::Division(expression, expression_) => vec![
      codegen::expression(*expression),
      codegen::expression(*expression_),
      vec![Token::MacroRef(div_macro)],
    ],
    Expression::Modulo(expression, expression_) => vec![
      codegen::expression(*expression),
      codegen::expression(*expression_),
      vec![Token::MacroRef(mod_macro)],
    ],
    Expression::Negation(expression) => vec![codegen::expression(*expression), vec![Token::Neg]], // TODO type must match
    Expression::BitwiseComplement(expression) => {
      vec![codegen::expression(*expression), vec![Token::Not]] // TODO type must match
    }
    Expression::LogicalNegation(expression) => {
      vec![
        codegen::expression(*expression),
        vec![
          Token::Buf,
          Token::AtDyn,
          Token::Pop,
          Token::XXX(0x00),
          Token::Shl,
          Token::AtDyn,
        ],
      ] // TODO type must match
    }
    Expression::LogicalAnd(_, _) => todo!(), // TODO short circuit
    Expression::LogicalOr(_, _) => todo!(),  // TODO short circuit
    Expression::BitwiseAnd(expression, expression_) => vec![
      codegen::expression(*expression),
      codegen::expression(*expression_),
      vec![Token::And],
    ],
    Expression::BitwiseInclusiveOr(expression, expression_) => vec![
      codegen::expression(*expression),
      codegen::expression(*expression_),
      vec![Token::Orr],
    ],
    Expression::BitwiseExclusiveOr(expression, expression_) => vec![
      codegen::expression(*expression),
      codegen::expression(*expression_),
      vec![Token::Xor],
    ],
    Expression::EqualTo(expression, expression_) => vec![
      codegen::expression(*expression),
      codegen::expression(*expression_),
      vec![
        Token::Xor,
        Token::AtDyn,
        Token::Pop,
        Token::XXX(0x00),
        Token::Shl,
        Token::AtDyn,
      ],
    ],
    Expression::NotEqualTo(expression, expression_) => vec![
      codegen::expression(*expression),
      codegen::expression(*expression_),
      vec![
        Token::Xor,
        Token::AtDyn,
        Token::Pop,
        Token::Flc,
        Token::XXX(0x00),
        Token::Shl,
        Token::AtDyn,
      ],
    ],
    Expression::LessThan(expression, expression_) => vec![
      codegen::expression(*expression),
      codegen::expression(*expression_),
      vec![
        Token::Sub,
        Token::AtDyn,
        Token::Pop,
        Token::XXX(0x00),
        Token::Shl,
        Token::AtDyn,
      ],
    ],
    Expression::LessThanOrEqualTo(expression, expression_) => vec![
      codegen::expression(*expression_),
      codegen::expression(*expression),
      vec![
        Token::Sub,
        Token::AtDyn,
        Token::Pop,
        Token::Flc,
        Token::XXX(0x00),
        Token::Shl,
        Token::AtDyn,
      ],
    ],
    Expression::GreaterThan(expression, expression_) => vec![
      codegen::expression(*expression_),
      codegen::expression(*expression),
      vec![
        Token::Sub,
        Token::AtDyn,
        Token::Pop,
        Token::XXX(0x00),
        Token::Shl,
        Token::AtDyn,
      ],
    ],
    Expression::GreaterThanOrEqualTo(expression, expression_) => vec![
      codegen::expression(*expression),
      codegen::expression(*expression_),
      vec![
        Token::Sub,
        Token::AtDyn,
        Token::Pop,
        Token::Flc,
        Token::XXX(0x00),
        Token::Shl,
        Token::AtDyn,
      ],
    ],
    Expression::RightShift(_, _) => todo!(),
    Expression::LeftShift(_, _) => todo!(),
    Expression::Conditional(expression, expression_, expression__) => vec![
      codegen::expression(*expression_),
      codegen::expression(*expression__),
      codegen::expression(*expression),
      vec![Token::Buf, Token::AtDyn, Token::Pop, Token::Iff],
    ], // TODO short circuit
    Expression::Cast(_, _) => todo!(),
    Expression::IntegerConstant(integer_constant) => vec![vec![Token::XXX(integer_constant)]],
  }
  .into_iter()
  .flatten()
  .collect()
}
