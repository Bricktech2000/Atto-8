use crate::*;

#[rustfmt::skip] macro_rules! ret_label { () => { Label::Local(format!("ret"), None) }; }
#[rustfmt::skip] macro_rules! end_label { ($name:expr) => { Label::Local(format!("{}.end", $name), None) }; }
#[rustfmt::skip] macro_rules! else_label { ($name:expr) => { Label::Local(format!("{}.else", $name), None) }; }
#[rustfmt::skip] macro_rules! cond_label { ($name:expr) => { Label::Local(format!("{}.cond", $name), None) }; }
#[rustfmt::skip] macro_rules! begin_label { ($name:expr) => { Label::Local(format!("{}.begin", $name), None) }; }

#[rustfmt::skip] pub(crate) use ret_label;
#[rustfmt::skip] pub(crate) use end_label;
#[rustfmt::skip] pub(crate) use else_label;
#[rustfmt::skip] pub(crate) use cond_label;
#[rustfmt::skip] pub(crate) use begin_label;

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
    TypedGlobal::Data(label, value) => codegen::data_global(label, value),

    TypedGlobal::Macro(label, statement) => std::iter::empty()
      .chain([Ok(Token::MacroDef(link::global_macro!(&label)))])
      .chain(codegen::statement(statement))
      .chain([Ok(Token::LabelDef(codegen::ret_label!()))])
      .chain([Err(format!(""))])
      .collect(),

    TypedGlobal::Function(label, statement) => std::iter::empty()
      .chain([
        Ok(Token::MacroDef(link::def_macro!(&label))),
        Ok(Token::LabelDef(link::global_label!(&label))),
      ])
      .chain(codegen::statement(statement))
      .chain([Err(format!(""))])
      .collect(),

    // raw assembly that might not be valid is encoded through the `Err` variant
    TypedGlobal::Assembly(assembly) => std::iter::empty().chain([Err(assembly)]).collect(),
  }
}

fn data_global(label: String, value: Vec<TypedExpression>) -> Vec<Result<Token, String>> {
  let tokens: Vec<Result<Token, String>> = value
    .into_iter()
    .flat_map(|expression| codegen::expression(expression, 0))
    .collect();

  let bytes: Vec<u8> = tokens
    .iter()
    .map(|token| match token {
      Ok(Token::XXX(value)) => *value,
      _ => b'\0', // represent unknown value as null byte in comment
    })
    .collect();

  let comment = [Err(match bytes.last() {
    Some(0x00) => format!("# {}", c_quote(&bytes[..bytes.len() - 1], '"')),
    Some(_) => format!("# {}...", c_quote(&bytes, '"')),
    None => format!(""),
  })];

  let datas: Vec<Result<Token, String>> = tokens
    .into_iter()
    .flat_map(|token| match token {
      Ok(Token::XXX(value)) => vec![Ok(Token::AtDD(value))], // shorthand
      token => vec![token, Ok(Token::AtData)],               // longhand
    })
    .collect();

  std::iter::empty()
    .chain([
      Ok(Token::MacroDef(link::def_macro!(&label))),
      Ok(Token::LabelDef(link::global_label!(&label))),
    ])
    .chain(datas)
    .chain(comment)
    .collect()
}

