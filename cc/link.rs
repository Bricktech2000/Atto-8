use crate::*;
use std::collections::{BTreeMap, BTreeSet, HashSet};

#[rustfmt::skip] macro_rules! global_label { ($name:expr) => { Label::Global(format!("{}", $name)) }; }
#[rustfmt::skip] macro_rules! deps_macro { ($name:expr) => { Macro(format!("{}.deps", $name)) }; }
#[rustfmt::skip] macro_rules! def_macro { ($name:expr) => { Macro(format!("{}.def", $name)) }; }
#[rustfmt::skip] macro_rules! jmp_macro { () => { Macro(format!("jmp")) }; }
#[rustfmt::skip] macro_rules! ret_macro { () => { Macro(format!("ret")) }; }
#[rustfmt::skip] macro_rules! bcc_macro { () => { Macro(format!("bcc")) }; }
#[rustfmt::skip] macro_rules! bcs_macro { () => { Macro(format!("bcs")) }; }
#[rustfmt::skip] macro_rules! zr_macro { () => { Macro(format!("zr")) }; }
#[rustfmt::skip] macro_rules! cl_macro { () => { Macro(format!("cl")) }; }
#[rustfmt::skip] macro_rules! eq_macro { () => { Macro(format!("eq")) }; }
#[rustfmt::skip] macro_rules! call_macro { () => { Macro(format!("call")) }; }
#[rustfmt::skip] macro_rules! mul_macro { () => { Macro(format!("mul")) }; }
#[rustfmt::skip] macro_rules! div_macro { () => { Macro(format!("div")) }; }
#[rustfmt::skip] macro_rules! mod_macro { () => { Macro(format!("mod")) }; }

#[rustfmt::skip] pub(crate) use global_label;
#[rustfmt::skip] pub(crate) use deps_macro;
#[rustfmt::skip] pub(crate) use def_macro;
#[rustfmt::skip] pub(crate) use jmp_macro;
#[rustfmt::skip] pub(crate) use ret_macro;
#[rustfmt::skip] pub(crate) use bcc_macro;
#[rustfmt::skip] pub(crate) use bcs_macro;
#[rustfmt::skip] pub(crate) use zr_macro;
#[rustfmt::skip] pub(crate) use cl_macro;
#[rustfmt::skip] pub(crate) use eq_macro;
#[rustfmt::skip] pub(crate) use call_macro;
#[rustfmt::skip] pub(crate) use mul_macro;
#[rustfmt::skip] pub(crate) use div_macro;
#[rustfmt::skip] pub(crate) use mod_macro;

pub fn link(program: &TypedProgram, _errors: &mut Vec<(Pos, Error)>) -> Vec<Result<Token, String>> {
  let mut dependencies: BTreeMap<String, BTreeSet<(bool, String)>> = match program {
    TypedProgram(globals) => globals
      .into_iter()
      .filter_map(|global| match global {
        TypedGlobal::String(_label, _value) => None,
        TypedGlobal::Macro(label, statement) => Some((label.clone(), link::statement(statement))),
        TypedGlobal::Function(label, statement) => {
          Some((label.clone(), link::statement(statement)))
        }
        TypedGlobal::Assembly(_assembly) => None,
      })
      .collect(),
  };

  let labeled_globals: BTreeSet<String> = match program {
    TypedProgram(globals) => globals
      .into_iter()
      .filter_map(|global| match global {
        TypedGlobal::String(label, _value) => Some(label.clone()),
        TypedGlobal::Macro(_label, _statement) => None,
        TypedGlobal::Function(label, _statement) => Some(label.clone()),
        TypedGlobal::Assembly(_assembly) => None,
      })
      .collect(),
  };

  // brute-force reflexive transitive closure of dependencies.
  // if A depends on B and B depends on C then ensure A depends on C,
  // for all A, B, C. ensure A depends on A, for all A.
  let original_dependencies = dependencies.clone();
  for name in original_dependencies.keys() {
    let mut stack: Vec<String> = vec![name.clone()];
    let mut visited: HashSet<String> = HashSet::new();

    let deps = dependencies.get_mut(name).unwrap_or_else(|| unreachable!());
    deps.insert((labeled_globals.contains(name), name.clone())); // reflexive closure

    // depth-first iteration because recursion would be inconvenient
    while let Some(node) = stack.pop() {
      for (is_labeled, dep) in original_dependencies.get(&node).unwrap_or(&BTreeSet::new()) {
        if !visited.contains(dep) {
          stack.push(dep.clone());
          visited.insert(dep.clone());
          deps.insert((*is_labeled, dep.clone())); // transitive closure
        }
      }
    }
  }

  dependencies
    .iter()
    .flat_map(|(name, deps)| {
      std::iter::empty()
        .chain(vec![Ok(Token::MacroDef(link::deps_macro!(&name)))])
        .chain(deps.iter().filter_map(|(is_labeled, dep)| {
          is_labeled.then_some(Ok(Token::MacroRef(link::def_macro!(&dep))))
        }))
        .chain(vec![Err("".to_string())])
    })
    .collect()
}

