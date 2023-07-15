use std::collections::HashMap;

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
      std::fs::write(
        memory_image_file,
        bytes.iter().map(|(_, b)| *b).collect::<Vec<u8>>(),
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
    .map(|line| line.split("#").next().unwrap())
    .map(|line| match line.find("@ ") {
      Some(i) => {
        line[..i].to_owned()
          + preprocess(
            File {
              path: Path::new(&file.path)
                .parent()
                .unwrap()
                .join(&line[i..][2..])
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
    match u8::from_str_radix(literal, 16) {
      Ok(value) => value,
      Err(e) => {
        match e.kind() {
          InvalidDigit => errors.push((
            position.clone(),
            Error(format!(
              "Invalid digits in hexadecimal literal `{}`",
              literal
            )),
          )),
          Empty => errors.push((
            position.clone(),
            Error(format!("Invalid empty hexadecimal literal `{}`", literal)),
          )),
          NegOverflow | PosOverflow => errors.push((
            position.clone(),
            Error(format!("Hexadecimal literal `{}` out of range", literal)),
          )),
          _ => panic!("Unexpected error parsing hexadecimal literal"),
        };
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

  let mut macro_definitions: HashMap<Macro, Vec<(Pos, Token)>> = HashMap::new();
  let mut current_macro: Option<Macro> = None;

  for token in tokens.into_iter() {
    match token.1 {
      Token::MacroDef(macro_) => {
        if macro_definitions.contains_key(&macro_) {
          errors.push((
            token.0.clone(),
            Error(format!("Macro `{}` has already been defined", macro_)),
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
        _ => vec![token.clone()],
      })
      .collect()
  }

  fn assert_immediate(immediate: u8, errors: &mut Vec<(Pos, Error)>, position: &Pos) -> u8 {
    match immediate {
      0b00000000..=0b11111111 => immediate,
      #[allow(unreachable_patterns)]
      _ => {
        errors.push((
          position.clone(),
          Error(format!("Invalid immediate operand `{:02X}`", immediate)),
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
          Error(format!("Invalid size operand `{:02X}`", size)),
        ));
        0x01
      }
    }
  }

  fn assert_offset(offset: u8, errors: &mut Vec<(Pos, Error)>, position: &Pos) -> u8 {
    match offset {
      0b00000000..=0b00001111 => offset,
      _ => {
        errors.push((
          position.clone(),
          Error(format!("Invalid offset operand `{:02X}`", offset)),
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

  #[derive(Clone, Eq, PartialEq)]
  enum Node {
    LabelRef(Label),
    Immediate(u8),
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
        Token::XXX(immediate) => Root::Node(Node::Immediate(assert_immediate(
          immediate, errors, &position,
        ))),
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
        Token::LdO(offset) => {
          Root::Instruction(Instruction::Ldo(assert_offset(offset, errors, &position)))
        }
        Token::StO(offset) => {
          Root::Instruction(Instruction::Sto(assert_offset(offset, errors, &position)))
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
        Token::DDD(immediate) => Root::Instruction(Instruction::Raw(immediate)),
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

  // returns `true` if the given root effectively just pushes a value onto the stack
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
      [Root::Node(Node::Immediate(immediate)), Root::Dyn(None)] => Some(
        make_push_instruction(immediate.clone(), &Pos("".to_string(), 0))
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
        [Root::Node(Node::Immediate(0x01)), Root::Instruction(Instruction::Add(0x01))] => {
          Some(vec![Root::Instruction(Instruction::Inc)])
        }
        [Root::Node(Node::Immediate(0x01)), Root::Instruction(Instruction::Sub(0x01))] => {
          Some(vec![Root::Instruction(Instruction::Dec)])
        }
        [Root::Node(node), Root::Instruction(Instruction::Inc)] => Some(vec![Root::Node(
          Node::Add(Box::new(Node::Immediate(1)), Box::new(node.clone())),
        )]),
        [Root::Node(node), Root::Instruction(Instruction::Dec)] => Some(vec![Root::Node(
          Node::Sub(Box::new(Node::Immediate(1)), Box::new(node.clone())),
        )]),
        [Root::Node(node), Root::Instruction(Instruction::Neg)] => Some(vec![Root::Node(
          Node::Sub(Box::new(node.clone()), Box::new(Node::Immediate(0))),
        )]),
        [Root::Node(node), Root::Instruction(Instruction::Shl)] => {
          Some(vec![Root::Node(Node::Shl(Box::new(node.clone())))])
        }
        [Root::Node(node), Root::Instruction(Instruction::Shr)] => {
          Some(vec![Root::Node(Node::Shr(Box::new(node.clone())))])
        }
        [Root::Node(node), Root::Instruction(Instruction::Not)] => {
          Some(vec![Root::Node(Node::Not(Box::new(node.clone())))])
        }
        [Root::Node(node), Root::Instruction(Instruction::Buf)] => {
          Some(vec![Root::Node(node.clone())])
        }
        [Root::Node(node), Root::Instruction(Instruction::Ldo(0x00))] => {
          Some(vec![Root::Node(node.clone()), Root::Node(node.clone())])
        }
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
      [Root::Instruction(Instruction::Ldo(offset)), Root::Node(node2), Root::Instruction(Instruction::Swp)] => {
        Some(vec![
          Root::Node(node2.clone()),
          Root::Instruction(Instruction::Ldo(*offset + 1)),
        ])
      }
      [Root::Node(node1), Root::Instruction(Instruction::Ldo(offset)), Root::Instruction(Instruction::Swp)] => {
        Some(vec![
          Root::Instruction(Instruction::Ldo(*offset - 1)),
          Root::Node(node1.clone()),
        ])
      }
      [Root::Instruction(Instruction::Ldo(offset1)), Root::Instruction(Instruction::Ldo(offset2)), Root::Instruction(Instruction::Swp)] => {
        Some(vec![
          Root::Instruction(Instruction::Ldo(*offset2 - 1)),
          Root::Instruction(Instruction::Ldo(*offset1 + 1)),
        ])
      }
      _ => None,
    });

    roots = match_replace(&roots, |window| match window {
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
      [Root::Node(node1), Root::Instruction(Instruction::Rot(0x01)), Root::Node(node2), Root::Instruction(Instruction::Rot(0x01))] => {
        Some(vec![
          Root::Node(Node::Add(Box::new(node2.clone()), Box::new(node1.clone()))),
          Root::Instruction(Instruction::Rot(0x01)),
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
      Node::Immediate(immediate) => *immediate,
      Node::Add(node1, node2) => {
        eval(node2, label_definitions)?.wrapping_add(eval(node1, label_definitions)?)
      }
      Node::Sub(node1, node2) => {
        eval(node2, label_definitions)?.wrapping_sub(eval(node1, label_definitions)?)
      }
      Node::Rot(node1, node2) => {
        let a = eval(node1, label_definitions)? as u16;
        let b = eval(node2, label_definitions)? as u16;

        let shifted = if a as i8 >= 0 {
          (b as u16).wrapping_shl(a as u32)
        } else {
          (b as u16).wrapping_shr(a.wrapping_neg() as u32)
        } as u16;

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

  fn make_push_instruction(immediate: u8, position: &Pos) -> Vec<(Pos, Instruction)> {
    // the `Psh` instruction allows us to push arbitrary 7-bit immediates onto the stack.
    // we then optionally use `Neg` and `Inc` to get the ability to push arbitrary 8-bit
    // immediates. we also use `Phn` as a shorthand when possible.

    if immediate & 0b11110000 == 0b11110000 {
      vec![(position.clone(), Instruction::Phn(immediate & 0b00001111))]
    } else if immediate == 0b10000000 {
      vec![
        (position.clone(), Instruction::Psh(0b01111111)),
        (position.clone(), Instruction::Inc),
      ]
    } else {
      match immediate & 0b10000000 {
        0b00000000 => vec![(position.clone(), Instruction::Psh(immediate & 0b01111111))],
        0b10000000 => vec![
          (position.clone(), Instruction::Psh(immediate.wrapping_neg())),
          (position.clone(), Instruction::Neg),
        ],
        _ => unreachable!(),
      }
    }
  }

  // if every label a node depends on could be resolved, we can replace it with an immediate.
  // if not, assume the worst case and reserve two bytes for pushing an immediate later

  let mut label_definitions: HashMap<Label, u8> = HashMap::new();
  let mut unevaluated_nodes: HashMap<u8, (Pos, Node)> = HashMap::new();

  let mut location_counter: u8 = 0;
  let instructions: Vec<(Pos, Instruction)> = roots
    .into_iter()
    .flat_map(|root| match root.1 {
      Root::Instruction(instruction) | Root::Dyn(Some(instruction)) => {
        let instructions = vec![(root.0, instruction)];
        location_counter = location_counter.wrapping_add(instructions.len() as u8);
        instructions
      }

      Root::Node(node) => match eval(&node, &label_definitions) {
        Ok(value) => {
          let instructions = make_push_instruction(value, &root.0);
          location_counter = location_counter.wrapping_add(instructions.len() as u8);
          instructions
        }
        Err(_) => {
          let instructions = vec![
            (root.0.clone(), Instruction::Nop),
            (root.0.clone(), Instruction::Nop),
          ];
          unevaluated_nodes.insert(location_counter, (root.0, node));
          location_counter = location_counter.wrapping_add(instructions.len() as u8);
          instructions
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
            Error(format!("Label `{}` has already been defined", label)),
          ));
        }
        label_definitions.insert(label, location_counter);
        vec![]
      }

      Root::Org(Some(node)) => match eval(&node, &label_definitions) {
        Ok(value) => {
          if value >= location_counter {
            let difference = value - location_counter;
            location_counter = location_counter.wrapping_sub(difference);
            vec![(root.0, Instruction::Raw(0x00)); difference as usize]
          } else {
            errors.push((
              root.0,
              Error(format!(
                "Origin cannot move location counter backward from `{}` to `{}`",
                location_counter, value
              )),
            ));
            vec![]
          }
        }
        Err(label) => {
          errors.push((
            root.0,
            Error(format!(
              "Origin argument contains currently unresolved label `{}`",
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
            "Origin argument could not be reduced to a constant expression"
          )),
        ));
        vec![]
      }

      Root::Const => {
        errors.push((
          root.0,
          Error(format!(
            "Constant argument could not be reduced to a constant expression"
          )),
        ));
        vec![]
      }

      Root::Dyn(None) => {
        errors.push((
          root.0,
          Error(format!(
            "Dynamic argument could not be reduced to an instruction"
          )),
        ));
        vec![]
      }
    })
    .collect();

  // poke into the instructions and evaluate all nodes that couldn't be evaluated before

  let mut instructions = instructions;

  for (location_counter, node) in unevaluated_nodes.iter() {
    let immediate = match eval(&node.1, &label_definitions) {
      Ok(value) => value,
      Err(label) => {
        errors.push((
          node.0.clone(),
          Error(format!("Definition for label `{}` not found", label)),
        ));
        0x00
      }
    };

    for (index, instruction) in make_push_instruction(immediate, &node.0).iter().enumerate() {
      instructions[*location_counter as usize + index] = instruction.clone();
    }
  }

  instructions
}

#[allow(unused)]
fn codegen(
  instructions: Vec<(Pos, Instruction)>,
  errors: &mut Vec<(Pos, Error)>,
) -> Vec<(Pos, u8)> {
  fn encode_immediate(immediate: u8) -> u8 {
    match immediate {
      0b00000000..=0b01111111 => immediate,
      _ => panic!("Invalid immediate in codegen stage"),
    }
  }

  fn encode_size(size: u8) -> u8 {
    match size {
      0x01 => 0x00,
      0x02 => 0x01,
      0x04 => 0x02,
      0x08 => 0x03,
      _ => panic!("Invalid size in codegen stage"),
    }
  }

  fn encode_offset(offset: u8) -> u8 {
    match offset {
      0b00000000..=0b00001111 => offset,
      _ => panic!("Invalid offset in codegen stage"),
    }
  }

  // codegen instructions into bytes and sanity-check operands

  let bytes: Vec<(Pos, u8)> = instructions
    .into_iter()
    .map(|instruction| {
      let position = instruction.0;
      let instruction = match instruction.1 {
        Instruction::Psh(immediate) => 0b00000000 | encode_immediate(immediate),
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
        Instruction::Ldo(offset) => 0b11000000 | encode_offset(offset),
        Instruction::Sto(offset) => 0b11010000 | encode_offset(offset),
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
        Instruction::Phn(immediate) => 0b11110000 | encode_offset(immediate),
        Instruction::Raw(data) => data,
      };

      (position, instruction)
    })
    .collect();

  let available_memory = 0x100;
  let mut bytes = bytes;
  let position = Pos("[codegen]".to_string(), 0);

  if bytes.len() > available_memory {
    errors.push((
      position,
      Error(format!(
        "Program size `{:02X}` exceeds available memory of size `{:02X}`",
        bytes.len(),
        available_memory
      )),
    ));
  } else {
    bytes.extend(vec![(position, 0x00); available_memory - bytes.len()]);
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
      Token::DDD(n) => write!(f, "d{:02X}", n),
      Token::XXX(n) => write!(f, "x{:02X}", n),
      Token::Add => write!(f, "add"),
      Token::AdS(n) => write!(f, "ad{:01X}", n),
      Token::Sub => write!(f, "sub"),
      Token::SuS(n) => write!(f, "su{:01X}", n),
      Token::Iff => write!(f, "iff"),
      Token::IfS(n) => write!(f, "if{:01X}", n),
      Token::Rot => write!(f, "rot"),
      Token::RoS(n) => write!(f, "ro{:01X}", n),
      Token::Orr => write!(f, "orr"),
      Token::OrS(n) => write!(f, "or{:01X}", n),
      Token::And => write!(f, "and"),
      Token::AnS(n) => write!(f, "an{:01X}", n),
      Token::Xor => write!(f, "xor"),
      Token::XoS(n) => write!(f, "xo{:01X}", n),
      Token::Xnd => write!(f, "xnd"),
      Token::XnS(n) => write!(f, "xn{:01X}", n),
      Token::Inc => write!(f, "inc"),
      Token::Dec => write!(f, "dec"),
      Token::Neg => write!(f, "neg"),
      Token::Shl => write!(f, "shl"),
      Token::Shr => write!(f, "shr"),
      Token::Not => write!(f, "not"),
      Token::Buf => write!(f, "buf"),
      Token::LdO(n) => write!(f, "ld{:01X}", n),
      Token::StO(n) => write!(f, "st{:01X}", n),
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
