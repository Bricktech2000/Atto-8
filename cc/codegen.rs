use crate::*;

pub fn codegen(
  program: TypedProgram,
  _errors: &mut Vec<(Pos, Error)>,
) -> Vec<Result<Token, String>> {
  codegen::program(program)
}

fn program(program: TypedProgram) -> Vec<Result<Token, String>> {
  match program {
    TypedProgram(globals) => globals
      .into_iter()
      .flat_map(|global| codegen::global(global))
      .collect(),
  }
}

fn global(global: TypedGlobal) -> Vec<Result<Token, String>> {
  match global {
    TypedGlobal::String(label, value) => {
      std::iter::empty()
        .chain(vec![
          Ok(Token::MacroDef(Macro(format!("{}.def", label.clone())))),
          Ok(Token::LabelDef(Label::Global(label.clone()))),
        ])
        .chain(
          value
            .chars()
            .map(|c| Ok(Token::AtDD(c as u8)))
            .collect::<Vec<Result<Token, String>>>(),
        )
        .chain(vec![Ok(Token::AtDD(0x00))])
        // TODO uses debug formatting
        .chain(vec![Err(format!("# {:?}", value))])
        .collect()
    }

    TypedGlobal::Macro(label, statement) => std::iter::empty()
      .chain(vec![Ok(Token::MacroDef(Macro(label)))])
      .chain(codegen::statement(statement))
      .chain(vec![Ok(Token::LabelDef(Label::Local(
        format!("macro.end"),
        None,
      )))])
      .chain(vec![Err(format!(""))])
      .collect(),

    TypedGlobal::Function(label, statement) => std::iter::empty()
      .chain(vec![
        Ok(Token::MacroDef(Macro(format!("{}.def", label.clone())))),
        Ok(Token::LabelDef(Label::Global(label))),
      ])
      .chain(codegen::statement(statement))
      .chain(vec![Err(format!(""))])
      .collect(),

    // raw assembly that might not be valid is encoded through the `Err` variant
    TypedGlobal::Assembly(assembly) => std::iter::empty().chain(vec![Err(assembly)]).collect(),
  }
}

