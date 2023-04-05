use std::collections::HashMap;
use std::path::Path;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 2 {
    println!("Usage: asm <filename>");
    std::process::exit(1);
  }

  let filename = &args[1];
  let translation: String = preprocess(filename, |filename| println!("Reading {}...", filename));
  let tokens: Vec<Token> = tokenize(&translation);
  let instructions: Vec<Instruction> = compile(tokens, "main");
  let bytes: Vec<u8> = codegen(instructions);

  let mut bytes = bytes;
  bytes.extend(vec![0; 0x100 - bytes.len()]);
  std::fs::write(Path::new(&args[1]).with_extension("bin"), bytes).expect("Unable to write file");

  println!("\nDone.");
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Token<'a> {
  LabelDef(&'a str),
  LabelRef(&'a str),
  MacroDef(&'a str),
  MacroRef(&'a str),
  Nop,
  Hlt,
  Dbg,
  Clc,
  Sec,
  Flc,
  Inc,
  Dec,
  Add,
  AdS(u8),
  Adc,
  AcS(u8),
  Sub,
  SuS(u8),
  Sbc,
  ScS(u8),
  Shf,
  ShS(u8),
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
  Not,
  Ntp,
  Buf,
  Bfp,
  Iff,
  IfS(u8),
  Swp,
  Pop,
  XXX(u8),
  Lda,
  Sta,
  Ldi,
  Sti,
  Lds,
  Sts,
  LdO(u8),
  StO(u8),
  DDD(u8),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Instruction {
  Nop,
  Hlt,
  Dbg,
  Clc,
  Sec,
  Flc,
  Inc,
  Dec,
  Add(u8),
  Adc(u8),
  Sub(u8),
  Sbc(u8),
  Shf(u8),
  Rot(u8),
  Orr(u8),
  And(u8),
  Xor(u8),
  Xnd(u8),
  Not,
  Ntp,
  Buf,
  Bfp,
  Iff(u8),
  Swp,
  Pop,
  Psh(u8),
  Lda,
  Sta,
  Ldi,
  Sti,
  Lds,
  Sts,
  Ldo(u8),
  Sto(u8),
  Raw(u8),
}

fn preprocess(filename: &str, callback: fn(&str)) -> String {
  callback(filename);

  let source: String =
    std::fs::read_to_string(filename).expect(format!("Unable to read file: {}", filename).as_str());

  // remove comments and resolve includes

  let source = source
    .lines()
    .map(|line| line.split("#").next().unwrap())
    .map(|line| match line.find('@') {
      Some(i) => preprocess(
        Path::new(filename)
          .parent()
          .unwrap()
          .join(line[i..][1..].trim())
          .to_str()
          .unwrap(),
        callback,
      ),
      None => line.to_string(),
    })
    .collect::<Vec<String>>()
    .join("\n");

  source
}

fn tokenize(source: &str) -> Vec<Token> {
  let tokens: Vec<&str> = source.split_whitespace().collect();

  // tokenize to valid tokens. tokens might be invalid instructions

  let tokens: Vec<Token> = tokens
    .iter()
    .map(|token| match token {
      &_ if token.ends_with(":") => Token::LabelDef(&token[..token.len() - 1]),
      &_ if token.starts_with(":") => Token::LabelRef(&token[1..]),
      &_ if token.ends_with("%") => Token::MacroDef(&token[..token.len() - 1]),
      &_ if token.starts_with("%") => Token::MacroRef(&token[1..]),
      &"nop" => Token::Nop,
      &"hlt" => Token::Hlt,
      &"dbg" => Token::Dbg,
      &"clc" => Token::Clc,
      &"sec" => Token::Sec,
      &"flc" => Token::Flc,
      &"inc" => Token::Inc,
      &"dec" => Token::Dec,
      &"add" => Token::Add,
      &_ if token.starts_with("ad") => Token::AdS(u8::from_str_radix(&token[2..], 16).unwrap()),
      &"adc" => Token::Adc,
      &_ if token.starts_with("ac") => Token::AcS(u8::from_str_radix(&token[2..], 16).unwrap()),
      &"sub" => Token::Sub,
      &_ if token.starts_with("su") => Token::SuS(u8::from_str_radix(&token[2..], 16).unwrap()),
      &"sbc" => Token::Sbc,
      &_ if token.starts_with("sc") => Token::ScS(u8::from_str_radix(&token[2..], 16).unwrap()),
      &"shf" => Token::Shf,
      &_ if token.starts_with("sh") => Token::ShS(u8::from_str_radix(&token[2..], 16).unwrap()),
      &"rot" => Token::Rot,
      &_ if token.starts_with("ro") => Token::RoS(u8::from_str_radix(&token[2..], 16).unwrap()),
      &"orr" => Token::Orr,
      &_ if token.starts_with("or") => Token::OrS(u8::from_str_radix(&token[2..], 16).unwrap()),
      &"and" => Token::And,
      &_ if token.starts_with("an") => Token::AnS(u8::from_str_radix(&token[2..], 16).unwrap()),
      &"xor" => Token::Xor,
      &_ if token.starts_with("xo") => Token::XoS(u8::from_str_radix(&token[2..], 16).unwrap()),
      &"xnd" => Token::Xnd,
      &_ if token.starts_with("xn") => Token::XnS(u8::from_str_radix(&token[2..], 16).unwrap()),
      &"not" => Token::Not,
      &"ntp" => Token::Ntp,
      &"buf" => Token::Buf,
      &"bfp" => Token::Bfp,
      &"iff" => Token::Iff,
      &_ if token.starts_with("if") => Token::IfS(u8::from_str_radix(&token[2..], 16).unwrap()),
      &"swp" => Token::Swp,
      &"pop" => Token::Pop,
      &_ if token.starts_with("x") => Token::XXX(u8::from_str_radix(&token[1..], 16).unwrap()),
      &"lda" => Token::Lda,
      &"sta" => Token::Sta,
      &"ldi" => Token::Ldi,
      &"sti" => Token::Sti,
      &"lds" => Token::Lds,
      &"sts" => Token::Sts,
      &_ if token.starts_with("ld") => Token::LdO(u8::from_str_radix(&token[2..], 16).unwrap()),
      &_ if token.starts_with("st") => Token::StO(u8::from_str_radix(&token[2..], 16).unwrap()),
      &_ if token.starts_with("d") => Token::DDD(u8::from_str_radix(&token[1..], 16).unwrap()),
      &_ => panic!("Unknown token: {}", token),
    })
    .collect();

  tokens
}

fn compile(tokens: Vec<Token>, entry_point: &str) -> Vec<Instruction> {
  // resolve macros recursively from `entry_point`

  let mut macros: HashMap<&str, Vec<Token>> = HashMap::new();
  let mut current_macro_name = "";

  for token in tokens {
    match token {
      Token::MacroDef(name) => {
        current_macro_name = name;
        macros.entry(current_macro_name).or_insert(vec![]);
      }
      _ => {
        macros
          .get_mut(current_macro_name)
          .expect("Orphan instructions found")
          .push(token);
      }
    }
  }

  let entry_point = &vec![Token::MacroRef(entry_point)];
  let tokens: Vec<Token> = expand_macros(&macros, &entry_point);

  fn expand_macros<'a>(
    macros: &'a HashMap<&'a str, Vec<Token<'a>>>,
    tokens: &'a Vec<Token>,
  ) -> Vec<Token<'a>> {
    tokens
      .iter()
      .flat_map(|token| match token {
        Token::MacroRef(name) => {
          let tokens = macros
            .get(name)
            .expect(&format!("Could not find macro: {}", name));
          expand_macros(&macros, &tokens)
        }
        _ => vec![*token],
      })
      .collect()
  }

  fn assert_size(size: u8) -> u8 {
    match size {
      0x01 | 0x02 | 0x04 | 0x08 => size,
      _ => panic!("Invalid size: {}", size),
    }
  }

  fn assert_offset(offset: u8) -> u8 {
    match offset {
      0b00000000..=0b00001111 => offset,
      _ => panic!("Invalid offset: {}", offset),
    }
  }

  // turn assembly tokens into roots, an intermediate representation. roots correspond to valid instructions

  #[derive(Debug, Clone, Eq, PartialEq)]
  enum Root {
    Instruction(Instruction),
    Node(Node),
    LabelDef(String),
  }

  #[derive(Debug, Clone, Eq, PartialEq)]
  enum Node {
    NextAddress,
    LabelRef(String),
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
      Token::LabelDef(label) => Root::LabelDef(label.to_string()),
      Token::LabelRef(label) => Root::Node(Node::LabelRef(label.to_string())),
      Token::MacroDef(_) => unreachable!(),
      Token::MacroRef(_) => unreachable!(),
      Token::Nop => Root::Instruction(Instruction::Nop),
      Token::Hlt => Root::Instruction(Instruction::Hlt),
      Token::Dbg => Root::Instruction(Instruction::Dbg),
      Token::Clc => Root::Instruction(Instruction::Clc),
      Token::Sec => Root::Instruction(Instruction::Sec),
      Token::Flc => Root::Instruction(Instruction::Flc),
      Token::Inc => Root::Instruction(Instruction::Inc),
      Token::Dec => Root::Instruction(Instruction::Dec),
      Token::Add => Root::Instruction(Instruction::Add(assert_size(0x01))),
      Token::AdS(size) => Root::Instruction(Instruction::Add(assert_size(*size))),
      Token::Adc => Root::Instruction(Instruction::Adc(assert_size(0x01))),
      Token::AcS(size) => Root::Instruction(Instruction::Adc(assert_size(*size))),
      Token::Sub => Root::Instruction(Instruction::Sub(assert_size(0x01))),
      Token::SuS(size) => Root::Instruction(Instruction::Sub(assert_size(*size))),
      Token::Sbc => Root::Instruction(Instruction::Sbc(assert_size(0x01))),
      Token::ScS(size) => Root::Instruction(Instruction::Sbc(assert_size(*size))),
      Token::Shf => Root::Instruction(Instruction::Shf(assert_size(0x01))),
      Token::ShS(size) => Root::Instruction(Instruction::Shf(assert_size(*size))),
      Token::Rot => Root::Instruction(Instruction::Rot(assert_size(0x01))),
      Token::RoS(size) => Root::Instruction(Instruction::Rot(assert_size(*size))),
      Token::Orr => Root::Instruction(Instruction::Orr(assert_size(0x01))),
      Token::OrS(size) => Root::Instruction(Instruction::Orr(assert_size(*size))),
      Token::And => Root::Instruction(Instruction::And(assert_size(0x01))),
      Token::AnS(size) => Root::Instruction(Instruction::And(assert_size(*size))),
      Token::Xor => Root::Instruction(Instruction::Xor(assert_size(0x01))),
      Token::XoS(size) => Root::Instruction(Instruction::Xor(assert_size(*size))),
      Token::Xnd => Root::Instruction(Instruction::Xnd(assert_size(0x01))),
      Token::XnS(size) => Root::Instruction(Instruction::Xnd(assert_size(*size))),
      Token::Not => Root::Instruction(Instruction::Not),
      Token::Ntp => Root::Instruction(Instruction::Ntp),
      Token::Buf => Root::Instruction(Instruction::Buf),
      Token::Bfp => Root::Instruction(Instruction::Bfp),
      Token::Iff => Root::Instruction(Instruction::Iff(assert_size(0x01))),
      Token::IfS(size) => Root::Instruction(Instruction::Iff(assert_size(*size))),
      Token::Swp => Root::Instruction(Instruction::Swp),
      Token::Pop => Root::Instruction(Instruction::Pop),
      Token::XXX(immediate) => Root::Node(Node::Immediate(*immediate)),
      Token::Lda => Root::Instruction(Instruction::Lda),
      Token::Sta => Root::Instruction(Instruction::Sta),
      Token::Ldi => Root::Instruction(Instruction::Ldi),
      Token::Sti => Root::Instruction(Instruction::Sti),
      Token::Lds => Root::Instruction(Instruction::Lds),
      Token::Sts => Root::Instruction(Instruction::Sts),
      Token::LdO(offset) => Root::Instruction(Instruction::Ldo(assert_offset(*offset))),
      Token::StO(offset) => Root::Instruction(Instruction::Sto(assert_offset(*offset))),
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
      // TODO: this is broken. this is another catch-22; `Ldi` pushes the next instruction onto
      // the stack, but we can't know what the address the next instruction is until we've `eval`ed it
      // [Root::Instruction(Instruction::Ldi)] => Some(vec![Root::Node(Node::NextAddress)]),
      _ => None,
    });

    roots =
      match_replace(&roots, |window| match window {
        [Root::Node(node), Root::Instruction(Instruction::Inc)] => Some(vec![Root::Node(
          Node::Add(Box::new(node.clone()), Box::new(Node::Immediate(1))),
        )]),
        [Root::Node(node), Root::Instruction(Instruction::Dec)] => Some(vec![Root::Node(
          Node::Sub(Box::new(node.clone()), Box::new(Node::Immediate(1))),
        )]),
        [Root::Node(node), Root::Instruction(Instruction::Not)] => {
          Some(vec![Root::Node(Node::Not(Box::new(node.clone())))])
        }
        [Root::Node(node), Root::Instruction(Instruction::Ntp)] => Some(vec![Root::Node(
          Node::Not(Box::new(Node::Not(Box::new(node.clone())))),
        )]),
        [Root::Node(node), Root::Instruction(Instruction::Buf)] => {
          Some(vec![Root::Node(node.clone())])
        }
        [Root::Node(node), Root::Instruction(Instruction::Bfp)] => {
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
      [Root::Node(node1), Root::Node(node2), Root::Instruction(Instruction::Ldo(0x01))] => {
        Some(vec![
          Root::Node(node1.clone()),
          Root::Node(node2.clone()),
          Root::Node(node1.clone()),
        ])
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

  // compile roots into instructions by computing the value of every node and resolving labels

  fn assert_immediate(immediate: u8) -> u8 {
    match immediate {
      0b00000000..=0b01111111 => immediate,
      _ => panic!("Invalid immediate: {}", immediate),
    }
  }

  fn eval<'a>(node: &'a Node, labels: &HashMap<&str, u8>, address: u8) -> Result<u8, &'a str> {
    Ok(match node {
      Node::NextAddress => address,
      Node::LabelRef(label) => *labels.get(label.as_str()).ok_or(label.as_str())?,
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

  fn make_push_instruction(immediate: u8) -> Vec<Instruction> {
    // the `Psh` instruction allows us to push arbitrary 7-bit immediates onto the stack.
    // we then optionally use `Bfp` and `Ntp` to get the ability to push arbitrary 8-bit immediates.

    let lower_bits = immediate & 0b01111111;
    match immediate & 0b10000000 {
      0b00000000 => vec![Instruction::Psh(assert_immediate(lower_bits))],
      0b10000000 => vec![
        Instruction::Psh(assert_immediate(lower_bits ^ 0b01111111)),
        Instruction::Ntp,
      ],
      _ => unreachable!(),
    }
  }

  // if every label a node depends on could be resolved, we can replace it with an immediate.
  // if not, assume the worst case and reserve two bytes for pushing an immediate later

  let mut labels: HashMap<&str, u8> = HashMap::new();
  let mut nodes: HashMap<u8, Node> = HashMap::new();

  let mut address = 0;
  let instructions: Vec<Instruction> = roots
    .iter()
    .flat_map(|root| match root {
      Root::Instruction(instruction) => {
        let instructions = vec![instruction.clone()];
        address += instructions.len() as u8;
        instructions
      }
      Root::Node(node) => match eval(node, &labels, address) {
        Ok(value) => {
          let instructions = make_push_instruction(value);
          address += instructions.len() as u8;
          instructions
        }
        Err(_) => {
          let instructions = vec![Instruction::Bfp, Instruction::Bfp];
          nodes.insert(address, node.clone());
          address += instructions.len() as u8;
          instructions
        }
      },
      Root::LabelDef(label) => {
        // empty labels are used as an optimization blocker
        if label != "" {
          if labels.contains_key(label.as_str()) {
            panic!("Duplicate label: {}", label);
          }
          labels.insert(label, address);
        }
        vec![]
      }
    })
    .collect();

  // poke into the instructions and evaluate all nodes that couldn't be evaluated before

  let mut instructions = instructions;

  for (address, node) in nodes.iter() {
    let push_instructions = make_push_instruction(match eval(node, &labels, *address) {
      Ok(value) => value,
      Err(label) => panic!("Could not resolve label: {}", label),
    });
    for (i, instruction) in push_instructions.iter().enumerate() {
      instructions[*address as usize + i] = *instruction;
    }
  }

  instructions
}

fn codegen(instructions: Vec<Instruction>) -> Vec<u8> {
  fn encode_immediate(immediate: u8) -> u8 {
    match immediate {
      0b00000000..=0b01111111 => immediate,
      _ => unreachable!(),
    }
  }

  fn encode_size(size: u8) -> u8 {
    (match size {
      0x01 => 0x00,
      0x02 => 0x01,
      0x04 => 0x02,
      0x08 => 0x03,
      _ => unreachable!(),
    }) << 4
  }

  fn encode_offset(offset: u8) -> u8 {
    match offset {
      0b00000000..=0b00001111 => offset,
      _ => unreachable!(),
    }
  }

  // codegen instructions into bytes and sanity-check operands

  instructions
    .iter()
    .flat_map(|instruction| match instruction {
      Instruction::Nop => vec![0xA0],
      Instruction::Hlt => vec![0xAF],
      Instruction::Dbg => vec![0xAA],
      Instruction::Clc => vec![0xA1],
      Instruction::Sec => vec![0xA2],
      Instruction::Flc => vec![0xA3],
      Instruction::Inc => vec![0xC0 | 0b00000000],
      Instruction::Dec => vec![0xC1 | 0b00000000],
      Instruction::Add(size) => vec![0xC2 | encode_size(*size)],
      Instruction::Adc(size) => vec![0xC3 | encode_size(*size)],
      Instruction::Sub(size) => vec![0xC4 | encode_size(*size)],
      Instruction::Sbc(size) => vec![0xC5 | encode_size(*size)],
      Instruction::Shf(size) => vec![0xC6 | encode_size(*size)],
      Instruction::Rot(size) => vec![0xC7 | encode_size(*size)],
      Instruction::Orr(size) => vec![0xC8 | encode_size(*size)],
      Instruction::And(size) => vec![0xC9 | encode_size(*size)],
      Instruction::Xor(size) => vec![0xCA | encode_size(*size)],
      Instruction::Xnd(size) => vec![0xCB | encode_size(*size)],
      Instruction::Not => vec![0xCC | 0b00000000],
      Instruction::Ntp => vec![0xCC | 0b00010000],
      Instruction::Buf => vec![0xCD | 0b00000000],
      Instruction::Bfp => vec![0xCD | 0b00010000],
      Instruction::Iff(size) => vec![0xCE | encode_size(*size)],
      Instruction::Swp => vec![0xB0],
      Instruction::Pop => vec![0xB1],
      Instruction::Psh(immediate) => vec![0b00000000 | encode_immediate(*immediate)],
      Instruction::Lda => vec![0xB8],
      Instruction::Sta => vec![0xB9],
      Instruction::Ldi => vec![0xBA],
      Instruction::Sti => vec![0xBB],
      Instruction::Lds => vec![0xBC],
      Instruction::Sts => vec![0xBD],
      Instruction::Ldo(offset) => vec![0x80 | encode_offset(*offset)],
      Instruction::Sto(offset) => vec![0x90 | encode_offset(*offset)],
      Instruction::Raw(data) => vec![*data],
    })
    .collect()
}
