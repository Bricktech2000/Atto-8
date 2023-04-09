use path_clean::PathClean;
use std::collections::HashMap;
use std::path::Path;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 3 {
    println!("Usage: asm <input> <output>");
    std::process::exit(1);
  }

  let mut errors = vec![];
  let input = &args[1];
  let preprocessed: String = preprocess(input, &mut errors);
  let tokens: Vec<Token> = tokenize(preprocessed, &mut errors);
  let instructions: Vec<Instruction> = assemble(tokens, "main", &mut errors);
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
          .map(|error| format!("Error: {}", error))
          .collect::<Vec<_>>()
          .join("\n")
      );
      std::process::exit(1);
    }
  }
}

type Label = (Option<usize>, String);

type Error = String;

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

fn preprocess(filename: &str, errors: &mut Vec<Error>) -> String {
  // remove comments and resolve includes

  let source = match std::fs::read_to_string(filename) {
    Ok(source) => source,
    Err(_) => {
      errors.push(format!(
        "Unable to read file: {}",
        Path::new(filename).clean().to_str().unwrap()
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
          )
          .as_str()
      }
      None => line.to_string(),
    })
    .collect::<Vec<_>>()
    .join("\n");

  source
}

fn tokenize(source: String, errors: &mut Vec<Error>) -> Vec<Token> {
  let tokens: Vec<&str> = source.split_whitespace().collect();

  // tokenize to valid tokens. tokens might be invalid instructions

  let tokens: Vec<Token> = tokens
    .iter()
    .map(|token| {
      use std::num::IntErrorKind::*;

      fn hex(string: &str, errors: &mut Vec<Error>) -> u8 {
        match u8::from_str_radix(string, 16) {
          Ok(value) => value,
          Err(e) => {
            match e.kind() {
              InvalidDigit => {
                errors.push(format!("Invalid digits in hexadecimal literal: {}", string))
              }
              Empty => errors.push(format!("Invalid empty hexadecimal literal",)),
              NegOverflow | PosOverflow => {
                errors.push(format!("Overflow in hexadecimal literal: {}", string))
              }
              _ => panic!("Unexpected error: {:?}", e),
            };
            0x00
          }
        }
      }

      match token {
        &_ if token.ends_with(":") => Token::LabelDef((None, token[..token.len() - 1].to_string())),
        &_ if token.starts_with(":") => Token::LabelRef((None, token[1..].to_string())),
        &_ if token.ends_with(".") => {
          Token::LabelDef((Some(0), token[..token.len() - 1].to_string()))
        }
        &_ if token.starts_with(".") => Token::LabelRef((Some(0), token[1..].to_string())),
        &_ if token.ends_with("%") => Token::MacroDef(token[..token.len() - 1].to_string()),
        &_ if token.starts_with("%") => Token::MacroRef(token[1..].to_string()),
        &"add" => Token::Add,
        &"adc" => Token::Adc,
        &_ if token.starts_with("add") => Token::AddS(hex(&token[3..], errors)),
        &_ if token.starts_with("adc") => Token::AdcS(hex(&token[3..], errors)),
        &"sub" => Token::Sub,
        &"sbc" => Token::Sbc,
        &_ if token.starts_with("sub") => Token::SubS(hex(&token[3..], errors)),
        &_ if token.starts_with("sbc") => Token::SbcS(hex(&token[3..], errors)),
        &"shf" => Token::Shf,
        &_ if token.starts_with("shf") => Token::ShfS(hex(&token[3..], errors)),
        &"shc" => Token::Sfc,
        &_ if token.starts_with("sfc") => Token::SfcS(hex(&token[3..], errors)),
        &"rot" => Token::Rot,
        &_ if token.starts_with("rot") => Token::RotS(hex(&token[3..], errors)),
        &"iff" => Token::Iff,
        &_ if token.starts_with("iff") => Token::IffS(hex(&token[3..], errors)),
        &"orr" => Token::Orr,
        &_ if token.starts_with("orr") => Token::OrrS(hex(&token[3..], errors)),
        &"and" => Token::And,
        &_ if token.starts_with("and") => Token::AndS(hex(&token[3..], errors)),
        &"xor" => Token::Xor,
        &_ if token.starts_with("xor") => Token::XorS(hex(&token[3..], errors)),
        &"xnd" => Token::Xnd,
        &_ if token.starts_with("xnd") => Token::XndS(hex(&token[3..], errors)),
        &"inc" => Token::Inc,
        &"dec" => Token::Dec,
        &"neg" => Token::Neg,
        &"not" => Token::Not,
        &"buf" => Token::Buf,
        &"nop" => Token::Nop,
        &"clc" => Token::Clc,
        &"sec" => Token::Sec,
        &"flc" => Token::Flc,
        &"swp" => Token::Swp,
        &"pop" => Token::Pop,
        &"lda" => Token::Lda,
        &"sta" => Token::Sta,
        &"ldi" => Token::Ldi,
        &"sti" => Token::Sti,
        &"lds" => Token::Lds,
        &"sts" => Token::Sts,
        &_ if token.starts_with("d") => Token::DDD(hex(&token[1..], errors)),
        &_ if token.starts_with("x") => Token::XXX(hex(&token[1..], errors)),
        &_ if token.starts_with("ld") => Token::LdO(hex(&token[2..], errors)),
        &_ if token.starts_with("st") => Token::StO(hex(&token[2..], errors)),
        &_ => {
          errors.push(format!("Invalid token: {}", token));
          Token::Nop
        }
      }
    })
    .collect();

  tokens
}

