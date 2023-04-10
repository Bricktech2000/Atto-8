use std::collections::HashMap;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 3 {
    println!("Usage: asm <input> <output>");
    std::process::exit(1);
  }

  let mut errors: Vec<(Position, Error)> = vec![];
  let input: &String = &args[1];

  let preprocessed: String = preprocess(input, &mut errors, "[bootstrap]");
  let tokens: Vec<(Position, Token)> = tokenize(preprocessed, &mut errors, "[stream]");
  let instructions: Vec<(Position, Instruction)> = assemble(tokens, "main", &mut errors);
  let bytes: Vec<u8> = codegen(instructions, &mut errors);

  match errors[..] {
    [] => {
      let mut bytes = bytes;
      bytes.extend(vec![0; 0x100 - bytes.len()]);

      let output = &args[2];
      std::fs::write(output, bytes).expect("Unable to write file");

      println!("Done.");
    }
    _ => {
      println!(
        "{}\n\nAborting.",
        errors
          .iter()
          .map(|error| format!("{}:{}: {}", error.0.scope, error.0.index, error.1.message))
          .collect::<Vec<_>>()
          .join("\n")
      );
      std::process::exit(1);
    }
  }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Label {
  scope_id: Option<usize>,
  identifier: String,
}

#[derive(Clone, Eq, PartialEq)]
struct Error {
  message: String,
}

#[derive(Clone, Eq, PartialEq)]
struct Position {
  scope: String,
  index: usize,
}

#[derive(Clone, Eq, PartialEq)]
enum Token {
  LabelDef(Label),
  LabelRef(Label),
  MacroDef(String),
  MacroRef(String),
  DDD(u8),
  XXX(u8),
  LdO(u8),
  StO(u8),
  Add,
  Adc,
  AddS(u8),
  AdcS(u8),
  Sub,
  Sbc,
  SubS(u8),
  SbcS(u8),
  Shf,
  Sfc,
  ShfS(u8),
  SfcS(u8),
  Rot,
  RotS(u8),
  Iff,
  IffS(u8),
  Orr,
  OrrS(u8),
  And,
  AndS(u8),
  Xor,
  XorS(u8),
  Xnd,
  XndS(u8),
  Inc,
  Dec,
  Neg,
  Not,
  Buf,
  Nop,
  Clc,
  Sec,
  Flc,
  Swp,
  Pop,
  Lda,
  Sta,
  Ldi,
  Sti,
  Lds,
  Sts,
}

#[derive(Clone, Eq, PartialEq)]
enum Instruction {
  Psh(u8),
  Phn(u8),
  Ldo(u8),
  Sto(u8),
  Add(u8),
  Adc(u8),
  Sub(u8),
  Sbc(u8),
  Shf(u8),
  Sfc(u8),
  Rot(u8),
  Iff(u8),
  Orr(u8),
  And(u8),
  Xor(u8),
  Xnd(u8),
  Not,
  Buf,
  Inc,
  Dec,
  Neg,
  Nop,
  Clc,
  Sec,
  Flc,
  Swp,
  Pop,
  Lda,
  Sta,
  Ldi,
  Sti,
  Lds,
  Sts,
  Raw(u8),
}

fn preprocess(filename: &str, errors: &mut Vec<(Position, Error)>, scope: &str) -> String {
  // remove comments and resolve includes

  use path_clean::PathClean;
  use std::path::Path;
  let source = match std::fs::read_to_string(filename) {
    Ok(source) => source,
    Err(_) => {
      errors.push((
        Position {
          scope: scope.to_string(),
          index: 0,
        },
        Error {
          message: format!(
            "Unable to read file: {}",
            Path::new(filename).clean().to_str().unwrap()
          ),
        },
      ));
      "".to_string()
    }
  };

  let source: String = source
    .lines()
    .map(|line| line.split("#").next().unwrap())
    .map(|line| match line.find('@') {
      Some(i) => {
        line[..i].to_owned()
          + preprocess(
            Path::new(filename)
              .parent()
              .unwrap()
              .join(&line[i..][1..])
              .to_str()
              .unwrap(),
            errors,
            &format!(
              "@{}",
              Path::new(filename).file_name().unwrap().to_str().unwrap()
            ),
          )
          .as_str()
      }
      None => line.to_string(),
    })
    .collect::<Vec<_>>()
    .join("\n");

  source
}

