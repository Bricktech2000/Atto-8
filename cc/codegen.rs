use crate::*;

pub fn codegen(translation_unit: TranslationUnit, entry_point: &str) -> Result<Vec<Token>, Error> {
  let here_label = Label {
    identifier: "here".to_string(),
    scope_uid: None,
  };
  let entry_macro = Macro {
    identifier: entry_point.to_string(),
  };
  let entry_label = Label {
    identifier: "main".to_string(),
    scope_uid: None,
  };

  let tokens: Vec<Token> = vec![
    vec![
      Token::MacroDef(entry_macro),
      Token::LabelRef(entry_label),
      Token::Sti,
    ],
    codegen::translation_unit(translation_unit),
    vec![
      Token::LabelDef(here_label.clone()),
      Token::LabelRef(here_label.clone()),
      Token::Sti,
    ],
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
  match function_definition {
    FunctionDefinition::NameBody(name, body) => vec![
      vec![Token::LabelDef(Label {
        identifier: name.clone(),
        scope_uid: None,
      })],
      codegen::compound_statement(body),
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
    MagicReturn::UnaryExpression(magic_return) => codegen::unary_expression(magic_return),
  }
}

fn unary_expression(unary_expression: UnaryExpression) -> Vec<Token> {
  match unary_expression {
    UnaryExpression::UnaryOperatorCastExpression(unary_operator, cast_expression) => vec![
      codegen::cast_expression(cast_expression),
      codegen::unary_operator(unary_operator),
    ]
    .into_iter()
    .flatten()
    .collect(),
    UnaryExpression::IntegerConstant(integer_constant) => {
      codegen::integer_constant(integer_constant)
    }
  }
}

fn cast_expression(cast_expression: CastExpression) -> Vec<Token> {
  match cast_expression {
    CastExpression::UnaryExpression(unary_expression) => {
      codegen::unary_expression(*unary_expression)
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