fn statement(statement: TypedStatement) -> Vec<Result<Token, String>> {
  match statement {
    TypedStatement::ExpressionN0(expression) => std::iter::empty()
      .chain(codegen::n0_expression(flatten_expression(expression), 0))
      .collect(),

    TypedStatement::Compound(statements) => statements
      .into_iter()
      .flat_map(|statement| codegen::statement(statement))
      .collect(),

    TypedStatement::IfN1(label, condition, if_body, else_body) => codegen::if_n1_statement(
      label,
      flatten_expression(condition),
      *if_body,
      else_body.map(|else_body| *else_body),
    ),

    TypedStatement::WhileN1(label, condition, body, is_do_while) => {
      codegen::while_n1_statement(label, flatten_expression(condition), *body, is_do_while)
    }

    TypedStatement::MacroReturnN0(parameters_size, locals_size, expression) => {
      match (parameters_size, locals_size, expression) {
        (parameters_size, locals_size, Some(expression)) => std::iter::empty()
          .chain(codegen::n0_expression(flatten_expression(expression), 0))
          .chain(std::iter::repeat(Ok(Token::Pop)).take(parameters_size + locals_size))
          .chain([
            Ok(Token::LabelRef(codegen::ret_label!())),
            Ok(Token::MacroRef(link::jmp_macro!())),
          ])
          .collect(),
        (parameters_size, locals_size, None) => std::iter::empty()
          .chain(std::iter::repeat(Ok(Token::Pop)).take(parameters_size + locals_size))
          .chain([
            Ok(Token::LabelRef(codegen::ret_label!())),
            Ok(Token::MacroRef(link::jmp_macro!())),
          ])
          .collect(),
      }
    }

    TypedStatement::MacroReturnN1(parameters_size, locals_size, expression)
    | TypedStatement::MacroReturnN8(parameters_size, locals_size, expression) => {
      match (parameters_size, locals_size, expression) {
        (0, 0, Some(expression)) => std::iter::empty()
          .chain(codegen::expression(flatten_expression(expression), 0))
          .chain([
            Ok(Token::LabelRef(codegen::ret_label!())),
            Ok(Token::MacroRef(link::jmp_macro!())),
          ])
          .collect(),
        (parameters_size, locals_size, Some(expression)) => std::iter::empty()
          .chain(codegen::expression(flatten_expression(expression), 0))
          .chain(store_to_offset(parameters_size + locals_size - 1))
          .chain(std::iter::repeat(Ok(Token::Pop)).take(parameters_size + locals_size - 1))
          .chain([
            Ok(Token::LabelRef(codegen::ret_label!())),
            Ok(Token::MacroRef(link::jmp_macro!())),
          ])
          .collect(),
        (0, 0, None) => std::iter::empty()
          .chain([Ok(Token::XXX(0x00))])
          .chain([
            Ok(Token::LabelRef(codegen::ret_label!())),
            Ok(Token::MacroRef(link::jmp_macro!())),
          ])
          .collect(),
        (parameters_size, locals_size, None) => std::iter::empty()
          .chain(std::iter::repeat(Ok(Token::Pop)).take(parameters_size + locals_size - 1))
          .chain([
            Ok(Token::LabelRef(codegen::ret_label!())),
            Ok(Token::MacroRef(link::jmp_macro!())),
          ])
          .collect(),
      }
    }

    TypedStatement::FunctionReturnN0(parameters_size, locals_size, expression) => {
      match (parameters_size, locals_size, expression) {
        (0, locals_size, None) => std::iter::empty()
          .chain(std::iter::repeat(Ok(Token::Pop)).take(locals_size))
          .chain([Ok(Token::MacroRef(link::ret_macro!()))])
          .collect(),
        (0, locals_size, Some(expression)) => std::iter::empty()
          .chain(codegen::n0_expression(flatten_expression(expression), 0))
          .chain(std::iter::repeat(Ok(Token::Pop)).take(locals_size))
          .chain([Ok(Token::MacroRef(link::ret_macro!()))])
          .collect(),
        (parameters_size, locals_size, None) => std::iter::empty()
          .chain(std::iter::repeat(Ok(Token::Pop)).take(locals_size))
          .chain(store_to_offset(parameters_size - 1))
          .chain(std::iter::repeat(Ok(Token::Pop)).take(parameters_size - 1))
          .chain([Ok(Token::MacroRef(link::ret_macro!()))])
          .collect(),
        (parameters_size, locals_size, Some(expression)) => std::iter::empty()
          .chain(codegen::n0_expression(flatten_expression(expression), 0))
          .chain(std::iter::repeat(Ok(Token::Pop)).take(locals_size))
          .chain(store_to_offset(parameters_size - 1))
          .chain(std::iter::repeat(Ok(Token::Pop)).take(parameters_size - 1))
          .chain([Ok(Token::MacroRef(link::ret_macro!()))])
          .collect(),
      }
    }

    TypedStatement::FunctionReturnN1(parameters_size, locals_size, expression)
    | TypedStatement::FunctionReturnN8(parameters_size, locals_size, expression) => {
      match (parameters_size, locals_size, expression) {
        (0, 0, Some(expression)) => std::iter::empty()
          .chain(codegen::expression(flatten_expression(expression), 0))
          .chain([Ok(Token::Swp)])
          .chain([Ok(Token::MacroRef(link::ret_macro!()))])
          .collect(),
        (0, locals_size, Some(expression)) => std::iter::empty()
          .chain(codegen::expression(flatten_expression(expression), 0))
          .chain(store_to_offset(locals_size - 1))
          .chain(std::iter::repeat(Ok(Token::Pop)).take(locals_size - 1))
          .chain([Ok(Token::Swp)])
          .chain([Ok(Token::MacroRef(link::ret_macro!()))])
          .collect(),
        (0, 0, None) => std::iter::empty()
          .chain([Ok(Token::XXX(0x00)), Ok(Token::Swp)])
          .chain([Ok(Token::MacroRef(link::ret_macro!()))])
          .collect(),
        (0, locals_size, None) => std::iter::empty()
          .chain(std::iter::repeat(Ok(Token::Pop)).take(locals_size - 1))
          .chain([Ok(Token::MacroRef(link::ret_macro!()))])
          .collect(),
        (1, locals_size, None) => std::iter::empty()
          .chain(std::iter::repeat(Ok(Token::Pop)).take(locals_size))
          .chain([Ok(Token::MacroRef(link::ret_macro!()))])
          .collect(),
        (1, locals_size, Some(expression)) => std::iter::empty()
          .chain(codegen::expression(flatten_expression(expression), 0))
          .chain(store_to_offset(locals_size + 1))
          .chain(std::iter::repeat(Ok(Token::Pop)).take(locals_size))
          .chain([Ok(Token::MacroRef(link::ret_macro!()))])
          .collect(),
        (parameters_size, locals_size, None) => std::iter::empty()
          .chain(std::iter::repeat(Ok(Token::Pop)).take(locals_size))
          .chain(store_to_offset(parameters_size - 2))
          .chain(std::iter::repeat(Ok(Token::Pop)).take(parameters_size - 2))
          .chain([Ok(Token::MacroRef(link::ret_macro!()))])
          .collect(),
        (parameters_size, locals_size, Some(expression)) => std::iter::empty()
          .chain(codegen::expression(flatten_expression(expression), 0))
          .chain(store_to_offset(parameters_size + locals_size))
          .chain(std::iter::repeat(Ok(Token::Pop)).take(locals_size))
          .chain(store_to_offset(parameters_size - 2))
          .chain(std::iter::repeat(Ok(Token::Pop)).take(parameters_size - 2))
          .chain([Ok(Token::MacroRef(link::ret_macro!()))])
          .collect(),
      }
    }

    TypedStatement::InitLocalN0(expression) => match expression {
      Some(expression) => std::iter::empty()
        .chain(codegen::n0_expression(flatten_expression(expression), 0))
        .collect(),
      None => std::iter::empty().collect(),
    },

    TypedStatement::InitLocalN1(expression) => match expression {
      Some(expression) => std::iter::empty()
        .chain(codegen::n1_expression(flatten_expression(expression), 0))
        .collect(),
      None => std::iter::empty().chain([Ok(Token::XXX(0x00))]).collect(),
    },

    TypedStatement::InitLocalN8(expression) => match expression {
      Some(expression) => std::iter::empty()
        .chain(codegen::n8_expression(flatten_expression(expression), 0))
        .collect(),
      None => std::iter::empty().chain([Ok(Token::XXX(0x00))]).collect(),
    },

    TypedStatement::UninitLocalN0 => std::iter::empty().collect(),

    TypedStatement::UninitLocalN1 => std::iter::empty().chain([Ok(Token::Pop)]).collect(),

    TypedStatement::UninitLocalN8 => std::iter::empty().chain([Ok(Token::Pop)]).collect(),

    TypedStatement::Assembly(assembly) => std::iter::empty().chain([Err(assembly)]).collect(),
  }
}

fn if_n1_statement(
  label: String,
  condition: TypedExpression,
  if_body: TypedStatement,
  else_body: Option<TypedStatement>,
) -> Vec<Result<Token, String>> {
  let (precheck, condition) = match condition {
    TypedExpression::N1SecondN0N1(expression1, expression2) => {
      (codegen::n0_expression(*expression1, 0), *expression2)
    }
    _ => (vec![], condition),
  };

  let (negated, condition) = match condition {
    TypedExpression::N1BitwiseComplement(expression) => (true, *expression),
    _ => (false, condition),
  };

  let cf_inverted = match condition {
    TypedExpression::N1CastN8(_) => true,
    _ => false,
  };

  match condition {
    TypedExpression::N1BitwiseComplement(expression) if negated => {
      codegen::if_n1_statement(label, *expression, if_body, else_body)
    }

    TypedExpression::N1Constant(constant) => match constant ^ negated {
      true => std::iter::empty()
        .chain(precheck)
        .chain(codegen::statement(if_body))
        .collect(),
      false => std::iter::empty()
        .chain(precheck)
        .chain(else_body.map(codegen::statement).unwrap_or_else(Vec::new))
        .collect(),
    },

    _ => std::iter::empty()
      .chain(precheck)
      .chain(match condition {
        TypedExpression::N1EqualToN8(expression1, expression2) => {
          codegen::cf_equal_to_n8(*expression1, *expression2, 0)
        }
        TypedExpression::N1LessThanU8(expression1, expression2) => {
          codegen::cf_less_than_u8(*expression1, *expression2, 0)
        }
        TypedExpression::N1LessThanI8(expression1, expression2) => {
          codegen::cf_less_than_i8(*expression1, *expression2, 0)
        }
        TypedExpression::N1CastN8(expression) => std::iter::empty()
          .chain(codegen::n8_expression(*expression, 0))
          .chain([Ok(Token::XXX(0x01)), Ok(Token::MacroRef(link::cl_macro!()))])
          .collect(),
        _ => unreachable!(),
      })
      .chain([
        Ok(Token::LabelRef(match else_body {
          Some(_) => codegen::else_label!(&label),
          None => codegen::end_label!(&label),
        })),
        Ok(Token::MacroRef(match negated ^ cf_inverted {
          true => link::bcs_macro!(),
          false => link::bcc_macro!(),
        })),
      ])
      .chain(codegen::statement(if_body))
      .chain(match else_body {
        Some(else_body) => std::iter::empty()
          .chain([
            Ok(Token::LabelRef(codegen::end_label!(&label))),
            Ok(Token::MacroRef(link::jmp_macro!())),
            Ok(Token::LabelDef(codegen::else_label!(&label))),
          ])
          .chain(codegen::statement(else_body))
          .collect::<Vec<_>>(),
        None => std::iter::empty().collect(),
      })
      .chain([Ok(Token::LabelDef(codegen::end_label!(&label)))])
      .collect(),
  }
}

