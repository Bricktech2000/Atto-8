#[path = "../misc/common/common.rs"]
mod common;
use common::*;
use std::collections::HashMap;

mod codegen;
mod link;
mod optimize;
mod parse;
mod preprocess;
mod typecheck;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 3 {
    println!("CC: Usage: cc <C source files> <assembly output file>");
    std::process::exit(1);
  }

  let mut errors: Vec<(Pos, Error)> = vec![];
  let c_source_files = args[1..args.len() - 1].to_vec();
  let assembly_output_file = &args[args.len() - 1];

  let preprocessed: Vec<String> = c_source_files
    .into_iter()
    .map(|c_source_file| File(c_source_file.into()))
    .map(|c_source_file| {
      [
        format!("\nasm {{ # translation {} }}\n", c_source_file.clone()),
        preprocess::preprocess(c_source_file, &mut HashMap::new(), &mut errors, None),
      ]
    })
    .flatten()
    .collect();

  // println!("CC: Preprocessed: {:#?}", preprocessed);

  let parsed: Vec<Program> = preprocessed
    .into_iter()
    .map(|preprocessed| parse::parse(preprocessed, &mut errors))
    .collect();

  // println!("CC: Parsed: {:#?}", parsed);

  let typechecked: Vec<TypedProgram> = parsed
    .into_iter()
    .map(|program| typecheck::typecheck(program, &mut errors))
    .collect();

  // println!("CC: Typechecked: {:#?}", typechecked);

  let optimized: Vec<TypedProgram> = typechecked
    .into_iter()
    .map(|typed_program| optimize::optimize(typed_program, &mut errors))
    .collect();

  // println!("CC: Optimized: {:#?}", optimized);

  let linked: Vec<Result<Token, String>> = std::iter::empty()
    .chain([Err(format!("# dependency graph"))])
    .chain(link::link(
      &TypedProgram(optimized.iter().cloned().flat_map(|p| p.0).collect()),
      &mut errors,
    ))
    .collect();

  // println!("CC: Linked: {:#?}", linked);

  let codegened: Vec<Vec<Result<Token, String>>> = optimized
    .into_iter()
    .map(|typed_program| codegen::codegen(typed_program, &mut errors))
    .collect();

  // println!("CC: Codegened: {:#?}", codegened);

  let tokens: Vec<Result<Token, String>> = codegened.into_iter().flatten().chain(linked).collect();

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

  let assembly = format!("# Generated by CC\n\n{}", assembly);

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

// abstract syntax tree

#[derive(Clone, PartialEq, Debug)]
pub struct Object(Type, String);

#[derive(Clone, PartialEq, Debug)]
pub enum Type {
  Void,
  Bool,
  Char,
  SignedChar,
  UnsignedChar,
  Short,
  UnsignedShort,
  Int,
  UnsignedInt,
  Long,
  UnsignedLong,
  LongLong,
  UnsignedLongLong,
  Array(Box<Type>),
  Structure(Vec<Object>),
  Union(Vec<Object>),
  Enumeration(Vec<String>),
  Macro(Box<Type>, String, Vec<Type>, bool), // not using `Box<Object>` because pattern matching
  Function(Box<Type>, Vec<Type>, bool),
  Pointer(Box<Type>),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Program(Vec<Global>);

#[derive(Clone, PartialEq, Debug)]
pub enum Global {
  FunctionDeclaration(bool, Object, Vec<Object>, bool),
  FunctionDefinition(bool, Object, Vec<Object>, bool, Statement),
  GlobalDeclaration(Object),
  GlobalDefinition(Object, Expression),
  GlobalAssembly(String),
}

#[derive(Clone, PartialEq, Debug)]
pub enum Expression {
  AddressOf(Box<Expression>),
  Dereference(Box<Expression>),
  Positive(Box<Expression>),
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
  LeftShift(Box<Expression>, Box<Expression>),
  RightShift(Box<Expression>, Box<Expression>),

  EqualTo(Box<Expression>, Box<Expression>),
  NotEqualTo(Box<Expression>, Box<Expression>),
  LessThan(Box<Expression>, Box<Expression>),
  LessThanOrEqualTo(Box<Expression>, Box<Expression>),
  GreaterThan(Box<Expression>, Box<Expression>),
  GreaterThanOrEqualTo(Box<Expression>, Box<Expression>),

