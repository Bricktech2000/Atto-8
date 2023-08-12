fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 3 {
    println!("Asm: Usage: asm <assembly source file> <memory image file>");
    std::process::exit(1);
  }

  let mut errors: Vec<(Pos, Error)> = vec![];
  let memory_image_file = &args[2];
  let assembly_source_file: File = File {
    path: args[1].clone(),
  };

  let preprocessed: String = preprocess(assembly_source_file, &mut errors, None);
  let tokens: Vec<(Pos, Token)> = tokenize(preprocessed, &mut errors);
  let instructions: Vec<(Pos, Instruction)> = assemble(tokens, &mut errors, "main");
  let bytes: Vec<(Pos, u8)> = codegen(instructions, &mut errors);

  match errors[..] {
    [] => {
      std::fs::write::<&String, [u8; MEM_SIZE]>(
        memory_image_file,
        bytes
          .iter()
          .map(|(_, b)| *b)
          .collect::<Vec<u8>>()
          .try_into()
          .unwrap(),
      )
      .unwrap();

      println!("Asm: Done");
    }
    _ => {
      let errors = errors
        .iter()
        .map(|error| format!("Asm: Error: {}: {}", error.0, error.1))
        .collect::<Vec<String>>()
        .join("\n");

      println!("{}", errors);
      std::process::exit(1);
    }
  }
}

const MEM_SIZE: usize = 0x100;