fn while_n1_statement(
  label: String,
  condition: TypedExpression,
  body: TypedStatement,
  is_do_while: bool,
) -> Vec<Result<Token, String>> {
  let (precheck, condition) = match condition {
    TypedExpression::N1SecondN0N1(expression1, expression2) => {
      (codegen::n0_expression(*expression1, 0), *expression2)
    }
    _ => (vec![], condition),
  };

  let (negated, condition) = match condition {
    TypedExpression::N1BitwiseComplement(expression) => (true, *expression),
    _ => (false, condition),
  };

  let cf_inverted = match condition {
    TypedExpression::N1CastN8(_) => true,
    _ => false,
  };

  match condition {
    TypedExpression::N1BitwiseComplement(expression) if negated => {
      codegen::while_n1_statement(label, *expression, body, is_do_while)
    }

    TypedExpression::N1Constant(constant) => match constant ^ negated {
      true => std::iter::empty()
        .chain([Ok(Token::LabelDef(codegen::begin_label!(&label)))])
        .chain(match is_do_while {
          true => std::iter::empty()
            .chain(codegen::statement(body))
            .chain(precheck),
          false => std::iter::empty()
            .chain(precheck)
            .chain(codegen::statement(body)),
        })
        .chain([
          Ok(Token::LabelRef(codegen::begin_label!(&label))),
          Ok(Token::MacroRef(link::jmp_macro!())),
        ])
        .collect(),
      false => std::iter::empty()
        .chain(match is_do_while {
          true => codegen::statement(body),
          false => std::iter::empty().collect(),
        })
        .chain(precheck)
        .collect(),
    },

    _ => std::iter::empty()
      .chain(match is_do_while {
        true => std::iter::empty().collect::<Vec<_>>(),
        false => std::iter::empty()
          .chain([
            Ok(Token::LabelRef(codegen::cond_label!(&label))),
            Ok(Token::MacroRef(link::jmp_macro!())),
          ])
          .collect(),
      })
      .chain([Ok(Token::LabelDef(codegen::begin_label!(&label)))])
      .chain(codegen::statement(body))
      .chain(match is_do_while {
        true => std::iter::empty().collect::<Vec<_>>(),
        false => std::iter::empty()
          .chain([Ok(Token::LabelDef(codegen::cond_label!(&label)))])
          .collect(),
      })
      .chain(precheck)
      .chain(match condition {
        TypedExpression::N1EqualToN8(expression1, expression2) => {
          codegen::cf_equal_to_n8(*expression1, *expression2, 0)
        }
        TypedExpression::N1LessThanU8(expression1, expression2) => {
          codegen::cf_less_than_u8(*expression1, *expression2, 0)
        }
        TypedExpression::N1LessThanI8(expression1, expression2) => {
          codegen::cf_less_than_i8(*expression1, *expression2, 0)
        }
        TypedExpression::N1CastN8(expression) => std::iter::empty()
          .chain(codegen::n8_expression(*expression, 0))
          .chain([Ok(Token::XXX(0x01)), Ok(Token::MacroRef(link::cl_macro!()))])
          .collect(),
        _ => unreachable!(),
      })
      .chain([
        Ok(Token::LabelRef(codegen::begin_label!(&label))),
        Ok(Token::MacroRef(match negated ^ cf_inverted {
          true => link::bcc_macro!(),
          false => link::bcs_macro!(),
        })),
      ])
      .collect(),
  }
}

fn expression(expression: TypedExpression, temporaries_size: usize) -> Vec<Result<Token, String>> {
  match expression {
    TypedExpression::N0DereferenceN8(_) => codegen::n0_expression(expression, temporaries_size),
    TypedExpression::N1DereferenceN8(_) => codegen::n1_expression(expression, temporaries_size),
    TypedExpression::N8DereferenceN8(_) => codegen::n8_expression(expression, temporaries_size),
    TypedExpression::N1BitwiseComplement(_) => codegen::n1_expression(expression, temporaries_size),
    TypedExpression::N8BitwiseComplement(_) => codegen::n8_expression(expression, temporaries_size),

    TypedExpression::N8Addition(_, _) => codegen::n8_expression(expression, temporaries_size),
    TypedExpression::N8Subtraction(_, _) => codegen::n8_expression(expression, temporaries_size),
    TypedExpression::N8Multiplication(_, _) => codegen::n8_expression(expression, temporaries_size),
    TypedExpression::U8Division(_, _) => codegen::n8_expression(expression, temporaries_size),
    TypedExpression::U8Modulo(_, _) => codegen::n8_expression(expression, temporaries_size),

    TypedExpression::N1EqualToN8(_, _) => codegen::n1_expression(expression, temporaries_size),
    TypedExpression::N1LessThanU8(_, _) => codegen::n1_expression(expression, temporaries_size),
    TypedExpression::N1LessThanI8(_, _) => codegen::n1_expression(expression, temporaries_size),

    TypedExpression::N0SecondN0N0(_, _) => codegen::n0_expression(expression, temporaries_size),
    TypedExpression::N1SecondN0N1(_, _) => codegen::n1_expression(expression, temporaries_size),
    TypedExpression::N8SecondN0N8(_, _) => codegen::n8_expression(expression, temporaries_size),
    TypedExpression::N0CastN1(_) => codegen::n0_expression(expression, temporaries_size),
    TypedExpression::N0CastN8(_) => codegen::n0_expression(expression, temporaries_size),
    TypedExpression::N1CastN8(_) => codegen::n1_expression(expression, temporaries_size),
    TypedExpression::N0Constant(_) => codegen::n0_expression(expression, temporaries_size),
    TypedExpression::N1Constant(_) => codegen::n1_expression(expression, temporaries_size),
    TypedExpression::N8Constant(_) => codegen::n8_expression(expression, temporaries_size),
    TypedExpression::N8LoadLocal(_) => codegen::n8_expression(expression, temporaries_size),
    TypedExpression::N8AddrLocal(_) => codegen::n8_expression(expression, temporaries_size),
    TypedExpression::N8LoadGlobal(_) => codegen::n8_expression(expression, temporaries_size),
    TypedExpression::N8AddrGlobal(_) => codegen::n8_expression(expression, temporaries_size),
    TypedExpression::N0MacroCall(_, _) => codegen::n0_expression(expression, temporaries_size),
    TypedExpression::N1MacroCall(_, _) => codegen::n1_expression(expression, temporaries_size),
    TypedExpression::N8MacroCall(_, _) => codegen::n8_expression(expression, temporaries_size),
    TypedExpression::N0FunctionCall(_, _) => codegen::n0_expression(expression, temporaries_size),
    TypedExpression::N1FunctionCall(_, _) => codegen::n1_expression(expression, temporaries_size),
    TypedExpression::N8FunctionCall(_, _) => codegen::n8_expression(expression, temporaries_size),
  }
}

