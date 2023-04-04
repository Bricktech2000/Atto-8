use std::collections::HashMap;
use std::path::Path;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 2 {
    println!("Usage: asm <filename>");
    std::process::exit(1);
  }

  let source = preprocess(&args[1]);
  let tokens: Vec<Token> = tokenize(&source);
  let instructions: Vec<Instruction> = compile(tokens, "main");
  let mut bytes: Vec<u8> = codegen(&instructions);

  bytes.extend(vec![0; 0x100 - bytes.len()]);
  std::fs::write(Path::new(&args[1]).with_extension("bin"), bytes).expect("Unable to write file");

  println!("\nDone.");
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone)]
enum Root {
  Instruction(Instruction),
  Node(Node),
  LabelDef(String),
}

#[derive(Debug, Clone)]
enum Node {
  LabelRef(String),
  Immediate(u8),
  CurrentAddress,
  Not(Box<Node>),
  Add(Box<Node>, Box<Node>),
  Sub(Box<Node>, Box<Node>),
}

#[derive(Debug, Clone, Copy)]
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
  Phs(u8),
  Phl(u8),
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

fn preprocess(filename: &str) -> String {
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
      ),
      None => line.to_string(),
    })
    .collect::<Vec<String>>()
    .join("\n");

  source
}

fn tokenize(source: &str) -> Vec<Token> {
  let tokens: Vec<&str> = source.split_whitespace().collect();

  // tokenize to valid tokens that might not be valid instructions

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

  for token in &tokens {
    match token {
      Token::MacroDef(name) => {
        current_macro_name = name;
        macros.entry(current_macro_name).or_insert(vec![]);
      }
      _ => {
        macros
          .get_mut(current_macro_name)
          .expect("Orphan instructions found")
          .push(*token);
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

  let nodes: Vec<Root> = tokens
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
      Token::Add => Root::Instruction(Instruction::Add(0x00)),
      Token::AdS(size) => Root::Instruction(Instruction::Add(*size)),
      Token::Adc => Root::Instruction(Instruction::Adc(0x00)),
      Token::AcS(size) => Root::Instruction(Instruction::Adc(*size)),
      Token::Sub => Root::Instruction(Instruction::Sub(0x00)),
      Token::SuS(size) => Root::Instruction(Instruction::Sub(*size)),
      Token::Sbc => Root::Instruction(Instruction::Sbc(0x00)),
      Token::ScS(size) => Root::Instruction(Instruction::Sbc(*size)),
      Token::Shf => Root::Instruction(Instruction::Shf(0x00)),
      Token::ShS(size) => Root::Instruction(Instruction::Shf(*size)),
      Token::Rot => Root::Instruction(Instruction::Rot(0x00)),
      Token::RoS(size) => Root::Instruction(Instruction::Rot(*size)),
      Token::Orr => Root::Instruction(Instruction::Orr(0x00)),
      Token::OrS(size) => Root::Instruction(Instruction::Orr(*size)),
      Token::And => Root::Instruction(Instruction::And(0x00)),
      Token::AnS(size) => Root::Instruction(Instruction::And(*size)),
      Token::Xor => Root::Instruction(Instruction::Xor(0x00)),
      Token::XoS(size) => Root::Instruction(Instruction::Xor(*size)),
      Token::Xnd => Root::Instruction(Instruction::Xnd(0x00)),
      Token::XnS(size) => Root::Instruction(Instruction::Xnd(*size)),
      Token::Not => Root::Instruction(Instruction::Not),
      Token::Ntp => Root::Instruction(Instruction::Ntp),
      Token::Buf => Root::Instruction(Instruction::Buf),
      Token::Bfp => Root::Instruction(Instruction::Bfp),
      Token::Iff => Root::Instruction(Instruction::Iff(0x00)),
      Token::IfS(size) => Root::Instruction(Instruction::Iff(*size)),
      Token::Swp => Root::Instruction(Instruction::Swp),
      Token::Pop => Root::Instruction(Instruction::Pop),
      Token::XXX(immediate) => Root::Node(Node::Immediate(*immediate)),
      Token::Lda => Root::Instruction(Instruction::Lda),
      Token::Sta => Root::Instruction(Instruction::Sta),
      Token::Ldi => Root::Instruction(Instruction::Ldi),
      Token::Sti => Root::Instruction(Instruction::Sti),
      Token::Lds => Root::Instruction(Instruction::Lds),
      Token::Sts => Root::Instruction(Instruction::Sts),
      Token::LdO(offset) => Root::Instruction(Instruction::Ldo(*offset)),
      Token::StO(offset) => Root::Instruction(Instruction::Ldo(*offset)),
      Token::DDD(immediate) => Root::Instruction(Instruction::Raw(*immediate)),
    })
    .collect();

  fn match_replace<const N: usize>(
    nodes: &Vec<Root>,
    func: fn(&[Root; N]) -> Option<Vec<Root>>,
  ) -> Vec<Root> {
    let mut output = vec![];

    let mut skip_next = 0;
    for window in nodes.windows(N) {
      if skip_next > 0 {
        skip_next -= 1;
      } else {
        match func(window.try_into().unwrap()) {
          Some(nodes) => {
            output.extend(nodes);
            skip_next = N - 1;
          }
          None => output.push(window[0].clone()),
        }
      }
    }
    output.extend(nodes.iter().skip(nodes.len() - N + skip_next + 1).cloned());

    output
  }

  let mut nodes = nodes;
  let mut last_len = 0;

  while nodes.len() != last_len {
    println!("{:?} {}", nodes, nodes.len());
    last_len = nodes.len();

    nodes = match_replace(&nodes, |window| match window {
      [Root::Instruction(Instruction::Nop)] => Some(vec![]),
      // TODO: this will break in a case like `x05 ldi add`, where the `ldi` instruction
      // is moved somewhere else
      [Root::Instruction(Instruction::Ldi)] => Some(vec![Root::Node(Node::CurrentAddress)]),
      _ => None,
    });

    nodes =
      match_replace(&nodes, |window| match window {
        [Root::Node(eval), Root::Instruction(Instruction::Inc)] => Some(vec![Root::Node(
          Node::Add(Box::new(eval.clone()), Box::new(Node::Immediate(1))),
        )]),
        [Root::Node(eval), Root::Instruction(Instruction::Dec)] => Some(vec![Root::Node(
          Node::Sub(Box::new(eval.clone()), Box::new(Node::Immediate(1))),
        )]),
        [Root::Node(eval), Root::Instruction(Instruction::Not)] => {
          Some(vec![Root::Node(Node::Not(Box::new(eval.clone())))])
        }
        [Root::Node(eval), Root::Instruction(Instruction::Ntp)] => Some(vec![Root::Node(
          Node::Not(Box::new(Node::Not(Box::new(eval.clone())))),
        )]),
        [Root::Node(eval), Root::Instruction(Instruction::Buf)] => {
          Some(vec![Root::Node(eval.clone())])
        }
        [Root::Node(eval), Root::Instruction(Instruction::Bfp)] => {
          Some(vec![Root::Node(eval.clone())])
        }
        [Root::Node(_), Root::Instruction(Instruction::Pop)] => Some(vec![]),
        _ => None,
      });

    nodes = match_replace(&nodes, |window| match window {
      [Root::Node(eval1), Root::Node(eval2), Root::Instruction(Instruction::Add(_))] => {
        Some(vec![Root::Node(Node::Add(
          Box::new(eval2.clone()),
          Box::new(eval1.clone()),
        ))])
      }
      [Root::Node(eval1), Root::Node(eval2), Root::Instruction(Instruction::Adc(_))] => {
        Some(vec![Root::Node(Node::Add(
          Box::new(eval2.clone()),
          Box::new(eval1.clone()),
        ))])
      }
      [Root::Node(eval1), Root::Node(eval2), Root::Instruction(Instruction::Sub(_))] => {
        Some(vec![Root::Node(Node::Sub(
          Box::new(eval2.clone()),
          Box::new(eval1.clone()),
        ))])
      }
      [Root::Node(eval1), Root::Node(eval2), Root::Instruction(Instruction::Sbc(_))] => {
        Some(vec![Root::Node(Node::Sub(
          Box::new(eval2.clone()),
          Box::new(eval1.clone()),
        ))])
      }
      [Root::Node(eval1), Root::Node(eval2), Root::Instruction(Instruction::Swp)] => {
        Some(vec![Root::Node(eval2.clone()), Root::Node(eval1.clone())])
      }
      // TODO: Shf, Rot, Orr, And, Xor, Xnd... see IS
      _ => None,
    });
  }

  // estimate address of label definitions

  let mut label_defs: HashMap<&str, u8> = HashMap::new();
  let mut label_refs: HashMap<u8, &str> = HashMap::new();
  let mut current_address: u8 = 0;

  for token in &tokens {
    current_address += match token {
      Token::LabelDef(_) => 0,
      Token::LabelRef(_) => make_push_instruction(0x00).len() as u8,
      Token::MacroDef(_) => 0,
      Token::MacroRef(_) => make_push_instruction(0x00).len() as u8,
      Token::Nop => 1,
      Token::Hlt => 1,
      Token::Dbg => 1,
      Token::Clc => 1,
      Token::Sec => 1,
      Token::Flc => 1,
      Token::Inc => 1,
      Token::Dec => 1,
      Token::Add => 1,
      Token::AdS(_) => 1,
      Token::Adc => 1,
      Token::AcS(_) => 1,
      Token::Sub => 1,
      Token::SuS(_) => 1,
      Token::Sbc => 1,
      Token::ScS(_) => 1,
      Token::Shf => 1,
      Token::ShS(_) => 1,
      Token::Rot => 1,
      Token::RoS(_) => 1,
      Token::Orr => 1,
      Token::OrS(_) => 1,
      Token::And => 1,
      Token::AnS(_) => 1,
      Token::Xor => 1,
      Token::XoS(_) => 1,
      Token::Xnd => 1,
      Token::XnS(_) => 1,
      Token::Not => 1,
      Token::Ntp => 1,
      Token::Buf => 1,
      Token::Bfp => 1,
      Token::Iff => 1,
      Token::IfS(_) => 1,
      Token::Swp => 1,
      Token::Pop => 1,
      Token::XXX(immediate) => make_push_instruction(*immediate).len() as u8,
      Token::Lda => 1,
      Token::Sta => 1,
      Token::Ldi => 1,
      Token::Sti => 1,
      Token::Lds => 1,
      Token::Sts => 1,
      Token::LdO(_) => 1,
      Token::StO(_) => 1,
      Token::DDD(_) => 1,
    };

    if let Token::LabelDef(label) = token {
      if label_defs.contains_key(label) {
        panic!("Label already defined: {}", label);
      }
      label_defs.insert(label, current_address);
    }
    if let Token::LabelRef(label) = token {
      if label_refs.contains_key(&current_address) {
        panic!("Label already referenced: {}", label);
      }
      label_refs.insert(current_address, label);
    }
  }

  fn assert_immediate<T>(immediate: u8, success: T) -> T {
    match immediate {
      0b00000000..=0b00111111 => success,
      _ => panic!("Invalid immediate: {}", immediate),
    }
  }

  fn assert_size<T>(size: u8, success: T) -> T {
    match size {
      0x01 | 0x02 | 0x04 | 0x08 => success,
      _ => panic!("Invalid size: {}", size),
    }
  }

  fn assert_offset<T>(offset: u8, success: T) -> T {
    match offset {
      0b00000000..=0b00001111 => success,
      _ => panic!("Invalid offset: {}", offset),
    }
  }

  fn make_push_instruction(immediate: u8) -> Vec<Instruction> {
    // `Phs` and `Phl` instructions allow us to push arbitrary 7-bit immediates onto the stack.
    // we then optionally use `Bfp` and `Ntp` to get the ability to push arbitrary 8-bit immediates.

    let lower_bits = immediate & 0b00111111;
    match immediate & 0b11000000 {
      0b00000000 => assert_immediate(
        lower_bits,
        vec![Instruction::Phs(lower_bits), Instruction::Bfp],
      ),
      0b01000000 => assert_immediate(
        lower_bits,
        vec![Instruction::Phl(lower_bits), Instruction::Bfp],
      ),
      0b10000000 => assert_immediate(
        lower_bits,
        vec![Instruction::Phl(lower_bits ^ 0b00111111), Instruction::Ntp],
      ),
      0b11000000 => assert_immediate(
        lower_bits,
        vec![Instruction::Phs(lower_bits ^ 0b00111111), Instruction::Ntp],
      ),
      _ => unreachable!(),
    }
  }

  // compile tokens into valid instructions and resolve labels

  let instructions: Vec<Instruction> = tokens
    .iter()
    .flat_map(|instruction| match instruction {
      Token::LabelDef(_) => vec![],
      Token::LabelRef(label) => make_push_instruction(
        *label_defs
          .get(label)
          .expect(&format!("Could not find label: {}", label)) as u8,
      ),
      Token::MacroDef(_) => vec![],
      Token::MacroRef(_) => unreachable!(),
      Token::Nop => vec![Instruction::Nop],
      Token::Hlt => vec![Instruction::Hlt],
      Token::Dbg => vec![Instruction::Dbg],
      Token::Clc => vec![Instruction::Clc],
      Token::Sec => vec![Instruction::Sec],
      Token::Flc => vec![Instruction::Flc],
      Token::Inc => vec![Instruction::Inc],
      Token::Dec => vec![Instruction::Dec],
      Token::Add => vec![Instruction::Add(0x01)],
      Token::AdS(size) => assert_size(*size, vec![Instruction::Add(*size)]),
      Token::Adc => vec![Instruction::Adc(0x01)],
      Token::AcS(size) => assert_size(*size, vec![Instruction::Adc(*size)]),
      Token::Sub => vec![Instruction::Sub(0x01)],
      Token::SuS(size) => assert_size(*size, vec![Instruction::Sub(*size)]),
      Token::Sbc => vec![Instruction::Sbc(0x01)],
      Token::ScS(size) => assert_size(*size, vec![Instruction::Sbc(*size)]),
      Token::Shf => vec![Instruction::Shf(0x01)],
      Token::ShS(size) => assert_size(*size, vec![Instruction::Shf(*size)]),
      Token::Rot => vec![Instruction::Rot(0x01)],
      Token::RoS(size) => assert_size(*size, vec![Instruction::Rot(*size)]),
      Token::Orr => vec![Instruction::Orr(0x01)],
      Token::OrS(size) => assert_size(*size, vec![Instruction::Orr(*size)]),
      Token::And => vec![Instruction::And(0x01)],
      Token::AnS(size) => assert_size(*size, vec![Instruction::And(*size)]),
      Token::Xor => vec![Instruction::Xor(0x01)],
      Token::XoS(size) => assert_size(*size, vec![Instruction::Xor(*size)]),
      Token::Xnd => vec![Instruction::Xnd(0x01)],
      Token::XnS(size) => assert_size(*size, vec![Instruction::Xnd(*size)]),
      Token::Not => vec![Instruction::Not],
      Token::Ntp => vec![Instruction::Ntp],
      Token::Buf => vec![Instruction::Buf],
      Token::Bfp => vec![Instruction::Bfp],
      Token::Iff => vec![Instruction::Iff(0x01)],
      Token::IfS(size) => assert_size(*size, vec![Instruction::Iff(*size)]),
      Token::Swp => vec![Instruction::Swp],
      Token::Pop => vec![Instruction::Pop],
      Token::XXX(immediate) => make_push_instruction(*immediate),
      Token::Lda => vec![Instruction::Lda],
      Token::Sta => vec![Instruction::Sta],
      Token::Ldi => vec![Instruction::Ldi],
      Token::Sti => vec![Instruction::Sti],
      Token::Lds => vec![Instruction::Lds],
      Token::Sts => vec![Instruction::Sts],
      Token::LdO(offset) => assert_offset(*offset, vec![Instruction::Ldo(*offset)]),
      Token::StO(offset) => assert_offset(*offset, vec![Instruction::Sto(*offset)]),
      Token::DDD(immediate) => vec![Instruction::Raw(*immediate)],
    })
    .collect();

  instructions
}

fn codegen(instructions: &Vec<Instruction>) -> Vec<u8> {
  fn encode_immediate(immediate: u8) -> u8 {
    match immediate {
      0b00000000..=0b00111111 => immediate,
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
      Instruction::Phs(immediate) => vec![0b00000000 | encode_immediate(*immediate)],
      Instruction::Phl(immediate) => vec![0b01000000 | encode_immediate(*immediate)],
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
