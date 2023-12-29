#[path = "../misc/common/common.rs"]
mod common;
use common::*;
use std::collections::HashMap;

mod codegen;
mod parse;
mod preprocess;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 3 {
    println!("CC: Usage: cc <C source files> <assembly output file>");
    std::process::exit(1);
  }

  let mut errors: Vec<(Pos, Error)> = vec![];
  let c_source_files = args[1..args.len() - 1].to_vec();
  let assembly_output_file = &args[args.len() - 1];

  let preprocessed: String = c_source_files
    .into_iter()
    .map(|c_source_file| File(c_source_file))
    .map(|c_source_file| {
      format!(
        "asm {{ # translation {} }}\n{}\n",
        c_source_file.clone(),
        preprocess::preprocess(c_source_file, &mut HashMap::new(), &mut errors, None)
      )
    })
    .collect::<Vec<String>>()
    .join("\n");

  // println!("CC: Preprocessed: {:#?}", preprocessed);

  let program: Program = parse::parse(preprocessed, &mut errors);

  // println!("CC: Program: {:#?}", program);

  let tokens: Vec<Result<Token, String>> = codegen::codegen(program, &mut errors);

  let mnemonics: Vec<Result<Mnemonic, String>> = tokens
    .into_iter()
    .map(|token| token.map(common::token_to_mnemonic))
    .collect();

  let assembly: String = mnemonics
    .into_iter()
    .map(|mnemonic| match mnemonic {
      Ok(mnemonic) => format!("{} ", mnemonic),
      Err(assembly) => format!("{}\n", assembly),
    })
    .collect::<String>()
    .replace(" \n", "\n");

  // println!("CC: Assembly: {:#?}", assembly);

  match errors[..] {
    [] => std::fs::write(assembly_output_file, assembly).unwrap(),
    _ => {
      let errors = errors
        .iter()
        .map(|(pos, error)| format!("CC: Error: {}: {}", pos, error))
        .collect::<Vec<String>>()
        .join("\n");

      println!("{}", errors);
      std::process::exit(1);
    }
  }

  println!("CC: Done");
}

#[derive(Clone, PartialEq, Debug)]
pub struct Object(Type, String);

#[derive(Clone, PartialEq, Debug)]
pub enum Type {
  Void,
  Bool,
  Char,
  Short,
  Int,
  Long,
  LongLong,
  // TODO float, double
  Array(Box<Type>),
  Structure(Vec<Object>),
  Union(Vec<Object>),
  Function(Box<Type>, Vec<Type>, bool),
  Pointer(Box<Type>),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Program(Vec<Global>);

#[derive(Clone, PartialEq, Debug)]
pub enum Global {
  FunctionDeclaration(FunctionDeclaration),
  FunctionDefinition(FunctionDefinition),
  AsmStatement(String),
}

#[derive(Clone, PartialEq, Debug)]
pub struct FunctionDeclaration(bool, Object, Vec<Object>, bool);

#[derive(Clone, PartialEq, Debug)]
pub struct FunctionDefinition(bool, Object, Vec<Object>, bool, Statement);

#[derive(Clone, PartialEq, Debug)]
pub enum Expression {
  Negation(Box<Expression>),
  LogicalNegation(Box<Expression>),
  BitwiseComplement(Box<Expression>),

  Addition(Box<Expression>, Box<Expression>),
  Subtraction(Box<Expression>, Box<Expression>),
  Multiplication(Box<Expression>, Box<Expression>),
  Division(Box<Expression>, Box<Expression>),
  Modulo(Box<Expression>, Box<Expression>),
  LogicalAnd(Box<Expression>, Box<Expression>),
  LogicalOr(Box<Expression>, Box<Expression>),
  BitwiseAnd(Box<Expression>, Box<Expression>),
  BitwiseExclusiveOr(Box<Expression>, Box<Expression>),
  BitwiseInclusiveOr(Box<Expression>, Box<Expression>),
  RightShift(Box<Expression>, Box<Expression>),
  LeftShift(Box<Expression>, Box<Expression>),

  EqualTo(Box<Expression>, Box<Expression>),
  NotEqualTo(Box<Expression>, Box<Expression>),
  LessThan(Box<Expression>, Box<Expression>),
  LessThanOrEqualTo(Box<Expression>, Box<Expression>),
  GreaterThan(Box<Expression>, Box<Expression>),
  GreaterThanOrEqualTo(Box<Expression>, Box<Expression>),

  Conditional(Box<Expression>, Box<Expression>, Box<Expression>),

  Cast(Type, Box<Expression>),
  IntegerConstant(u8),
  CharacterConstant(char),
  StringLiteral(String),
  Identifier(String),
  FunctionCall(String, Vec<Expression>),
}

#[derive(Clone, PartialEq, Debug)]
pub enum Statement {
  Expression(Expression),
  Compound(Vec<Statement>),
  While(Expression, Box<Statement>),
  Return(Option<Expression>),
  Asm(String),
}