fn n0_expression(
  expression: TypedExpression,
  temporaries_size: usize,
) -> Vec<Result<Token, String>> {
  match expression {
    TypedExpression::N0DereferenceN8(expression) => std::iter::empty()
      .chain(codegen::n8_expression(*expression, temporaries_size))
      .chain([Ok(Token::Lda), Ok(Token::Pop)])
      .collect(),

    TypedExpression::N0SecondN0N0(expression1, expression2) => std::iter::empty()
      .chain(codegen::n0_expression(*expression1, temporaries_size))
      .chain(codegen::n0_expression(*expression2, temporaries_size))
      .collect(),

    TypedExpression::N0CastN1(expression) => std::iter::empty()
      .chain(codegen::n1_expression(*expression, temporaries_size))
      .chain([Ok(Token::Pop)])
      .collect(),

    TypedExpression::N0CastN8(expression) => std::iter::empty()
      .chain(codegen::n8_expression(*expression, temporaries_size))
      .chain([Ok(Token::Pop)])
      .collect(),

    TypedExpression::N0Constant(_constant) => std::iter::empty().collect(),

    TypedExpression::N0MacroCall(label, arguments) => arguments
      .into_iter()
      .enumerate()
      // TODO assumes all expressions are one byte in size
      .flat_map(|(index, expression)| codegen::expression(expression, temporaries_size + index))
      .chain([Ok(Token::MacroRef(link::global_macro!(&label)))])
      .collect(),

    TypedExpression::N0FunctionCall(designator, arguments) => {
      let arguments_size = arguments.len();

      arguments
        .into_iter()
        .enumerate()
        // TODO assumes all expressions are one byte in size
        .flat_map(|(index, expression)| codegen::expression(expression, temporaries_size + index))
        .chain(codegen::n8_expression(
          *designator,
          // TODO assumes all arguments are one byte in size
          temporaries_size + arguments_size,
        ))
        .chain([Ok(Token::MacroRef(link::call_macro!()))])
        .collect()
    }

    _ => unreachable!(),
  }
}

fn n1_expression(
  expression: TypedExpression,
  temporaries_size: usize,
) -> Vec<Result<Token, String>> {
  match expression {
    TypedExpression::N1DereferenceN8(expression) => std::iter::empty()
      .chain(codegen::n8_expression(*expression, temporaries_size))
      .chain([Ok(Token::Lda)])
      .collect(),

    TypedExpression::N1BitwiseComplement(expression) => std::iter::empty()
      .chain(codegen::n1_expression(*expression, temporaries_size))
      .chain([Ok(Token::XXX(0x01)), Ok(Token::Xor)])
      .collect(),

    TypedExpression::N1SecondN0N1(expression1, expression2) => std::iter::empty()
      .chain(codegen::n0_expression(*expression1, temporaries_size))
      .chain(codegen::n1_expression(*expression2, temporaries_size))
      .collect(),

    TypedExpression::N1CastN8(expression) => std::iter::empty()
      .chain(codegen::n8_expression(*expression, temporaries_size))
      .chain([Ok(Token::XXX(0x01)), Ok(Token::And)])
      .collect(),

    TypedExpression::N1EqualToN8(expression1, expression2) => std::iter::empty()
      .chain(cf_equal_to_n8(*expression1, *expression2, temporaries_size))
      .chain([Ok(Token::XXX(0x00)), Ok(Token::Shl), Ok(Token::AtDyn)])
      .collect(),

    TypedExpression::N1LessThanU8(expression1, expression2)
      if *expression1 == TypedExpression::N8Constant(0xFF)
        || *expression2 == TypedExpression::N8Constant(0x00) =>
    {
      std::iter::empty().chain([Ok(Token::XXX(0x00))]).collect()
    }

    TypedExpression::N1LessThanU8(expression1, expression2) => std::iter::empty()
      .chain(codegen::cf_less_than_u8(
        *expression1,
        *expression2,
        temporaries_size,
      ))
      .into_iter()
      .chain([Ok(Token::XXX(0x00)), Ok(Token::Shl), Ok(Token::AtDyn)])
      .collect(),

    TypedExpression::N1LessThanI8(expression1, expression2)
      if *expression1 == TypedExpression::N8Constant(0x7F)
        || *expression2 == TypedExpression::N8Constant(0x80) =>
    {
      std::iter::empty().chain([Ok(Token::XXX(0x00))]).collect()
    }

    TypedExpression::N1LessThanI8(expression1, expression2) => std::iter::empty()
      .chain(codegen::cf_less_than_i8(
        *expression1,
        *expression2,
        temporaries_size,
      ))
      .into_iter()
      .chain([Ok(Token::XXX(0x00)), Ok(Token::Shl), Ok(Token::AtDyn)])
      .collect(),

    TypedExpression::N1Constant(constant) => match constant {
      true => vec![Ok(Token::XXX(0x01))],
      false => vec![Ok(Token::XXX(0x00))],
    },

    TypedExpression::N1MacroCall(_, _) => todo!(),

    TypedExpression::N1FunctionCall(_, _) => todo!(),

    _ => unreachable!(),
  }
}

