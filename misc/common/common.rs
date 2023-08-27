#![allow(dead_code)]

use std::collections::VecDeque;

pub const MEM_SIZE: usize = 0x100;
pub const MIC_SIZE: usize = 0x80 * 0x02 * 0x20;
pub const DISPLAY_BUFFER: usize = 0xE0;

#[derive(Clone, Copy, Debug, Default)]
pub struct ControlWord {
  pub clr_sc: Signal,       // clear to step counter
  pub data_il: Signal,      // data bus to instruction latch
  pub size_and_cin: Signal, // size and carry in
  pub ofst_and_cf: Signal,  // offset and carry flag

  pub ip_data: Signal, // instruction pointer to data bus
  pub data_ip: Signal, // data bus to instruction pointer

  pub sp_data: Signal, // stack pointer to data bus
  pub data_sp: Signal, // data bus to stack pointer

  pub data_al: Signal,  // data bus to address latch
  pub mem_data: Signal, // data bus to memory
  pub data_mem: Signal, // memory to data bus

  pub data_xl: Signal,   // data bus to X latch
  pub data_yl: Signal,   // data bus to Y latch
  pub data_zl: Signal,   // data bus to Z latch
  pub sum_data: Signal,  // sum to data bus
  pub nand_data: Signal, // not-and to data bus
}

#[derive(Clone, Copy, Debug, Default)]
pub enum Signal {
  #[default]
  Inactive,
  Active,
}

#[derive(Clone, Copy, Debug)]
pub enum TickTrap {
  MicrocodeFault,
  IllegalOpcode,
  DebugRequest,
  BusFault,
}

pub trait Tickable {
  fn reset(
    &mut self,
    stdin: &mut VecDeque<u8>,
    stdout: &mut VecDeque<u8>,
    display: &mut [u8; 0x20],
    controller: &mut u8,
  );
  fn tick(
    &mut self,
    stdin: &mut VecDeque<u8>,
    stdout: &mut VecDeque<u8>,
    display: &mut [u8; 0x20],
    controller: &mut u8,
  ) -> Result<u128, TickTrap>;
}

