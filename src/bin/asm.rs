use std::collections::HashMap;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 2 {
    println!("Usage: asm <filename>");
    std::process::exit(1);
  }

  let source: String = std::fs::read_to_string(&args[1]).expect("Unable to read file");
  let instructions: Vec<Instruction> = parse(&source, "main");
  let mut bytes: Vec<u8> = assemble(&instructions);
  bytes.extend(vec![0; 0x100 - bytes.len()]);
  std::fs::write(format!("{}.bin", &args[1]), bytes).expect("Unable to write file");

  println!("");
  println!("Done.");
}

#[derive(Debug, Clone)]
enum Token {
  LabelDef(String),
  LabelRef(String),
  MacroDef(String),
  MacroRef(String),
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
  Buf,
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
  Ldo,
  LdO(u8),
  Sto,
  StO(u8),
  DDD(u8),
}

#[derive(Debug, Clone)]
enum Instruction {
  Nop,
  Hlt,
  Dbg,
  Clc,
  Sec,
  Flc,
  Inc(u8),
  Dec(u8),
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
  Not(u8),
  Buf(u8),
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

fn parse(source: &str, entry_point: &str) -> Vec<Instruction> {
  let source = source
    .lines()
    .map(|line| line.split("#").next().unwrap())
    .collect::<Vec<&str>>()
    .join("\n");

  let tokens: Vec<&str> = source.split_whitespace().collect();

  let tokens: Vec<Token> = tokens
    .iter()
    .map(|token| match token {
      &_ if token.ends_with(":") => Token::LabelDef(token[..token.len() - 1].to_string()),
      &_ if token.starts_with(":") => Token::LabelRef(token[1..].to_string()),
      &_ if token.ends_with("%") => Token::MacroDef(token[..token.len() - 1].to_string()),
      &_ if token.starts_with("%") => Token::MacroRef(token[1..].to_string()),
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
      &"buf" => Token::Buf,
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
      &"ldo" => Token::Ldo,
      &_ if token.starts_with("ld") => Token::LdO(u8::from_str_radix(&token[2..], 16).unwrap()),
      &"sto" => Token::Sto,
      &_ if token.starts_with("st") => Token::StO(u8::from_str_radix(&token[2..], 16).unwrap()),
      &_ if token.starts_with("d") => Token::DDD(u8::from_str_radix(&token[1..], 16).unwrap()),
      &_ => panic!("Unknown token: {}", token),
    })
    .collect();

  let mut macros: HashMap<&str, Vec<Token>> = HashMap::new();
  let mut current_macro_name = "".to_string();

  for token in &tokens {
    match token {
      Token::MacroDef(name) => {
        current_macro_name = name.clone();
        macros.entry(name.as_str()).or_insert(vec![]);
      }
      _ => {
        macros
          .get_mut(current_macro_name.as_str())
          .expect("Orphan instructions found")
          .push(token.clone());
      }
    }
  }

  let tokens: Vec<Token> = expand_macros(&macros, &vec![Token::MacroRef(entry_point.to_string())]);

  fn expand_macros(macros: &HashMap<&str, Vec<Token>>, tokens: &Vec<Token>) -> Vec<Token> {
    tokens
      .iter()
      .flat_map(|token| match token {
        Token::MacroRef(name) => {
          let tokens = macros
            .get(name.as_str())
            .expect(format!("Could not find macro: {}", name).as_str())
            .clone();
          expand_macros(&macros, &tokens)
        }
        _ => vec![token.clone()],
      })
      .collect()
  }

  let mut labels: HashMap<&str, u8> = HashMap::new();
  let mut current_address: u8 = 0;

  fn make_push_instruction(immediate: u8, pad: bool) -> Vec<Instruction> {
    // note: if `pad` is true, all output vectors must be the same length
    match immediate & 0b11000000 {
      0b00000000 => vec![Instruction::Phs(immediate & 0b00111111)]
        .into_iter()
        .chain(if pad { vec![Instruction::Nop] } else { vec![] })
        .collect(),
      0b01000000 => vec![Instruction::Phl(immediate & 0b00111111)]
        .into_iter()
        .chain(if pad { vec![Instruction::Nop] } else { vec![] })
        .collect(),
      0b10000000 => vec![
        Instruction::Phl(!immediate & 0b00111111),
        Instruction::Not(0x01),
      ],
      0b11000000 => vec![
        Instruction::Phs(!immediate & 0b00111111),
        Instruction::Not(0x01),
      ],
      _ => unreachable!(),
    }
  }

  for token in &tokens {
    current_address += match token {
      Token::LabelDef(_) => 0,
      Token::LabelRef(_) => make_push_instruction(0x00, true).len() as u8,
      Token::MacroDef(_) => 0,
      Token::MacroRef(_) => make_push_instruction(0x00, true).len() as u8,
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
      Token::Buf => 1,
      Token::Iff => 1,
      Token::IfS(_) => 1,
      Token::Swp => 1,
      Token::Pop => 1,
      Token::XXX(immediate) => make_push_instruction(*immediate, false).len() as u8,
      Token::Lda => 1,
      Token::Sta => 1,
      Token::Ldi => 1,
      Token::Sti => 1,
      Token::Lds => 1,
      Token::Sts => 1,
      Token::Ldo => 1,
      Token::LdO(_) => 1,
      Token::Sto => 1,
      Token::StO(_) => 1,
      Token::DDD(_) => 1,
    };

    if let Token::LabelDef(label) = token {
      if labels.contains_key(label.as_str()) {
        panic!("Label already defined: {}", label);
      }
      labels.insert(label, current_address);
    }
  }

  fn assert_immediate<T>(immediate: u8, success: T) -> T {
    match immediate {
      0x00..=0xFF => success,
      #[allow(unreachable_patterns)]
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
      0x00..=0x0F => success,
      _ => panic!("Invalid offset: {}", offset),
    }
  }

  let instructions: Vec<Instruction> = tokens
    .iter()
    .flat_map(|instruction| match instruction {
      Token::LabelDef(_) => vec![],
      Token::LabelRef(label) => make_push_instruction(
        *labels
          .get(label.as_str())
          .expect(format!("Could not find label: {}", label).as_str()) as u8,
        true,
      ),
      Token::MacroDef(_) => vec![],
      Token::MacroRef(_) => unreachable!(),
      Token::Nop => vec![Instruction::Nop],
      Token::Hlt => vec![Instruction::Hlt],
      Token::Dbg => vec![Instruction::Dbg],
      Token::Clc => vec![Instruction::Clc],
      Token::Sec => vec![Instruction::Sec],
      Token::Flc => vec![Instruction::Flc],
      Token::Inc => vec![Instruction::Inc(0x01)],
      Token::Dec => vec![Instruction::Dec(0x01)],
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
      Token::Not => vec![Instruction::Not(0x01)],
      Token::Buf => vec![Instruction::Buf(0x01)],
      Token::Iff => vec![Instruction::Iff(0x01)],
      Token::IfS(size) => assert_size(*size, vec![Instruction::Iff(*size)]),
      Token::Swp => vec![Instruction::Swp],
      Token::Pop => vec![Instruction::Pop],
      Token::XXX(immediate) => {
        assert_immediate(*immediate, make_push_instruction(*immediate, false))
      }
      Token::Lda => vec![Instruction::Lda],
      Token::Sta => vec![Instruction::Sta],
      Token::Ldi => vec![Instruction::Ldi],
      Token::Sti => vec![Instruction::Sti],
      Token::Lds => vec![Instruction::Lds],
      Token::Sts => vec![Instruction::Sts],
      Token::Ldo => vec![Instruction::Ldo(0)],
      Token::LdO(offset) => assert_offset(*offset, vec![Instruction::Ldo(*offset)]),
      Token::Sto => vec![Instruction::Sto(0)],
      Token::StO(offset) => assert_offset(*offset, vec![Instruction::Sto(*offset)]),
      Token::DDD(immediate) => vec![Instruction::Raw(*immediate)],
    })
    .collect();

  instructions
}

fn assemble(instructions: &Vec<Instruction>) -> Vec<u8> {
  fn encode_immediate(immediate: u8) -> u8 {
    match immediate {
      0b00000000..=0b00111111 => immediate,
      #[allow(unreachable_patterns)]
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

  instructions
    .iter()
    .flat_map(|instruction| match instruction {
      Instruction::Nop => vec![0xA0],
      Instruction::Hlt => vec![0xAF],
      Instruction::Dbg => vec![0xAA],
      Instruction::Clc => vec![0xA1],
      Instruction::Sec => vec![0xA2],
      Instruction::Flc => vec![0xA3],
      Instruction::Inc(size) => vec![0xC0 | encode_size(*size)],
      Instruction::Dec(size) => vec![0xC1 | encode_size(*size)],
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
      Instruction::Not(size) => vec![0xCC | encode_size(*size)],
      Instruction::Buf(size) => vec![0xCD | encode_size(*size)],
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