fn n8_expression(
  expression: TypedExpression,
  temporaries_size: usize,
) -> Vec<Result<Token, String>> {
  match expression {
    TypedExpression::N8DereferenceN8(expression) => std::iter::empty()
      .chain(codegen::n8_expression(*expression, temporaries_size))
      .chain([Ok(Token::Lda)])
      .collect(),

    TypedExpression::N8BitwiseComplement(expression) => std::iter::empty()
      .chain(codegen::n8_expression(*expression, temporaries_size))
      .chain([Ok(Token::Not)])
      .collect(),

    TypedExpression::N8Addition(expression1, expression2) => match (*expression1, *expression2) {
      (expression, TypedExpression::N8Constant(0x02))
      | (TypedExpression::N8Constant(0x02), expression) => std::iter::empty()
        .chain(codegen::n8_expression(expression, temporaries_size))
        .chain([Ok(Token::Inc)])
        .chain([Ok(Token::Inc)])
        .collect(),
      (expression, TypedExpression::N8Constant(0x01))
      | (TypedExpression::N8Constant(0x01), expression) => std::iter::empty()
        .chain(codegen::n8_expression(expression, temporaries_size))
        .chain([Ok(Token::Inc)])
        .collect(),
      (expression, TypedExpression::N8Constant(0x00))
      | (TypedExpression::N8Constant(0x00), expression) => std::iter::empty()
        .chain(codegen::n8_expression(expression, temporaries_size))
        .collect(),
      (expression, TypedExpression::N8Constant(0xFF))
      | (TypedExpression::N8Constant(0xFF), expression) => std::iter::empty()
        .chain(codegen::n8_expression(expression, temporaries_size))
        .chain([Ok(Token::Dec)])
        .collect(),
      (expression, TypedExpression::N8Constant(0xFE))
      | (TypedExpression::N8Constant(0xFE), expression) => std::iter::empty()
        .chain(codegen::n8_expression(expression, temporaries_size))
        .chain([Ok(Token::Dec)])
        .chain([Ok(Token::Dec)])
        .collect(),
      (expression1, expression2) => std::iter::empty()
        .chain(codegen::n8_expression(expression1, temporaries_size))
        .chain(codegen::n8_expression(expression2, temporaries_size + 1))
        .chain([Ok(Token::Clc), Ok(Token::Add)])
        .collect(),
    },

    TypedExpression::N8Subtraction(expression1, expression2) => {
      match (*expression1, *expression2) {
        (expression, TypedExpression::N8Constant(0x02)) => std::iter::empty()
          .chain(codegen::n8_expression(expression, temporaries_size))
          .chain([Ok(Token::Dec)])
          .chain([Ok(Token::Dec)])
          .collect(),
        (expression, TypedExpression::N8Constant(0x01)) => std::iter::empty()
          .chain(codegen::n8_expression(expression, temporaries_size))
          .chain([Ok(Token::Dec)])
          .collect(),
        (expression, TypedExpression::N8Constant(0x00)) => std::iter::empty()
          .chain(codegen::n8_expression(expression, temporaries_size))
          .collect(),
        (expression, TypedExpression::N8Constant(0xFF)) => std::iter::empty()
          .chain(codegen::n8_expression(expression, temporaries_size))
          .chain([Ok(Token::Inc)])
          .collect(),
        (expression, TypedExpression::N8Constant(0xFE)) => std::iter::empty()
          .chain(codegen::n8_expression(expression, temporaries_size))
          .chain([Ok(Token::Inc)])
          .chain([Ok(Token::Inc)])
          .collect(),
        (TypedExpression::N8Constant(0x01), expression) => std::iter::empty()
          .chain(codegen::n8_expression(expression, temporaries_size))
          .chain([Ok(Token::Neg)])
          .chain([Ok(Token::Inc)])
          .collect(),
        (TypedExpression::N8Constant(0x00), expression) => std::iter::empty()
          .chain(codegen::n8_expression(expression, temporaries_size))
          .chain([Ok(Token::Neg)])
          .collect(),
        (TypedExpression::N8Constant(0xFF), expression) => std::iter::empty()
          .chain(codegen::n8_expression(expression, temporaries_size))
          .chain([Ok(Token::Neg)])
          .chain([Ok(Token::Dec)])
          .collect(),
        (expression1, expression2) => std::iter::empty()
          .chain(codegen::n8_expression(expression1, temporaries_size))
          .chain(codegen::n8_expression(expression2, temporaries_size + 1))
          .chain([Ok(Token::Clc), Ok(Token::Sub)])
          .collect(),
      }
    }

    TypedExpression::N8Multiplication(expression1, expression2) => {
      match (*expression1, *expression2) {
        (expression, TypedExpression::N8Constant(0x04))
        | (TypedExpression::N8Constant(0x04), expression) => std::iter::empty()
          .chain(codegen::n8_expression(expression, temporaries_size))
          .chain([
            Ok(Token::Clc),
            Ok(Token::Shl),
            Ok(Token::Clc),
            Ok(Token::Shl),
          ])
          .collect(),
        // TODO implement universal multiplication by constant
        (expression, TypedExpression::N8Constant(0x03))
        | (TypedExpression::N8Constant(0x03), expression) => std::iter::empty()
          .chain(codegen::n8_expression(expression, temporaries_size))
          .chain([
            Ok(Token::LdO(Ofst(0x00))),
            Ok(Token::Clc),
            Ok(Token::Shl),
            Ok(Token::Clc),
            Ok(Token::Add),
          ])
          .collect(),
        (expression, TypedExpression::N8Constant(0x02))
        | (TypedExpression::N8Constant(0x02), expression) => std::iter::empty()
          .chain(codegen::n8_expression(expression, temporaries_size))
          .chain([Ok(Token::Clc), Ok(Token::Shl)])
          .collect(),
        (expression, TypedExpression::N8Constant(0x01))
        | (TypedExpression::N8Constant(0x01), expression) => std::iter::empty()
          .chain(codegen::n8_expression(expression, temporaries_size))
          .collect(),
        (_expression, TypedExpression::N8Constant(0x00))
        | (TypedExpression::N8Constant(0x00), _expression) => {
          std::iter::empty().chain([Ok(Token::XXX(0x00))]).collect()
        }
        (expression1, expression2) => std::iter::empty()
          .chain(codegen::n8_expression(expression1, temporaries_size))
          .chain(codegen::n8_expression(expression2, temporaries_size + 1))
          .chain([Ok(Token::MacroRef(link::mul_macro!()))])
          .collect(),
      }
    }

    TypedExpression::U8Division(expression1, expression2) => {
      match (*expression1, *expression2) {
        (expression1, TypedExpression::N8Constant(0x04)) => std::iter::empty()
          .chain(codegen::n8_expression(expression1, temporaries_size))
          .chain([
            Ok(Token::Clc),
            Ok(Token::Shr),
            Ok(Token::Clc),
            Ok(Token::Shr),
          ])
          .collect(),
        (expression1, TypedExpression::N8Constant(0x02)) => std::iter::empty()
          .chain(codegen::n8_expression(expression1, temporaries_size))
          .chain([Ok(Token::Clc), Ok(Token::Shr)])
          .collect(),
        (expression1, TypedExpression::N8Constant(0x01)) => std::iter::empty()
          .chain(codegen::n8_expression(expression1, temporaries_size))
          .collect(),
        (_expression1, TypedExpression::N8Constant(0x00)) => std::iter::empty().collect(), // behavior is undefined
        (TypedExpression::N8Constant(0x00), _expression) => {
          std::iter::empty().chain([Ok(Token::XXX(0x00))]).collect()
        }
        (expression1, expression2) => std::iter::empty()
          .chain(codegen::n8_expression(expression1, temporaries_size))
          .chain(codegen::n8_expression(expression2, temporaries_size + 1))
          .chain([Ok(Token::MacroRef(link::div_macro!()))])
          .collect(),
      }
    }

    TypedExpression::U8Modulo(expression1, expression2) => {
      match (*expression1, *expression2) {
        (expression1, TypedExpression::N8Constant(0x04)) => std::iter::empty()
          .chain(codegen::n8_expression(expression1, temporaries_size))
          .chain([Ok(Token::XXX(0x03)), Ok(Token::And)])
          .collect(),
        (expression1, TypedExpression::N8Constant(0x02)) => std::iter::empty()
          .chain(codegen::n8_expression(expression1, temporaries_size))
          .chain([Ok(Token::XXX(0x01)), Ok(Token::And)])
          .collect(),
        (_expression1, TypedExpression::N8Constant(0x01)) => std::iter::empty()
          .chain(codegen::n8_expression(TypedExpression::N8Constant(0x00), 0))
          .collect(),
        (_expression1, TypedExpression::N8Constant(0x00)) => std::iter::empty().collect(), // behavior is undefined
        (TypedExpression::N8Constant(0x00), _expression) => {
          std::iter::empty().chain([Ok(Token::XXX(0x00))]).collect()
        }
        (expression1, expression2) => std::iter::empty()
          .chain(codegen::n8_expression(expression1, temporaries_size))
          .chain(codegen::n8_expression(expression2, temporaries_size + 1))
          .chain([Ok(Token::MacroRef(link::mod_macro!()))])
          .collect(),
      }
    }

    TypedExpression::N8SecondN0N8(expression1, expression2) => std::iter::empty()
      .chain(codegen::n0_expression(*expression1, temporaries_size))
      .chain(codegen::n8_expression(*expression2, temporaries_size))
      .collect(),

    TypedExpression::N8Constant(constant) => vec![Ok(Token::XXX(constant))],

    TypedExpression::N8LoadLocal(offset) => std::iter::empty()
      .chain(load_from_offset(offset + temporaries_size))
      .collect(),

    TypedExpression::N8AddrLocal(_offset) => todo!(),

    TypedExpression::N8LoadGlobal(label) => std::iter::empty()
      .chain([Ok(Token::LabelRef(link::global_label!(&label)))])
      .chain([Ok(Token::Lda)])
      .collect(),

    TypedExpression::N8AddrGlobal(label) => std::iter::empty()
      .chain([Ok(Token::LabelRef(link::global_label!(&label)))])
      .collect(),

    TypedExpression::N8MacroCall(label, arguments) => arguments
      .into_iter()
      .enumerate()
      // TODO assumes all expressions are one byte in size
      .flat_map(|(index, expression)| codegen::expression(expression, temporaries_size + index))
      .chain([Ok(Token::MacroRef(link::global_macro!(&label)))])
      .collect(),

    TypedExpression::N8FunctionCall(designator, arguments) => {
      let arguments_size = arguments.len();

      arguments
        .into_iter()
        .enumerate()
        // TODO assumes all expressions are one byte in size
        .flat_map(|(index, expression)| codegen::expression(expression, temporaries_size + index))
        .chain(codegen::n8_expression(
          *designator,
          // TODO assumes all arguments are one byte in size
          temporaries_size + arguments_size,
        ))
        .chain([Ok(Token::MacroRef(link::call_macro!()))])
        .collect()
    }

    _ => unreachable!(),
  }
}