#[derive(Clone, Eq, PartialEq, Hash)]
struct Label {
  scope_uid: Option<usize>,
  identifier: String,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Macro {
  identifier: String,
}

#[derive(Clone, Eq, PartialEq)]
struct Error(String);

#[derive(Clone, Eq, PartialEq)]
struct Pos(String, usize);

#[derive(Clone, Eq, PartialEq)]
struct File {
  path: String,
}

#[derive(Clone, Eq, PartialEq)]
enum Token {
  LabelDef(Label),
  LabelRef(Label),
  MacroDef(Macro),
  MacroRef(Macro),
  AtConst,
  AtDyn,
  AtOrg,
  AtErr,
  DDD(u8),
  XXX(u8),
  Add,
  AdS(u8),
  Sub,
  SuS(u8),
  Iff,
  IfS(u8),
  Rot,
  RoS(u8),
  Orr,
  OrS(u8),
  And,
  AnS(u8),
  Xor,
  XoS(u8),
  Xnd,
  XnS(u8),
  Inc,
  Dec,
  Neg,
  Shl,
  Shr,
  Not,
  Buf,
  LdO(u8),
  StO(u8),
  Lda,
  Sta,
  Ldi,
  Sti,
  Lds,
  Sts,
  Nop,
  Clc,
  Sec,
  Flc,
  Swp,
  Pop,
}

#[derive(Clone, Eq, PartialEq)]
enum Instruction {
  Psh(u8),
  Phn(u8),
  Add(u8),
  Sub(u8),
  Iff(u8),
  Rot(u8),
  Orr(u8),
  And(u8),
  Xor(u8),
  Xnd(u8),
  Inc,
  Dec,
  Neg,
  Shl,
  Shr,
  Not,
  Buf,
  Ldo(u8),
  Sto(u8),
  Lda,
  Sta,
  Ldi,
  Sti,
  Lds,
  Sts,
  Nop,
  Clc,
  Sec,
  Flc,
  Swp,
  Pop,
  Raw(u8),
}

fn preprocess(file: File, errors: &mut Vec<(Pos, Error)>, scope: Option<&str>) -> String {
  // remove comments and resolve includes

  use std::path::Path;
  let source = match std::fs::read_to_string(&file.path) {
    Ok(data) => data,
    Err(_) => {
      errors.push((
        Pos(scope.unwrap_or("[bootstrap]").to_string(), 0),
        Error(format!("Unable to read file `{}`", file)),
      ));
      "".to_string()
    }
  };

  let source: String = source
    .lines()
    .map(|line| line.strip_suffix("#").unwrap_or(line))
    .map(|line| line.split("# ").next().unwrap_or(line))
    .map(|line| match line.find("@ ") {
      Some(i) => {
        line[..i].to_owned()
          + preprocess(
            File {
              path: Path::new(&file.path)
                .parent()
                .unwrap()
                .join(&line[i..]["@ ".len()..])
                .to_str()
                .unwrap()
                .to_string(),
            },
            errors,
            Some(&format!("{}", file)),
          )
          .as_str()
      }
      None => line.to_string(),
    })
    .collect::<Vec<_>>()
    .join("\n");

  source
}

fn tokenize(source: String, errors: &mut Vec<(Pos, Error)>) -> Vec<(Pos, Token)> {
  // tokenize to valid tokens. tokens might be invalid instructions

  fn parse_hex(literal: &str, errors: &mut Vec<(Pos, Error)>, position: &Pos) -> u8 {
    use std::num::IntErrorKind::*;
    match (literal.to_uppercase() == literal)
      .then_some(literal)
      .ok_or(InvalidDigit)
      .and(u8::from_str_radix(literal, 16).map_err(|e| e.kind().clone()))
    {
      Ok(value) => value,
      Err(kind) => {
        errors.push((
          position.clone(),
          Error(match kind {
            Empty => format!("Invalid empty hexadecimal literal `x{}`", literal),
            InvalidDigit => format!("Invalid digits in hexadecimal literal `x{}`", literal),
            NegOverflow | PosOverflow => format!("Out-of-range hexadecimal literal `x{}`", literal),
            _ => panic!("Unexpected error parsing hexadecimal literal"),
          }),
        ));
        0x00
      }
    }
  }

  let tokens: Vec<&str> = source.split_whitespace().collect();

  let tokens: Vec<(Pos, Token)> = tokens
    .into_iter()
    .enumerate()
    .map(|(index, token)| {
      let position = Pos("[token stream]".to_string(), index);

      let token = match token {
        _ if token.ends_with(":") => Token::LabelDef(Label {
          scope_uid: None,
          identifier: token[..token.len() - 1].to_string(),
        }),
        _ if token.starts_with(":") => Token::LabelRef(Label {
          scope_uid: None,
          identifier: token[1..].to_string(),
        }),
        _ if token.ends_with(".") => Token::LabelDef(Label {
          scope_uid: Some(0),
          identifier: token[..token.len() - 1].to_string(),
        }),
        _ if token.starts_with(".") => Token::LabelRef(Label {
          scope_uid: Some(0),
          identifier: token[1..].to_string(),
        }),
        _ if token.ends_with("!") => Token::MacroDef(Macro {
          identifier: token[..token.len() - 1].to_string(),
        }),
        _ if token.starts_with("!") => Token::MacroRef(Macro {
          identifier: token[1..].to_string(),
        }),
        "@const" => Token::AtConst,
        "@dyn" => Token::AtDyn,
        "@org" => Token::AtOrg,
        "@err" => Token::AtErr,
        "add" => Token::Add,
        "sub" => Token::Sub,
        "iff" => Token::Iff,
        "rot" => Token::Rot,
        "orr" => Token::Orr,
        "and" => Token::And,
        "xor" => Token::Xor,
        "xnd" => Token::Xnd,
        "inc" => Token::Inc,
        "dec" => Token::Dec,
        "neg" => Token::Neg,
        "shl" => Token::Shl,
        "shr" => Token::Shr,
        "not" => Token::Not,
        "buf" => Token::Buf,
        "lda" => Token::Lda,
        "sta" => Token::Sta,
        "ldi" => Token::Ldi,
        "sti" => Token::Sti,
        "lds" => Token::Lds,
        "sts" => Token::Sts,
        "nop" => Token::Nop,
        "clc" => Token::Clc,
        "sec" => Token::Sec,
        "flc" => Token::Flc,
        "swp" => Token::Swp,
        "pop" => Token::Pop,
        _ if token.starts_with("ad") => Token::AdS(parse_hex(&token[2..], errors, &position)),
        _ if token.starts_with("su") => Token::SuS(parse_hex(&token[2..], errors, &position)),
        _ if token.starts_with("if") => Token::IfS(parse_hex(&token[2..], errors, &position)),
        _ if token.starts_with("ro") => Token::RoS(parse_hex(&token[2..], errors, &position)),
        _ if token.starts_with("or") => Token::OrS(parse_hex(&token[2..], errors, &position)),
        _ if token.starts_with("an") => Token::AnS(parse_hex(&token[2..], errors, &position)),
        _ if token.starts_with("xo") => Token::XoS(parse_hex(&token[2..], errors, &position)),
        _ if token.starts_with("xn") => Token::XnS(parse_hex(&token[2..], errors, &position)),
        _ if token.starts_with("ld") => Token::LdO(parse_hex(&token[2..], errors, &position)),
        _ if token.starts_with("st") => Token::StO(parse_hex(&token[2..], errors, &position)),
        _ if token.starts_with("d") => Token::DDD(parse_hex(&token[1..], errors, &position)),
        _ if token.starts_with("x") => Token::XXX(parse_hex(&token[1..], errors, &position)),
        _ => {
          errors.push((
            position.clone(),
            Error(format!("Unexpected token `{}`", token)),
          ));
          Token::Nop
        }
      };

      (position, token)
    })
    .collect();

  tokens
}

fn assemble(
  tokens: Vec<(Pos, Token)>,
  errors: &mut Vec<(Pos, Error)>,
  entry_point: &str,
) -> Vec<(Pos, Instruction)> {
  // resolve macros recursively from `entry_point`

  use std::collections::HashMap;

  let mut macro_definitions: HashMap<Macro, Vec<(Pos, Token)>> = HashMap::new();
  let mut current_macro: Option<Macro> = None;

  for token in tokens.into_iter() {
    match token.1 {
      Token::MacroDef(macro_) => {
        if macro_definitions.contains_key(&macro_) {
          errors.push((
            token.0.clone(),
            Error(format!("Duplicate macro definition `{}`", macro_)),
          ));
        }
        current_macro = Some(macro_.clone());
        macro_definitions.entry(macro_).or_insert(vec![]);
      }
      _ => match current_macro
        .as_ref()
        .and_then(|macro_| macro_definitions.get_mut(&macro_))
      {
        Some(macro_tokens) => macro_tokens.push((
          Pos(
            format!("{}", current_macro.as_ref().unwrap()),
            macro_tokens.len(),
          ),
          token.1,
        )),
        None => errors.push((
          token.0,
          Error(format!("Orphan token `{}` encountered", token.1)),
        )),
      },
    }
  }

  let entry_point = vec![(
    Pos("[bootstrap]".to_string(), 0),
    Token::MacroRef(Macro {
      identifier: entry_point.to_string(),
    }),
  )];
  let mut scope_uid: usize = 1;
  let mut parent_macros: Vec<Macro> = vec![];
  let tokens: Vec<(Pos, Token)> = expand_macros(
    &entry_point,
    &mut scope_uid,
    &mut parent_macros,
    &macro_definitions,
    errors,
  );

  fn expand_macros(
    tokens: &Vec<(Pos, Token)>,
    scope_uid: &mut usize,
    parent_macros: &mut Vec<Macro>,
    macro_definitions: &HashMap<Macro, Vec<(Pos, Token)>>,
    errors: &mut Vec<(Pos, Error)>,
  ) -> Vec<(Pos, Token)> {
    tokens
      .into_iter()
      .flat_map(|token| match &token.1 {
        Token::MacroRef(macro_) => {
          if parent_macros.contains(macro_) {
            errors.push((
              token.0.clone(),
              Error(format!(
                "Macro self-reference {} -> `{}`",
                parent_macros
                  .iter()
                  .map(|macro_| format!("`{}`", macro_))
                  .collect::<Vec<String>>()
                  .join(" -> "),
                macro_
              )),
            ));
            vec![]
          } else {
            let tokens = match macro_definitions.get(macro_) {
              Some(tokens) => tokens.clone(),
              None => {
                errors.push((
                  token.0.clone(),
                  Error(format!("Definition for macro `{}` not found", macro_)),
                ));
                vec![]
              }
            };

            let tokens = tokens
              .into_iter()
              .map(|token| match token.1 {
                Token::LabelDef(Label {
                  scope_uid: Some(_),
                  identifier,
                }) => (
                  token.0,
                  Token::LabelDef(Label {
                    scope_uid: Some(*scope_uid),
                    identifier,
                  }),
                ),
                Token::LabelRef(Label {
                  scope_uid: Some(_),
                  identifier,
                }) => (
                  token.0,
                  Token::LabelRef(Label {
                    scope_uid: Some(*scope_uid),
                    identifier,
                  }),
                ),
                _ => token,
              })
              .collect();

            *scope_uid += 1;
            parent_macros.push(macro_.clone());
            let expanded = expand_macros(
              &tokens,
              scope_uid,
              parent_macros,
              &macro_definitions,
              errors,
            );
            parent_macros.pop();
            expanded
          }
        }
        Token::AtErr => {
          errors.push((
            token.0.clone(),
            Error(format!(
              "`{}` directive encountered during macro expansion",
              token.1
            )),
          ));
          vec![]
        }
        _ => vec![token.clone()],
      })
      .collect()
  }

  #[allow(dead_code)]
  fn assert_imm(imm: u8, errors: &mut Vec<(Pos, Error)>, position: &Pos) -> u8 {
    match imm {
      0b00000000..=0b01111111 => imm,
      _ => {
        errors.push((
          position.clone(),
          Error(format!("Invalid IMM operand `{:02X}`", imm)),
        ));
        0b00000000
      }
    }
  }

  fn assert_size(size: u8, errors: &mut Vec<(Pos, Error)>, position: &Pos) -> u8 {
    match size {
      0x01 | 0x02 | 0x04 | 0x08 => size,
      _ => {
        errors.push((
          position.clone(),
          Error(format!("Invalid SIZE operand `{:02X}`", size)),
        ));
        0x01
      }
    }
  }

  fn assert_ofst(ofst: u8, errors: &mut Vec<(Pos, Error)>, position: &Pos) -> u8 {
    match ofst {
      0b00000000..=0b00001111 => ofst,
      _ => {
        errors.push((
          position.clone(),
          Error(format!("Invalid OFST operand `{:02X}`", ofst)),
        ));
        0b00000000
      }
    }
  }

  // turn assembly tokens into roots, an intermediate representation. roots correspond to valid instructions

  #[derive(Clone, Eq, PartialEq)]
  enum Root {
    Instruction(Instruction),
    Node(Node),
    LabelDef(Label),
    Const,
    Dyn(Option<Instruction>),
    Org(Option<Node>),
  }

  #[derive(Clone, Eq, PartialEq, Hash)]
  enum Node {
    LabelRef(Label),
    Value(u8),
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Rot(Box<Node>, Box<Node>),
    Orr(Box<Node>, Box<Node>),
    And(Box<Node>, Box<Node>),
    Xor(Box<Node>, Box<Node>),
    Xnd(Box<Node>, Box<Node>),
    Shl(Box<Node>),
    Shr(Box<Node>),
    Not(Box<Node>),
  }

  let roots: Vec<(Pos, Root)> = tokens
    .into_iter()
    .map(|token| {
      let position = token.0;
      let token = match token.1 {
        Token::LabelDef(label) => Root::LabelDef(label),
        Token::LabelRef(label) => Root::Node(Node::LabelRef(label)),
        Token::MacroDef(_) => panic!("Macro definition found in intermediate representation"),
        Token::MacroRef(_) => panic!("Macro reference found in intermediate representation"),
        Token::AtConst => Root::Const,
        Token::AtDyn => Root::Dyn(None),
        Token::AtOrg => Root::Org(None),
        Token::AtErr => panic!("Error directive found in intermediate representation"),
        Token::XXX(value) => Root::Node(Node::Value(value)),
        Token::Add => Root::Instruction(Instruction::Add(assert_size(0x01, errors, &position))),
        Token::AdS(size) => {
          Root::Instruction(Instruction::Add(assert_size(size, errors, &position)))
        }
        Token::Sub => Root::Instruction(Instruction::Sub(assert_size(0x01, errors, &position))),
        Token::SuS(size) => {
          Root::Instruction(Instruction::Sub(assert_size(size, errors, &position)))
        }
        Token::Iff => Root::Instruction(Instruction::Iff(assert_size(0x01, errors, &position))),
        Token::IfS(size) => {
          Root::Instruction(Instruction::Iff(assert_size(size, errors, &position)))
        }
        Token::Rot => Root::Instruction(Instruction::Rot(assert_size(0x01, errors, &position))),
        Token::RoS(size) => {
          Root::Instruction(Instruction::Rot(assert_size(size, errors, &position)))
        }
        Token::Orr => Root::Instruction(Instruction::Orr(assert_size(0x01, errors, &position))),
        Token::OrS(size) => {
          Root::Instruction(Instruction::Orr(assert_size(size, errors, &position)))
        }
        Token::And => Root::Instruction(Instruction::And(assert_size(0x01, errors, &position))),
        Token::AnS(size) => {
          Root::Instruction(Instruction::And(assert_size(size, errors, &position)))
        }
        Token::Xor => Root::Instruction(Instruction::Xor(assert_size(0x01, errors, &position))),
        Token::XoS(size) => {
          Root::Instruction(Instruction::Xor(assert_size(size, errors, &position)))
        }
        Token::Xnd => Root::Instruction(Instruction::Xnd(assert_size(0x01, errors, &position))),
        Token::XnS(size) => {
          Root::Instruction(Instruction::Xnd(assert_size(size, errors, &position)))
        }
        Token::Inc => Root::Instruction(Instruction::Inc),
        Token::Dec => Root::Instruction(Instruction::Dec),
        Token::Neg => Root::Instruction(Instruction::Neg),
        Token::Shl => Root::Instruction(Instruction::Shl),
        Token::Shr => Root::Instruction(Instruction::Shr),
        Token::Not => Root::Instruction(Instruction::Not),
        Token::Buf => Root::Instruction(Instruction::Buf),
        Token::LdO(ofst) => {
          Root::Instruction(Instruction::Ldo(assert_ofst(ofst, errors, &position)))
        }
        Token::StO(ofst) => {
          Root::Instruction(Instruction::Sto(assert_ofst(ofst, errors, &position)))
        }
        Token::Lda => Root::Instruction(Instruction::Lda),
        Token::Sta => Root::Instruction(Instruction::Sta),
        Token::Ldi => Root::Instruction(Instruction::Ldi),
        Token::Sti => Root::Instruction(Instruction::Sti),
        Token::Lds => Root::Instruction(Instruction::Lds),
        Token::Sts => Root::Instruction(Instruction::Sts),
        Token::Nop => Root::Instruction(Instruction::Nop),
        Token::Clc => Root::Instruction(Instruction::Clc),
        Token::Sec => Root::Instruction(Instruction::Sec),
        Token::Flc => Root::Instruction(Instruction::Flc),
        Token::Swp => Root::Instruction(Instruction::Swp),
        Token::Pop => Root::Instruction(Instruction::Pop),
        Token::DDD(value) => Root::Instruction(Instruction::Raw(value)),
      };

      (position, token)
    })
    .collect();

  // build a tree of nodes representing everything we can compute at compile time
  // this removes redundant instructions and makes macros usable

  // a convenience function to replace slice patterns within a vector
  fn match_replace<const N: usize>(
    roots: &Vec<(Pos, Root)>,
    replacer: fn(&[Root; N]) -> Option<Vec<Root>>,
  ) -> Vec<(Pos, Root)> {
    if roots.len() < N {
      return roots.clone();
    }

    let mut output: Vec<(Pos, Root)> = vec![];

    let mut skip_next_n_roots = 0;
    for window in roots.windows(N) {
      if skip_next_n_roots > 0 {
        skip_next_n_roots -= 1;
      } else {
        match replacer(
          window
            .iter()
            .map(|(_, root)| root.clone())
            .collect::<Vec<Root>>()
            .as_slice()
            .try_into()
            .unwrap(),
        ) {
          Some(roots) => {
            output.extend(
              roots
                .into_iter()
                .map(|root| (window[0].0.clone(), root))
                .collect::<Vec<(Pos, Root)>>(),
            );
            skip_next_n_roots = N - 1;
          }
          None => output.push(window[0].clone()),
        }
      }
    }
    output.extend(
      roots
        .iter()
        .skip(1 + roots.len() - N + skip_next_n_roots)
        .cloned(),
    );

    output
  }

  // convenience function, returns `true` if the given root effectively just pushes a value onto the stack
  fn just_pushes(root: &Root) -> bool {
    match root {
      Root::Instruction(Instruction::Ldo(_)) => true,
      Root::Node(_) => true,
      _ => false,
    }
  }

  let mut roots = roots;

  // optimize as much as possible into `Node`s for assembly-time evaluation

  let mut last_roots = vec![];
  while roots != last_roots {
    last_roots = roots.clone();
    // println!("roots: {:?}\nlen: {}", roots, roots.len());

    roots = match_replace(&roots, |window| match window {
      [Root::Node(node), Root::Const] => Some(vec![Root::Node(node.clone())]),
      [Root::Instruction(instruction), Root::Dyn(None)] => {
        Some(vec![Root::Dyn(Some(instruction.clone()))])
      }
      [Root::Node(Node::Value(value)), Root::Dyn(None)] => Some(
        make_push_instruction(value.clone(), &Pos("".to_string(), 0))
          .iter()
          .map(|(_, instruction)| Root::Dyn(Some(instruction.clone())))
          .collect(),
      ),
      [Root::Node(node), Root::Org(None)] => Some(vec![Root::Org(Some(node.clone()))]),
      _ => None,
    });

    roots = match_replace(&roots, |window| match window {
      [Root::Instruction(Instruction::Nop)] => Some(vec![]),
      _ => None,
    });

    roots =
      match_replace(&roots, |window| match window {
        [Root::Node(node), Root::Instruction(Instruction::Xor(0x01))]
          if eval(&node, &HashMap::new()) == Ok(0x00) =>
        {
          Some(vec![Root::Instruction(Instruction::Buf)])
        }
        [Root::Node(node), Root::Instruction(Instruction::Add(0x01))]
          if eval(&node, &HashMap::new()) == Ok(0x00) =>
        {
          Some(vec![])
        }
        [Root::Node(node), Root::Instruction(Instruction::Add(0x01))]
          if eval(&node, &HashMap::new()) == Ok(0x01) =>
        {
          Some(vec![Root::Instruction(Instruction::Inc)])
        }
        [Root::Node(node), Root::Instruction(Instruction::Sub(0x01))]
          if eval(&node, &HashMap::new()) == Ok(0x00) =>
        {
          Some(vec![])
        }
        [Root::Node(node), Root::Instruction(Instruction::Sub(0x01))]
          if eval(&node, &HashMap::new()) == Ok(0x01) =>
        {
          Some(vec![Root::Instruction(Instruction::Dec)])
        }
        [Root::Node(node), Root::Instruction(Instruction::Inc)] => Some(vec![Root::Node(
          Node::Add(Box::new(Node::Value(1)), Box::new(node.clone())),
        )]),
        [Root::Node(node), Root::Instruction(Instruction::Dec)] => Some(vec![Root::Node(
          Node::Sub(Box::new(Node::Value(1)), Box::new(node.clone())),
        )]),
        [Root::Node(node), Root::Instruction(Instruction::Neg)] => Some(vec![Root::Node(
          Node::Sub(Box::new(node.clone()), Box::new(Node::Value(0))),
        )]),
        [Root::Instruction(Instruction::Neg), Root::Instruction(Instruction::Neg)] => Some(vec![]),
        [Root::Node(node), Root::Instruction(Instruction::Shl)] => {
          Some(vec![Root::Node(Node::Shl(Box::new(node.clone())))])
        }
        [Root::Node(node), Root::Instruction(Instruction::Shr)] => {
          Some(vec![Root::Node(Node::Shr(Box::new(node.clone())))])
        }
        [Root::Node(node), Root::Instruction(Instruction::Not)] => {
          Some(vec![Root::Node(Node::Not(Box::new(node.clone())))])
        }
        [Root::Instruction(Instruction::Not), Root::Instruction(Instruction::Not)] => {
          Some(vec![Root::Instruction(Instruction::Buf)])
        }
        [Root::Node(node), Root::Instruction(Instruction::Buf)] => {
          Some(vec![Root::Node(node.clone())])
        }
        [Root::Instruction(Instruction::Buf), Root::Instruction(Instruction::Buf)] => {
          Some(vec![Root::Instruction(Instruction::Buf)])
        }
        [Root::Node(node), Root::Instruction(Instruction::Ldo(0x00))] => {
          Some(vec![Root::Node(node.clone()), Root::Node(node.clone())])
        }
        [Root::Instruction(Instruction::Clc), Root::Instruction(Instruction::Clc)] => {
          Some(vec![Root::Instruction(Instruction::Clc)])
        }
        [Root::Instruction(Instruction::Sec), Root::Instruction(Instruction::Sec)] => {
          Some(vec![Root::Instruction(Instruction::Sec)])
        }
        [Root::Instruction(Instruction::Flc), Root::Instruction(Instruction::Flc)] => Some(vec![]),
        [Root::Instruction(Instruction::Swp), Root::Instruction(Instruction::Swp)] => Some(vec![]),
        [Root::Node(_), Root::Instruction(Instruction::Pop)] => Some(vec![]),
        _ => None,
      });

    roots = match_replace(&roots, |window| match window {
      [Root::Node(node1), Root::Node(node2), Root::Instruction(Instruction::Add(0x01))] => {
        Some(vec![Root::Node(Node::Add(
          Box::new(node2.clone()),
          Box::new(node1.clone()),
        ))])
      }
      [Root::Node(node1), Root::Node(node2), Root::Instruction(Instruction::Sub(0x01))] => {
        Some(vec![Root::Node(Node::Sub(
          Box::new(node2.clone()),
          Box::new(node1.clone()),
        ))])
      }
      [Root::Node(node1), Root::Node(node2), Root::Instruction(Instruction::Rot(0x01))] => {
        Some(vec![Root::Node(Node::Rot(
          Box::new(node2.clone()),
          Box::new(node1.clone()),
        ))])
      }
      [Root::Node(node1), Root::Node(node2), Root::Instruction(Instruction::Orr(0x01))] => {
        Some(vec![Root::Node(Node::Orr(
          Box::new(node2.clone()),
          Box::new(node1.clone()),
        ))])
      }
      [Root::Node(node1), Root::Node(node2), Root::Instruction(Instruction::And(0x01))] => {
        Some(vec![Root::Node(Node::And(
          Box::new(node2.clone()),
          Box::new(node1.clone()),
        ))])
      }
      [Root::Node(node1), Root::Node(node2), Root::Instruction(Instruction::Xor(0x01))] => {
        Some(vec![Root::Node(Node::Xor(
          Box::new(node2.clone()),
          Box::new(node1.clone()),
        ))])
      }
      [Root::Instruction(Instruction::Swp), Root::Instruction(Instruction::Inc), Root::Instruction(Instruction::Swp)] => {
        Some(vec![
          Root::Node(Node::Value(0x01)),
          Root::Instruction(Instruction::Add(0x02)),
        ])
      }
      [Root::Instruction(Instruction::Swp), Root::Instruction(Instruction::Dec), Root::Instruction(Instruction::Swp)] => {
        Some(vec![
          Root::Node(Node::Value(0x01)),
          Root::Instruction(Instruction::Sub(0x02)),
        ])
      }
      [Root::Node(node1), Root::Node(node2), Root::Instruction(Instruction::Xnd(0x01))] => {
        Some(vec![Root::Node(Node::Xnd(
          Box::new(node2.clone()),
          Box::new(node1.clone()),
        ))])
      }
      [Root::Node(node1), root, Root::Instruction(Instruction::Ldo(0x01))] if just_pushes(root) => {
        Some(vec![
          Root::Node(node1.clone()),
          root.clone(),
          Root::Node(node1.clone()),
        ])
      }
      [Root::Node(node1), Root::Node(node2), Root::Instruction(Instruction::Swp)] => {
        Some(vec![Root::Node(node2.clone()), Root::Node(node1.clone())])
      }
      [Root::Instruction(Instruction::Ldo(ofst)), Root::Node(node2), Root::Instruction(Instruction::Swp)]
        if *ofst < 0b00001111 =>
      {
        Some(vec![
          Root::Node(node2.clone()),
          Root::Instruction(Instruction::Ldo(*ofst + 1)),
        ])
      }
      [Root::Node(node1), Root::Instruction(Instruction::Ldo(ofst)), Root::Instruction(Instruction::Swp)]
        if *ofst > 0b00000000 =>
      {
        Some(vec![
          Root::Instruction(Instruction::Ldo(*ofst - 1)),
          Root::Node(node1.clone()),
        ])
      }
      [Root::Instruction(Instruction::Ldo(ofst1)), Root::Instruction(Instruction::Ldo(ofst2)), Root::Instruction(Instruction::Swp)]
        if *ofst1 < 0b00001111 && *ofst2 > 0b00000000 =>
      {
        Some(vec![
          Root::Instruction(Instruction::Ldo(*ofst2 - 1)),
          Root::Instruction(Instruction::Ldo(*ofst1 + 1)),
        ])
      }
      [Root::Instruction(Instruction::Pop), Root::Node(node), Root::Instruction(Instruction::Sto(0x07))]
      | [Root::Node(node), Root::Instruction(Instruction::Sto(0x08)), Root::Instruction(Instruction::Pop)]
        if eval(&node, &HashMap::new()) == Ok(0x00) =>
      {
        Some(vec![Root::Instruction(Instruction::Xnd(0x08))])
      }
      [Root::Instruction(Instruction::Pop), Root::Node(node), Root::Instruction(Instruction::Sto(0x03))]
      | [Root::Node(node), Root::Instruction(Instruction::Sto(0x04)), Root::Instruction(Instruction::Pop)]
        if eval(&node, &HashMap::new()) == Ok(0x00) =>
      {
        Some(vec![Root::Instruction(Instruction::Xnd(0x04))])
      }
      [Root::Instruction(Instruction::Pop), Root::Node(node), Root::Instruction(Instruction::Sto(0x01))]
      | [Root::Node(node), Root::Instruction(Instruction::Sto(0x02)), Root::Instruction(Instruction::Pop)]
        if eval(&node, &HashMap::new()) == Ok(0x00) =>
      {
        Some(vec![Root::Instruction(Instruction::Xnd(0x02))])
      }
      [Root::Instruction(Instruction::Pop), Root::Instruction(Instruction::Pop), Root::Node(node)]
      | [Root::Instruction(Instruction::Pop), Root::Node(node), Root::Instruction(Instruction::Sto(0x00))]
      | [Root::Node(node), Root::Instruction(Instruction::Sto(0x01)), Root::Instruction(Instruction::Pop)]
        if eval(&node, &HashMap::new()) == Ok(0x00) =>
      {
        Some(vec![Root::Instruction(Instruction::Xnd(0x01))])
      }
      [Root::Instruction(Instruction::Pop), Root::Instruction(Instruction::Pop), Root::Node(node)]
      | [Root::Instruction(Instruction::Pop), Root::Node(node), Root::Instruction(Instruction::Sto(0x00))]
      | [Root::Node(node), Root::Instruction(Instruction::Sto(0x01)), Root::Instruction(Instruction::Pop)]
        if eval(&node, &HashMap::new()) == Ok(0x01) =>
      {
        Some(vec![
          Root::Instruction(Instruction::Xnd(0x01)),
          Root::Instruction(Instruction::Shl),
        ])
      }
      [Root::Instruction(Instruction::Pop), Root::Instruction(Instruction::Pop), Root::Node(node)]
      | [Root::Instruction(Instruction::Pop), Root::Node(node), Root::Instruction(Instruction::Sto(0x00))]
      | [Root::Node(node), Root::Instruction(Instruction::Sto(0x01)), Root::Instruction(Instruction::Pop)]
        if eval(&node, &HashMap::new()) == Ok(0x80) =>
      {
        Some(vec![
          Root::Instruction(Instruction::Xnd(0x01)),
          Root::Instruction(Instruction::Shr),
        ])
      }
      [Root::Instruction(Instruction::Pop), Root::Instruction(Instruction::Pop), Root::Node(node)]
      | [Root::Instruction(Instruction::Pop), Root::Node(node), Root::Instruction(Instruction::Sto(0x00))]
      | [Root::Node(node), Root::Instruction(Instruction::Sto(0x01)), Root::Instruction(Instruction::Pop)]
        if eval(&node, &HashMap::new()) == Ok(0xFF) =>
      {
        Some(vec![
          Root::Instruction(Instruction::Xnd(0x01)),
          Root::Instruction(Instruction::Not),
        ])
      }
      _ => None,
    });

    roots = match_replace(&roots, |window| match window {
      [Root::Node(node1), Root::Instruction(Instruction::Add(size1)), Root::Node(node2), Root::Instruction(Instruction::Add(size2))]
        if size1 == size2 =>
      {
        Some(vec![
          Root::Node(Node::Add(Box::new(node2.clone()), Box::new(node1.clone()))),
          Root::Instruction(Instruction::Add(*size1)),
        ])
      }
      [Root::Node(node1), Root::Instruction(Instruction::Sub(size1)), Root::Node(node2), Root::Instruction(Instruction::Sub(size2))]
        if size1 == size2 =>
      {
        Some(vec![
          Root::Node(Node::Add(Box::new(node2.clone()), Box::new(node1.clone()))),
          Root::Instruction(Instruction::Sub(*size1)),
        ])
      }
      [Root::Node(node1), Root::Instruction(Instruction::Rot(size1)), Root::Node(node2), Root::Instruction(Instruction::Rot(size2))]
        if size1 == size2 =>
      {
        Some(vec![
          Root::Node(Node::Add(Box::new(node2.clone()), Box::new(node1.clone()))),
          Root::Instruction(Instruction::Rot(*size1)),
        ])
      }
      [Root::Node(node1), Root::Instruction(Instruction::Orr(size1)), Root::Node(node2), Root::Instruction(Instruction::Orr(size2))]
        if size1 == size2 =>
      {
        Some(vec![
          Root::Node(Node::Orr(Box::new(node2.clone()), Box::new(node1.clone()))),
          Root::Instruction(Instruction::Orr(*size1)),
        ])
      }
      [Root::Node(node1), Root::Instruction(Instruction::And(size1)), Root::Node(node2), Root::Instruction(Instruction::And(size2))]
        if size1 == size2 =>
      {
        Some(vec![
          Root::Node(Node::And(Box::new(node2.clone()), Box::new(node1.clone()))),
          Root::Instruction(Instruction::And(*size1)),
        ])
      }
      [Root::Node(node1), Root::Instruction(Instruction::Xor(size1)), Root::Node(node2), Root::Instruction(Instruction::Xor(size2))]
        if size1 == size2 =>
      {
        Some(vec![
          Root::Node(Node::Xor(Box::new(node2.clone()), Box::new(node1.clone()))),
          Root::Instruction(Instruction::Xor(*size1)),
        ])
      }
      [Root::Node(node1), Root::Instruction(Instruction::Xnd(size1)), Root::Node(node2), Root::Instruction(Instruction::Xnd(size2))]
        if size1 == size2 =>
      {
        Some(vec![
          Root::Node(Node::Xnd(Box::new(node2.clone()), Box::new(node1.clone()))),
          Root::Instruction(Instruction::Xnd(*size1)),
        ])
      }
      [Root::Node(node1), Root::Instruction(Instruction::And(size1)), Root::Node(node2), Root::Instruction(Instruction::Orr(size2))]
        if size1 == size2
          && eval(&node1, &HashMap::new())
            .and_then(|value1| eval(&node2, &HashMap::new()).map(|value2| value1 ^ value2 == 0xFF))
            .unwrap_or(false) =>
      {
        Some(vec![
          Root::Node(node2.clone()),
          Root::Instruction(Instruction::Orr(*size1)),
        ])
      }
      [Root::Node(node1), Root::Instruction(Instruction::Orr(size1)), Root::Node(node2), Root::Instruction(Instruction::And(size2))]
        if size1 == size2
          && eval(&node1, &HashMap::new())
            .and_then(|value1| eval(&node2, &HashMap::new()).map(|value2| value1 ^ value2 == 0xFF))
            .unwrap_or(false) =>
      {
        Some(vec![
          Root::Node(node2.clone()),
          Root::Instruction(Instruction::And(*size1)),
        ])
      }
      [Root::Node(node1), root1, root2, Root::Instruction(Instruction::Ldo(0x02))]
        if just_pushes(root1) && just_pushes(root2) =>
      {
        Some(vec![
          Root::Node(node1.clone()),
          root1.clone(),
          root2.clone(),
          Root::Node(node1.clone()),
        ])
      }
      _ => None,
    });
  }

  // optimize duplicate `Node`s (pushing them might take up two bytes) into `Ldo`s (always take up one byte)

  let mut last_roots = vec![];
  while roots != last_roots {
    last_roots = roots.clone();

    roots = match_replace(&roots, |window| match window {
      [Root::Node(node1), Root::Node(node2)] if node1 == node2 => Some(vec![
        Root::Node(node1.clone()),
        Root::Instruction(Instruction::Ldo(0x00)),
      ]),
      [Root::Instruction(Instruction::Swp), Root::Instruction(Instruction::Pop)] => {
        Some(vec![Root::Instruction(Instruction::Sto(0x00))])
      }
      _ => None,
    });

    roots = match_replace(&roots, |window| match window {
      [Root::Node(node1), root, Root::Node(node2)] if node1 == node2 && just_pushes(root) => {
        Some(vec![
          Root::Node(node1.clone()),
          root.clone(),
          Root::Instruction(Instruction::Ldo(0x01)),
        ])
      }
      _ => None,
    });

    roots = match_replace(&roots, |window| match window {
      [Root::Node(node1), root1, root2, Root::Node(node2)]
        if node1 == node2 && just_pushes(root1) && just_pushes(root2) =>
      {
        Some(vec![
          Root::Node(node1.clone()),
          root1.clone(),
          root2.clone(),
          Root::Instruction(Instruction::Ldo(0x02)),
        ])
      }
      _ => None,
    });
  }

  // assemble roots into instructions by computing the value of every node and resolving labels

  fn eval(node: &Node, label_definitions: &HashMap<Label, u8>) -> Result<u8, Label> {
    Ok(match node {
      Node::LabelRef(label) => *label_definitions.get(label).ok_or(label.clone())?,
      Node::Value(value) => *value,
      Node::Add(node1, node2) => {
        eval(node2, label_definitions)?.wrapping_add(eval(node1, label_definitions)?)
      }
      Node::Sub(node1, node2) => {
        eval(node2, label_definitions)?.wrapping_sub(eval(node1, label_definitions)?)
      }
      Node::Rot(node1, node2) => {
        let a = eval(node1, label_definitions)? as u16;
        let b = eval(node2, label_definitions)? as u16;
        let shifted = (b as u16) << a % 8;
        (shifted & 0xFF) as u8 | (shifted >> 8) as u8
      }
      Node::Orr(node1, node2) => eval(node2, label_definitions)? | eval(node1, label_definitions)?,
      Node::And(node1, node2) => eval(node2, label_definitions)? & eval(node1, label_definitions)?,
      Node::Xor(node1, node2) => eval(node2, label_definitions)? ^ eval(node1, label_definitions)?,
      Node::Xnd(_, _) => 0,
      Node::Shl(node) => eval(node, label_definitions)?.wrapping_shl(1),
      Node::Shr(node) => eval(node, label_definitions)?.wrapping_shl(1),
      Node::Not(node) => !eval(node, label_definitions)?,
    })
  }

  fn make_push_instruction(value: u8, position: &Pos) -> Vec<(Pos, Instruction)> {
    // the `Psh` instruction allows us to push arbitrary 7-bit immediates onto the stack.
    // we then optionally use `Neg` and `Inc` to get the ability to push arbitrary 8-bit
    // values. we also use `Phn` as a shorthand when possible.

    if value & 0b11110000 == 0b11110000 {
      vec![(position.clone(), Instruction::Phn(value & 0b00001111))]
    } else if value == 0b10000000 {
      vec![
        (position.clone(), Instruction::Psh(0b01111111)),
        (position.clone(), Instruction::Inc),
      ]
    } else {
      match value & 0b10000000 {
        0b00000000 => vec![(position.clone(), Instruction::Psh(value & 0b01111111))],
        0b10000000 => vec![
          (position.clone(), Instruction::Psh(value.wrapping_neg())),
          (position.clone(), Instruction::Neg),
        ],
        _ => unreachable!(),
      }
    }
  }

  // if every label a node depends on could be resolved, we can replace it with a value.
  // if not, start by allocating one byte for pushing the node later. if pushing the node turns
  // out to require more than one byte, iteratively `'bruteforce` allocation sizes until we
  // find one that works. repeat for every node.

  let mut instructions: Vec<(Pos, Instruction)>;
  let mut allocation_sizes: HashMap<Node, usize> = HashMap::new();

  'bruteforce: loop {
    let mut location_counter: u8 = 0;
    let mut label_definitions: HashMap<Label, u8> = HashMap::new();
    let mut unevaluated_nodes: HashMap<u8, (Pos, Node)> = HashMap::new();

    instructions = roots
      .clone()
      .into_iter()
      .flat_map(|root| match root.1 {
        Root::Instruction(instruction) | Root::Dyn(Some(instruction)) => {
          let instructions_ = vec![(root.0, instruction)];
          location_counter = location_counter.wrapping_add(instructions_.len() as u8);
          instructions_
        }

        Root::Node(node) => match eval(&node, &label_definitions) {
          Ok(value) => {
            let instructions_ = make_push_instruction(value, &root.0);
            location_counter = location_counter.wrapping_add(instructions_.len() as u8);
            instructions_
          }
          Err(_) => {
            let instructions_ = vec![
              (root.0.clone(), Instruction::Nop);
              allocation_sizes.get(&node).copied().unwrap_or(1)
            ];
            unevaluated_nodes.insert(location_counter, (root.0, node));
            location_counter = location_counter.wrapping_add(instructions_.len() as u8);
            instructions_
          }
        },

        Root::LabelDef(Label {
          scope_uid: Some(0),
          identifier: _,
        }) => panic!("Local label has no scope specified"),

        Root::LabelDef(label) => {
          if label_definitions.contains_key(&label) {
            errors.push((
              root.0,
              Error(format!("Duplicate label definition `{}`", label)),
            ));
          }
          label_definitions.insert(label, location_counter);
          vec![]
        }

        Root::Org(Some(node)) => match eval(&node, &label_definitions) {
          Ok(value) => {
            if value >= location_counter {
              let difference = value - location_counter;
              location_counter += difference;
              vec![(root.0, Instruction::Raw(0x00)); difference as usize]
            } else {
              errors.push((
                root.0,
                Error(format!(
                  "`{}` cannot move location counter backward from `{}` to `{}`",
                  Token::AtOrg,
                  location_counter,
                  value
                )),
              ));
              vec![]
            }
          }
          Err(label) => {
            errors.push((
              root.0,
              Error(format!(
                "`{}` argument contains currently unresolved label `{}`",
                Token::AtOrg,
                label
              )),
            ));
            vec![]
          }
        },

        Root::Org(None) => {
          errors.push((
            root.0,
            Error(format!(
              "`{}` argument could not be reduced to a constant expression",
              Token::AtOrg,
            )),
          ));
          vec![]
        }

        Root::Const => {
          errors.push((
            root.0,
            Error(format!(
              "`{}` argument could not be reduced to a constant expression",
              Token::AtConst,
            )),
          ));
          vec![]
        }

        Root::Dyn(None) => {
          errors.push((
            root.0,
            Error(format!(
              "`{}` argument could not be reduced to an instruction",
              Token::AtDyn,
            )),
          ));
          vec![]
        }
      })
      .collect();

    // abort brute force if errors were found
    if errors.len() > 0 {
      break 'bruteforce;
    }

    // poke into `instructions` and evaluate the nodes that couldn't be evaluated before
    'poke: {
      for (location_counter, node) in unevaluated_nodes.iter() {
        let value = match eval(&node.1, &label_definitions) {
          Ok(value) => value,
          Err(label) => {
            errors.push((
              node.0.clone(),
              Error(format!("Definition for label `{}` not found", label)),
            ));
            0x00
          }
        };

        // if the evaluated node doesn't fit in the allocated memory, note down the right amount of
        // memory to allocate on the next iteration of `'bruteforce` and try again

        let instructions_ = make_push_instruction(value, &node.0);
        if instructions_.len() > allocation_sizes.get(&node.1).copied().unwrap_or(1) {
          allocation_sizes.insert(node.1.clone(), instructions_.len());
          break 'poke;
        }

        for (index, instruction) in instructions_.into_iter().enumerate() {
          instructions[*location_counter as usize + index] = instruction.clone();
        }
      }

      // all unevaluated nodes have been evaluated, break out of the bruteforce loop
      break 'bruteforce;
    }
  }

  instructions
}