  Conditional(Box<Expression>, Box<Expression>, Box<Expression>),

  Comma(Box<Expression>, Box<Expression>),
  Cast(Type, Box<Expression>),
  IntegerConstant(u8),
  CharacterConstant(char),
  StringLiteral(String),
  Identifier(String),
  Subscript(Box<Expression>, Box<Expression>),
  FunctionCall(Box<Expression>, Vec<Expression>),
}

#[derive(Clone, PartialEq, Debug)]
pub enum Statement {
  Expression(Option<Expression>), // expression (`None` for null statement)
  Compound(Vec<Statement>),
  If(Expression, Box<Statement>, Option<Box<Statement>>), // condition, if_body, else_body
  While(Expression, Box<Statement>, bool),                // condition, body, is_do_while
  Break,
  Continue,
  Return(Option<Expression>),
  Declaration(Object, Option<Expression>),
  Assembly(String),
}

// typed intermediate representation

#[derive(Clone, PartialEq, Debug)]
pub struct TypedProgram(Vec<TypedGlobal>);

#[derive(Clone, PartialEq, Debug)]
pub enum TypedGlobal {
  Data(String, Vec<TypedExpression>),
  Macro(String, TypedStatement, TypedStatement), // label, body, return_template
  Function(String, TypedStatement, TypedStatement), // label, body, return_template
  Assembly(String),
}

#[derive(Clone, PartialEq, Debug)]
pub enum TypedExpression {
  N1DereferenceN8(Box<TypedExpression>),
  N8DereferenceN8(Box<TypedExpression>),
  N1BitwiseComplement(Box<TypedExpression>),
  N8BitwiseComplement(Box<TypedExpression>),

  N8Addition(Box<TypedExpression>, Box<TypedExpression>),
  N8Subtraction(Box<TypedExpression>, Box<TypedExpression>),
  N8Multiplication(Box<TypedExpression>, Box<TypedExpression>),
  U8Division(Box<TypedExpression>, Box<TypedExpression>),
  U8Modulo(Box<TypedExpression>, Box<TypedExpression>),
  N8BitwiseAnd(Box<TypedExpression>, Box<TypedExpression>),
  N8BitwiseInclusiveOr(Box<TypedExpression>, Box<TypedExpression>),
  N8BitwiseExclusiveOr(Box<TypedExpression>, Box<TypedExpression>),

  N1EqualToN8(Box<TypedExpression>, Box<TypedExpression>),
  N1LessThanU8(Box<TypedExpression>, Box<TypedExpression>),
  N1LessThanI8(Box<TypedExpression>, Box<TypedExpression>),

  N0SecondN0N0(Box<TypedExpression>, Box<TypedExpression>),
  N1SecondN0N1(Box<TypedExpression>, Box<TypedExpression>),
  N8SecondN0N8(Box<TypedExpression>, Box<TypedExpression>),
  N0CastN1(Box<TypedExpression>), // bitwise truncation
  N0CastN8(Box<TypedExpression>), // bitwise truncation
  N1CastN8(Box<TypedExpression>), // bitwise truncation
  N8CastN1(Box<TypedExpression>), // bitwise extension
  N0Constant(()),
  N1Constant(bool),
  N8Constant(u8),
  N8LoadLocal(usize), // offset (from last local)
  N8AddrLocal(usize), // offset (from last local)
  N8LoadGlobal(String),
  N8AddrGlobal(String),
  N0MacroCall(String, Vec<TypedExpression>),
  N1MacroCall(String, Vec<TypedExpression>),
  N8MacroCall(String, Vec<TypedExpression>),
  N0FunctionCall(Box<TypedExpression>, Vec<TypedExpression>),
  N1FunctionCall(Box<TypedExpression>, Vec<TypedExpression>),
  N8FunctionCall(Box<TypedExpression>, Vec<TypedExpression>),
}

#[derive(Clone, PartialEq, Debug)]
pub enum TypedStatement {
  ExpressionN0(TypedExpression),
  Compound(Vec<TypedStatement>),

  IfN1(
    String,
    TypedExpression,
    Box<TypedStatement>,
    Option<Box<TypedStatement>>,
  ), // label, condition, if_body, else_body
  WhileN1(String, TypedExpression, Box<TypedStatement>, bool), // label, condition, body, is_do_while