fn tokenize(
  source: String,
  errors: &mut Vec<(Position, Error)>,
  scope: &str,
) -> Vec<(Position, Token)> {
  let tokens: Vec<&str> = source.split_whitespace().collect();

  // tokenize to valid tokens. tokens might be invalid instructions

  let tokens: Vec<(Position, Token)> = tokens
    .into_iter()
    .enumerate()
    .map(|(index, token)| {
      fn hex(string: &str, errors: &mut Vec<(Position, Error)>, position: &Position) -> u8 {
        use std::num::IntErrorKind::*;
        match u8::from_str_radix(string, 16) {
          Ok(value) => value,
          Err(e) => {
            match e.kind() {
              InvalidDigit => errors.push((
                position.clone(),
                Error {
                  message: format!("Invalid digits in hexadecimal literal: {}", string),
                },
              )),
              Empty => errors.push((
                position.clone(),
                Error {
                  message: format!("Empty hexadecimal literal: {}", string),
                },
              )),
              NegOverflow | PosOverflow => errors.push((
                position.clone(),
                Error {
                  message: format!("Hexadecimal literal out of range: {}", string),
                },
              )),
              _ => panic!("Unexpected error: {:?}", e),
            };
            0x00
          }
        }
      }

      let position = Position {
        scope: scope.to_string(),
        index,
      };

      let token = match token {
        _ if token.ends_with(":") => Token::LabelDef(Label {
          scope_id: None,
          identifier: token[..token.len() - 1].to_string(),
        }),
        _ if token.starts_with(":") => Token::LabelRef(Label {
          scope_id: None,
          identifier: token[1..].to_string(),
        }),
        _ if token.ends_with(".") => Token::LabelDef(Label {
          scope_id: Some(0),
          identifier: token[..token.len() - 1].to_string(),
        }),
        _ if token.starts_with(".") => Token::LabelRef(Label {
          scope_id: Some(0),
          identifier: token[1..].to_string(),
        }),
        _ if token.ends_with("!") => Token::MacroDef(token[..token.len() - 1].to_string()),
        _ if token.starts_with("!") => Token::MacroRef(token[1..].to_string()),
        "add" => Token::Add,
        "adc" => Token::Adc,
        _ if token.starts_with("add") => Token::AddS(hex(&token[3..], errors, &position)),
        _ if token.starts_with("adc") => Token::AdcS(hex(&token[3..], errors, &position)),
        "sub" => Token::Sub,
        "sbc" => Token::Sbc,
        _ if token.starts_with("sub") => Token::SubS(hex(&token[3..], errors, &position)),
        _ if token.starts_with("sbc") => Token::SbcS(hex(&token[3..], errors, &position)),
        "shf" => Token::Shf,
        _ if token.starts_with("shf") => Token::ShfS(hex(&token[3..], errors, &position)),
        "shc" => Token::Sfc,
        _ if token.starts_with("sfc") => Token::SfcS(hex(&token[3..], errors, &position)),
        "rot" => Token::Rot,
        _ if token.starts_with("rot") => Token::RotS(hex(&token[3..], errors, &position)),
        "iff" => Token::Iff,
        _ if token.starts_with("iff") => Token::IffS(hex(&token[3..], errors, &position)),
        "orr" => Token::Orr,
        _ if token.starts_with("orr") => Token::OrrS(hex(&token[3..], errors, &position)),
        "and" => Token::And,
        _ if token.starts_with("and") => Token::AndS(hex(&token[3..], errors, &position)),
        "xor" => Token::Xor,
        _ if token.starts_with("xor") => Token::XorS(hex(&token[3..], errors, &position)),
        "xnd" => Token::Xnd,
        _ if token.starts_with("xnd") => Token::XndS(hex(&token[3..], errors, &position)),
        "inc" => Token::Inc,
        "dec" => Token::Dec,
        "neg" => Token::Neg,
        "not" => Token::Not,
        "buf" => Token::Buf,
        "nop" => Token::Nop,
        "clc" => Token::Clc,
        "sec" => Token::Sec,
        "flc" => Token::Flc,
        "swp" => Token::Swp,
        "pop" => Token::Pop,
        "lda" => Token::Lda,
        "sta" => Token::Sta,
        "ldi" => Token::Ldi,
        "sti" => Token::Sti,
        "lds" => Token::Lds,
        "sts" => Token::Sts,
        _ if token.starts_with("d") => Token::DDD(hex(&token[1..], errors, &position)),
        _ if token.starts_with("x") => Token::XXX(hex(&token[1..], errors, &position)),
        _ if token.starts_with("ld") => Token::LdO(hex(&token[2..], errors, &position)),
        _ if token.starts_with("st") => Token::StO(hex(&token[2..], errors, &position)),
        _ => {
          errors.push((
            position.clone(),
            Error {
              message: format!("Invalid token: {}", token),
            },
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
  tokens: Vec<(Position, Token)>,
  entry_point: &str,
  errors: &mut Vec<(Position, Error)>,
) -> Vec<(Position, Instruction)> {
  // resolve macros recursively from `entry_point`

  let mut macro_definitions: HashMap<String, Vec<(Position, Token)>> = HashMap::new();
  let mut current_macro_name = "".to_string();

  for token in tokens.into_iter() {
    match token.1 {
      Token::MacroDef(name) => {
        current_macro_name = name;
        macro_definitions
          .entry(current_macro_name.clone())
          .or_insert(vec![]);
      }
      _ => match macro_definitions.get_mut(&current_macro_name) {
        Some(macro_tokens) => macro_tokens.push((
          Position {
            scope: format!("!{}", current_macro_name),
            index: macro_tokens.len(),
          },
          token.1,
        )),
        None => errors.push((
          token.0,
          Error {
            message: format!("Orphan instruction found"),
          },
        )),
      },
    }
  }

  let mut scope: usize = 1;
  let entry_point = vec![(
    Position {
      scope: "[bootstrap]".to_string(),
      index: 0,
    },
    Token::MacroRef(entry_point.to_string()),
  )];
  let tokens: Vec<(Position, Token)> =
    expand_macros(&macro_definitions, &entry_point, &mut scope, errors);

  fn expand_macros<'a>(
    macro_definitions: &HashMap<String, Vec<(Position, Token)>>,
    tokens: &Vec<(Position, Token)>,
    scope_id: &mut usize,
    errors: &mut Vec<(Position, Error)>,
  ) -> Vec<(Position, Token)> {
    tokens
      .into_iter()
      .flat_map(|token| match &token.1 {
        Token::MacroRef(name) => {
          let tokens = match macro_definitions.get(name) {
            Some(tokens) => tokens.clone(),
            None => {
              errors.push((
                token.0.clone(),
                Error {
                  message: format!("Macro definition not found: {}{}", "!", name),
                },
              ));
              vec![]
            }
          };
          let tokens = tokens
            .into_iter()
            .map(|token| match token.1 {
              Token::LabelDef(Label {
                scope_id: Some(_),
                identifier,
              }) => (
                token.0,
                Token::LabelDef(Label {
                  scope_id: Some(*scope_id),
                  identifier,
                }),
              ),
              Token::LabelRef(Label {
                scope_id: Some(_),
                identifier,
              }) => (
                token.0,
                Token::LabelRef(Label {
                  scope_id: Some(*scope_id),
                  identifier,
                }),
              ),
              _ => token,
            })
            .collect();
          *scope_id += 1;
          expand_macros(&macro_definitions, &tokens, scope_id, errors)
        }
        _ => vec![token.clone()],
      })
      .collect()
  }

  fn assert_immediate(
    immediate: u8,
    errors: &mut Vec<(Position, Error)>,
    position: &Position,
  ) -> u8 {
    match immediate {
      0b00000000..=0b11111111 => immediate,
      #[allow(unreachable_patterns)]
      _ => {
        errors.push((
          position.clone(),
          Error {
            message: format!("Invalid immediate operand: {}", immediate),
          },
        ));
        0b00000000
      }
    }
  }

  fn assert_size(size: u8, errors: &mut Vec<(Position, Error)>, position: &Position) -> u8 {
    match size {
      0x01 | 0x02 | 0x04 | 0x08 => size,
      _ => {
        errors.push((
          position.clone(),
          Error {
            message: format!("Invalid size operand: {}", size),
          },
        ));
        0x01
      }
    }
  }

  fn assert_offset(offset: u8, errors: &mut Vec<(Position, Error)>, position: &Position) -> u8 {
    match offset {
      0b00000000..=0b00001111 => offset,
      _ => {
        errors.push((
          position.clone(),
          Error {
            message: format!("Invalid offset operand: {}", offset),
          },
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
  }

  #[derive(Clone, Eq, PartialEq)]
  enum Node {
    LabelRef(Label),
    Immediate(u8),
    Not(Box<Node>),
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Shf(Box<Node>, Box<Node>),
    Rot(Box<Node>, Box<Node>),
    Orr(Box<Node>, Box<Node>),
    And(Box<Node>, Box<Node>),
    Xor(Box<Node>, Box<Node>),
    Xnd(Box<Node>, Box<Node>),
  }

  let roots: Vec<(Position, Root)> = tokens
    .into_iter()
    .map(|token| {
      let position = token.0;
      let token = match token.1 {
        Token::LabelDef(label) => Root::LabelDef(label),
        Token::LabelRef(label) => Root::Node(Node::LabelRef(label)),
        Token::MacroDef(_) => panic!("Macro definition found in intermediate representation"),
        Token::MacroRef(_) => panic!("Macro reference found in intermediate representation"),
        Token::XXX(immediate) => Root::Node(Node::Immediate(assert_immediate(
          immediate, errors, &position,
        ))),
        Token::LdO(offset) => {
          Root::Instruction(Instruction::Ldo(assert_offset(offset, errors, &position)))
        }
        Token::StO(offset) => {
          Root::Instruction(Instruction::Sto(assert_offset(offset, errors, &position)))
        }
        Token::Add => Root::Instruction(Instruction::Add(assert_size(0x01, errors, &position))),
        Token::Adc => Root::Instruction(Instruction::Adc(assert_size(0x01, errors, &position))),
        Token::AddS(size) => {
          Root::Instruction(Instruction::Add(assert_size(size, errors, &position)))
        }
        Token::AdcS(size) => {
          Root::Instruction(Instruction::Adc(assert_size(size, errors, &position)))
        }
        Token::Sub => Root::Instruction(Instruction::Sub(assert_size(0x01, errors, &position))),
        Token::Sbc => Root::Instruction(Instruction::Sbc(assert_size(0x01, errors, &position))),
        Token::SubS(size) => {
          Root::Instruction(Instruction::Sub(assert_size(size, errors, &position)))
        }
        Token::SbcS(size) => {
          Root::Instruction(Instruction::Sbc(assert_size(size, errors, &position)))
        }
        Token::Shf => Root::Instruction(Instruction::Shf(assert_size(0x01, errors, &position))),
        Token::Sfc => Root::Instruction(Instruction::Sfc(assert_size(0x01, errors, &position))),
        Token::ShfS(size) => {
          Root::Instruction(Instruction::Shf(assert_size(size, errors, &position)))
        }
        Token::SfcS(size) => {
          Root::Instruction(Instruction::Sfc(assert_size(size, errors, &position)))
        }
        Token::Rot => Root::Instruction(Instruction::Rot(assert_size(0x01, errors, &position))),
        Token::RotS(size) => {
          Root::Instruction(Instruction::Rot(assert_size(size, errors, &position)))
        }
        Token::Iff => Root::Instruction(Instruction::Iff(assert_size(0x01, errors, &position))),
        Token::IffS(size) => {
          Root::Instruction(Instruction::Iff(assert_size(size, errors, &position)))
        }
        Token::Orr => Root::Instruction(Instruction::Orr(assert_size(0x01, errors, &position))),
        Token::OrrS(size) => {
          Root::Instruction(Instruction::Orr(assert_size(size, errors, &position)))
        }
        Token::And => Root::Instruction(Instruction::And(assert_size(0x01, errors, &position))),
        Token::AndS(size) => {
          Root::Instruction(Instruction::And(assert_size(size, errors, &position)))
        }
        Token::Xor => Root::Instruction(Instruction::Xor(assert_size(0x01, errors, &position))),
        Token::XorS(size) => {
          Root::Instruction(Instruction::Xor(assert_size(size, errors, &position)))
        }
        Token::Xnd => Root::Instruction(Instruction::Xnd(assert_size(0x01, errors, &position))),
        Token::XndS(size) => {
          Root::Instruction(Instruction::Xnd(assert_size(size, errors, &position)))
        }
        Token::Inc => Root::Instruction(Instruction::Inc),
        Token::Dec => Root::Instruction(Instruction::Dec),
        Token::Neg => Root::Instruction(Instruction::Neg),
        Token::Not => Root::Instruction(Instruction::Not),
        Token::Buf => Root::Instruction(Instruction::Buf),
        Token::Nop => Root::Instruction(Instruction::Nop),
        Token::Clc => Root::Instruction(Instruction::Clc),
        Token::Sec => Root::Instruction(Instruction::Sec),
        Token::Flc => Root::Instruction(Instruction::Flc),
        Token::Swp => Root::Instruction(Instruction::Swp),
        Token::Pop => Root::Instruction(Instruction::Pop),
        Token::Lda => Root::Instruction(Instruction::Lda),
        Token::Sta => Root::Instruction(Instruction::Sta),
        Token::Ldi => Root::Instruction(Instruction::Ldi),
        Token::Sti => Root::Instruction(Instruction::Sti),
        Token::Lds => Root::Instruction(Instruction::Lds),
        Token::Sts => Root::Instruction(Instruction::Sts),
        Token::DDD(immediate) => Root::Instruction(Instruction::Raw(immediate)),
      };

      (position, token)
    })
    .collect();

  // build a tree of nodes representing everything we can compute at compile time
  // this removes redundant instructions and makes macros usable

  // a convenience function to replace slice patterns within a vector
  fn match_replace<const N: usize>(
    roots: &Vec<(Position, Root)>,
    func: fn(&[Root; N]) -> Option<Vec<Root>>,
  ) -> Vec<(Position, Root)> {
    if roots.len() < N {
      return roots.clone();
    }

    let mut output: Vec<(Position, Root)> = vec![];

    let mut skip_next = 0;
    for window in roots.windows(N) {
      if skip_next > 0 {
        skip_next -= 1;
      } else {
        match func(
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
                .collect::<Vec<(Position, Root)>>(),
            );
            skip_next = N - 1;
          }
          None => output.push(window[0].clone()),
        }
      }
    }
    output.extend(roots.iter().skip(1 + roots.len() - N + skip_next).cloned());

    output
  }

  let mut roots = roots;
  let mut last_roots = vec![];

  while roots != last_roots {
    last_roots = roots.clone();
    // println!("roots: {:?}\nlen: {}", roots, roots.len());

    roots = match_replace(&roots, |window| match window {
      [Root::Instruction(Instruction::Nop)] => Some(vec![]),
      _ => None,
    });

    roots =
      match_replace(&roots, |window| match window {
        [Root::Node(node), Root::Instruction(Instruction::Ldo(0x00))] => {
          Some(vec![Root::Node(node.clone()), Root::Node(node.clone())])
        }
        [Root::Instruction(Instruction::Swp), Root::Instruction(Instruction::Pop)] => {
          Some(vec![Root::Instruction(Instruction::Sto(0x00))])
        }
        [Root::Node(node), Root::Instruction(Instruction::Inc)] => Some(vec![Root::Node(
          Node::Add(Box::new(node.clone()), Box::new(Node::Immediate(1))),
        )]),
        [Root::Node(node), Root::Instruction(Instruction::Dec)] => Some(vec![Root::Node(
          Node::Sub(Box::new(node.clone()), Box::new(Node::Immediate(1))),
        )]),
        [Root::Node(node), Root::Instruction(Instruction::Neg)] => Some(vec![Root::Node(
          Node::Sub(Box::new(node.clone()), Box::new(Node::Immediate(0))),
        )]),
        [Root::Node(node), Root::Instruction(Instruction::Not)] => {
          Some(vec![Root::Node(Node::Not(Box::new(node.clone())))])
        }
        [Root::Node(node), Root::Instruction(Instruction::Buf)] => {
          Some(vec![Root::Node(node.clone())])
        }
        [Root::Node(_), Root::Instruction(Instruction::Pop)] => Some(vec![]),
        _ => None,
      });

    roots = match_replace(&roots, |window| match window {
      [Root::Node(node1), Root::Node(node2), Root::Instruction(Instruction::Ldo(0x01))] => {
        Some(vec![
          Root::Node(node1.clone()),
          Root::Node(node2.clone()),
          Root::Node(node1.clone()),
        ])
      }
      [Root::Node(node1), Root::Node(node2), Root::Instruction(Instruction::Add(0x01))] => {
        Some(vec![Root::Node(Node::Add(
          Box::new(node2.clone()),
          Box::new(node1.clone()),
        ))])
      }
      [Root::Node(node1), Root::Node(node2), Root::Instruction(Instruction::Adc(0x01))] => {
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
      [Root::Node(node1), Root::Node(node2), Root::Instruction(Instruction::Sbc(0x01))] => {
        Some(vec![Root::Node(Node::Sub(
          Box::new(node2.clone()),
          Box::new(node1.clone()),
        ))])
      }
      [Root::Node(node1), Root::Node(node2), Root::Instruction(Instruction::Shf(0x01))] => {
        Some(vec![Root::Node(Node::Shf(
          Box::new(node2.clone()),
          Box::new(node1.clone()),
        ))])
      }
      [Root::Node(node1), Root::Node(node2), Root::Instruction(Instruction::Sfc(0x01))] => {
        Some(vec![Root::Node(Node::Shf(
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
      [Root::Node(node1), Root::Node(node2), Root::Instruction(Instruction::Swp)] => {
        Some(vec![Root::Node(node2.clone()), Root::Node(node1.clone())])
      }
      _ => None,
    });
  }

  roots = match_replace(&roots, |window| match window {
    [Root::Node(node1), Root::Node(node2)] if node1 == node2 => Some(vec![
      Root::Node(node1.clone()),
      Root::Instruction(Instruction::Ldo(0x00)),
    ]),
    _ => None,
  });

  roots = match_replace(&roots, |window| match window {
    [Root::Node(node1), Root::Node(node2), Root::Node(node3)] if node1 == node3 => Some(vec![
      Root::Node(node1.clone()),
      Root::Node(node2.clone()),
      Root::Instruction(Instruction::Ldo(0x01)),
    ]),
    _ => None,
  });

  // assemble roots into instructions by computing the value of every node and resolving labels

  fn eval<'a>(node: &'a Node, labels: &HashMap<Label, u8>, address: u8) -> Result<u8, Label> {
    Ok(match node {
      Node::LabelRef(label) => *labels.get(label).ok_or(label.clone())?,
      Node::Immediate(immediate) => *immediate,
      Node::Not(node) => !eval(node, labels, address)?,
      Node::Add(node1, node2) => {
        eval(node2, labels, address)?.wrapping_add(eval(node1, labels, address)?)
      }
      Node::Sub(node1, node2) => {
        eval(node2, labels, address)?.wrapping_sub(eval(node1, labels, address)?)
      }
      Node::Shf(node1, node2) => {
        let a = eval(node1, labels, address)? as u16;
        let b = eval(node2, labels, address)? as u16;

        let shifted = if a as i8 >= 0 {
          (b as u16).wrapping_shl(a as u32)
        } else {
          (b as u16).wrapping_shr(a.wrapping_neg() as u32)
        } as u16;

        shifted as u8
      }
      Node::Rot(node1, node2) => {
        let a = eval(node1, labels, address)? as u16;
        let b = eval(node2, labels, address)? as u16;

        let shifted = if a as i8 >= 0 {
          (b as u16).wrapping_shl(a as u32)
        } else {
          (b as u16).wrapping_shr(a.wrapping_neg() as u32)
        } as u16;

        (shifted & 0xFF) as u8 | (shifted >> 8) as u8
      }
      Node::Orr(node1, node2) => eval(node2, labels, address)? | eval(node1, labels, address)?,
      Node::And(node1, node2) => eval(node2, labels, address)? & eval(node1, labels, address)?,
      Node::Xor(node1, node2) => eval(node2, labels, address)? ^ eval(node1, labels, address)?,
      Node::Xnd(_, _) => 0,
    })
  }

  fn make_push_instruction(immediate: u8, position: &Position) -> Vec<(Position, Instruction)> {
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
  let mut unevaluated_nodes: HashMap<u8, (Position, Node)> = HashMap::new();

  let mut address: u8 = 0;
  let instructions: Vec<(Position, Instruction)> = roots
    .into_iter()
    .flat_map(|root| {
      match root.1 {
        Root::Instruction(instruction) => {
          let instructions = vec![(root.0, instruction.clone())];
          address = address.wrapping_add(instructions.len() as u8);
          instructions
        }
        Root::Node(node) => match eval(&node, &label_definitions, address) {
          Ok(value) => {
            let instructions = make_push_instruction(value, &root.0);
            address = address.wrapping_add(instructions.len() as u8);
            instructions
          }
          Err(_) => {
            let instructions = vec![
              (root.0.clone(), Instruction::Nop),
              (root.0.clone(), Instruction::Nop),
            ];
            unevaluated_nodes.insert(address, (root.0, node));
            address = address.wrapping_add(instructions.len() as u8);
            instructions
          }
        },
        Root::LabelDef(Label {
          scope_id: Some(0),
          identifier: _,
        }) => panic!("Local label has no scope specified"),
        Root::LabelDef(label) => {
          // empty labels are used as an optimization blocker
          if label.identifier != "" {
            if label_definitions.contains_key(&label) {
              errors.push((
                root.0,
                Error {
                  message: format!(
                    "Label already defined: {}{}",
                    match label.scope_id {
                      Some(_) => ".",
                      None => ":",
                    },
                    label.identifier,
                  ),
                },
              ));
            }
            label_definitions.insert(label.clone(), address);
          }
          vec![]
        }
      }
    })
    .collect();

  // poke into the instructions and evaluate all nodes that couldn't be evaluated before

  let mut instructions = instructions;

  for (address, node) in unevaluated_nodes.iter() {
    let push_instructions = make_push_instruction(
      match eval(&node.1, &label_definitions, *address) {
        Ok(value) => value,
        Err(label) => {
          errors.push((
            node.0.clone(),
            Error {
              message: format!(
                "Label definition not found: {}{}",
                match label.scope_id {
                  Some(_) => ".",
                  None => ":",
                },
                label.identifier,
              ),
            },
          ));
          0x00
        }
      },
      &node.0,
    );
    for (index, instruction) in push_instructions.iter().enumerate() {
      instructions[*address as usize + index] = instruction.clone();
    }
  }

  instructions
}

#[allow(unused)]
fn codegen(
  instructions: Vec<(Position, Instruction)>,
  errors: &mut Vec<(Position, Error)>,
) -> Vec<u8> {
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

  let bytes: Vec<u8> = instructions
    .into_iter()
    .map(|instruction| match instruction.1 {
      Instruction::Psh(immediate) => 0b00000000 | encode_immediate(immediate),
      Instruction::Phn(immediate) => 0b11110000 | encode_offset(immediate),
      Instruction::Ldo(offset) => 0b11000000 | encode_offset(offset),
      Instruction::Sto(offset) => 0b11010000 | encode_offset(offset),
      Instruction::Add(size) => 0b10000000 | encode_size(size),
      Instruction::Adc(size) => 0b10000100 | encode_size(size),
      Instruction::Sub(size) => 0b10001000 | encode_size(size),
      Instruction::Sbc(size) => 0b10001100 | encode_size(size),
      Instruction::Shf(size) => 0b10010000 | encode_size(size),
      Instruction::Sfc(size) => 0b10010100 | encode_size(size),
      Instruction::Rot(size) => 0b10011000 | encode_size(size),
      Instruction::Iff(size) => 0b10011100 | encode_size(size),
      Instruction::Orr(size) => 0b10100000 | encode_size(size),
      Instruction::And(size) => 0b10100100 | encode_size(size),
      Instruction::Xor(size) => 0b10101000 | encode_size(size),
      Instruction::Xnd(size) => 0b10101100 | encode_size(size),
      Instruction::Inc => 0b10110000,
      Instruction::Dec => 0b10110001,
      Instruction::Neg => 0b10110010,
      Instruction::Not => 0b10110100,
      Instruction::Buf => 0b10110101,
      Instruction::Nop => 0xE0,
      Instruction::Clc => 0xE1,
      Instruction::Sec => 0xE2,
      Instruction::Flc => 0xE3,
      Instruction::Swp => 0xE4,
      Instruction::Pop => 0xE5,
      Instruction::Lda => 0xE8,
      Instruction::Sta => 0xE9,
      Instruction::Ldi => 0xEA,
      Instruction::Sti => 0xEB,
      Instruction::Lds => 0xEC,
      Instruction::Sts => 0xED,
      Instruction::Raw(data) => data,
    })
    .collect();

  bytes
}
