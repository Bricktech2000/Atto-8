use std::collections::HashMap;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 2 {
    println!("Usage: asm <filename>");
    std::process::exit(1);
  }

  let source: String = std::fs::read_to_string(&args[1]).expect("Unable to read file.");
  let instructions: Vec<Instruction> = parse(&source);
  let mut bytes: Vec<u8> = assemble(&instructions);
  bytes.extend(vec![0; 0x100 - bytes.len()]);
  std::fs::write(format!("{}.bin", &args[1]), bytes).expect("Unable to write file.");

  println!("");
  println!("Done.");
}

#[derive(Debug, Clone)]
enum Mnemonic {
  Nop,
  Hlt,
  Dbg,
  Clc,
  Sec,
  Flc,
  Inc,
  Dec,
  Add,
  Sub,
  Rol,
  Ror,
  Oor,
  And,
  Xor,
  Xnd,
  Not,
  Iif,
  Swp,
  Dup,
  Str,
  Pop,
  XXX(u8),
  AWW(u8),
  DDD(u8),
  Ldi,
  Sti,
  Ldw,
  Stw,
  Lds,
  Sts,
  LabelDef(String),
  LabelRef(String),
  MacroDef(String),
  MacroRef(String),
}

#[derive(Debug, Clone)]
enum Instruction {
  Nop,
  Hlt,
  Dbg,
  Clc,
  Sec,
  Flc,
  Inc,
  Dec,
  Add,
  Sub,
  Rol,
  Ror,
  Oor,
  And,
  Xor,
  Xnd,
  Not,
  Iif,
  Swp,
  Dup,
  Str,
  Pop,
  PushPositive(u8),
  PushNegative(u8),
  PushNext(u8),
  RelativeWork(u8),
  RawData(u8),
  Ldi,
  Sti,
  Ldw,
  Stw,
  Lds,
  Sts,
}