pub fn execute<MC: std::fmt::Display + Tickable>(mut mc: MC, clock_speed: u128) {
  let mut start_time = std::time::Instant::now();
  let mut next_print_time = std::time::Instant::now();
  let mut current_clocks = 0;
  let mut status_line = "".to_string();
  let mut debug_mode = false;
  let mut show_state = false;
  let mut stdin = VecDeque::new();
  let mut stdout = VecDeque::new();
  let mut display = [0x00; 0x20];
  let mut controller = [None; 8];

  // this call will switch the termital to raw mode
  let input_channel = spawn_input_channel();

  mc.reset(&mut stdin, &mut stdout, &mut display, &mut 0);

  loop {
    if debug_mode {
      'until_valid: loop {
        match input_channel.recv() {
          Ok(console::Key::Del) => {
            stdout = VecDeque::new();
            break 'until_valid;
          }

          Ok(console::Key::Tab) => {
            status_line = "Single stepped".to_string();
            break 'until_valid;
          }

          Ok(console::Key::Escape) => {
            debug_mode = !debug_mode;
            break 'until_valid;
          }

          _ => continue 'until_valid,
        }
      }

      // conceptually hacky but does the job
      start_time = std::time::Instant::now();
      current_clocks = 0;
    }

    use std::sync::mpsc::TryRecvError;
    match input_channel.try_recv() {
      Ok(console::Key::Del) => {
        stdout = VecDeque::new();
      }

      Ok(console::Key::Tab) => {
        show_state = !show_state;
      }

      Ok(console::Key::Escape) => {
        debug_mode = !debug_mode;
        status_line = "Force debug".to_string();
      }

      Ok(key) => {
        let keys = [
          console::Key::ArrowUp,
          console::Key::ArrowDown,
          console::Key::ArrowLeft,
          console::Key::ArrowRight,
          console::Key::PageUp,
          console::Key::PageDown,
          console::Key::Home,
          console::Key::End,
        ];

        controller = keys
          .iter()
          .map(|k| (k == &key).then_some(std::time::Instant::now()))
          .zip(controller.iter())
          .map(|(next, curr)| next.or(*curr))
          .collect::<Vec<_>>()
          .try_into()
          .unwrap();

        stdin.extend(match key {
          console::Key::Char(c) => vec![c as u8],
          console::Key::Backspace => vec![0x08],
          console::Key::Enter => vec![0x0A],
          console::Key::Tab => vec![0x09],
          console::Key::Del => vec![0x7F],
          _ => vec![],
        });
      }

      Err(TryRecvError::Empty) => (),
      Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
    }

    let timestamp_threshold = std::time::Duration::from_millis(200);
    controller = controller
      .iter()
      .map(|timestamp| timestamp.and_then(|t| (t.elapsed() < timestamp_threshold).then_some(t)))
      .collect::<Vec<_>>()
      .try_into()
      .unwrap();
    let controller = controller
      .iter()
      .enumerate()
      .fold(0x00, |acc, (index, timestamp)| {
        acc | ((timestamp.is_some() as u8) << index)
      });

    let realtime = std::cmp::max(start_time.elapsed().as_millis(), 1); // prevent division by zero
    let realtime_offset = (1000 * current_clocks / clock_speed) as i128 - realtime as i128;
    let realtime_ratio = realtime_offset as f64 / realtime as f64;
    std::thread::sleep(std::time::Duration::from_millis(
      std::cmp::max(realtime_offset, 0) as u64,
    ));

    if !debug_mode {
      let realtime_tolerance = 0.01;
      status_line = if -realtime_ratio > realtime_tolerance {
        format!("Execution behind by {:.0}%", -realtime_ratio * 100.0)
      } else if realtime_ratio > realtime_tolerance {
        format!("Execution ahead by {:.0}%", realtime_ratio * 100.0)
      } else {
        format!("Execution on time")
      };
    }

    let mut controller = controller;
    match mc.tick(&mut stdin, &mut stdout, &mut display, &mut controller) {
      Ok(clocks) => {
        current_clocks += clocks;
      }
      Err(tick_trap) => {
        debug_mode = true;
        status_line = match tick_trap {
          TickTrap::MicrocodeFault => format!("Microcode fault"),
          TickTrap::IllegalOpcode => format!("Illegal opcode"),
          TickTrap::DebugRequest => format!("Debug request"),
          TickTrap::BusFault => format!("Bus fault"),
        }
      }
    };

    // print at most 30 times per second
    if next_print_time <= std::time::Instant::now() || debug_mode {
      next_print_time += std::time::Duration::from_millis(1000 / 30);

      print!("\x1B[2J"); // clear screen
      print!("\x1B[1;1H"); // move cursor to top left
      print!("{}\r\n", status_line);
      print!("\r\n");
      if show_state || debug_mode {
        print!("{}", mc);
      } else {
        print!(
          "{}\r\n{}",
          render_display(&display),
          render_controller(&controller)
        );
      }
      print!("\r\n");
      print!("{}", stdout.iter().map(|c| *c as char).collect::<String>());
      use std::io::Write;
      std::io::stdout().flush().unwrap();
    }
  }
}

use std::sync::mpsc;
use std::sync::mpsc::Receiver;
fn spawn_input_channel() -> Receiver<console::Key> {
  let stdout = console::Term::stdout();

  let (tx, rx) = mpsc::channel::<console::Key>();
  std::thread::spawn(move || loop {
    if let Ok(key) = stdout.read_key() {
      tx.send(key).unwrap();
    }
  });

  rx
}

pub fn render_memory(memory: &[u8; MEM_SIZE], ip: u8, sp: u8, cf: bool) -> String {
  let mut fmt: String = "".to_string();

  fmt += "MEM\r\n";
  for y in 0..0x10 {
    for x in 0..0x10 {
      let address: u8 = (y << 0x04 | x) as u8;
      fmt += &format!(
        "{:02X}{}",
        memory[address as usize],
        if address == sp.wrapping_sub(1) {
          if cf {
            "/"
          } else {
            "|"
          }
        } else if address == ip.wrapping_sub(1) {
          "["
        } else if address == ip {
          "]"
        } else {
          " "
        }
      );
    }
    fmt += "\r\n";
  }

  fmt
}

