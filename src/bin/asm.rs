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
enum Mnemonic {
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
  Phb(u8),
  Php(u8),
  Phn(u8),
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

  let mnemonics: Vec<Mnemonic> = tokens
    .iter()
    .map(|token| match token {
      &_ if token.ends_with(":") => Mnemonic::LabelDef(token[..token.len() - 1].to_string()),
      &_ if token.starts_with(":") => Mnemonic::LabelRef(token[1..].to_string()),
      &_ if token.ends_with("%") => Mnemonic::MacroDef(token[..token.len() - 1].to_string()),
      &_ if token.starts_with("%") => Mnemonic::MacroRef(token[1..].to_string()),
      &"nop" => Mnemonic::Nop,
      &"hlt" => Mnemonic::Hlt,
      &"dbg" => Mnemonic::Dbg,
      &"clc" => Mnemonic::Clc,
      &"sec" => Mnemonic::Sec,
      &"flc" => Mnemonic::Flc,
      &"inc" => Mnemonic::Inc,
      &"dec" => Mnemonic::Dec,
      &"add" => Mnemonic::Add,
      &_ if token.starts_with("ad") => Mnemonic::AdS(u8::from_str_radix(&token[2..], 16).unwrap()),
      &"adc" => Mnemonic::Adc,
      &_ if token.starts_with("ac") => Mnemonic::AcS(u8::from_str_radix(&token[2..], 16).unwrap()),
      &"sub" => Mnemonic::Sub,
      &_ if token.starts_with("su") => Mnemonic::SuS(u8::from_str_radix(&token[2..], 16).unwrap()),
      &"sbc" => Mnemonic::Sbc,
      &_ if token.starts_with("sc") => Mnemonic::ScS(u8::from_str_radix(&token[2..], 16).unwrap()),
      &"shf" => Mnemonic::Shf,
      &_ if token.starts_with("sh") => Mnemonic::ShS(u8::from_str_radix(&token[2..], 16).unwrap()),
      &"rot" => Mnemonic::Rot,
      &_ if token.starts_with("ro") => Mnemonic::RoS(u8::from_str_radix(&token[2..], 16).unwrap()),
      &"orr" => Mnemonic::Orr,
      &_ if token.starts_with("or") => Mnemonic::OrS(u8::from_str_radix(&token[2..], 16).unwrap()),
      &"and" => Mnemonic::And,
      &_ if token.starts_with("an") => Mnemonic::AnS(u8::from_str_radix(&token[2..], 16).unwrap()),
      &"xor" => Mnemonic::Xor,
      &_ if token.starts_with("xo") => Mnemonic::XoS(u8::from_str_radix(&token[2..], 16).unwrap()),
      &"xnd" => Mnemonic::Xnd,
      &_ if token.starts_with("xn") => Mnemonic::XnS(u8::from_str_radix(&token[2..], 16).unwrap()),
      &"not" => Mnemonic::Not,
      &"buf" => Mnemonic::Buf,
      &"iff" => Mnemonic::Iff,
      &_ if token.starts_with("if") => Mnemonic::IfS(u8::from_str_radix(&token[2..], 16).unwrap()),
      &"swp" => Mnemonic::Swp,
      &"pop" => Mnemonic::Pop,
      &_ if token.starts_with("x") => Mnemonic::XXX(u8::from_str_radix(&token[1..], 16).unwrap()),
      &"lda" => Mnemonic::Lda,
      &"sta" => Mnemonic::Sta,
      &"ldi" => Mnemonic::Ldi,
      &"sti" => Mnemonic::Sti,
      &"lds" => Mnemonic::Lds,
      &"sts" => Mnemonic::Sts,
      &"ldo" => Mnemonic::Ldo,
      &_ if token.starts_with("ld") => Mnemonic::LdO(u8::from_str_radix(&token[2..], 16).unwrap()),
      &"sto" => Mnemonic::Sto,
      &_ if token.starts_with("st") => Mnemonic::StO(u8::from_str_radix(&token[2..], 16).unwrap()),
      &_ if token.starts_with("d") => Mnemonic::DDD(u8::from_str_radix(&token[1..], 16).unwrap()),
      &_ => panic!("Unknown token: {}", token),
    })
    .collect();

  let mut macros: HashMap<&str, Vec<Mnemonic>> = HashMap::new();
  let mut current_macro_name = "".to_string();

  for mnemonic in &mnemonics {
    match mnemonic {
      Mnemonic::MacroDef(name) => {
        current_macro_name = name.clone();
        macros.entry(name.as_str()).or_insert(vec![]);
      }
      _ => {
        macros
          .get_mut(current_macro_name.as_str())
          .expect("Orphan instructions found")
          .push(mnemonic.clone());
      }
    }
  }