fn parse(source: &String) -> Vec<Instruction> {
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
      &"sub" => Mnemonic::Sub,
      &"rol" => Mnemonic::Rol,
      &"ror" => Mnemonic::Ror,
      &"oor" => Mnemonic::Oor,
      &"and" => Mnemonic::And,
      &"xor" => Mnemonic::Xor,
      &"xnd" => Mnemonic::Xnd,
      &"not" => Mnemonic::Not,
      &"iif" => Mnemonic::Iif,
      &"swp" => Mnemonic::Swp,
      &"dup" => Mnemonic::Dup,
      &"str" => Mnemonic::Str,
      &"pop" => Mnemonic::Pop,
      &_ if token.starts_with("x") => Mnemonic::XXX(u8::from_str_radix(&token[1..], 16).unwrap()),
      &_ if token.starts_with("@") => Mnemonic::AWW(u8::from_str_radix(&token[1..], 16).unwrap()),
      &_ if token.starts_with("d") => Mnemonic::DDD(u8::from_str_radix(&token[1..], 16).unwrap()),
      &"ldi" => Mnemonic::Ldi,
      &"sti" => Mnemonic::Sti,
      &"ldw" => Mnemonic::Ldw,
      &"stw" => Mnemonic::Stw,
      &"lds" => Mnemonic::Lds,
      &"sts" => Mnemonic::Sts,
      &_ => panic!("Unknown token: {}", token),
    })
    .collect();

  let mut labels: HashMap<&str, u8> = HashMap::new();
  let mut macros: HashMap<&str, u8> = HashMap::new();
  let mut current_address: u8 = 0;

  for (index, mnemonic) in mnemonics.iter().enumerate() {
    match mnemonic {
      Mnemonic::LabelDef(label) => {
        labels.insert(label, current_address);
      }
      Mnemonic::MacroDef(name) => {
        match mnemonics[index + 1] {
          Mnemonic::DDD(value) => {
            macros.insert(name, value);
          }
          _ => panic!("Macro definition must be followed by exactly one raw data instruction."),
        };
      }
      Mnemonic::XXX(immediate) if *immediate & 0b11000000 == 0b00000000 => {
        // push positive
        current_address += 1;
      }
      Mnemonic::XXX(immediate) if *immediate & 0b11000000 == 0b11000000 => {
        // push negative
        current_address += 1;
      }
      Mnemonic::MacroRef(_) | Mnemonic::LabelRef(_) | Mnemonic::XXX(_) => {
        // push next
        current_address += 2;
      }
      _ => {
        current_address += 1;
      }
    }
  }

  let instructions: Vec<Instruction> = mnemonics
    .iter()
    .flat_map(|instruction| match instruction {
      Mnemonic::Nop => vec![Instruction::Nop],
      Mnemonic::Hlt => vec![Instruction::Hlt],
      Mnemonic::Dbg => vec![Instruction::Dbg],
      Mnemonic::Clc => vec![Instruction::Clc],
      Mnemonic::Sec => vec![Instruction::Sec],
      Mnemonic::Flc => vec![Instruction::Flc],
      Mnemonic::Inc => vec![Instruction::Inc],
      Mnemonic::Dec => vec![Instruction::Dec],
      Mnemonic::Add => vec![Instruction::Add],
      Mnemonic::Sub => vec![Instruction::Sub],
      Mnemonic::Rol => vec![Instruction::Rol],
      Mnemonic::Ror => vec![Instruction::Ror],
      Mnemonic::Oor => vec![Instruction::Oor],
      Mnemonic::And => vec![Instruction::And],
      Mnemonic::Xor => vec![Instruction::Xor],
      Mnemonic::Xnd => vec![Instruction::Xnd],
      Mnemonic::Not => vec![Instruction::Not],
      Mnemonic::Iif => vec![Instruction::Iif],
      Mnemonic::Swp => vec![Instruction::Swp],
      Mnemonic::Dup => vec![Instruction::Dup],
      Mnemonic::Str => vec![Instruction::Str],
      Mnemonic::Pop => vec![Instruction::Pop],
      Mnemonic::XXX(immediate) => match immediate {
        _ if immediate & 0b11000000 == 0b00000000 => vec![Instruction::PushPositive(*immediate)],
        _ if immediate & 0b11000000 == 0b11000000 => vec![Instruction::PushNegative(*immediate)],
        _ => vec![Instruction::PushNext(*immediate)],
      },
      Mnemonic::AWW(immediate) => vec![Instruction::RelativeWork(*immediate)],
      Mnemonic::DDD(immediate) => vec![Instruction::RawData(*immediate)],
      Mnemonic::Ldi => vec![Instruction::Ldi],
      Mnemonic::Sti => vec![Instruction::Sti],
      Mnemonic::Ldw => vec![Instruction::Ldw],
      Mnemonic::Stw => vec![Instruction::Stw],
      Mnemonic::Lds => vec![Instruction::Lds],
      Mnemonic::Sts => vec![Instruction::Sts],
      Mnemonic::LabelRef(label) => {
        vec![Instruction::PushNext(
          *labels
            .get(label.as_str())
            .expect(format!("Unknown label: {}", label).as_str()),
        )]
      }
      Mnemonic::MacroRef(name) => vec![Instruction::PushNext(
        *macros
          .get(name.as_str())
          .expect(format!("Unknown macro: {:?}", name).as_str()),
      )],
      Mnemonic::LabelDef(_) | Mnemonic::MacroDef(_) => vec![],
    })
    .collect();

  instructions
}

fn assemble(instructions: &Vec<Instruction>) -> Vec<u8> {
  // flat_map
  instructions
    .iter()
    .flat_map(|instruction| match instruction {
      Instruction::Nop => vec![0x80],
      Instruction::Hlt => vec![0x81],
      Instruction::Dbg => vec![0x88],
      Instruction::Clc => vec![0x82],
      Instruction::Sec => vec![0x83],
      Instruction::Flc => vec![0x84],
      Instruction::Inc => vec![0xA0],
      Instruction::Dec => vec![0xA1],
      Instruction::Add => vec![0xA2],
      Instruction::Sub => vec![0xA3],
      Instruction::Rol => vec![0xA4],
      Instruction::Ror => vec![0xA5],
      Instruction::Oor => vec![0xA6],
      Instruction::And => vec![0xA7],
      Instruction::Xor => vec![0xA8],
      Instruction::Xnd => vec![0xA9],
      Instruction::Not => vec![0xAA],
      Instruction::Iif => vec![0x90],
      Instruction::Swp => vec![0x91],
      Instruction::Dup => vec![0x92],
      Instruction::Str => vec![0x93],
      Instruction::Pop => vec![0x94],
      Instruction::PushPositive(immediate) => vec![0b00000000 | immediate],
      Instruction::PushNegative(immediate) => vec![0b11000000 | immediate],
      Instruction::PushNext(immediate) => vec![0x95, *immediate],
      Instruction::RelativeWork(offset) => vec![0b01000000 | offset],
      Instruction::RawData(data) => vec![*data],
      Instruction::Ldi => vec![0x96],
      Instruction::Sti => vec![0x97],
      Instruction::Ldw => vec![0x98],
      Instruction::Stw => vec![0x99],
      Instruction::Lds => vec![0x9A],
      Instruction::Sts => vec![0x9B],
    })
    .collect()
}
