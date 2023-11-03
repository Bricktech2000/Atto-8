use crate::*;
use std::collections::{HashMap, HashSet};

pub fn codegen(program: Program) -> Result<Vec<Token>, Error> {
  let tokens: Vec<Token> = codegen::program(program, &mut State::default());

  Ok(tokens)
}

#[derive(Clone, PartialEq, Default, Debug)]
struct State {
  declarations: HashMap<String, (bool, Type)>, // map from global declaration to its type and whether it is inlined
  definitions: HashSet<String>,                // set of currently defined globals
  parameters: HashMap<String, Type>,           // map from function paramater to its type
  locals: HashMap<String, Type>,               // map from local variable to its type
  global: Option<String>,                      // current global name
  stack: Vec<StackEntry>,                      // current nesting stack
  uid: usize,                                  // unique identifier for temporary variables
  strings: HashMap<String, String>,            // map from string literal to its label
  dependencies: HashMap<String, HashSet<String>>, // map from global to its dependencies
}

#[allow(dead_code)]
#[derive(Clone, PartialEq, Debug)]
enum StackEntry {
  ProgramBoundary,
  FunctionBoundary(Type),
  LoopBoundary,
  BlockBoundary,
  Temporary(Type),
}

impl StackEntry {
  #[allow(dead_code)]
  pub fn size(&self) -> usize {
    match self {
      StackEntry::ProgramBoundary => 0,
      StackEntry::FunctionBoundary(type_) => type_.size(),
      StackEntry::LoopBoundary => 0,
      StackEntry::BlockBoundary => 0,
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

fn program(program: Program, state: &mut State) -> Vec<Token> {
  state.stack.push(StackEntry::ProgramBoundary);

  let program_tokens: Vec<Token> = match program {
    Program(globals) => globals.into_iter().flat_map(|global| match global {
      Global::FunctionDeclaration(function_declaration) => {
        codegen::function_declaration(function_declaration.clone(), state)
      }

      Global::FunctionDefinition(function_definition) => {
        codegen::function_definition(function_definition.clone(), state)
      }

      Global::AsmStatement(assembly) => codegen::asm_statement(assembly, state),
    }),
  }
  .collect();

  let strings_tokens: Vec<Token> = state
    .strings
    .iter()
    .flat_map(|(string, label)| {
      std::iter::empty()
        .chain(vec![
          Token::MacroDef(Macro(format!("{}.def", label.clone()))),
          Token::LabelDef(Label::Global(label.clone())),
        ])
        .chain(
          string
            .chars()
            .map(|c| Token::AtDD(c as u8))
            .collect::<Vec<Token>>(),
        )
        .chain(vec![Token::AtDD(0x00)])
    })
    .collect();

  // brute-force transitive closure of dependencies
  // if A depends on B and B depends on C, then A depends on C
  let original_dependencies = state.dependencies.clone();
  for name in original_dependencies.keys() {
    let mut stack: Vec<String> = vec![name.clone()];
    let mut visited: HashSet<String> = HashSet::new();

    // depth-first iteration because recursion would be inconvenient
    while let Some(node) = stack.pop() {
      for dependency in original_dependencies.get(&node).unwrap_or(&HashSet::new()) {
        if !visited.contains(dependency) {
          stack.push(dependency.clone());
          visited.insert(dependency.clone());
          state
            .dependencies
            .get_mut(name)
            .unwrap_or_else(|| unreachable!())
            .insert(dependency.clone());
        }
      }
    }
  }

  let dependencies_tokens: Vec<Token> = state
    .dependencies
    .iter()
    .flat_map(|(name, dependencies)| {
      std::iter::empty()
        .chain(vec![Token::MacroDef(Macro(format!(
          "{}.deps",
          name.clone()
        )))])
        .chain(
          dependencies
            .iter()
            .filter(|dependency| {
              !state
                .declarations
                .get(dependency.as_str())
                .map(|(inline, _type)| *inline)
                .unwrap_or(false)
            })
            .flat_map(|dependency| {
              vec![Token::MacroRef(Macro(format!(
                "{}.def",
                dependency.clone()
              )))]
            }),
        )
    })
    .collect();

  match state.stack.pop() {
    Some(StackEntry::ProgramBoundary) => (),
    _ => panic!("Expected program boundary"),
  }

  std::iter::empty()
    .chain(program_tokens)
    .chain(strings_tokens)
    .chain(dependencies_tokens)
    .collect()
}

fn function_declaration(
  function_declaration: FunctionDeclaration,
  state: &mut State,
) -> Vec<Token> {
  match function_declaration {
    FunctionDeclaration(inline, Object(return_type, name), parameters) => {
      let parameter_types = parameters
        .iter()
        .map(|Object(type_, _name)| type_.clone())
        .collect::<Vec<Type>>();

      state
        .declarations
        .entry(name.clone())
        .and_modify(
          |(existing_inline, existing_type)| match (existing_inline, existing_type) {
            (existing_inline, Type::Function(existing_return_type, existing_parameter_types)) => {
              if *existing_inline != inline
                || **existing_return_type != return_type
                || *existing_parameter_types != parameter_types
              {
                panic!(
                  "Function `{}` already declared with different signature",
                  name.clone()
                )
              }
            }
            _ => (),
          },
        )
        .or_insert((
          inline,
          Type::Function(Box::new(return_type.clone()), parameter_types),
        ));

      vec![]
    }
  }
}

fn function_definition(function_definition: FunctionDefinition, state: &mut State) -> Vec<Token> {
  match function_definition {
    FunctionDefinition(inline, Object(return_type, name), parameters, body) => {
      // TODO function parameters

      let tokens = codegen::function_declaration(
        FunctionDeclaration(
          inline,
          Object(return_type.clone(), name.clone()),
          parameters.clone(),
        ),
        state,
      );

      state
        .definitions
        .get(&name)
        .is_some()
        .then(|| panic!("Function `{}` already defined", name.clone()));

      state.definitions.insert(name.clone());

      state
        .dependencies
        .entry(name.clone())
        .or_insert(HashSet::new())
        .insert(name.clone());

      state
        .stack
        .push(StackEntry::FunctionBoundary(return_type.clone()));

      match state.global.replace(name.clone()) {
        None => (),
        _ => panic!("Expected no global"),
      }

      let tokens = std::iter::empty()
        .chain(tokens)
        .chain(match inline {
          true => vec![Token::MacroDef(Macro(format!("{}", name.clone())))],
          false => vec![
            Token::MacroDef(Macro(format!("{}.def", name.clone()))),
            Token::LabelDef(Label::Global(name.clone())),
          ],
        })
        .chain(codegen::statement(body, state))
        .collect();

      match state.global.take() {
        Some(name_) if name_ == name => (),
        _ => panic!("Expected global"),
      }

      match state.stack.pop() {
        Some(StackEntry::FunctionBoundary(type_)) if type_ == return_type => (),
        _ => panic!("Expected function boundary"),
      }

      tokens
    }
  }
}

fn statement(statement: Statement, state: &mut State) -> Vec<Token> {
  match statement {
    Statement::Expression(expression) => codegen::expression_statement(expression, state),
    Statement::Compound(statements) => codegen::compound_statement(statements, state),
    Statement::While(condition, body) => codegen::while_statement(condition, *body, state),
    Statement::Return(expression) => codegen::return_statement(expression, state),
    Statement::Asm(assembly) => codegen::asm_statement(assembly, state),
  }
}

fn expression_statement(expression: Expression, state: &mut State) -> Vec<Token> {
  let (_type, tokens) =
    codegen::expression(Expression::Cast(Type::Void, Box::new(expression)), state);

  tokens
}

fn compound_statement(statements: Vec<Statement>, state: &mut State) -> Vec<Token> {
  state.stack.push(StackEntry::BlockBoundary);

  let tokens: Vec<Token> = statements
    .into_iter()
    .flat_map(|statement| codegen::statement(statement, state))
    .collect();

  match state.stack.pop() {
    Some(StackEntry::BlockBoundary) => (),
    _ => panic!("Expected block boundary"),
  }

  tokens
}

fn while_statement(condition: Expression, body: Statement, state: &mut State) -> Vec<Token> {
  state.stack.push(StackEntry::LoopBoundary);

  let jmp_macro = Macro("jmp".to_string());
  let bcc_macro = Macro("bcc".to_string());
  let zr_macro = Macro("zr".to_string());

  let loop_label = Label::Global(format!("loop.{}", state.uid));
  let while_label = Label::Global(format!("while.{}", state.uid));
  let check_label = Label::Global(format!("check.{}", state.uid));
  state.uid += 1;

  let tokens = match condition {
    Expression::IntegerConstant(0x00) => vec![],
    Expression::IntegerConstant(_) => std::iter::empty()
      .chain(vec![Token::LabelDef(loop_label.clone())])
      .chain(codegen::statement(body, state))
      .chain(vec![
        Token::LabelRef(loop_label.clone()),
        Token::MacroRef(jmp_macro.clone()),
      ])
      .collect(),
    _ => std::iter::empty()
      .chain(vec![
        Token::LabelRef(check_label.clone()),
        Token::MacroRef(jmp_macro.clone()),
        Token::LabelDef(while_label.clone()),
      ])
      .chain(codegen::statement(body, state))
      .chain(vec![Token::LabelDef(check_label.clone())])
      .chain({
        let (type_, tokens) = codegen::expression(condition, state);
        match type_ {
          type_ if type_.size() == 1 => std::iter::empty()
            .chain(tokens)
            .chain(vec![Token::MacroRef(zr_macro.clone())]),
          _ => todo!(),
        }
      })
      .chain(vec![
        Token::LabelRef(while_label.clone()),
        Token::MacroRef(bcc_macro.clone()),
        Token::Clc,
      ])
      .collect(),
  };

  match state.stack.pop() {
    Some(StackEntry::LoopBoundary) => (),
    _ => panic!("Expected loop boundary"),
  }

  tokens
}

fn return_statement(expression: Option<Expression>, state: &mut State) -> Vec<Token> {
  let ret_macro = Macro("ret".to_string());

  // TODO pop off items from stack until we reach a function boundary
  let return_type = state
    .stack
    .iter()
    .rev()
    .find_map(|stack_entry| match stack_entry {
      StackEntry::FunctionBoundary(return_type) => Some(return_type),
      _ => None,
    })
    .unwrap_or_else(|| panic!("`return` outside of function"));

  let (type_, tokens) = match expression {
    Some(expression) => codegen::expression(
      Expression::Cast(return_type.clone(), Box::new(expression)),
      state,
    ),
    None => (Type::Void, vec![]),
  };

  match type_ {
    type_ if type_.size() == 0 => std::iter::empty()
      .chain(tokens)
      .chain(vec![Token::MacroRef(ret_macro)])
      .collect(),
    type_ if type_.size() == 1 => std::iter::empty()
      .chain(tokens)
      .chain(vec![Token::Swp, Token::MacroRef(ret_macro)])
      .collect(),
    _ => todo!(),
  }
}

fn asm_statement(assembly: String, _state: &mut State) -> Vec<Token> {
  // TODO partially copied from `asm.rs`
  // TODO does not support file includes
  let assembly = assembly
    .lines()
    .map(|line| line.strip_suffix("#").unwrap_or(line))
    .map(|line| line.split("# ").next().unwrap_or(line))
    .map(|line| line.to_string() + "\n")
    .collect::<String>();

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

  tokens
}

fn expression(expression: Expression, state: &mut State) -> (Type, Vec<Token>) {
  match expression.clone() {
    Expression::Negation(expression) => codegen::negation_expression(*expression, state),
    Expression::LogicalNegation(expression) => {
      codegen::logical_negation_expression(*expression, state)
    }
    Expression::BitwiseComplement(expression) => {
      codegen::bitwise_complement_expression(*expression, state)
    }

    Expression::Addition(expression1, expression2) => {
      codegen::addition_expression(*expression1, *expression2, state)
    }
    Expression::Subtraction(expression1, expression2) => {
      codegen::subtraction_expression(*expression1, *expression2, state)
    }
    Expression::Multiplication(expression1, expression2) => {
      codegen::multiplication_expression(*expression1, *expression2, state)
    }
    Expression::Division(expression1, expression2) => {
      codegen::division_expression(*expression1, *expression2, state)
    }
    Expression::Modulo(expression1, expression2) => {
      codegen::modulo_expression(*expression1, *expression2, state)
    }
    Expression::LogicalAnd(expression1, expression2) => {
      codegen::logical_and_expression(*expression1, *expression2, state)
    }
    Expression::LogicalOr(expression1, expression2) => {
      codegen::logical_or_expression(*expression1, *expression2, state)
    }
    Expression::BitwiseAnd(expression1, expression2) => {
      codegen::bitwise_and_expression(*expression1, *expression2, state)
    }
    Expression::BitwiseInclusiveOr(expression1, expression2) => {
      codegen::bitwise_inclusive_or_expression(*expression1, *expression2, state)
    }
    Expression::BitwiseExclusiveOr(expression1, expression2) => {
      codegen::bitwise_exclusive_or_expression(*expression1, *expression2, state)
    }
    Expression::LeftShift(expression1, expression2) => {
      codegen::left_shift_expression(*expression1, *expression2, state)
    }
    Expression::RightShift(expression1, expression2) => {
      codegen::right_shift_expression(*expression1, *expression2, state)
    }

    Expression::EqualTo(expression1, expression2) => {
      codegen::equal_to_expression(*expression1, *expression2, state)
    }
    Expression::NotEqualTo(expression1, expression2) => {
      codegen::not_equal_to_expression(*expression1, *expression2, state)
    }
    Expression::LessThan(expression1, expression2) => {
      codegen::less_than_expression(*expression1, *expression2, state)
    }
    Expression::LessThanOrEqualTo(expression1, expression2) => {
      codegen::less_than_or_equal_to_expression(*expression1, *expression2, state)
    }
    Expression::GreaterThan(expression1, expression2) => {
      codegen::greater_than_expression(*expression1, *expression2, state)
    }
    Expression::GreaterThanOrEqualTo(expression1, expression2) => {
      codegen::greater_than_or_equal_to_expression(*expression1, *expression2, state)
    }

    Expression::Conditional(expression1, expression2, expression3) => {
      codegen::conditional_expression(*expression1, *expression2, *expression3, state)
    }

    Expression::Cast(type_, expression) => codegen::cast_expression(type_, *expression, state),
    Expression::IntegerConstant(value) => codegen::integer_constant_expression(value, state),
    Expression::CharacterConstant(value) => codegen::character_constant_expression(value, state),
    Expression::StringLiteral(value) => codegen::string_literal_expression(value, state),
    Expression::Identifier(_) => todo!(),
    Expression::FunctionCall(name, arguments) => {
      codegen::function_call_expression(name, arguments, state)
    }
  }
}

fn negation_expression(expression: Expression, state: &mut State) -> (Type, Vec<Token>) {
  let (type_, tokens) = codegen::expression(expression, state);

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

fn logical_negation_expression(expression: Expression, state: &mut State) -> (Type, Vec<Token>) {
  let (type_, tokens) = codegen::expression(expression, state);

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

fn bitwise_complement_expression(expression: Expression, state: &mut State) -> (Type, Vec<Token>) {
  let (type_, tokens) = codegen::expression(expression, state);

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
  state: &mut State,
) -> (Type, Vec<Token>) {
  let (type_, tokens1, tokens2) =
    codegen::usual_arithmetic_conversion(expression1, expression2, state);

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
  state: &mut State,
) -> (Type, Vec<Token>) {
  let (type_, tokens1, tokens2) =
    codegen::usual_arithmetic_conversion(expression1, expression2, state);

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
  state: &mut State,
) -> (Type, Vec<Token>) {
  let (type_, tokens1, tokens2) =
    codegen::usual_arithmetic_conversion(expression1, expression2, state);

  let mul_macro = Macro("mul".to_string());

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
  state: &mut State,
) -> (Type, Vec<Token>) {
  let (type_, tokens1, tokens2) =
    codegen::usual_arithmetic_conversion(expression1, expression2, state);

  let div_macro = Macro("div".to_string());

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
  state: &mut State,
) -> (Type, Vec<Token>) {
  let (type_, tokens1, tokens2) =
    codegen::usual_arithmetic_conversion(expression1, expression2, state);

  let mod_macro = Macro("mod".to_string());

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
  _state: &mut State,
) -> (Type, Vec<Token>) {
  todo!()
}

fn logical_or_expression(
  _expression1: Expression,
  _expression2: Expression,
  _state: &mut State,
) -> (Type, Vec<Token>) {
  todo!()
}

fn bitwise_and_expression(
  expression1: Expression,
  expression2: Expression,
  state: &mut State,
) -> (Type, Vec<Token>) {
  let (type_, tokens1, tokens2) =
    codegen::usual_arithmetic_conversion(expression1, expression2, state);

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
  state: &mut State,
) -> (Type, Vec<Token>) {
  let (type_, tokens1, tokens2) =
    codegen::usual_arithmetic_conversion(expression1, expression2, state);

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
  state: &mut State,
) -> (Type, Vec<Token>) {
  let (type_, tokens1, tokens2) =
    codegen::usual_arithmetic_conversion(expression1, expression2, state);

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
  _state: &mut State,
) -> (Type, Vec<Token>) {
  todo!()
}

fn right_shift_expression(
  _expression1: Expression,
  _expression2: Expression,
  _state: &mut State,
) -> (Type, Vec<Token>) {
  todo!()
}

fn equal_to_expression(
  expression1: Expression,
  expression2: Expression,
  state: &mut State,
) -> (Type, Vec<Token>) {
  let (type_, tokens1, tokens2) =
    codegen::usual_arithmetic_conversion(expression1, expression2, state);

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
  state: &mut State,
) -> (Type, Vec<Token>) {
  let (type_, tokens1, tokens2) =
    codegen::usual_arithmetic_conversion(expression1, expression2, state);

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
  state: &mut State,
) -> (Type, Vec<Token>) {
  let (type_, tokens1, tokens2) =
    codegen::usual_arithmetic_conversion(expression1, expression2, state);

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
  state: &mut State,
) -> (Type, Vec<Token>) {
  let (type_, tokens1, tokens2) =
    codegen::usual_arithmetic_conversion(expression1, expression2, state);

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
  state: &mut State,
) -> (Type, Vec<Token>) {
  let (type_, tokens1, tokens2) =
    codegen::usual_arithmetic_conversion(expression1, expression2, state);

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
  state: &mut State,
) -> (Type, Vec<Token>) {
  let (type_, tokens1, tokens2) =
    codegen::usual_arithmetic_conversion(expression1, expression2, state);

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
  state: &mut State,
) -> (Type, Vec<Token>) {
  let (type1, tokens1) = codegen::expression(expression1, state);
  let (type2, tokens2, tokens3) =
    codegen::usual_arithmetic_conversion(expression2, expression3, state);

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

fn cast_expression(type_: Type, expression: Expression, state: &mut State) -> (Type, Vec<Token>) {
  let (type1, tokens1) = codegen::expression(expression, state);

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

fn integer_constant_expression(value: u8, _state: &mut State) -> (Type, Vec<Token>) {
  (
    Type::Int, // TODO assumes all integer literals are ints
    vec![Token::XXX(value)],
  )
}

fn character_constant_expression(value: char, _state: &mut State) -> (Type, Vec<Token>) {
  (
    Type::Char, // TODO character constants are `int`s in C
    vec![Token::XXX(value as u8)],
  )
}

fn string_literal_expression(value: String, state: &mut State) -> (Type, Vec<Token>) {
  use std::collections::hash_map::DefaultHasher;
  use std::hash::Hasher;

  let mut hasher = DefaultHasher::new();
  hasher.write(value.as_bytes());
  let value_hash = hasher.finish();

  let label = format!(
    "str_{}.{:X}",
    value
      .chars()
      .filter_map(|c| match c {
        'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' => Some(c),
        ' ' => Some('_'),
        _ => None,
      })
      .collect::<String>(),
    value_hash
  );

  let label = state.strings.entry(value.clone()).or_insert(label.clone());

  state
    .dependencies
    .entry(
      state
        .global
        .clone()
        .unwrap_or_else(|| panic!("Expected global")),
    )
    .or_insert(HashSet::new())
    .insert(label.clone());

  (
    Type::Int, // TODO should be `char *`
    vec![Token::LabelRef(Label::Global(label.clone()))],
  )
}

fn function_call_expression(
  name: String,
  arguments: Vec<Expression>,
  state: &mut State,
) -> (Type, Vec<Token>) {
  let call_macro = Macro("call".to_string());

  // TODO assumes all functions are globals
  let (inline, object_type) = state
    .declarations
    .get(&name)
    .unwrap_or_else(|| panic!("Declaration for object `{}` not found", name));

  let inline = *inline;

  let (return_type, parameter_types) = match object_type {
    Type::Function(return_type, parameter_types) => (return_type.clone(), parameter_types.clone()),
    _ => panic!("`{}` is not a function", name),
  };

  // TODO assumes all functions are globals
  state
    .dependencies
    .entry(
      state
        .global
        .clone()
        .unwrap_or_else(|| panic!("Expected global")),
    )
    .or_insert(HashSet::new())
    .insert(name.clone());

  (
    *return_type,
    std::iter::empty()
      .chain(
        arguments
          .into_iter()
          .zip(parameter_types.into_iter())
          .rev()
          .flat_map(|(argument, parameter_type)| {
            let (_type, tokens) =
              codegen::expression(Expression::Cast(parameter_type, Box::new(argument)), state);
            tokens
          }),
      )
      .chain(match inline {
        true => vec![Token::MacroRef(Macro(format!("{}", name.clone())))],
        false => vec![
          Token::LabelRef(Label::Global(format!("{}", name.clone()))),
          Token::MacroRef(call_macro),
        ],
      })
      .collect(),
  )
}

fn usual_arithmetic_conversion(
  expression1: Expression,
  expression2: Expression,
  state: &mut State,
) -> (Type, Vec<Token>, Vec<Token>) {
  let (type1, tokens1) = codegen::expression(expression1, state);
  let (type2, tokens2) = codegen::expression(expression2, state);
  match (type1.clone(), type2.clone()) {
    (type1, type2) if type1 == type2 => (type1, tokens1, tokens2),

    (Type::Char, Type::Int) | (Type::Int, Type::Char) => (Type::Int, tokens1, tokens2),

    _ => panic!(
      "Unimplemented usual arithmetic conversion between `{:?}` and `{:?}`",
      type1, type2
    ),
  }
}