fn statement(statement: TypedStatement) -> Vec<Result<Token, String>> {
  match statement {
    TypedStatement::ExpressionN0(expression) => std::iter::empty()
      .chain(codegen::n0_expression(expression))
      .collect(),

    TypedStatement::Compound(statements) => statements
      .into_iter()
      .flat_map(|statement| codegen::statement(statement))
      .collect(),

    TypedStatement::WhileN1(label, condition, body) => {
      codegen::while_n1_statement(label, condition, *body)
    }

    TypedStatement::MacroReturnN0(parameters_size, locals_size, expression) => {
      let jmp_macro = Macro("jmp".to_string());
      let end_label = Label::Local(format!("macro.end"), None);

      match (parameters_size, locals_size, expression) {
        (parameters_size, locals_size, Some(expression)) => std::iter::empty()
          .chain(codegen::n0_expression(expression))
          .chain(vec![Ok(Token::Pop); parameters_size + locals_size])
          .chain(vec![
            Ok(Token::LabelRef(end_label)),
            Ok(Token::MacroRef(jmp_macro)),
          ])
          .collect(),
        (parameters_size, locals_size, None) => std::iter::empty()
          .chain(vec![Ok(Token::Pop); parameters_size + locals_size])
          .chain(vec![
            Ok(Token::LabelRef(end_label)),
            Ok(Token::MacroRef(jmp_macro)),
          ])
          .collect(),
      }
    }

    TypedStatement::MacroReturnN1(parameters_size, locals_size, expression)
    | TypedStatement::MacroReturnN8(parameters_size, locals_size, expression) => {
      let jmp_macro = Macro("jmp".to_string());
      let end_label = Label::Local(format!("macro.end"), None);

      match (parameters_size, locals_size, expression) {
        (0, 0, Some(expression)) => std::iter::empty()
          .chain(codegen::expression(expression))
          .chain(vec![
            Ok(Token::LabelRef(end_label)),
            Ok(Token::MacroRef(jmp_macro)),
          ])
          .collect(),
        (parameters_size, locals_size, Some(expression)) => std::iter::empty()
          .chain(codegen::expression(expression))
          .chain(vec![Ok(Token::StO(
            (parameters_size + locals_size - 1) as u8,
          ))])
          .chain(vec![Ok(Token::Pop); parameters_size + locals_size - 1])
          .chain(vec![
            Ok(Token::LabelRef(end_label)),
            Ok(Token::MacroRef(jmp_macro)),
          ])
          .collect(),
        (0, 0, None) => std::iter::empty()
          .chain(vec![Ok(Token::XXX(0x00))])
          .chain(vec![
            Ok(Token::LabelRef(end_label)),
            Ok(Token::MacroRef(jmp_macro)),
          ])
          .collect(),
        (parameters_size, locals_size, None) => std::iter::empty()
          .chain(vec![Ok(Token::Pop); parameters_size + locals_size - 1])
          .chain(vec![
            Ok(Token::LabelRef(end_label)),
            Ok(Token::MacroRef(jmp_macro)),
          ])
          .collect(),
      }
    }

    TypedStatement::FunctionReturnN0(parameters_size, locals_size, expression) => {
      let ret_macro = Macro("ret".to_string());

      match (parameters_size, locals_size, expression) {
        (0, locals_size, Some(expression)) => std::iter::empty()
          .chain(codegen::n0_expression(expression))
          .chain(vec![Ok(Token::Pop); locals_size])
          .chain(vec![Ok(Token::MacroRef(ret_macro))])
          .collect(),
        (0, locals_size, None) => std::iter::empty()
          .chain(vec![Ok(Token::Pop); locals_size])
          .chain(vec![Ok(Token::MacroRef(ret_macro))])
          .collect(),
        (_, _, _) => todo!(),
      }
    }

    TypedStatement::FunctionReturnN1(parameters_size, locals_size, expression)
    | TypedStatement::FunctionReturnN8(parameters_size, locals_size, expression) => {
      let ret_macro = Macro("ret".to_string());

      match (parameters_size, locals_size, expression) {
        (0, 0, Some(expression)) => std::iter::empty()
          .chain(codegen::expression(expression))
          .chain(vec![Ok(Token::Swp)])
          .chain(vec![Ok(Token::MacroRef(ret_macro))])
          .collect(),
        (0, locals_size, Some(expression)) => std::iter::empty()
          .chain(codegen::expression(expression))
          .chain(vec![Ok(Token::StO((locals_size - 1) as u8))])
          .chain(vec![Ok(Token::Pop); locals_size - 1])
          .chain(vec![Ok(Token::Swp)])
          .chain(vec![Ok(Token::MacroRef(ret_macro))])
          .collect(),
        (0, 0, None) => std::iter::empty()
          .chain(vec![Ok(Token::XXX(0x00)), Ok(Token::Swp)])
          .chain(vec![Ok(Token::MacroRef(ret_macro))])
          .collect(),
        (0, locals_size, None) => std::iter::empty()
          .chain(vec![Ok(Token::Pop); locals_size - 1])
          .chain(vec![Ok(Token::MacroRef(ret_macro))])
          .collect(),
        (1, locals_size, None) => std::iter::empty()
          .chain(vec![Ok(Token::Pop); locals_size])
          .chain(vec![Ok(Token::MacroRef(ret_macro))])
          .collect(),
        (1, locals_size, Some(expression)) => std::iter::empty()
          .chain(codegen::expression(expression))
          .chain(vec![Ok(Token::StO((locals_size + 1) as u8))])
          .chain(vec![Ok(Token::Pop); locals_size])
          .chain(vec![Ok(Token::MacroRef(ret_macro))])
          .collect(),
        (parameters_size, locals_size, None) => std::iter::empty()
          .chain(vec![Ok(Token::Pop); locals_size])
          .chain(vec![Ok(Token::StO((parameters_size - 2) as u8))])
          .chain(vec![Ok(Token::Pop); parameters_size - 2])
          .chain(vec![Ok(Token::MacroRef(ret_macro))])
          .collect(),
        (parameters_size, locals_size, Some(expression)) => std::iter::empty()
          .chain(codegen::expression(expression))
          .chain(vec![Ok(Token::StO((parameters_size + locals_size) as u8))])
          .chain(vec![Ok(Token::Pop); locals_size])
          .chain(vec![Ok(Token::StO((parameters_size - 2) as u8))])
          .chain(vec![Ok(Token::Pop); parameters_size - 2])
          .chain(vec![Ok(Token::MacroRef(ret_macro))])
          .collect(),
      }
    }

    TypedStatement::InitLocalN0(_expression) => todo!(),

    TypedStatement::InitLocalN1(_expression) => todo!(),

    TypedStatement::InitLocalN8(_expression) => todo!(),

    TypedStatement::Assembly(assembly) => std::iter::empty().chain(vec![Err(assembly)]).collect(),
  }
}