fn load_from_offset(offset: usize) -> Vec<Result<Token, String>> {
  match u8::try_from(offset) {
    Ok(offset) => match Ofst::new(offset) {
      Some(ofst) => vec![Ok(Token::LdO(ofst))],
      None => vec![
        Ok(Token::Lds),
        Ok(Token::XXX(offset)),
        Ok(Token::Clc),
        Ok(Token::Add),
        Ok(Token::Lda),
      ],
    },
    // throw assembly-time error
    Err(_) => vec![Ok(Token::AtError), Err(format!("# `Ldo` stack overflow"))],
  }
}

fn store_to_offset(offset: usize) -> Vec<Result<Token, String>> {
  match u8::try_from(offset) {
    Ok(offset) => match Ofst::new(offset) {
      Some(ofst) => vec![Ok(Token::StO(ofst))],
      None => vec![
        Ok(Token::Lds),
        Ok(Token::XXX(offset)),
        Ok(Token::Clc),
        Ok(Token::Add),
        Ok(Token::Sta),
      ],
    },
    // throw assembly-time error
    Err(_) => vec![Ok(Token::AtError), Err(format!("# `Sto` stack overflow"))],
  }
}

fn cf_less_than_u8(
  expression1: TypedExpression,
  expression2: TypedExpression,
  temporaries_size: usize,
) -> Vec<Result<Token, String>> {
  match (expression1, expression2) {
    (expression, TypedExpression::N8Constant(0x01)) => std::iter::empty()
      .chain(codegen::n8_expression(expression, temporaries_size))
      .chain([Ok(Token::MacroRef(link::zr_macro!()))])
      .collect(),
    (TypedExpression::N8Constant(0x00), expression) => std::iter::empty()
      .chain(codegen::n8_expression(expression, temporaries_size))
      .chain([Ok(Token::MacroRef(link::zr_macro!())), Ok(Token::Flc)])
      .collect(),
    (expression, TypedExpression::N8Constant(0xFF)) => std::iter::empty()
      .chain(codegen::n8_expression(expression, temporaries_size))
      .chain([Ok(Token::MacroRef(link::on_macro!())), Ok(Token::Flc)])
      .collect(),
    (TypedExpression::N8Constant(0xFE), expression) => std::iter::empty()
      .chain(codegen::n8_expression(expression, temporaries_size))
      .chain([Ok(Token::MacroRef(link::on_macro!()))])
      .collect(),
    (expression, TypedExpression::N8Constant(0x80)) => std::iter::empty()
      .chain(codegen::n8_expression(expression, temporaries_size))
      .chain([Ok(Token::XXX(0x80)), Ok(Token::MacroRef(link::cl_macro!()))])
      .collect(),
    (TypedExpression::N8Constant(0x7F), expression) => std::iter::empty()
      .chain(codegen::n8_expression(expression, temporaries_size))
      .chain([
        Ok(Token::XXX(0x80)),
        Ok(Token::MacroRef(link::cl_macro!())),
        Ok(Token::Flc),
      ])
      .collect(),
    (expression, TypedExpression::N8Constant(0x02)) => std::iter::empty()
      .chain(codegen::n8_expression(expression, temporaries_size))
      .chain([
        Ok(Token::Clc),
        Ok(Token::Shr),
        Ok(Token::MacroRef(link::zr_macro!())),
      ])
      .collect(),
    (TypedExpression::N8Constant(0x01), expression) => std::iter::empty()
      .chain(codegen::n8_expression(expression, temporaries_size))
      .chain([
        Ok(Token::Clc),
        Ok(Token::Shr),
        Ok(Token::MacroRef(link::zr_macro!())),
        Ok(Token::Flc),
      ])
      .collect(),
    (expression, TypedExpression::N8Constant(0x04)) => std::iter::empty()
      .chain(codegen::n8_expression(expression, temporaries_size))
      .chain([
        Ok(Token::Clc),
        Ok(Token::Shr),
        Ok(Token::Clc),
        Ok(Token::Shr),
        Ok(Token::MacroRef(link::zr_macro!())),
      ])
      .collect(),
    (TypedExpression::N8Constant(0x03), expression) => std::iter::empty()
      .chain(codegen::n8_expression(expression, temporaries_size))
      .chain([
        Ok(Token::Clc),
        Ok(Token::Shr),
        Ok(Token::Clc),
        Ok(Token::Shr),
        Ok(Token::MacroRef(link::zr_macro!())),
        Ok(Token::Flc),
      ])
      .collect(),
    (expression1, expression2) => std::iter::empty()
      .chain(codegen::n8_expression(expression2, temporaries_size))
      .chain(codegen::n8_expression(expression1, temporaries_size + 1))
      .chain([Ok(Token::Clc), Ok(Token::MacroRef(link::lt_macro!()))])
      .collect(),
  }
}