  Break(String, usize),                                    // label, locals_size
  Continue(String, usize),                                 // label, locals_size
  MacroReturnN0(usize, usize, Option<TypedExpression>), // parameters_size, locals_size, return_value
  MacroReturnN1(usize, usize, Option<TypedExpression>), // parameters_size, locals_size, return_value
  MacroReturnN8(usize, usize, Option<TypedExpression>), // parameters_size, locals_size, return_value
  FunctionReturnN0(usize, usize, Option<TypedExpression>), // parameters_size, locals_size, return_value
  FunctionReturnN1(usize, usize, Option<TypedExpression>), // parameters_size, locals_size, return_value
  FunctionReturnN8(usize, usize, Option<TypedExpression>), // parameters_size, locals_size, return_value

  InitLocalN0(Option<TypedExpression>),
  InitLocalN1(Option<TypedExpression>),
  InitLocalN8(Option<TypedExpression>),
  UninitLocalN0,
  UninitLocalN1,
  UninitLocalN8,

  Assembly(String),
}

impl std::fmt::Display for Type {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    fn format_object_list(objects: &Vec<Object>) -> String {
      objects
        .iter()
        .map(|Object(r#type, name)| format!("{} {}", r#type, name))
        .collect::<Vec<String>>()
        .join(", ")
    }

    fn format_type_list(types: &Vec<Type>) -> String {
      types
        .iter()
        .map(|r#type| format!("{}", r#type))
        .collect::<Vec<String>>()
        .join(", ")
    }

    fn format_param_type_list(params: &Vec<Type>, is_variadic: bool) -> String {
      format!(
        "{}{}",
        match params[..] {
          [] => format!("void"),
          _ => format_type_list(params),
        },
        if is_variadic { ", ..." } else { "" }
      )
    }

    match self {
      Type::Void => write!(f, "void"),
      Type::Bool => write!(f, "_Bool"),
      Type::Char => write!(f, "char"),
      Type::SignedChar => write!(f, "signed char"),
      Type::UnsignedChar => write!(f, "unsigned char"),
      Type::Short => write!(f, "short"),
      Type::UnsignedShort => write!(f, "unsigned short"),
      Type::Int => write!(f, "int"),
      Type::UnsignedInt => write!(f, "unsigned int"),
      Type::Long => write!(f, "long"),
      Type::UnsignedLong => write!(f, "unsigned long"),
      Type::LongLong => write!(f, "long long"),
      Type::UnsignedLongLong => write!(f, "unsigned long long"),
      Type::Array(r#type) => write!(f, "{} []", r#type),
      Type::Structure(objects) => write!(f, "struct {{ {} }}", format_object_list(objects)),
      Type::Union(objects) => write!(f, "union {{ {} }}", format_object_list(objects)),
      Type::Enumeration(constants) => write!(f, "enum {{ {} }}", constants.join(", ")),
      Type::Macro(return_type, name, parameter_types, is_variadic) => write!(
        f,
        "{} {}({})",
        return_type,
        name,
        format_param_type_list(parameter_types, *is_variadic),
      ),
      Type::Function(return_type, parameter_types, is_variadic) => write!(
        f,
        "{}({})",
        return_type,
        format_param_type_list(parameter_types, *is_variadic),
      ),
      Type::Pointer(r#type) => write!(f, "{} *", r#type),
    }
  }
}

pub fn c_quote(bytes: &[u8], quote: char) -> String {
  // quotes and escapes a byte slice into a C-compatible string literal or character constant
  // the output shall be parsable either by `parse::string_literal` or by `parse::character_constant`

  std::iter::empty()
    .chain([quote.to_string()])
    .chain(bytes.iter().map(|&byte| match byte {
      byte if byte as char == quote => format!("\\{}", byte as char),
      b'\\' => "\\\\".to_string(),
      b'\x07' => "\\a".to_string(),
      b'\x08' => "\\b".to_string(),
      b'\x0C' => "\\f".to_string(),
      b'\n' => "\\n".to_string(),
      b'\r' => "\\r".to_string(),
      b'\t' => "\\t".to_string(),
      b'\x0B' => "\\v".to_string(),
      b' '..=b'~' => format!("{}", byte as char),
      b'\0' => "\\0".to_string(),
      byte => format!("\\x{:02x}", byte),
    }))
    .chain([quote.to_string()])
    .collect()
}
