#[path = "../misc/common/common.rs"]
mod common;
use common::*;
use std::collections::HashMap;

mod codegen;
mod parse;
mod preprocess;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 3 {
    println!("CC: Usage: cc <C source file> <assembly output file>");
    std::process::exit(1);
  }

  let c_source_file = File(args[1].clone());
  let assembly_output_file = &args[2];

  let preprocessed: String = preprocess::preprocess(c_source_file, &mut HashMap::new())
    .unwrap_or_else(|e| {
      println!("CC: Error: {}", e);
      std::process::exit(1);
    });

  let program: Program = parse::parse(preprocessed).unwrap_or_else(|e| {
    println!("CC: Error: {}", e);
    std::process::exit(1);
  });

  // println!("{:#?}", program);

  let tokens: Vec<Token> = codegen::codegen(program, "main").unwrap_or_else(|e| {
    println!("CC: Error: {}", e);
    std::process::exit(1);
  });

  let mnemonics: Vec<Mnemonic> = tokens.into_iter().map(common::token_to_mnemonic).collect();

  let assembly: String = mnemonics
    .into_iter()
    .map(|mnemonic| mnemonic.to_string())
    .collect::<Vec<String>>()
    .join(" ");

  let assembly = format!("# Generated by CC\n\n{}", assembly);

  std::fs::write(assembly_output_file, assembly).unwrap();

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
  Function(Box<Type>, Vec<Object>),
  Pointer(Box<Type>),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Program(Vec<Global>);

#[derive(Clone, PartialEq, Debug)]
pub enum Global {
  FunctionDefinition(FunctionDefinition),
  AsmStatement(String),
}

#[derive(Clone, PartialEq, Debug)]
pub struct FunctionDefinition(Object, Vec<Object>, Vec<Statement>);

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
  Identifier(String),
  FunctionCall(String),
}

#[derive(Clone, PartialEq, Debug)]
pub enum Statement {
  Expression(Expression),
  Return(Expression),
  Asm(String),
}