pub fn render_display(display: &[u8; 0x20]) -> String {
  let mut fmt = "".to_string();

  // https://en.wikipedia.org/wiki/Block_Elements
  let line_top: &str = "\u{25aa}                \u{25aa}\r\n";
  let line_bottom: &str = "\u{25aa}                \u{25aa}\r\n";
  let col_left: &str = " ";
  let col_right: &str = " ";

  fmt += &line_top;
  for y in (0..0x10).step_by(2) {
    fmt += &col_left;
    for x in 0..0x10 {
      let mut pixel_pair = 0;
      for y2 in 0..2 {
        let address: u8 = (x >> 0x03) | ((y + y2) << 0x01);
        let pixel = display[address as usize] >> (0x07 - (x & 0x07)) & 0x01;
        pixel_pair |= pixel << y2;
      }
      fmt += match pixel_pair {
        0b00 => " ",
        0b01 => "\u{2580}",
        0b10 => "\u{2584}",
        0b11 => "\u{2588}",
        _ => unreachable!(),
      };
    }
    fmt += &col_right;
    fmt += "\r\n";
  }

  fmt += &line_bottom;
  fmt += "\r\n";

  fmt
}

pub fn render_controller(controller: &u8) -> String {
  let mut fmt = "".to_string();

  fn bit_to_str(controller: &u8, bit: u8) -> &'static str {
    match controller >> bit & 0x01 {
      0b0 => "\u{2591}\u{2591}",
      0b1 => "\u{2588}\u{2588}",
      _ => unreachable!(),
    }
  }

  fmt += &format!(
    "    {}      {}    \r\n",
    bit_to_str(controller, 0),
    bit_to_str(controller, 4),
  );
  fmt += &format!(
    "  {}  {}  {}  {}  \r\n",
    bit_to_str(controller, 2),
    bit_to_str(controller, 3),
    bit_to_str(controller, 6),
    bit_to_str(controller, 7),
  );
  fmt += &format!(
    "    {}      {}    \r\n",
    bit_to_str(controller, 1),
    bit_to_str(controller, 5),
  );

  fmt
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Label {
  pub scope_uid: Option<usize>,
  pub identifier: String,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Macro {
  pub identifier: String,
}

#[derive(Clone, Eq, PartialEq)]
pub struct Error(pub String);

#[derive(Clone, Eq, PartialEq)]
pub struct Pos(pub String, pub usize);

#[derive(Clone, Eq, PartialEq)]
pub struct Mnemonic(pub String);

#[derive(Clone, Eq, PartialEq)]
pub enum Token {
  LabelDef(Label),
  LabelRef(Label),
  MacroDef(Macro),
  MacroRef(Macro),
  AtConst,
  AtDyn,
  AtOrg,
  AtErr,
  AtDD(u8),
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
pub enum Instruction {
  Psh(u8),
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
  Dbg,
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
  Phn(u8),
}

pub fn opcode_to_instruction(opcode: u8) -> Result<Instruction, u8> {
  fn decode_imm(opcode: u8) -> u8 {
    return opcode & 0b1111111;
  }

  fn decode_size(opcode: u8) -> u8 {
    return 1 << (opcode & 0b00000011);
  }

  fn decode_ofst(opcode: u8) -> u8 {
    return opcode & 0b00001111;
  }

  match (opcode & 0b10000000) >> 7 {
    0b0 => Ok(Instruction::Psh(decode_imm(opcode))),
    0b1 => {
      match (opcode & 0b01000000) >> 6 {
        0b0 => {
          // arithmetic and logic
          match (opcode & 0b00111100) >> 2 {
            0x0 => Ok(Instruction::Add(decode_size(opcode))),
            0x1 => Ok(Instruction::Sub(decode_size(opcode))),
            0x4 => Ok(Instruction::Iff(decode_size(opcode))),
            0x5 => Ok(Instruction::Rot(decode_size(opcode))),
            0x8 => Ok(Instruction::Orr(decode_size(opcode))),
            0x9 => Ok(Instruction::And(decode_size(opcode))),
            0xA => Ok(Instruction::Xor(decode_size(opcode))),
            0xB => Ok(Instruction::Xnd(decode_size(opcode))),
            _ => match opcode & 0b00111111 {
              // size used as part of opcode
              0b110000 => Ok(Instruction::Inc),
              0b110001 => Ok(Instruction::Dec),
              0b110010 => Ok(Instruction::Neg),
              0b110100 => Ok(Instruction::Shl),
              0b110101 => Ok(Instruction::Shr),
              0b110110 => Ok(Instruction::Not),
              0b110111 => Ok(Instruction::Buf),
              0b111011 => Ok(Instruction::Dbg),
              _ => Err(opcode),
            },
          }
        }
        0b1 => {
          match (opcode & 0b00100000) >> 5 {
            0b0 => {
              // offset operations
              match (opcode & 0b00010000) >> 4 {
                0b0 => Ok(Instruction::Ldo(decode_ofst(opcode))),
                0b1 => Ok(Instruction::Sto(decode_ofst(opcode))),
                _ => unreachable!(),
              }
            }
            0b1 => {
              match (opcode & 0b00010000) >> 4 {
                0b0 => {
                  // carry and flags and stack
                  match opcode & 0b00001111 {
                    0x0 => Ok(Instruction::Lda),
                    0x1 => Ok(Instruction::Sta),
                    0x2 => Ok(Instruction::Ldi),
                    0x3 => Ok(Instruction::Sti),
                    0x4 => Ok(Instruction::Lds),
                    0x5 => Ok(Instruction::Sts),
                    0x8 => Ok(Instruction::Nop),
                    0x9 => Ok(Instruction::Clc),
                    0xA => Ok(Instruction::Sec),
                    0xB => Ok(Instruction::Flc),
                    0xC => Ok(Instruction::Swp),
                    0xD => Ok(Instruction::Pop),
                    _ => Err(opcode),
                  }
                }
                0b1 => Ok(Instruction::Phn(decode_ofst(opcode))),
                _ => unreachable!(),
              }
            }
            _ => unreachable!(),
          }
        }
        _ => unreachable!(),
      }
    }
    _ => unreachable!(),
  }
}

pub fn instruction_to_opcode(instruction: Result<Instruction, u8>) -> u8 {
  fn encode_imm(imm: u8) -> u8 {
    match imm {
      0b00000000..=0b01111111 => imm,
      _ => panic!("Invalid IMM operand in Instruction"),
    }
  }

  fn encode_size(size: u8) -> u8 {
    match size {
      0x01 => 0x00,
      0x02 => 0x01,
      0x04 => 0x02,
      0x08 => 0x03,
      _ => panic!("Invalid SIZE operand in Instruction"),
    }
  }

  fn encode_ofst(ofst: u8) -> u8 {
    match ofst {
      0b00000000..=0b00001111 => ofst,
      _ => panic!("Invalid OFST operand in Instruction"),
    }
  }

  match instruction {
    Ok(Instruction::Psh(imm)) => 0b00000000 | encode_imm(imm),
    Ok(Instruction::Add(size)) => 0b10000000 | encode_size(size),
    Ok(Instruction::Sub(size)) => 0b10000100 | encode_size(size),
    Ok(Instruction::Iff(size)) => 0b10010000 | encode_size(size),
    Ok(Instruction::Rot(size)) => 0b10010100 | encode_size(size),
    Ok(Instruction::Orr(size)) => 0b10100000 | encode_size(size),
    Ok(Instruction::And(size)) => 0b10100100 | encode_size(size),
    Ok(Instruction::Xor(size)) => 0b10101000 | encode_size(size),
    Ok(Instruction::Xnd(size)) => 0b10101100 | encode_size(size),
    Ok(Instruction::Inc) => 0b10110000,
    Ok(Instruction::Dec) => 0b10110001,
    Ok(Instruction::Neg) => 0b10110010,
    Ok(Instruction::Shl) => 0b10110100,
    Ok(Instruction::Shr) => 0b10110101,
    Ok(Instruction::Not) => 0b10110110,
    Ok(Instruction::Buf) => 0b10110111,
    Ok(Instruction::Dbg) => 0b10111101,
    Ok(Instruction::Ldo(ofst)) => 0b11000000 | encode_ofst(ofst),
    Ok(Instruction::Sto(ofst)) => 0b11010000 | encode_ofst(ofst),
    Ok(Instruction::Lda) => 0b11100000,
    Ok(Instruction::Sta) => 0b11100001,
    Ok(Instruction::Ldi) => 0b11100010,
    Ok(Instruction::Sti) => 0b11100011,
    Ok(Instruction::Lds) => 0b11100100,
    Ok(Instruction::Sts) => 0b11100101,
    Ok(Instruction::Nop) => 0b11101000,
    Ok(Instruction::Clc) => 0b11101001,
    Ok(Instruction::Sec) => 0b11101010,
    Ok(Instruction::Flc) => 0b11101011,
    Ok(Instruction::Swp) => 0b11101100,
    Ok(Instruction::Pop) => 0b11101101,
    Ok(Instruction::Phn(imm)) => 0b11110000 | encode_ofst(imm),
    Err(opcode) => opcode,
  }
}

pub fn token_to_mnemonic(token: Token) -> Mnemonic {
  match token {
    Token::LabelDef(label) => match label.scope_uid {
      Some(scope_uid) => Mnemonic(format!("{}.{}.", label.identifier, scope_uid)),
      None => Mnemonic(format!("{}:", label.identifier)),
    },
    Token::LabelRef(label) => match label.scope_uid {
      Some(scope_uid) => Mnemonic(format!(".{}.{}", label.identifier, scope_uid)),
      None => Mnemonic(format!(":{}", label.identifier)),
    },
    Token::MacroDef(macro_) => Mnemonic(format!("{}!", macro_.identifier)),
    Token::MacroRef(macro_) => Mnemonic(format!("!{}", macro_.identifier)),
    Token::AtConst => Mnemonic(format!("@const")),
    Token::AtDyn => Mnemonic(format!("@dyn")),
    Token::AtOrg => Mnemonic(format!("@org")),
    Token::AtErr => Mnemonic(format!("@err")),
    Token::AtDD(value) => Mnemonic(format!("@{:02X}", value)),
    Token::XXX(value) => Mnemonic(format!("x{:02X}", value)),
    Token::Add => Mnemonic(format!("add")),
    Token::AdS(size) => Mnemonic(format!("ad{:01X}", size)),
    Token::Sub => Mnemonic(format!("sub")),
    Token::SuS(size) => Mnemonic(format!("su{:01X}", size)),
    Token::Iff => Mnemonic(format!("iff")),
    Token::IfS(size) => Mnemonic(format!("if{:01X}", size)),
    Token::Rot => Mnemonic(format!("rot")),
    Token::RoS(size) => Mnemonic(format!("ro{:01X}", size)),
    Token::Orr => Mnemonic(format!("orr")),
    Token::OrS(size) => Mnemonic(format!("or{:01X}", size)),
    Token::And => Mnemonic(format!("and")),
    Token::AnS(size) => Mnemonic(format!("an{:01X}", size)),
    Token::Xor => Mnemonic(format!("xor")),
    Token::XoS(size) => Mnemonic(format!("xo{:01X}", size)),
    Token::Xnd => Mnemonic(format!("xnd")),
    Token::XnS(size) => Mnemonic(format!("xn{:01X}", size)),
    Token::Inc => Mnemonic(format!("inc")),
    Token::Dec => Mnemonic(format!("dec")),
    Token::Neg => Mnemonic(format!("neg")),
    Token::Shl => Mnemonic(format!("shl")),
    Token::Shr => Mnemonic(format!("shr")),
    Token::Not => Mnemonic(format!("not")),
    Token::Buf => Mnemonic(format!("buf")),
    Token::LdO(ofst) => Mnemonic(format!("ld{:01X}", ofst)),
    Token::StO(ofst) => Mnemonic(format!("st{:01X}", ofst)),
    Token::Lda => Mnemonic(format!("lda")),
    Token::Sta => Mnemonic(format!("sta")),
    Token::Ldi => Mnemonic(format!("ldi")),
    Token::Sti => Mnemonic(format!("sti")),
    Token::Lds => Mnemonic(format!("lds")),
    Token::Sts => Mnemonic(format!("sts")),
    Token::Nop => Mnemonic(format!("nop")),
    Token::Clc => Mnemonic(format!("clc")),
    Token::Sec => Mnemonic(format!("sec")),
    Token::Flc => Mnemonic(format!("flc")),
    Token::Swp => Mnemonic(format!("swp")),
    Token::Pop => Mnemonic(format!("pop")),
  }
}

pub fn mnemonic_to_token(mnemonic: Mnemonic) -> Option<Token> {
  fn parse_hex(literal: &str) -> Option<u8> {
    (literal.to_uppercase() == literal)
      .then_some(literal)
      .and(u8::from_str_radix(literal, 16).ok())
  }

  let mnemonic = mnemonic.0.as_str();

  match mnemonic {
    _ if mnemonic.ends_with(":") => Some(Token::LabelDef(Label {
      scope_uid: None,
      identifier: mnemonic[..mnemonic.len() - 1].to_string(),
    })),
    _ if mnemonic.starts_with(":") => Some(Token::LabelRef(Label {
      scope_uid: None,
      identifier: mnemonic[1..].to_string(),
    })),
    _ if mnemonic.ends_with(".") => Some(Token::LabelDef(Label {
      scope_uid: Some(0),
      identifier: mnemonic[..mnemonic.len() - 1].to_string(),
    })),
    _ if mnemonic.starts_with(".") => Some(Token::LabelRef(Label {
      scope_uid: Some(0),
      identifier: mnemonic[1..].to_string(),
    })),
    _ if mnemonic.ends_with("!") => Some(Token::MacroDef(Macro {
      identifier: mnemonic[..mnemonic.len() - 1].to_string(),
    })),
    _ if mnemonic.starts_with("!") => Some(Token::MacroRef(Macro {
      identifier: mnemonic[1..].to_string(),
    })),
    "@const" => Some(Token::AtConst),
    "@dyn" => Some(Token::AtDyn),
    "@org" => Some(Token::AtOrg),
    "@err" => Some(Token::AtErr),
    "add" => Some(Token::Add),
    "sub" => Some(Token::Sub),
    "iff" => Some(Token::Iff),
    "rot" => Some(Token::Rot),
    "orr" => Some(Token::Orr),
    "and" => Some(Token::And),
    "xor" => Some(Token::Xor),
    "xnd" => Some(Token::Xnd),
    "inc" => Some(Token::Inc),
    "dec" => Some(Token::Dec),
    "neg" => Some(Token::Neg),
    "shl" => Some(Token::Shl),
    "shr" => Some(Token::Shr),
    "not" => Some(Token::Not),
    "buf" => Some(Token::Buf),
    "lda" => Some(Token::Lda),
    "sta" => Some(Token::Sta),
    "ldi" => Some(Token::Ldi),
    "sti" => Some(Token::Sti),
    "lds" => Some(Token::Lds),
    "sts" => Some(Token::Sts),
    "nop" => Some(Token::Nop),
    "clc" => Some(Token::Clc),
    "sec" => Some(Token::Sec),
    "flc" => Some(Token::Flc),
    "swp" => Some(Token::Swp),
    "pop" => Some(Token::Pop),
    _ if mnemonic.len() == 3 => match mnemonic.split_at(2) {
      ("ad", hex) => parse_hex(&hex).map(Token::AdS),
      ("su", hex) => parse_hex(&hex).map(Token::SuS),
      ("if", hex) => parse_hex(&hex).map(Token::IfS),
      ("ro", hex) => parse_hex(&hex).map(Token::RoS),
      ("or", hex) => parse_hex(&hex).map(Token::OrS),
      ("an", hex) => parse_hex(&hex).map(Token::AnS),
      ("xo", hex) => parse_hex(&hex).map(Token::XoS),
      ("xn", hex) => parse_hex(&hex).map(Token::XnS),
      ("ld", hex) => parse_hex(&hex).map(Token::LdO),
      ("st", hex) => parse_hex(&hex).map(Token::StO),
      _ => match mnemonic.split_at(1) {
        ("@", hex) => parse_hex(&hex).map(Token::AtDD),
        ("x", hex) => parse_hex(&hex).map(Token::XXX),
        _ => None,
      },
    },
    _ => None,
  }
}

pub fn instruction_to_token(instruction: Result<Instruction, u8>) -> Token {
  match instruction {
    Ok(Instruction::Psh(imm)) => Token::XXX(imm),
    Ok(Instruction::Add(0x01)) => Token::Add,
    Ok(Instruction::Add(size)) => Token::AdS(size),
    Ok(Instruction::Sub(0x01)) => Token::Sub,
    Ok(Instruction::Sub(size)) => Token::SuS(size),
    Ok(Instruction::Iff(0x01)) => Token::Iff,
    Ok(Instruction::Iff(size)) => Token::IfS(size),
    Ok(Instruction::Rot(0x01)) => Token::Rot,
    Ok(Instruction::Rot(size)) => Token::RoS(size),
    Ok(Instruction::Orr(0x01)) => Token::Orr,
    Ok(Instruction::Orr(size)) => Token::OrS(size),
    Ok(Instruction::And(0x01)) => Token::And,
    Ok(Instruction::And(size)) => Token::AnS(size),
    Ok(Instruction::Xor(0x01)) => Token::Xor,
    Ok(Instruction::Xor(size)) => Token::XoS(size),
    Ok(Instruction::Xnd(0x01)) => Token::Xnd,
    Ok(Instruction::Xnd(size)) => Token::XnS(size),
    Ok(Instruction::Inc) => Token::Inc,
    Ok(Instruction::Dec) => Token::Dec,
    Ok(Instruction::Neg) => Token::Neg,
    Ok(Instruction::Shl) => Token::Shl,
    Ok(Instruction::Shr) => Token::Shr,
    Ok(Instruction::Not) => Token::Not,
    Ok(Instruction::Buf) => Token::Buf,
    Ok(Instruction::Dbg) => Token::AtDD(0xBB),
    Ok(Instruction::Ldo(ofst)) => Token::LdO(ofst),
    Ok(Instruction::Sto(ofst)) => Token::StO(ofst),
    Ok(Instruction::Lda) => Token::Lda,
    Ok(Instruction::Sta) => Token::Sta,
    Ok(Instruction::Ldi) => Token::Ldi,
    Ok(Instruction::Sti) => Token::Sti,
    Ok(Instruction::Lds) => Token::Lds,
    Ok(Instruction::Sts) => Token::Sts,
    Ok(Instruction::Nop) => Token::Nop,
    Ok(Instruction::Clc) => Token::Clc,
    Ok(Instruction::Sec) => Token::Sec,
    Ok(Instruction::Flc) => Token::Flc,
    Ok(Instruction::Swp) => Token::Swp,
    Ok(Instruction::Pop) => Token::Pop,
    Ok(Instruction::Phn(imm)) => Token::XXX(0b11110000 | imm),
    Err(opcode) => Token::AtDD(opcode),
  }
}

impl std::fmt::Display for Signal {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Signal::Active => write!(f, "HI"),
      Signal::Inactive => write!(f, "LO"),
    }
  }
}