fn codegen(
  instructions: Vec<(Pos, Instruction)>,
  errors: &mut Vec<(Pos, Error)>,
) -> Vec<(Pos, u8)> {
  fn encode_imm(imm: u8) -> u8 {
    match imm {
      0b00000000..=0b01111111 => imm,
      _ => panic!("Invalid IMM in codegen stage"),
    }
  }

  fn encode_size(size: u8) -> u8 {
    match size {
      0x01 => 0x00,
      0x02 => 0x01,
      0x04 => 0x02,
      0x08 => 0x03,
      _ => panic!("Invalid SIZE in codegen stage"),
    }
  }

  fn encode_ofst(ofst: u8) -> u8 {
    match ofst {
      0b00000000..=0b00001111 => ofst,
      _ => panic!("Invalid OFST in codegen stage"),
    }
  }

  // codegen instructions into bytes and sanity-check operands

  let bytes: Vec<(Pos, u8)> = instructions
    .into_iter()
    .map(|instruction| {
      let position = instruction.0;
      let instruction = match instruction.1 {
        Instruction::Psh(imm) => 0b00000000 | encode_imm(imm),
        Instruction::Add(size) => 0b10000000 | encode_size(size),
        Instruction::Sub(size) => 0b10000100 | encode_size(size),
        Instruction::Iff(size) => 0b10010000 | encode_size(size),
        Instruction::Rot(size) => 0b10010100 | encode_size(size),
        Instruction::Orr(size) => 0b10100000 | encode_size(size),
        Instruction::And(size) => 0b10100100 | encode_size(size),
        Instruction::Xor(size) => 0b10101000 | encode_size(size),
        Instruction::Xnd(size) => 0b10101100 | encode_size(size),
        Instruction::Inc => 0b10110000,
        Instruction::Dec => 0b10110001,
        Instruction::Neg => 0b10110010,
        Instruction::Shl => 0b10110100,
        Instruction::Shr => 0b10110101,
        Instruction::Not => 0b10110110,
        Instruction::Buf => 0b10110111,
        Instruction::Ldo(ofst) => 0b11000000 | encode_ofst(ofst),
        Instruction::Sto(ofst) => 0b11010000 | encode_ofst(ofst),
        Instruction::Lda => 0b11100000,
        Instruction::Sta => 0b11100001,
        Instruction::Ldi => 0b11100010,
        Instruction::Sti => 0b11100011,
        Instruction::Lds => 0b11100100,
        Instruction::Sts => 0b11100101,
        Instruction::Nop => 0b11101000,
        Instruction::Clc => 0b11101001,
        Instruction::Sec => 0b11101010,
        Instruction::Flc => 0b11101011,
        Instruction::Swp => 0b11101100,
        Instruction::Pop => 0b11101101,
        Instruction::Phn(imm) => 0b11110000 | encode_ofst(imm),
        Instruction::Raw(value) => value,
      };

      (position, instruction)
    })
    .collect();

  let mut bytes = bytes;
  let position = Pos("[codegen]".to_string(), 0);

  if bytes.len() > MEM_SIZE {
    errors.push((
      position,
      Error(format!(
        "Program size `{:02X}` exceeds available memory of size `{:02X}`",
        bytes.len(),
        MEM_SIZE
      )),
    ));
  } else {
    bytes.extend(vec![(position, 0x00); MEM_SIZE - bytes.len()]);
  }

  bytes
}