fn cf_less_than_i8(
  expression1: TypedExpression,
  expression2: TypedExpression,
  temporaries_size: usize,
) -> Vec<Result<Token, String>> {
  match (expression1, expression2) {
    (expression, TypedExpression::N8Constant(0x81)) => std::iter::empty()
      .chain(codegen::n8_expression(expression, temporaries_size))
      .chain([Ok(Token::XXX(0x80)), Ok(Token::MacroRef(link::eq_macro!()))])
      .collect(),
    (TypedExpression::N8Constant(0x80), expression) => std::iter::empty()
      .chain(codegen::n8_expression(expression, temporaries_size))
      .chain([
        Ok(Token::XXX(0x80)),
        Ok(Token::MacroRef(link::eq_macro!())),
        Ok(Token::Flc),
      ])
      .collect(),
    (expression, TypedExpression::N8Constant(0x7F)) => std::iter::empty()
      .chain(codegen::n8_expression(expression, temporaries_size))
      .chain([
        Ok(Token::XXX(0x7F)),
        Ok(Token::MacroRef(link::eq_macro!())),
        Ok(Token::Flc),
      ])
      .collect(),
    (TypedExpression::N8Constant(0x7E), expression) => std::iter::empty()
      .chain(codegen::n8_expression(expression, temporaries_size))
      .chain([Ok(Token::XXX(0x7F)), Ok(Token::MacroRef(link::eq_macro!()))])
      .collect(),
    (expression, TypedExpression::N8Constant(0x00)) => std::iter::empty()
      .chain(codegen::n8_expression(expression, temporaries_size))
      .chain([
        Ok(Token::XXX(0x80)),
        Ok(Token::MacroRef(link::cl_macro!())),
        Ok(Token::Flc),
      ])
      .collect(),
    (TypedExpression::N8Constant(0xFF), expression) => std::iter::empty()
      .chain(codegen::n8_expression(expression, temporaries_size))
      .chain([Ok(Token::XXX(0x80)), Ok(Token::MacroRef(link::cl_macro!()))])
      .collect(),
    (expression1, expression2) => std::iter::empty()
      .chain(codegen::n8_expression(expression2, temporaries_size))
      .chain(codegen::n8_expression(expression1, temporaries_size + 1))
      .chain([
        Ok(Token::Clc),
        Ok(Token::Sub),
        Ok(Token::XXX(0x80)),
        Ok(Token::MacroRef(link::cl_macro!())),
      ])
      .collect(),
  }
}

fn cf_equal_to_n8(
  expression1: TypedExpression,
  expression2: TypedExpression,
  temporaries_size: usize,
) -> Vec<Result<Token, String>> {
  match (expression1, expression2) {
    (TypedExpression::N8Constant(0x00), expression)
    | (expression, TypedExpression::N8Constant(0x00)) => std::iter::empty()
      .chain(codegen::n8_expression(expression, temporaries_size))
      .chain([Ok(Token::MacroRef(link::zr_macro!()))])
      .collect(),
    (TypedExpression::N8Constant(0xFF), expression)
    | (expression, TypedExpression::N8Constant(0xFF)) => std::iter::empty()
      .chain(codegen::n8_expression(expression, temporaries_size))
      .chain([Ok(Token::MacroRef(link::on_macro!()))])
      .collect(),
    (expression1, expression2) => std::iter::empty()
      .chain(codegen::n8_expression(expression1, temporaries_size))
      .chain(codegen::n8_expression(expression2, temporaries_size + 1))
      .chain([Ok(Token::MacroRef(link::eq_macro!()))])
      .collect(),
  }
}

