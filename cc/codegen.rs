use crate::*;

pub fn codegen(translation_unit: TranslationUnit, entry_point: &str) -> Result<Vec<Token>, Error> {
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
    codegen::translation_unit(translation_unit),
  ]
  .into_iter()
  .flatten()
  .collect();

  Ok(tokens)
}

fn translation_unit(translation_unit: TranslationUnit) -> Vec<Token> {
  match translation_unit {
    TranslationUnit::ExternalDeclarations(external_declarations) => external_declarations
      .into_iter()
      .flat_map(codegen::external_declaration)
      .collect(),
  }
}

fn external_declaration(external_declaration: ExternalDeclaration) -> Vec<Token> {
  match external_declaration {
    ExternalDeclaration::FunctionDefinition(function_definition) => {
      codegen::function_definition(function_definition)
    }
  }
}

fn function_definition(function_definition: FunctionDefinition) -> Vec<Token> {
  let ret_macro = Macro {
    identifier: "ret".to_string(),
  };

  match function_definition {
    FunctionDefinition::NameBody(name, body) => vec![
      vec![Token::LabelDef(Label {
        identifier: name.clone(),
        scope_uid: None,
      })],
      codegen::compound_statement(body),
      vec![Token::Swp, Token::MacroRef(ret_macro)],
    ]
    .into_iter()
    .flatten()
    .collect(),
  }
}

fn compound_statement(compound_statement: CompoundStatement) -> Vec<Token> {
  match compound_statement {
    CompoundStatement::MagicReturn(magic_return) => codegen::magic_return(magic_return),
  }
}

fn magic_return(magic_return: MagicReturn) -> Vec<Token> {
  match magic_return {
    MagicReturn::AdditiveExpression(additive_expression) => {
      codegen::additive_expression(additive_expression)
    }
  }
}

fn additive_expression(additive_expression: AdditiveExpression) -> Vec<Token> {
  match additive_expression {
    AdditiveExpression::MultiplicativeExpression(multiplicative_expression) => {
      codegen::multiplicative_expression(multiplicative_expression)
    }
    AdditiveExpression::AdditiveExpressionAdditiveOperatorMultiplicativeExpression(
      additive_expression,
      additive_operator,
      multiplicative_expression,
    ) => vec![
      codegen::additive_expression(*additive_expression),
      codegen::multiplicative_expression(multiplicative_expression),
      codegen::additive_operator(additive_operator),
    ]
    .into_iter()
    .flatten()
    .collect(),
  }
}

fn multiplicative_expression(multiplicative_expression: MultiplicativeExpression) -> Vec<Token> {
  match multiplicative_expression {
    MultiplicativeExpression::CastExpression(cast_expression) => {
      codegen::cast_expression(cast_expression)
    }
    MultiplicativeExpression::MultiplicativeExpressionMultiplicativeOperatorCastExpression(
      multiplicative_expression,
      multiplicative_operator,
      cast_expression,
    ) => vec![
      codegen::multiplicative_expression(*multiplicative_expression),
      codegen::cast_expression(cast_expression),
      codegen::multiplicative_operator(multiplicative_operator),
    ]
    .into_iter()
    .flatten()
    .collect(),
  }
}

fn additive_operator(additive_operator: AdditiveOperator) -> Vec<Token> {
  match additive_operator {
    AdditiveOperator::Addition => vec![Token::Add],
    AdditiveOperator::Subtraction => vec![Token::Sub],
  }
}

fn multiplicative_operator(multiplicative_operator: MultiplicativeOperator) -> Vec<Token> {
  match multiplicative_operator {
    MultiplicativeOperator::Multiplication => vec![Token::MacroRef(Macro {
      identifier: "mul".to_string(), // TODO implement operation
    })],
    MultiplicativeOperator::Division => vec![Token::MacroRef(Macro {
      identifier: "div".to_string(), // TODO implement operation
    })],
    MultiplicativeOperator::Modulo => vec![Token::MacroRef(Macro {
      identifier: "mod".to_string(), // TODO implement operation
    })],
  }
}

fn unary_expression(unary_expression: UnaryExpression) -> Vec<Token> {
  match unary_expression {
    UnaryExpression::UnaryOperatorCastExpression(unary_operator, cast_expression) => vec![
      codegen::cast_expression(*cast_expression),
      codegen::unary_operator(unary_operator),
    ]
    .into_iter()
    .flatten()
    .collect(),
    UnaryExpression::ParenAdditiveExpressionParen(additive_expression) => {
      codegen::additive_expression(*additive_expression)
    }
    UnaryExpression::IntegerConstant(integer_constant) => {
      codegen::integer_constant(integer_constant)
    }
  }
}

fn cast_expression(cast_expression: CastExpression) -> Vec<Token> {
  match cast_expression {
    CastExpression::UnaryExpression(unary_expression) => {
      codegen::unary_expression(unary_expression)
    }
  }
}

fn unary_operator(unary_operator: UnaryOperator) -> Vec<Token> {
  match unary_operator {
    // TODO type must match
    UnaryOperator::Negation => vec![Token::Neg],
    UnaryOperator::BitwiseComplement => vec![Token::Not],
    UnaryOperator::LogicalNegation => vec![
      Token::Buf,
      Token::Pop,
      Token::XXX(0x00),
      Token::Shl,
      Token::AtDyn,
    ],
  }
}

fn integer_constant(integer_constant: IntegerConstant) -> Vec<Token> {
  match integer_constant {
    IntegerConstant::IntegerConstant(value) => vec![Token::XXX(value)],
  }
}