fn while_n1_statement(
  label: String,
  condition: TypedExpression,
  body: TypedStatement,
) -> Vec<Result<Token, String>> {
  let jmp_macro = Macro("jmp".to_string());
  let bcc_macro = Macro("bcc".to_string());
  let bcs_macro = Macro("bcs".to_string());
  let zr_macro = Macro("zr".to_string());
  let cl_macro = Macro("cl".to_string());

  let while_label = Label::Local(format!("{}", label), None);
  let cond_label = Label::Local(format!("{}.cond", label), None);

  let (negated, condition) = match condition {
    TypedExpression::N1BitwiseComplement(expression) => (true, *expression),
    _ => (false, condition),
  };

  match condition {
    TypedExpression::N1Constant(constant) if constant == negated => vec![],

    TypedExpression::N1Constant(constant) if constant != negated => std::iter::empty()
      .chain(vec![Ok(Token::LabelDef(while_label.clone()))])
      .chain(codegen::statement(body))
      .chain(vec![
        Ok(Token::LabelRef(while_label.clone())),
        Ok(Token::MacroRef(jmp_macro.clone())),
      ])
      .collect(),

    TypedExpression::N1CastN8(expression) => std::iter::empty()
      .chain(vec![
        Ok(Token::LabelRef(cond_label.clone())),
        Ok(Token::MacroRef(jmp_macro.clone())),
        Ok(Token::LabelDef(while_label.clone())),
      ])
      .chain(codegen::statement(body))
      .chain(vec![Ok(Token::LabelDef(cond_label.clone()))])
      .chain(codegen::n8_expression(*expression))
      .chain(vec![
        Ok(Token::XXX(0x01)),
        Ok(Token::MacroRef(cl_macro.clone())),
        Ok(Token::LabelRef(while_label.clone())),
        Ok(Token::MacroRef(match negated {
          true => bcs_macro.clone(),
          false => bcc_macro.clone(),
        })),
      ])
      .collect(),

    TypedExpression::N1IsZeroN8(expression) => std::iter::empty()
      .chain(vec![
        Ok(Token::LabelRef(cond_label.clone())),
        Ok(Token::MacroRef(jmp_macro.clone())),
        Ok(Token::LabelDef(while_label.clone())),
      ])
      .chain(codegen::statement(body))
      .chain(vec![Ok(Token::LabelDef(cond_label.clone()))])
      .chain(codegen::n8_expression(*expression))
      .chain(vec![
        Ok(Token::MacroRef(zr_macro.clone())),
        Ok(Token::LabelRef(while_label.clone())),
        Ok(Token::MacroRef(match negated {
          true => bcs_macro.clone(),
          false => bcc_macro.clone(),
        })),
      ])
      .collect(),

    TypedExpression::N1BitwiseComplement(expression) if negated => {
      codegen::while_n1_statement(label, *expression, body)
    }

    _ => unreachable!(),
  }
}