impl std::fmt::Display for Label {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self.scope_uid {
      Some(_) => write!(f, ".{}", self.identifier),
      None => write!(f, ":{}", self.identifier),
    }
  }
}

impl std::fmt::Display for Macro {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "!{}", self.identifier)
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl std::fmt::Display for Pos {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}#{}", self.0, self.1)
  }
}

impl std::fmt::Display for File {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    use path_clean::PathClean;
    use std::path::Path;
    write!(
      f,
      "@{}",
      Path::new(&self.path).clean().to_str().unwrap().to_string()
    )
  }
}

impl std::fmt::Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Token::LabelDef(label) => write!(f, "{}", label),
      Token::LabelRef(label) => write!(f, "{}", label),
      Token::MacroDef(macro_) => write!(f, "{}", macro_),
      Token::MacroRef(macro_) => write!(f, "{}", macro_),
      Token::AtConst => write!(f, "@const"),
      Token::AtDyn => write!(f, "@dyn"),
      Token::AtOrg => write!(f, "@org"),
      Token::AtErr => write!(f, "@err"),
      Token::DDD(value) => write!(f, "d{:02X}", value),
      Token::XXX(value) => write!(f, "x{:02X}", value),
      Token::Add => write!(f, "add"),
      Token::AdS(size) => write!(f, "ad{:01X}", size),
      Token::Sub => write!(f, "sub"),
      Token::SuS(size) => write!(f, "su{:01X}", size),
      Token::Iff => write!(f, "iff"),
      Token::IfS(size) => write!(f, "if{:01X}", size),
      Token::Rot => write!(f, "rot"),
      Token::RoS(size) => write!(f, "ro{:01X}", size),
      Token::Orr => write!(f, "orr"),
      Token::OrS(size) => write!(f, "or{:01X}", size),
      Token::And => write!(f, "and"),
      Token::AnS(size) => write!(f, "an{:01X}", size),
      Token::Xor => write!(f, "xor"),
      Token::XoS(size) => write!(f, "xo{:01X}", size),
      Token::Xnd => write!(f, "xnd"),
      Token::XnS(size) => write!(f, "xn{:01X}", size),
      Token::Inc => write!(f, "inc"),
      Token::Dec => write!(f, "dec"),
      Token::Neg => write!(f, "neg"),
      Token::Shl => write!(f, "shl"),
      Token::Shr => write!(f, "shr"),
      Token::Not => write!(f, "not"),
      Token::Buf => write!(f, "buf"),
      Token::LdO(ofst) => write!(f, "ld{:01X}", ofst),
      Token::StO(ofst) => write!(f, "st{:01X}", ofst),
      Token::Lda => write!(f, "lda"),
      Token::Sta => write!(f, "sta"),
      Token::Ldi => write!(f, "ldi"),
      Token::Sti => write!(f, "sti"),
      Token::Lds => write!(f, "lds"),
      Token::Sts => write!(f, "sts"),
      Token::Nop => write!(f, "nop"),
      Token::Clc => write!(f, "clc"),
      Token::Sec => write!(f, "sec"),
      Token::Flc => write!(f, "flc"),
      Token::Swp => write!(f, "swp"),
      Token::Pop => write!(f, "pop"),
    }
  }
}