impl std::fmt::Display for Label {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", token_to_mnemonic(Token::LabelRef(self.clone())))
  }
}

impl std::fmt::Display for Macro {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", token_to_mnemonic(Token::MacroRef(self.clone())))
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

impl std::fmt::Display for Mnemonic {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl std::fmt::Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", token_to_mnemonic(self.clone()))
  }
}

const MICROCODE_FAULT_MAGIC: u16 = -1i16 as u16;
const ILLEGAL_OPCODE_MAGIC: u16 = -2i16 as u16;
const DEBUG_REQUEST_MAGIC: u16 = -3i16 as u16;
const BUS_FAULT_MAGIC: u16 = -4i16 as u16;

impl From<u16> for ControlWord {
  fn from(control_word: u16) -> Self {
    let control_word = (0..16)
      .rev()
      .map(|i| (control_word >> i) as u8 & 1)
      .collect::<Vec<_>>()
      .try_into()
      .unwrap();

    unsafe {
      std::mem::transmute::<[u8; std::mem::size_of::<ControlWord>()], ControlWord>(control_word)
    }
  }
}

impl Into<u16> for ControlWord {
  fn into(self) -> u16 {
    let control_word =
      unsafe { std::mem::transmute::<ControlWord, [u8; std::mem::size_of::<ControlWord>()]>(self) };

    control_word
      .iter()
      .fold(0, |acc, &byte| (acc << 1) | byte as u16)
  }
}

pub fn u16_into_result(u16: u16) -> Result<ControlWord, TickTrap> {
  match u16 {
    MICROCODE_FAULT_MAGIC => Err(TickTrap::MicrocodeFault),
    ILLEGAL_OPCODE_MAGIC => Err(TickTrap::IllegalOpcode),
    DEBUG_REQUEST_MAGIC => Err(TickTrap::DebugRequest),
    BUS_FAULT_MAGIC => Err(TickTrap::BusFault),
    control_word => Ok(control_word.into()),
  }
}

pub fn result_into_u16(result: Result<ControlWord, TickTrap>) -> u16 {
  match result {
    Err(TickTrap::MicrocodeFault) => MICROCODE_FAULT_MAGIC,
    Err(TickTrap::IllegalOpcode) => ILLEGAL_OPCODE_MAGIC,
    Err(TickTrap::DebugRequest) => DEBUG_REQUEST_MAGIC,
    Err(TickTrap::BusFault) => BUS_FAULT_MAGIC,
    Ok(control_word) => control_word.into(),
  }
}