fn assemble(tokens: Vec<Token>, entry_point: &str, errors: &mut Vec<Error>) -> Vec<Instruction> {
  // resolve macros recursively from `entry_point`

  let mut macros: HashMap<String, Vec<Token>> = HashMap::new();
  let mut current_macro_name = "".to_string();

  for token in tokens {
    match token {
      Token::MacroDef(name) => {
        current_macro_name = name;
        macros.entry(current_macro_name.clone()).or_insert(vec![]);
      }
      _ => match macros.get_mut(&current_macro_name) {
        Some(macro_tokens) => macro_tokens.push(token),
        None => errors.push(format!("Orphan instruction found",)),
      },
    }
  }

  let mut scope: usize = 1;
  let entry_point = vec![Token::MacroRef(entry_point.to_string())];
  let tokens: Vec<Token> = expand_macros(&macros, entry_point, &mut scope, errors);

  fn expand_macros<'a>(
    macros: &HashMap<String, Vec<Token>>,
    tokens: Vec<Token>,
    scope: &mut usize,
    errors: &mut Vec<Error>,
  ) -> Vec<Token> {
    tokens
      .iter()
      .flat_map(|token| match token {
        Token::MacroRef(name) => {
          let tokens = match macros.get(name) {
            Some(tokens) => tokens.clone(),
            None => {
              errors.push(format!("Macro definition not found: {}{}", "%", name));
              vec![]
            }
          };
          let tokens = tokens
            .iter()
            .map(|token| match token {
              Token::LabelDef((Some(_), name)) => Token::LabelDef((Some(*scope), name.to_string())),
              Token::LabelRef((Some(_), name)) => Token::LabelRef((Some(*scope), name.to_string())),
              _ => token.clone(),
            })
            .collect();
          *scope += 1;
          expand_macros(macros, tokens, scope, errors)
        }
        _ => vec![token.clone()],
      })
      .collect()
  }

  fn assert_size(size: u8, errors: &mut Vec<Error>) -> u8 {
    match size {
      0x01 | 0x02 | 0x04 | 0x08 => size,
      _ => {
        errors.push(format!("Invalid size operand: {}", size));
        0x01
      }
    }
  }

  fn assert_offset(offset: u8, errors: &mut Vec<Error>) -> u8 {
    match offset {
      0b00000000..=0b00001111 => offset,
      _ => {
        errors.push(format!("Invalid offset operand: {}", offset));
        0b00000000
      }
    }
  }

  // turn assembly tokens into roots, an intermediate representation. roots correspond to valid instructions

  type RootLabel = (Option<usize>, String);

  #[derive(Clone, Eq, PartialEq)]
  enum Root {
    Instruction(Instruction),
    Node(Node),
    LabelDef(RootLabel),
  }

  #[derive(Clone, Eq, PartialEq)]
  enum Node {
    LabelRef(RootLabel),
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

  let roots: Vec<Root> = tokens
    .iter()
    .map(|token| match token {
      Token::LabelDef((scope, name)) => Root::LabelDef((*scope, name.clone())),
      Token::LabelRef((scope, name)) => Root::Node(Node::LabelRef((*scope, name.clone()))),
      Token::MacroDef(_) => panic!("Macro definition found in intermediate representation"),
      Token::MacroRef(_) => panic!("Macro reference found in intermediate representation"),
      Token::XXX(immediate) => Root::Node(Node::Immediate(*immediate)),
      Token::LdO(offset) => Root::Instruction(Instruction::Ldo(assert_offset(*offset, errors))),
      Token::StO(offset) => Root::Instruction(Instruction::Sto(assert_offset(*offset, errors))),
      Token::Add => Root::Instruction(Instruction::Add(assert_size(0x01, errors))),
      Token::Adc => Root::Instruction(Instruction::Adc(assert_size(0x01, errors))),
      Token::AddS(size) => Root::Instruction(Instruction::Add(assert_size(*size, errors))),
      Token::AdcS(size) => Root::Instruction(Instruction::Adc(assert_size(*size, errors))),
      Token::Sub => Root::Instruction(Instruction::Sub(assert_size(0x01, errors))),
      Token::Sbc => Root::Instruction(Instruction::Sbc(assert_size(0x01, errors))),
      Token::SubS(size) => Root::Instruction(Instruction::Sub(assert_size(*size, errors))),
      Token::SbcS(size) => Root::Instruction(Instruction::Sbc(assert_size(*size, errors))),
      Token::Shf => Root::Instruction(Instruction::Shf(assert_size(0x01, errors))),
      Token::Sfc => Root::Instruction(Instruction::Sfc(assert_size(0x01, errors))),
      Token::ShfS(size) => Root::Instruction(Instruction::Shf(assert_size(*size, errors))),
      Token::SfcS(size) => Root::Instruction(Instruction::Sfc(assert_size(*size, errors))),
      Token::Rot => Root::Instruction(Instruction::Rot(assert_size(0x01, errors))),
      Token::RotS(size) => Root::Instruction(Instruction::Rot(assert_size(*size, errors))),
      Token::Iff => Root::Instruction(Instruction::Iff(assert_size(0x01, errors))),
      Token::IffS(size) => Root::Instruction(Instruction::Iff(assert_size(*size, errors))),
      Token::Orr => Root::Instruction(Instruction::Orr(assert_size(0x01, errors))),
      Token::OrrS(size) => Root::Instruction(Instruction::Orr(assert_size(*size, errors))),
      Token::And => Root::Instruction(Instruction::And(assert_size(0x01, errors))),
      Token::AndS(size) => Root::Instruction(Instruction::And(assert_size(*size, errors))),
      Token::Xor => Root::Instruction(Instruction::Xor(assert_size(0x01, errors))),
      Token::XorS(size) => Root::Instruction(Instruction::Xor(assert_size(*size, errors))),
      Token::Xnd => Root::Instruction(Instruction::Xnd(assert_size(0x01, errors))),
      Token::XndS(size) => Root::Instruction(Instruction::Xnd(assert_size(*size, errors))),
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
      Token::DDD(immediate) => Root::Instruction(Instruction::Raw(*immediate)),
    })
    .collect();

  // build a tree of nodes representing everything we can compute at compile time
  // this removes redundant instructions and makes macros usable

  // a convenience function to replace slice patterns within a vector
  fn match_replace<const N: usize>(
    roots: &Vec<Root>,
    func: fn(&[Root; N]) -> Option<Vec<Root>>,
  ) -> Vec<Root> {
    if roots.len() < N {
      return roots.clone();
    }

    let mut output = vec![];

    let mut skip_next = 0;
    for window in roots.windows(N) {
      if skip_next > 0 {
        skip_next -= 1;
      } else {
        match func(window.try_into().unwrap()) {
          Some(roots) => {
            output.extend(roots);
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

  fn assert_immediate(immediate: u8, errors: &mut Vec<Error>) -> u8 {
    match immediate {
      0b00000000..=0b01111111 => immediate,
      _ => {
        errors.push(format!("Invalid immediate operand: {}", immediate));
        0b00000000
      }
    }
  }

  fn eval<'a>(node: &'a Node, labels: &HashMap<Label, u8>, address: u8) -> Result<u8, Label> {
    Ok(match node {
      Node::LabelRef((scope, name)) => *labels
        .get(&(*scope, name.clone()))
        .ok_or((*scope, name.clone()))?,
      Node::Immediate(immediate) => *immediate,
      Node::Not(node) => !eval(node, labels, address)?,
      Node::Add(node1, node2) => {
        eval(node2, labels, address)?.wrapping_add(eval(node1, labels, address)?)
      }
      Node::Sub(node1, node2) => {
        eval(node2, labels, address)?.wrapping_sub(eval(node1, labels, address)?)
      }
      Node::Shf(node1, node2) => {
        // TODO: negative shifts
        let shifted =
          (eval(node2, labels, address)? as u16) << eval(node1, labels, address)? as u16;
        shifted as u8
      }
      Node::Rot(node1, node2) => {
        // TODO: negative rotations
        let shifted =
          (eval(node2, labels, address)? as u16) << eval(node1, labels, address)? as u16;
        (shifted & 0xFF) as u8 | (shifted >> 8) as u8
      }
      Node::Orr(node1, node2) => eval(node2, labels, address)? | eval(node1, labels, address)?,
      Node::And(node1, node2) => eval(node2, labels, address)? & eval(node1, labels, address)?,
      Node::Xor(node1, node2) => eval(node2, labels, address)? ^ eval(node1, labels, address)?,
      Node::Xnd(_, _) => 0,
    })
  }

  fn make_push_instruction(immediate: u8, errors: &mut Vec<Error>) -> Vec<Instruction> {
    // the `Psh` instruction allows us to push arbitrary 7-bit immediates onto the stack.
    // we then optionally use `Neg` and `Inc` to get the ability to push arbitrary 8-bit
    // immediates. we also use `Phn` as a shorthand when possible.

    if immediate & 0b11110000 == 0b11110000 {
      vec![Instruction::Phn(assert_offset(
        immediate & 0b00001111,
        errors,
      ))]
    } else if immediate == 0b10000000 {
      vec![
        Instruction::Psh(assert_immediate(0b01111111, errors)),
        Instruction::Inc,
      ]
    } else {
      match immediate & 0b10000000 {
        0b00000000 => vec![Instruction::Psh(assert_immediate(
          immediate & 0b01111111,
          errors,
        ))],
        0b10000000 => vec![
          Instruction::Psh(assert_immediate(immediate.wrapping_neg(), errors)),
          Instruction::Neg,
        ],
        _ => unreachable!(),
      }
    }
  }

  // if every label a node depends on could be resolved, we can replace it with an immediate.
  // if not, assume the worst case and reserve two bytes for pushing an immediate later

  let mut labels: HashMap<Label, u8> = HashMap::new();
  let mut nodes: HashMap<u8, Node> = HashMap::new();

  let mut address: u8 = 0;
  let instructions: Vec<Instruction> = roots
    .iter()
    .flat_map(|root| match root {
      Root::Instruction(instruction) => {
        let instructions = vec![instruction.clone()];
        address = address.wrapping_add(instructions.len() as u8);
        instructions
      }
      Root::Node(node) => match eval(node, &labels, address) {
        Ok(value) => {
          let instructions = make_push_instruction(value, errors);
          address = address.wrapping_add(instructions.len() as u8);
          instructions
        }
        Err(_) => {
          let instructions = vec![Instruction::Nop, Instruction::Nop];
          nodes.insert(address, node.clone());
          address = address.wrapping_add(instructions.len() as u8);
          instructions
        }
      },
      Root::LabelDef((Some(0), _)) => panic!("Local label has no scope specified"),
      Root::LabelDef((scope, name)) => {
        // empty labels are used as an optimization blocker
        if name != "" {
          if labels.contains_key(&(*scope, name.to_string())) {
            errors.push(format!(
              "Label already defined : {}{}",
              match scope {
                Some(_) => ".",
                None => ":",
              },
              name,
            ));
          }
          labels.insert((*scope, name.to_string()), address);
        }
        vec![]
      }
    })
    .collect();

  // poke into the instructions and evaluate all nodes that couldn't be evaluated before

  let mut instructions = instructions;

  for (address, node) in nodes.iter() {
    let push_instructions = make_push_instruction(
      match eval(node, &labels, *address) {
        Ok(value) => value,
        Err((scope, name)) => {
          errors.push(format!(
            "Label definition not found: {}{}",
            match scope {
              Some(_) => ".",
              None => ":",
            },
            name,
          ));
          0x00
        }
      },
      errors,
    );
    for (i, instruction) in push_instructions.iter().enumerate() {
      instructions[*address as usize + i] = instruction.clone();
    }
  }

  instructions
}

#[allow(unused)]
fn codegen(instructions: Vec<Instruction>, errors: &mut Vec<Error>) -> Vec<u8> {
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
    .iter()
    .flat_map(|instruction| match instruction {
      Instruction::Psh(immediate) => vec![0b00000000 | encode_immediate(*immediate)],
      Instruction::Phn(immediate) => vec![0b11110000 | encode_offset(*immediate)],
      Instruction::Ldo(offset) => vec![0b11000000 | encode_offset(*offset)],
      Instruction::Sto(offset) => vec![0b11010000 | encode_offset(*offset)],
      Instruction::Add(size) => vec![0b10000000 | encode_size(*size)],
      Instruction::Adc(size) => vec![0b10000100 | encode_size(*size)],
      Instruction::Sub(size) => vec![0b10001000 | encode_size(*size)],
      Instruction::Sbc(size) => vec![0b10001100 | encode_size(*size)],
      Instruction::Shf(size) => vec![0b10010000 | encode_size(*size)],
      Instruction::Sfc(size) => vec![0b10010100 | encode_size(*size)],
      Instruction::Rot(size) => vec![0b10011000 | encode_size(*size)],
      Instruction::Iff(size) => vec![0b10011100 | encode_size(*size)],
      Instruction::Orr(size) => vec![0b10100000 | encode_size(*size)],
      Instruction::And(size) => vec![0b10100100 | encode_size(*size)],
      Instruction::Xor(size) => vec![0b10101000 | encode_size(*size)],
      Instruction::Xnd(size) => vec![0b10101100 | encode_size(*size)],
      Instruction::Inc => vec![0b10110000],
      Instruction::Dec => vec![0b10110001],
      Instruction::Neg => vec![0b10110010],
      Instruction::Not => vec![0b10110100],
      Instruction::Buf => vec![0b10110101],
      Instruction::Nop => vec![0xE0],
      Instruction::Clc => vec![0xE1],
      Instruction::Sec => vec![0xE2],
      Instruction::Flc => vec![0xE3],
      Instruction::Swp => vec![0xE4],
      Instruction::Pop => vec![0xE5],
      Instruction::Lda => vec![0xE8],
      Instruction::Sta => vec![0xE9],
      Instruction::Ldi => vec![0xEA],
      Instruction::Sti => vec![0xEB],
      Instruction::Lds => vec![0xEC],
      Instruction::Sts => vec![0xED],
      Instruction::Raw(data) => vec![*data],
    })
    .collect();

  bytes
}