fn expression(expression: TypedExpression) -> Vec<Result<Token, String>> {
  match expression {
    TypedExpression::N8Negation(_) => codegen::n8_expression(expression),
    TypedExpression::N1BitwiseComplement(_) => codegen::n1_expression(expression),
    TypedExpression::N8BitwiseComplement(_) => codegen::n8_expression(expression),

    TypedExpression::N8Addition(_, _) => codegen::n8_expression(expression),
    TypedExpression::N8Subtraction(_, _) => codegen::n8_expression(expression),
    TypedExpression::U8Multiplication(_, _) => codegen::n8_expression(expression),
    TypedExpression::U8Division(_, _) => codegen::n8_expression(expression),
    TypedExpression::U8Modulo(_, _) => codegen::n8_expression(expression),

    TypedExpression::N0CastN1(_) => codegen::n0_expression(expression),
    TypedExpression::N0CastN8(_) => codegen::n0_expression(expression),
    TypedExpression::N1CastN8(_) => codegen::n1_expression(expression),
    TypedExpression::N1IsZeroN8(_) => codegen::n1_expression(expression),
    TypedExpression::N0Constant(_) => codegen::n0_expression(expression),
    TypedExpression::N1Constant(_) => codegen::n1_expression(expression),
    TypedExpression::N8Constant(_) => codegen::n8_expression(expression),
    TypedExpression::N8GetLocal(_) => codegen::n8_expression(expression),
    TypedExpression::N8AddrLocal(_) => codegen::n8_expression(expression),
    TypedExpression::N8GetGlobal(_) => codegen::n8_expression(expression),
    TypedExpression::N8AddrGlobal(_) => codegen::n8_expression(expression),
    TypedExpression::N0MacroCall(_, _) => codegen::n0_expression(expression),
    TypedExpression::N1MacroCall(_, _) => codegen::n1_expression(expression),
    TypedExpression::N8MacroCall(_, _) => codegen::n8_expression(expression),
    TypedExpression::N0FunctionCall(_, _) => codegen::n0_expression(expression),
    TypedExpression::N1FunctionCall(_, _) => codegen::n1_expression(expression),
    TypedExpression::N8FunctionCall(_, _) => codegen::n8_expression(expression),
  }
}

fn n0_expression(expression: TypedExpression) -> Vec<Result<Token, String>> {
  match expression {
    TypedExpression::N0CastN1(expression) => std::iter::empty()
      .chain(codegen::n1_expression(*expression))
      .chain(vec![Ok(Token::Pop)])
      .collect(),

    TypedExpression::N0CastN8(expression) => std::iter::empty()
      .chain(codegen::n8_expression(*expression))
      .chain(vec![Ok(Token::Pop)])
      .collect(),

    TypedExpression::N0Constant(_constant) => vec![],

    TypedExpression::N0MacroCall(label, arguments) => arguments
      .into_iter()
      .flat_map(|expression| codegen::expression(expression))
      .chain(vec![Ok(Token::MacroRef(Macro(label)))])
      .collect(),

    TypedExpression::N0FunctionCall(designator, arguments) => {
      let call_macro = Macro("call".to_string());

      arguments
        .into_iter()
        .flat_map(|expression| codegen::expression(expression))
        .chain(codegen::n8_expression(*designator))
        .chain(vec![Ok(Token::MacroRef(call_macro))])
        .collect()
    }

    _ => unreachable!(),
  }
}

fn n1_expression(expression: TypedExpression) -> Vec<Result<Token, String>> {
  match expression {
    TypedExpression::N1BitwiseComplement(expression) => std::iter::empty()
      .chain(codegen::n1_expression(*expression))
      .chain(vec![Ok(Token::XXX(0x01)), Ok(Token::Xor)])
      .collect(),

    TypedExpression::N1CastN8(expression) => std::iter::empty()
      .chain(codegen::n8_expression(*expression))
      .chain(vec![Ok(Token::XXX(0x01)), Ok(Token::And)])
      .collect(),

    TypedExpression::N1IsZeroN8(expression) => {
      let zr_macro = Macro("zr".to_string());

      std::iter::empty()
        .chain(codegen::n8_expression(*expression))
        .chain(vec![
          Ok(Token::MacroRef(zr_macro)),
          Ok(Token::XXX(0x00)),
          Ok(Token::Shl),
          Ok(Token::AtDyn),
        ])
        .collect()
    }

    TypedExpression::N1Constant(constant) => match constant {
      true => vec![Ok(Token::XXX(0x01))],
      false => vec![Ok(Token::XXX(0x00))],
    },

    TypedExpression::N1MacroCall(_, _) => todo!(),

    TypedExpression::N1FunctionCall(_, _) => todo!(),

    _ => unreachable!(),
  }
}