fn flatten_expression(expression: TypedExpression) -> TypedExpression {
  // constant folding

  macro_rules! default {
    ($expression:expr, $second_variant:ident, $outer_variant:ident) => {
      match $expression {
        TypedExpression::N0SecondN0N0(expression1, expression2)
        | TypedExpression::N1SecondN0N1(expression1, expression2)
        | TypedExpression::N8SecondN0N8(expression1, expression2) => {
          flatten_expression(TypedExpression::$second_variant(
            expression1,
            Box::new(TypedExpression::$outer_variant(expression2)),
          ))
        }
        expression => TypedExpression::$outer_variant(Box::new(expression)),
      }
    };

    ($expression1:expr, $expression2:expr, $second_variant:ident, $outer_variant:ident) => {
      match ($expression1, $expression2) {
        (TypedExpression::N0SecondN0N0(expression1, expression2), expression3)
        | (TypedExpression::N1SecondN0N1(expression1, expression2), expression3)
        | (TypedExpression::N8SecondN0N8(expression1, expression2), expression3) => {
          flatten_expression(TypedExpression::$second_variant(
            expression1,
            Box::new(TypedExpression::$outer_variant(
              expression2,
              Box::new(expression3),
            )),
          ))
        }
        (expression1, TypedExpression::N0SecondN0N0(expression2, expression3))
        | (expression1, TypedExpression::N1SecondN0N1(expression2, expression3))
        | (expression1, TypedExpression::N8SecondN0N8(expression2, expression3)) => {
          flatten_expression(TypedExpression::$second_variant(
            expression2,
            Box::new(TypedExpression::$outer_variant(
              Box::new(expression1),
              expression3,
            )),
          ))
        }
        (expression1, expression2) => {
          TypedExpression::$outer_variant(Box::new(expression1), Box::new(expression2))
        }
      }
    };
  }

  match expression {
    TypedExpression::N0DereferenceN8(expression) => match flatten_expression(*expression) {
      expression => default!(expression, N0SecondN0N0, N0DereferenceN8),
    },

    TypedExpression::N1DereferenceN8(expression) => match flatten_expression(*expression) {
      expression => default!(expression, N1SecondN0N1, N1DereferenceN8),
    },

    TypedExpression::N8DereferenceN8(expression) => match flatten_expression(*expression) {
      expression => default!(expression, N8SecondN0N8, N8DereferenceN8),
    },

    TypedExpression::N1BitwiseComplement(expression) => match flatten_expression(*expression) {
      TypedExpression::N1BitwiseComplement(expression) => *expression,
      TypedExpression::N1Constant(constant) => TypedExpression::N1Constant(!constant),
      expression => default!(expression, N1SecondN0N1, N1BitwiseComplement),
    },

    TypedExpression::N8BitwiseComplement(expression) => match flatten_expression(*expression) {
      TypedExpression::N8BitwiseComplement(expression) => *expression,
      TypedExpression::N8Constant(constant) => TypedExpression::N8Constant(!constant),
      expression => default!(expression, N8SecondN0N8, N8BitwiseComplement),
    },

    TypedExpression::N8Addition(expression1, expression2) => {
      match (
        flatten_expression(*expression1),
        flatten_expression(*expression2),
      ) {
        (TypedExpression::N8Constant(constant1), TypedExpression::N8Constant(constant2)) => {
          TypedExpression::N8Constant(constant1.wrapping_add(constant2))
        }
        (expression1, expression2) => {
          default!(expression1, expression2, N8SecondN0N8, N8Addition)
        }
      }
    }

    TypedExpression::N8Subtraction(expression1, expression2) => {
      match (
        flatten_expression(*expression1),
        flatten_expression(*expression2),
      ) {
        (TypedExpression::N8Constant(constant1), TypedExpression::N8Constant(constant2)) => {
          TypedExpression::N8Constant(constant1.wrapping_sub(constant2))
        }
        (expression1, expression2) => {
          default!(expression1, expression2, N8SecondN0N8, N8Subtraction)
        }
      }
    }

    TypedExpression::N8Multiplication(expression1, expression2) => {
      match (
        flatten_expression(*expression1),
        flatten_expression(*expression2),
      ) {
        (TypedExpression::N8Constant(constant1), TypedExpression::N8Constant(constant2)) => {
          TypedExpression::N8Constant(constant1.wrapping_mul(constant2))
        }
        (expression1, expression2) => {
          default!(expression1, expression2, N8SecondN0N8, N8Multiplication)
        }
      }
    }

    TypedExpression::U8Division(expression1, expression2) => {
      match (
        flatten_expression(*expression1),
        flatten_expression(*expression2),
      ) {
        (TypedExpression::N8Constant(constant), TypedExpression::N8Constant(0x00)) => {
          TypedExpression::N8Constant(constant) // behavior is undefined
        }
        (TypedExpression::N8Constant(constant1), TypedExpression::N8Constant(constant2)) => {
          TypedExpression::N8Constant(constant1.wrapping_div(constant2))
        }
        (expression1, expression2) => {
          default!(expression1, expression2, N8SecondN0N8, U8Division)
        }
      }
    }

    TypedExpression::U8Modulo(expression1, expression2) => {
      match (
        flatten_expression(*expression1),
        flatten_expression(*expression2),
      ) {
        (TypedExpression::N8Constant(constant), TypedExpression::N8Constant(0x00)) => {
          TypedExpression::N8Constant(constant) // behavior is undefined
        }
        (TypedExpression::N8Constant(constant1), TypedExpression::N8Constant(constant2)) => {
          TypedExpression::N8Constant(constant1.wrapping_rem(constant2))
        }
        (expression1, expression2) => {
          default!(expression1, expression2, N8SecondN0N8, U8Modulo)
        }
      }
    }

    TypedExpression::N1EqualToN8(expression1, expression2) => {
      match (
        flatten_expression(*expression1),
        flatten_expression(*expression2),
      ) {
        (TypedExpression::N8Constant(constant1), TypedExpression::N8Constant(constant2)) => {
          TypedExpression::N1Constant(constant1 == constant2)
        }
        (expression1, expression2) => {
          default!(expression1, expression2, N1SecondN0N1, N1EqualToN8)
        }
      }
    }

    TypedExpression::N1LessThanU8(expression1, expression2) => {
      match (
        flatten_expression(*expression1),
        flatten_expression(*expression2),
      ) {
        (TypedExpression::N8Constant(constant1), TypedExpression::N8Constant(constant2)) => {
          TypedExpression::N1Constant(constant1 < constant2)
        }
        (expression1, expression2) => {
          default!(expression1, expression2, N1SecondN0N1, N1LessThanU8)
        }
      }
    }

    TypedExpression::N1LessThanI8(expression1, expression2) => {
      match (
        flatten_expression(*expression1),
        flatten_expression(*expression2),
      ) {
        (TypedExpression::N8Constant(constant1), TypedExpression::N8Constant(constant2)) => {
          TypedExpression::N1Constant((constant1 as i8) < constant2 as i8)
        }
        (expression1, expression2) => {
          default!(expression1, expression2, N1SecondN0N1, N1LessThanI8)
        }
      }
    }

    TypedExpression::N0SecondN0N0(expression1, expression2) => TypedExpression::N0SecondN0N0(
      Box::new(flatten_expression(*expression1)),
      Box::new(flatten_expression(*expression2)),
    ),

    TypedExpression::N1SecondN0N1(expression1, expression2) => TypedExpression::N1SecondN0N1(
      Box::new(flatten_expression(*expression1)),
      Box::new(flatten_expression(*expression2)),
    ),

    TypedExpression::N8SecondN0N8(expression1, expression2) => TypedExpression::N8SecondN0N8(
      Box::new(flatten_expression(*expression1)),
      Box::new(flatten_expression(*expression2)),
    ),

    TypedExpression::N0CastN1(expression) => match flatten_expression(*expression) {
      TypedExpression::N1Constant(_constant) => TypedExpression::N0Constant(()),
      expression => default!(expression, N0SecondN0N0, N0CastN1),
    },

    TypedExpression::N0CastN8(expression) => match flatten_expression(*expression) {
      expression => default!(expression, N0SecondN0N0, N0CastN8),
    },

    TypedExpression::N1CastN8(expression) => match flatten_expression(*expression) {
      TypedExpression::N8Constant(constant) => {
        TypedExpression::N1Constant((constant & 0x01) != 0x00)
      }
      expression => default!(expression, N1SecondN0N1, N1CastN8),
    },

    TypedExpression::N0Constant(constant) => TypedExpression::N0Constant(constant),

    TypedExpression::N1Constant(constant) => TypedExpression::N1Constant(constant),

    TypedExpression::N8Constant(constant) => TypedExpression::N8Constant(constant),

    TypedExpression::N8LoadLocal(offset) => TypedExpression::N8LoadLocal(offset),

    TypedExpression::N8AddrLocal(offset) => TypedExpression::N8AddrLocal(offset),

    TypedExpression::N8LoadGlobal(label) => TypedExpression::N8LoadGlobal(label),

    TypedExpression::N8AddrGlobal(label) => TypedExpression::N8AddrGlobal(label),

    TypedExpression::N0MacroCall(label, arguments) => TypedExpression::N0MacroCall(
      label,
      arguments.into_iter().map(flatten_expression).collect(),
    ),

    TypedExpression::N1MacroCall(label, arguments) => TypedExpression::N1MacroCall(
      label,
      arguments.into_iter().map(flatten_expression).collect(),
    ),

    TypedExpression::N8MacroCall(label, arguments) => TypedExpression::N8MacroCall(
      label,
      arguments.into_iter().map(flatten_expression).collect(),
    ),

    TypedExpression::N0FunctionCall(designator, arguments) => TypedExpression::N0FunctionCall(
      Box::new(flatten_expression(*designator)),
      arguments.into_iter().map(flatten_expression).collect(),
    ),

    TypedExpression::N1FunctionCall(designator, arguments) => TypedExpression::N1FunctionCall(
      Box::new(flatten_expression(*designator)),
      arguments.into_iter().map(flatten_expression).collect(),
    ),

    TypedExpression::N8FunctionCall(designator, arguments) => TypedExpression::N8FunctionCall(
      Box::new(flatten_expression(*designator)),
      arguments.into_iter().map(flatten_expression).collect(),
    ),
  }
}