  let mnemonics: Vec<Mnemonic> =
    expand_macros(&macros, &vec![Mnemonic::MacroRef(entry_point.to_string())]);

  let mut labels: HashMap<&str, u8> = HashMap::new();
  let mut current_address: u8 = 0;

  for mnemonic in &mnemonics {
    current_address += match mnemonic {
      Mnemonic::LabelDef(_) => 0,
      Mnemonic::LabelRef(_) => 2,
      Mnemonic::MacroDef(_) => 0,
      Mnemonic::MacroRef(_) => 2,
      Mnemonic::Nop => 1,
      Mnemonic::Hlt => 1,
      Mnemonic::Dbg => 1,
      Mnemonic::Clc => 1,
      Mnemonic::Sec => 1,
      Mnemonic::Flc => 1,
      Mnemonic::Inc => 1,
      Mnemonic::Dec => 1,
      Mnemonic::Add => 1,
      Mnemonic::AdS(_) => 1,
      Mnemonic::Adc => 1,
      Mnemonic::AcS(_) => 1,
      Mnemonic::Sub => 1,
      Mnemonic::SuS(_) => 1,
      Mnemonic::Sbc => 1,
      Mnemonic::ScS(_) => 1,
      Mnemonic::Shf => 1,
      Mnemonic::ShS(_) => 1,
      Mnemonic::Rot => 1,
      Mnemonic::RoS(_) => 1,
      Mnemonic::Orr => 1,
      Mnemonic::OrS(_) => 1,
      Mnemonic::And => 1,
      Mnemonic::AnS(_) => 1,
      Mnemonic::Xor => 1,
      Mnemonic::XoS(_) => 1,
      Mnemonic::Xnd => 1,
      Mnemonic::XnS(_) => 1,
      Mnemonic::Not => 1,
      Mnemonic::Buf => 1,
      Mnemonic::Iff => 1,
      Mnemonic::IfS(_) => 1,
      Mnemonic::Swp => 1,
      Mnemonic::Pop => 1,
      Mnemonic::XXX(immediate) => match immediate {
        _ if immediate & 0b11000000 == 0b00000000 => 1,
        _ if immediate & 0b11000000 == 0b11000000 => 1,
        _ => 2,
      },
      Mnemonic::Lda => 1,
      Mnemonic::Sta => 1,
      Mnemonic::Ldi => 1,
      Mnemonic::Sti => 1,
      Mnemonic::Lds => 1,
      Mnemonic::Sts => 1,
      Mnemonic::Ldo => 1,
      Mnemonic::LdO(_) => 1,
      Mnemonic::Sto => 1,
      Mnemonic::StO(_) => 1,
      Mnemonic::DDD(_) => 1,
    };

    if let Mnemonic::LabelDef(label) = mnemonic {
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

  let instructions: Vec<Instruction> = mnemonics
    .iter()
    .flat_map(|instruction| match instruction {
      Mnemonic::LabelDef(_) => vec![],
      Mnemonic::LabelRef(label) => {
        vec![Instruction::Phb(
          *labels
            .get(label.as_str())
            .expect(format!("Could not find label: {}", label).as_str()) as u8,
        )]
      }
      Mnemonic::MacroDef(_) => vec![],
      Mnemonic::MacroRef(_) => unreachable!(),
      Mnemonic::Nop => vec![Instruction::Nop],
      Mnemonic::Hlt => vec![Instruction::Hlt],
      Mnemonic::Dbg => vec![Instruction::Dbg],
      Mnemonic::Clc => vec![Instruction::Clc],
      Mnemonic::Sec => vec![Instruction::Sec],
      Mnemonic::Flc => vec![Instruction::Flc],
      Mnemonic::Inc => vec![Instruction::Inc(0x01)],
      Mnemonic::Dec => vec![Instruction::Dec(0x01)],
      Mnemonic::Add => vec![Instruction::Add(0x01)],
      Mnemonic::AdS(size) => assert_size(*size, vec![Instruction::Add(*size)]),
      Mnemonic::Adc => vec![Instruction::Adc(0x01)],
      Mnemonic::AcS(size) => assert_size(*size, vec![Instruction::Adc(*size)]),
      Mnemonic::Sub => vec![Instruction::Sub(0x01)],
      Mnemonic::SuS(size) => assert_size(*size, vec![Instruction::Sub(*size)]),
      Mnemonic::Sbc => vec![Instruction::Sbc(0x01)],
      Mnemonic::ScS(size) => assert_size(*size, vec![Instruction::Sbc(*size)]),
      Mnemonic::Shf => vec![Instruction::Shf(0x01)],
      Mnemonic::ShS(size) => assert_size(*size, vec![Instruction::Shf(*size)]),
      Mnemonic::Rot => vec![Instruction::Rot(0x01)],
      Mnemonic::RoS(size) => assert_size(*size, vec![Instruction::Rot(*size)]),
      Mnemonic::Orr => vec![Instruction::Orr(0x01)],
      Mnemonic::OrS(size) => assert_size(*size, vec![Instruction::Orr(*size)]),
      Mnemonic::And => vec![Instruction::And(0x01)],
      Mnemonic::AnS(size) => assert_size(*size, vec![Instruction::And(*size)]),
      Mnemonic::Xor => vec![Instruction::Xor(0x01)],
      Mnemonic::XoS(size) => assert_size(*size, vec![Instruction::Xor(*size)]),
      Mnemonic::Xnd => vec![Instruction::Xnd(0x01)],
      Mnemonic::XnS(size) => assert_size(*size, vec![Instruction::Xnd(*size)]),
      Mnemonic::Not => vec![Instruction::Not(0x01)],
      Mnemonic::Buf => vec![Instruction::Buf(0x01)],
      Mnemonic::Iff => vec![Instruction::Iff(0x01)],
      Mnemonic::IfS(size) => assert_size(*size, vec![Instruction::Iff(*size)]),
      Mnemonic::Swp => vec![Instruction::Swp],
      Mnemonic::Pop => vec![Instruction::Pop],
      Mnemonic::XXX(immediate) => assert_immediate(
        *immediate,
        match immediate {
          _ if immediate & 0b11000000 == 0b00000000 => vec![Instruction::Php(*immediate)],
          _ if immediate & 0b11000000 == 0b11000000 => vec![Instruction::Phn(*immediate)],
          _ => vec![Instruction::Phb(*immediate)],
        },
      ),
      Mnemonic::Lda => vec![Instruction::Lda],
      Mnemonic::Sta => vec![Instruction::Sta],
      Mnemonic::Ldi => vec![Instruction::Ldi],
      Mnemonic::Sti => vec![Instruction::Sti],
      Mnemonic::Lds => vec![Instruction::Lds],
      Mnemonic::Sts => vec![Instruction::Sts],
      Mnemonic::Ldo => vec![Instruction::Ldo(0)],
      Mnemonic::LdO(offset) => assert_offset(*offset, vec![Instruction::Ldo(*offset)]),
      Mnemonic::Sto => vec![Instruction::Sto(0)],
      Mnemonic::StO(offset) => assert_offset(*offset, vec![Instruction::Sto(*offset)]),
      Mnemonic::DDD(immediate) => vec![Instruction::Raw(*immediate)],
    })
    .collect();

  instructions
}

fn expand_macros(
  macros: &HashMap<&str, Vec<Mnemonic>>,
  mnemonics: &Vec<Mnemonic>,
) -> Vec<Mnemonic> {
  mnemonics
    .iter()
    .flat_map(|mnemonic| match mnemonic {
      Mnemonic::MacroRef(name) => {
        let mnemonics = macros
          .get(name.as_str())
          .expect(format!("Could not find macro: {}", name).as_str())
          .clone();
        expand_macros(&macros, &mnemonics)
      }
      _ => vec![mnemonic.clone()],
    })
    .collect()
}

fn assemble(instructions: &Vec<Instruction>) -> Vec<u8> {
  fn encode_immediate(immediate: u8) -> u8 {
    immediate
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
    offset
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
      Instruction::Inc(size) => vec![0x40 | encode_size(*size)],
      Instruction::Dec(size) => vec![0x41 | encode_size(*size)],
      Instruction::Add(size) => vec![0x42 | encode_size(*size)],
      Instruction::Adc(size) => vec![0x43 | encode_size(*size)],
      Instruction::Sub(size) => vec![0x44 | encode_size(*size)],
      Instruction::Sbc(size) => vec![0x45 | encode_size(*size)],
      Instruction::Shf(size) => vec![0x46 | encode_size(*size)],
      Instruction::Rot(size) => vec![0x47 | encode_size(*size)],
      Instruction::Orr(size) => vec![0x48 | encode_size(*size)],
      Instruction::And(size) => vec![0x49 | encode_size(*size)],
      Instruction::Xor(size) => vec![0x4A | encode_size(*size)],
      Instruction::Xnd(size) => vec![0x4B | encode_size(*size)],
      Instruction::Not(size) => vec![0x4C | encode_size(*size)],
      Instruction::Buf(size) => vec![0x4D | encode_size(*size)],
      Instruction::Iff(size) => vec![0x4E | encode_size(*size)],
      Instruction::Swp => vec![0xB0],
      Instruction::Pop => vec![0xB1],
      Instruction::Phb(immediate) => vec![0xB2, encode_immediate(*immediate)],
      Instruction::Php(immediate) => vec![0b00000000 | encode_immediate(*immediate)],
      Instruction::Phn(immediate) => vec![0b11000000 | encode_immediate(*immediate)],
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