fn n8_expression(expression: TypedExpression) -> Vec<Result<Token, String>> {
  match expression {
    TypedExpression::N8Negation(expression) => std::iter::empty()
      .chain(codegen::n8_expression(*expression))
      .chain(vec![Ok(Token::Neg)])
      .collect(),

    TypedExpression::N8BitwiseComplement(expression) => std::iter::empty()
      .chain(codegen::n8_expression(*expression))
      .chain(vec![Ok(Token::Not)])
      .collect(),

    TypedExpression::N8Addition(expression1, expression2) => match (*expression1, *expression2) {
      (expression, TypedExpression::N8Constant(0x01))
      | (TypedExpression::N8Constant(0x01), expression) => std::iter::empty()
        .chain(codegen::n8_expression(expression))
        .chain(vec![Ok(Token::Inc)])
        .collect(),
      (expression, TypedExpression::N8Constant(0x00))
      | (TypedExpression::N8Constant(0x00), expression) => std::iter::empty()
        .chain(codegen::n8_expression(expression))
        .collect(),
      (expression, TypedExpression::N8Constant(0xff))
      | (TypedExpression::N8Constant(0xff), expression) => std::iter::empty()
        .chain(codegen::n8_expression(expression))
        .chain(vec![Ok(Token::Dec)])
        .collect(),
      (expression1, expression2) => std::iter::empty()
        .chain(codegen::n8_expression(expression1))
        .chain(codegen::n8_expression(expression2))
        .chain(vec![Ok(Token::Clc), Ok(Token::Add)])
        .collect(),
    },

    TypedExpression::N8Subtraction(expression1, expression2) => {
      match (*expression1, *expression2) {
        (expression, TypedExpression::N8Constant(0x01))
        | (TypedExpression::N8Constant(0x01), expression) => std::iter::empty()
          .chain(codegen::n8_expression(expression))
          .chain(vec![Ok(Token::Dec)])
          .collect(),
        (expression, TypedExpression::N8Constant(0x00))
        | (TypedExpression::N8Constant(0x00), expression) => std::iter::empty()
          .chain(codegen::n8_expression(expression))
          .collect(),
        (expression, TypedExpression::N8Constant(0xff))
        | (TypedExpression::N8Constant(0xff), expression) => std::iter::empty()
          .chain(codegen::n8_expression(expression))
          .chain(vec![Ok(Token::Inc)])
          .collect(),
        (expression1, expression2) => std::iter::empty()
          .chain(codegen::n8_expression(expression1))
          .chain(codegen::n8_expression(expression2))
          .chain(vec![Ok(Token::Clc), Ok(Token::Sub)])
          .collect(),
      }
    }

    TypedExpression::U8Multiplication(expression1, expression2) => {
      let mul_macro = Macro("mul".to_string());

      match (*expression1, *expression2) {
        (expression, TypedExpression::N8Constant(0x02))
        | (TypedExpression::N8Constant(0x02), expression) => std::iter::empty()
          .chain(codegen::n8_expression(expression))
          .chain(vec![Ok(Token::Clc), Ok(Token::Shl)])
          .collect(),
        (expression, TypedExpression::N8Constant(0x01))
        | (TypedExpression::N8Constant(0x01), expression) => std::iter::empty()
          .chain(codegen::n8_expression(expression))
          .collect(),
        (_expression, TypedExpression::N8Constant(0x00))
        | (TypedExpression::N8Constant(0x00), _expression) => std::iter::empty()
          .chain(vec![Ok(Token::XXX(0x00))])
          .collect(),
        (expression1, expression2) => std::iter::empty()
          .chain(codegen::n8_expression(expression1))
          .chain(codegen::n8_expression(expression2))
          .chain(vec![Ok(Token::MacroRef(mul_macro))])
          .collect(),
      }
    }

    TypedExpression::U8Division(expression1, expression2) => {
      let div_macro = Macro("div".to_string());

      match (*expression1, *expression2) {
        (expression1, TypedExpression::N8Constant(0x02)) => std::iter::empty()
          .chain(codegen::n8_expression(expression1))
          .chain(vec![Ok(Token::Clc), Ok(Token::Shr)])
          .collect(),
        (expression1, TypedExpression::N8Constant(0x01)) => std::iter::empty()
          .chain(codegen::n8_expression(expression1))
          .collect(),
        (_expression1, TypedExpression::N8Constant(0x00)) => std::iter::empty().collect(), // behavior is undefined
        (TypedExpression::N8Constant(0x00), _expression) => std::iter::empty()
          .chain(vec![Ok(Token::XXX(0x00))])
          .collect(),
        (expression1, expression2) => std::iter::empty()
          .chain(codegen::n8_expression(expression1))
          .chain(codegen::n8_expression(expression2))
          .chain(vec![Ok(Token::MacroRef(div_macro))])
          .collect(),
      }
    }

    TypedExpression::U8Modulo(expression1, expression2) => {
      let mod_macro = Macro("mod".to_string());

      match (*expression1, *expression2) {
        (expression1, TypedExpression::N8Constant(0x02)) => std::iter::empty()
          .chain(codegen::n8_expression(expression1))
          .chain(vec![Ok(Token::XXX(0x01)), Ok(Token::And)])
          .collect(),
        (_expression1, TypedExpression::N8Constant(0x01)) => std::iter::empty()
          .chain(codegen::n8_expression(TypedExpression::N8Constant(0x00)))
          .collect(),
        (_expression1, TypedExpression::N8Constant(0x00)) => std::iter::empty().collect(), // behavior is undefined
        (TypedExpression::N8Constant(0x00), _expression) => std::iter::empty()
          .chain(vec![Ok(Token::XXX(0x00))])
          .collect(),
        (expression1, expression2) => std::iter::empty()
          .chain(codegen::n8_expression(expression1))
          .chain(codegen::n8_expression(expression2))
          .chain(vec![Ok(Token::MacroRef(mod_macro))])
          .collect(),
      }
    }

    TypedExpression::N8Constant(constant) => vec![Ok(Token::XXX(constant))],

    // TODO assumes the stack contains no temporaries
    TypedExpression::N8GetLocal(offset) => std::iter::empty()
      .chain(vec![Ok(Token::LdO(offset as u8))])
      .collect(),

    TypedExpression::N8AddrLocal(_offset) => todo!(),

    TypedExpression::N8GetGlobal(label) => std::iter::empty()
      .chain(vec![Ok(Token::LabelRef(Label::Global(label)))])
      .chain(vec![Ok(Token::Lda)])
      .collect(),

    TypedExpression::N8AddrGlobal(label) => std::iter::empty()
      .chain(vec![Ok(Token::LabelRef(Label::Global(label)))])
      .collect(),

    TypedExpression::N8MacroCall(label, arguments) => arguments
      .into_iter()
      .flat_map(|expression| codegen::expression(expression))
      .chain(vec![Ok(Token::MacroRef(Macro(label)))])
      .collect(),

    TypedExpression::N8FunctionCall(designator, arguments) => {
      let call_macro = Macro("call".to_string());

      arguments
        .into_iter()
        .flat_map(|expression| codegen::expression(expression))
        .chain(codegen::n8_expression(*designator))
        .chain(vec![Ok(Token::MacroRef(call_macro))])
        .collect()
    }

    _ => unreachable!(),
  }
}