fn statement(statement: &TypedStatement) -> BTreeSet<(bool, String)> {
  match statement {
    TypedStatement::ExpressionN0(expression) => link::expression(expression),
    TypedStatement::Compound(statements) => statements
      .into_iter()
      .flat_map(|statement| link::statement(statement))
      .collect(),
    TypedStatement::IfN1(_label, condition, if_body, else_body) => std::iter::empty()
      .chain(link::expression(condition))
      .chain(link::statement(if_body))
      .chain(
        else_body
          .as_ref()
          .map(|else_body| link::statement(&else_body))
          .unwrap_or_else(BTreeSet::new),
      )
      .collect(),
    TypedStatement::WhileN1(_label, condition, body) => std::iter::empty()
      .chain(link::expression(condition))
      .chain(link::statement(body))
      .collect(),
    TypedStatement::MacroReturnN0(_, _, expression)
    | TypedStatement::MacroReturnN1(_, _, expression)
    | TypedStatement::MacroReturnN8(_, _, expression)
    | TypedStatement::FunctionReturnN0(_, _, expression)
    | TypedStatement::FunctionReturnN1(_, _, expression)
    | TypedStatement::FunctionReturnN8(_, _, expression)
    | TypedStatement::InitLocalN0(expression)
    | TypedStatement::InitLocalN1(expression)
    | TypedStatement::InitLocalN8(expression) => match expression {
      Some(expression) => link::expression(expression),
      None => BTreeSet::new(),
    },
    TypedStatement::Assembly(_assembly) => BTreeSet::new(),
  }
}

fn expression(expression: &TypedExpression) -> BTreeSet<(bool, String)> {
  match expression {
    TypedExpression::N8Negation(expression)
    | TypedExpression::N1BitwiseComplement(expression)
    | TypedExpression::N8BitwiseComplement(expression) => link::expression(expression),

    TypedExpression::N8Addition(expression1, expression2)
    | TypedExpression::N8Subtraction(expression1, expression2)
    | TypedExpression::U8Multiplication(expression1, expression2)
    | TypedExpression::U8Division(expression1, expression2)
    | TypedExpression::U8Modulo(expression1, expression2) => std::iter::empty()
      .chain(link::expression(expression1))
      .chain(link::expression(expression2))
      .collect(),

    TypedExpression::N1EqualToN8(expression1, expression2) => std::iter::empty()
      .chain(link::expression(expression1))
      .chain(link::expression(expression2))
      .collect(),

    TypedExpression::N0CastN1(expression)
    | TypedExpression::N0CastN8(expression)
    | TypedExpression::N1CastN8(expression) => link::expression(expression),
    TypedExpression::N0Constant(_)
    | TypedExpression::N1Constant(_)
    | TypedExpression::N8Constant(_)
    | TypedExpression::N8GetLocal(_)
    | TypedExpression::N8AddrLocal(_) => BTreeSet::new(),
    TypedExpression::N8GetGlobal(label) | TypedExpression::N8AddrGlobal(label) => {
      std::iter::once((true, label.clone())).collect()
    }
    TypedExpression::N0MacroCall(label, parameters)
    | TypedExpression::N1MacroCall(label, parameters)
    | TypedExpression::N8MacroCall(label, parameters) => parameters
      .into_iter()
      .flat_map(|expression| link::expression(expression))
      .chain(std::iter::once((false, label.clone())))
      .collect(),
    TypedExpression::N0FunctionCall(designator, parameters)
    | TypedExpression::N1FunctionCall(designator, parameters)
    | TypedExpression::N8FunctionCall(designator, parameters) => parameters
      .into_iter()
      .flat_map(|expression| link::expression(expression))
      .chain(link::expression(designator))
      .collect(),
  }
}
