#![allow(dead_code)]

use std::collections::{BTreeMap, BTreeSet, VecDeque};

pub const MEM_SIZE: usize = 0x100;
pub const MIC_SIZE: usize = 0x80 * 0x02 * 0x20; // 0x2000
pub const DISPLAY_BUFFER: usize = 0xE0;
pub const DISPLAY_BUFFER_LEN: usize = 0x20;
pub const STDIO_BUFFER: usize = 0x00;

#[derive(Clone, Copy, Debug, Default)]
pub struct ControlWord {
  pub data_ip: Signal, // data bus to instruction pointer
  pub data_sp: Signal, // data bus to stack pointer
  pub data_cf: Signal, // data bus to carry flag
  pub data_il: Signal, // data bus to instruction latch
  pub data_al: Signal, // data bus to address latch
  pub data_xl: Signal, // data bus to X latch
  pub data_yl: Signal, // data bus to Y latch
  pub data_zl: Signal, // data bus to Z latch

  pub mem_data: Signal,  // data bus to memory
  pub data_mem: Signal,  // memory to data bus
  pub ip_data: Signal,   // instruction pointer to data bus
  pub sp_data: Signal,   // stack pointer to data bus
  pub clr_sc: Signal,    // clear to step counter
  pub set_cin: Signal,   // set to carry in
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
    display: &mut [u8; DISPLAY_BUFFER_LEN],
    controller: &mut u8,
  );
  fn tick(
    &mut self,
    stdin: &mut VecDeque<u8>,
    stdout: &mut VecDeque<u8>,
    display: &mut [u8; DISPLAY_BUFFER_LEN],
    controller: &mut u8,
  ) -> Result<u128, TickTrap>;
}

pub fn execute<MC: std::fmt::Display + Tickable>(mut mc: MC, clock_speed: u128) {
  let mut current_clocks = 0;
  let mut initial_time = std::time::Instant::now();
  let mut next_call_clocks = 0;
  let mut next_stdin_clocks = 0;
  let mut next_stdout_clocks = 0;
  let mut controller_timestamps = [None; 8];
  let mut status_line = "".to_string();
  let mut debug_mode = false;
  let mut show_state = false;

  let mut stdin = VecDeque::new();
  let mut stdout = VecDeque::new();
  let mut display = [0x00; DISPLAY_BUFFER_LEN];

  mc.reset(&mut stdin, &mut stdout, &mut display, &mut 0x00);

  // this call will switch the termital to raw mode
  let input_channel = spawn_input_channel();

  loop {
    let mut controller = controller_timestamps
      .iter()
      .enumerate()
      .fold(0x00, |acc, (index, timestamp)| {
        acc | ((timestamp.is_some() as u8) << index)
      });

    // call `std::Instant::now()` at most 1000 times per second
    if next_call_clocks <= current_clocks || debug_mode {
      next_call_clocks += if debug_mode { 0 } else { clock_speed / 1000 };

      let timestamp_threshold = std::time::Duration::from_millis(200);
      controller_timestamps = controller_timestamps
        .iter()
        .map(|timestamp: &Option<std::time::Instant>| {
          timestamp.and_then(|t| (t.elapsed() < timestamp_threshold).then_some(t))
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

      let realtime = std::cmp::max(initial_time.elapsed().as_millis(), 1); // prevent division by zero
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
    }

    // read input at most 60 times per second
    if next_stdin_clocks <= current_clocks || debug_mode {
      next_stdin_clocks += if debug_mode { 0 } else { clock_speed / 60 };

      'until_empty: loop {
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

            controller_timestamps = keys
              .iter()
              .map(|k| (k == &key).then_some(std::time::Instant::now()))
              .zip(controller_timestamps.iter())
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

          Err(TryRecvError::Empty) => break 'until_empty,
          Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
        }
      }
    }

    // write output at most 60 times per second
    if next_stdout_clocks <= current_clocks || debug_mode {
      next_stdout_clocks += if debug_mode { 0 } else { clock_speed / 60 };

      stdout = stdout
        .into_iter()
        .filter(|c| *c <= 0x7F) // outside ASCII
        .filter(|c| *c != 0x00) // NUL
        .collect::<VecDeque<_>>();
      let stdout_string = stdout
        .iter()
        .map(|c| *c as char)
        .collect::<String>()
        .replace("\n", "\r\n")
        .replace("\x08", "\x08 \x08");
      stdout = stdout
        .into_iter()
        .filter(|c| *c != 0x07) // BEL
        .collect::<VecDeque<_>>();

      let term = console::Term::stdout();
      term.clear_screen().unwrap();
      term.move_cursor_to(0, 0).unwrap();

      print!("{}\r\n", status_line);
      if show_state || debug_mode {
        print!("Clocks: {}\r\n", current_clocks);
        print!("\r\n");
        print!("{}", mc);
      } else {
        print!("\r\n");
        print!("{}", render_display(&display));
        print!("{}", render_controller(&controller));
      }
      print!("\r\n");
      print!("{}", stdout_string);
      use std::io::Write;
      std::io::stdout().flush().unwrap();
    }

    if debug_mode {
      'until_valid: loop {
        match input_channel.try_recv() {
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
      initial_time = std::time::Instant::now();
      current_clocks = 0;
      next_call_clocks = 0;
      next_stdin_clocks = 0;
      next_stdout_clocks = 0;
    }

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
}

pub fn render_memory(memory: &[u8; MEM_SIZE], ip: u8, sp: u8, cf: bool) -> String {
  let mut fmt = "".to_string();

  fmt += "MEM\r\n";
  for y in 0..0x10 {
    for x in 0..0x10 {
      let address: u8 = (y << 0x04 | x) as u8;
      fmt += &format!(
        "{:02X}{}",
        memory[address as usize],
        if address == sp.wrapping_sub(1) {
          match cf {
            true => "/",
            false => "|",
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

pub fn render_display(display: &[u8; DISPLAY_BUFFER_LEN]) -> String {
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

const MICROCODE_FAULT_SENTINEL: u16 = 0xFFFF;
const ILLEGAL_OPCODE_SENTINEL: u16 = 0xFFFE;
const DEBUG_REQUEST_SENTINEL: u16 = 0xFFFD;
const BUS_FAULT_SENTINEL: u16 = 0xFFFC;

impl From<u16> for ControlWord {
  fn from(control_word: u16) -> Self {
    let mut slice = [0x00; 16];
    for i in 0..slice.len() {
      slice[i] = (control_word >> i) as u8 & 1;
    }
    slice.reverse();
    let control_word = slice;

    // // causes a dynamic memory allocation
    // let control_word = (0..16)
    //   .rev()
    //   .map(|i| (control_word >> i) as u8 & 1)
    //   .collect::<Vec<_>>()
    //   .try_into()
    //   .unwrap();

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
    MICROCODE_FAULT_SENTINEL => Err(TickTrap::MicrocodeFault),
    ILLEGAL_OPCODE_SENTINEL => Err(TickTrap::IllegalOpcode),
    DEBUG_REQUEST_SENTINEL => Err(TickTrap::DebugRequest),
    BUS_FAULT_SENTINEL => Err(TickTrap::BusFault),
    control_word => Ok(control_word.into()),
  }
}

pub fn result_into_u16(result: Result<ControlWord, TickTrap>) -> u16 {
  match result {
    Err(TickTrap::MicrocodeFault) => MICROCODE_FAULT_SENTINEL,
    Err(TickTrap::IllegalOpcode) => ILLEGAL_OPCODE_SENTINEL,
    Err(TickTrap::DebugRequest) => DEBUG_REQUEST_SENTINEL,
    Err(TickTrap::BusFault) => BUS_FAULT_SENTINEL,
    Ok(control_word) => control_word.into(),
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

#[derive(Clone, Eq, PartialEq)]
pub struct File(pub String);

#[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Label {
  Local(String, Option<usize>),
  Global(String),
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Macro(pub String);

#[derive(Clone, Eq, PartialEq)]
pub struct Error(pub String);

#[derive(Clone, Eq, PartialEq)]
pub struct Pos(pub File, pub usize, pub usize);

#[derive(Clone, Eq, PartialEq)]
pub struct Mnemonic(pub String);

#[derive(Clone, Eq, PartialEq)]
pub enum Token {
  LabelDef(Label),
  LabelRef(Label),
  MacroDef(Macro),
  MacroRef(Macro),
  AtError,
  AtConst,
  AtData,
  AtDyn,
  AtOrg,
  AtDD(u8),
  XXX(u8),
  Add,
  AdS(Size),
  Sub,
  SuS(Size),
  Iff,
  IfS(Size),
  Swp,
  SwS(Size),
  Rot,
  RoS(Size),
  Orr,
  OrS(Size),
  And,
  AnS(Size),
  Xor,
  XoS(Size),
  Xnd,
  XnS(Size),
  Inc,
  Dec,
  Neg,
  Shl,
  Shr,
  Not,
  Buf,
  LdO(Ofst),
  StO(Ofst),
  Lda,
  Sta,
  Ldi,
  Sti,
  Lds,
  Sts,
  Clc,
  Sec,
  Flc,
  Nop,
  Pop,
}

#[derive(Clone, Eq, PartialEq)]
pub enum Instruction {
  Psh(Imm),
  Add(Size),
  Sub(Size),
  Iff(Size),
  Swp(Size),
  Rot(Size),
  Orr(Size),
  And(Size),
  Xor(Size),
  Xnd(Size),
  Inc,
  Dec,
  Neg,
  Shl,
  Shr,
  Not,
  Buf,
  Dbg,
  Ldo(Ofst),
  Sto(Ofst),
  Lda,
  Sta,
  Ldi,
  Sti,
  Lds,
  Sts,
  Clc,
  Sec,
  Flc,
  Nop,
  Pop,
  Phn(Nimm),
}

// `pub u8` required for destructuring and pattern matching. do not use constructors directly
#[derive(Clone, Eq, PartialEq)]
pub struct Imm(pub u8);
#[derive(Clone, Eq, PartialEq)]
pub struct Size(pub u8);
#[derive(Clone, Eq, PartialEq)]
pub struct Ofst(pub u8);
#[derive(Clone, Eq, PartialEq)]
pub struct Nimm(pub u8);

impl Imm {
  pub fn new(imm: u8) -> Option<Imm> {
    matches!(imm, 0b00000000..=0b01111111).then_some(Imm(imm))
  }

  pub fn assert(imm: u8) -> Imm {
    Imm::new(imm).unwrap()
  }
}

impl Size {
  pub fn new(size: u8) -> Option<Size> {
    matches!(size, 0x01 | 0x02 | 0x04 | 0x08).then_some(Size(size))
  }

  pub fn assert(size: u8) -> Size {
    Size::new(size).unwrap()
  }
}

impl Ofst {
  pub fn new(ofst: u8) -> Option<Ofst> {
    matches!(ofst, 0b00000000..=0b00001111).then_some(Ofst(ofst))
  }

  pub fn assert(ofst: u8) -> Ofst {
    Ofst::new(ofst).unwrap()
  }
}

impl Nimm {
  pub fn new(nimm: u8) -> Option<Nimm> {
    matches!(nimm, 0b11110000..=0b11111111).then_some(Nimm(nimm))
  }

  pub fn assert(nimm: u8) -> Nimm {
    Nimm::new(nimm).unwrap()
  }
}

pub fn opcode_to_instruction(opcode: u8) -> Result<Instruction, u8> {
  fn decode_imm(opcode: u8) -> Imm {
    Imm::assert(opcode & 0b01111111)
  }

  fn decode_size(opcode: u8) -> Size {
    Size::assert(1 << (opcode & 0b00000011))
  }

  fn decode_ofst(opcode: u8) -> Ofst {
    Ofst::assert(opcode & 0b00001111)
  }

  fn decode_nimm(opcode: u8) -> Nimm {
    Nimm::assert(opcode | 0b11110000)
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
            0x5 => Ok(Instruction::Swp(decode_size(opcode))),
            0x6 => Ok(Instruction::Rot(decode_size(opcode))),
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
                    0x8 => Ok(Instruction::Clc),
                    0x9 => Ok(Instruction::Sec),
                    0xA => Ok(Instruction::Flc),
                    0xE => Ok(Instruction::Nop),
                    0xF => Ok(Instruction::Pop),
                    _ => Err(opcode),
                  }
                }
                0b1 => Ok(Instruction::Phn(decode_nimm(opcode))),
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
  fn encode_imm(Imm(imm): Imm) -> u8 {
    let Imm(imm) = Imm::assert(imm); // sanity check
    imm
  }

  fn encode_size(Size(size): Size) -> u8 {
    let Size(size) = Size::assert(size); // sanity check
    size.trailing_zeros() as u8
  }

  fn encode_ofst(Ofst(ofst): Ofst) -> u8 {
    let Ofst(ofst) = Ofst::assert(ofst); // sanity check
    ofst
  }

  fn encode_nimm(Nimm(nimm): Nimm) -> u8 {
    let Nimm(nimm) = Nimm::assert(nimm); // sanity check
    nimm & 0b00001111
  }

  match instruction {
    Ok(Instruction::Psh(imm)) => 0b00000000 | encode_imm(imm),
    Ok(Instruction::Add(size)) => 0b10000000 | encode_size(size),
    Ok(Instruction::Sub(size)) => 0b10000100 | encode_size(size),
    Ok(Instruction::Iff(size)) => 0b10010000 | encode_size(size),
    Ok(Instruction::Swp(size)) => 0b10010100 | encode_size(size),
    Ok(Instruction::Rot(size)) => 0b10011000 | encode_size(size),
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
    Ok(Instruction::Dbg) => 0b10111011,
    Ok(Instruction::Ldo(ofst)) => 0b11000000 | encode_ofst(ofst),
    Ok(Instruction::Sto(ofst)) => 0b11010000 | encode_ofst(ofst),
    Ok(Instruction::Lda) => 0b11100000,
    Ok(Instruction::Sta) => 0b11100001,
    Ok(Instruction::Ldi) => 0b11100010,
    Ok(Instruction::Sti) => 0b11100011,
    Ok(Instruction::Lds) => 0b11100100,
    Ok(Instruction::Sts) => 0b11100101,
    Ok(Instruction::Clc) => 0b11101000,
    Ok(Instruction::Sec) => 0b11101001,
    Ok(Instruction::Flc) => 0b11101010,
    Ok(Instruction::Nop) => 0b11101110,
    Ok(Instruction::Pop) => 0b11101111,
    Ok(Instruction::Phn(nimm)) => 0b11110000 | encode_nimm(nimm),
    Err(opcode) => opcode,
  }
}

pub fn token_to_mnemonic(token: Token) -> Mnemonic {
  match token {
    Token::LabelDef(Label::Local(identifier, Some(scope_uid))) => {
      Mnemonic(format!("{}.{}.", identifier, scope_uid))
    }
    Token::LabelDef(Label::Local(identifier, None)) => Mnemonic(format!("{}.", identifier)),
    Token::LabelDef(Label::Global(identifier)) => Mnemonic(format!("{}:", identifier)),

    Token::LabelRef(Label::Local(identifier, Some(scope_uid))) => {
      Mnemonic(format!(".{}.{}", identifier, scope_uid))
    }
    Token::LabelRef(Label::Local(identifier, None)) => Mnemonic(format!(".{}", identifier)),
    Token::LabelRef(Label::Global(identifier)) => Mnemonic(format!(":{}", identifier)),
    Token::MacroDef(Macro(r#macro)) => Mnemonic(format!("{}!", r#macro)),
    Token::MacroRef(Macro(r#macro)) => Mnemonic(format!("!{}", r#macro)),
    Token::AtError => Mnemonic(format!("@error")),
    Token::AtConst => Mnemonic(format!("@const")),
    Token::AtData => Mnemonic(format!("@data")),
    Token::AtDyn => Mnemonic(format!("@dyn")),
    Token::AtOrg => Mnemonic(format!("@org")),
    Token::AtDD(value) => Mnemonic(format!("@{:02X}", value)),
    Token::XXX(value) => Mnemonic(format!("x{:02X}", value)),
    Token::Add => Mnemonic(format!("add")),
    Token::AdS(Size(size)) => Mnemonic(format!("ad{:01X}", size)),
    Token::Sub => Mnemonic(format!("sub")),
    Token::SuS(Size(size)) => Mnemonic(format!("su{:01X}", size)),
    Token::Iff => Mnemonic(format!("iff")),
    Token::IfS(Size(size)) => Mnemonic(format!("if{:01X}", size)),
    Token::Swp => Mnemonic(format!("swp")),
    Token::SwS(Size(size)) => Mnemonic(format!("sw{:01X}", size)),
    Token::Rot => Mnemonic(format!("rot")),
    Token::RoS(Size(size)) => Mnemonic(format!("ro{:01X}", size)),
    Token::Orr => Mnemonic(format!("orr")),
    Token::OrS(Size(size)) => Mnemonic(format!("or{:01X}", size)),
    Token::And => Mnemonic(format!("and")),
    Token::AnS(Size(size)) => Mnemonic(format!("an{:01X}", size)),
    Token::Xor => Mnemonic(format!("xor")),
    Token::XoS(Size(size)) => Mnemonic(format!("xo{:01X}", size)),
    Token::Xnd => Mnemonic(format!("xnd")),
    Token::XnS(Size(size)) => Mnemonic(format!("xn{:01X}", size)),
    Token::Inc => Mnemonic(format!("inc")),
    Token::Dec => Mnemonic(format!("dec")),
    Token::Neg => Mnemonic(format!("neg")),
    Token::Shl => Mnemonic(format!("shl")),
    Token::Shr => Mnemonic(format!("shr")),
    Token::Not => Mnemonic(format!("not")),
    Token::Buf => Mnemonic(format!("buf")),
    Token::LdO(Ofst(ofst)) => Mnemonic(format!("ld{:01X}", ofst)),
    Token::StO(Ofst(ofst)) => Mnemonic(format!("st{:01X}", ofst)),
    Token::Lda => Mnemonic(format!("lda")),
    Token::Sta => Mnemonic(format!("sta")),
    Token::Ldi => Mnemonic(format!("ldi")),
    Token::Sti => Mnemonic(format!("sti")),
    Token::Lds => Mnemonic(format!("lds")),
    Token::Sts => Mnemonic(format!("sts")),
    Token::Clc => Mnemonic(format!("clc")),
    Token::Sec => Mnemonic(format!("sec")),
    Token::Flc => Mnemonic(format!("flc")),
    Token::Nop => Mnemonic(format!("nop")),
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
    _ if mnemonic.ends_with(":") => Some(Token::LabelDef(Label::Global(
      mnemonic[..mnemonic.len() - 1].to_string(),
    ))),
    _ if mnemonic.starts_with(":") => {
      Some(Token::LabelRef(Label::Global(mnemonic[1..].to_string())))
    }
    _ if mnemonic.ends_with(".") => Some(Token::LabelDef(Label::Local(
      mnemonic[..mnemonic.len() - 1].to_string(),
      None,
    ))),
    _ if mnemonic.starts_with(".") => Some(Token::LabelRef(Label::Local(
      mnemonic[1..].to_string(),
      None,
    ))),
    _ if mnemonic.ends_with("!") => Some(Token::MacroDef(Macro(
      mnemonic[..mnemonic.len() - 1].to_string(),
    ))),
    _ if mnemonic.starts_with("!") => Some(Token::MacroRef(Macro(mnemonic[1..].to_string()))),
    "@error" => Some(Token::AtError),
    "@const" => Some(Token::AtConst),
    "@data" => Some(Token::AtData),
    "@dyn" => Some(Token::AtDyn),
    "@org" => Some(Token::AtOrg),
    "add" => Some(Token::Add),
    "sub" => Some(Token::Sub),
    "iff" => Some(Token::Iff),
    "swp" => Some(Token::Swp),
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
    "clc" => Some(Token::Clc),
    "sec" => Some(Token::Sec),
    "flc" => Some(Token::Flc),
    "nop" => Some(Token::Nop),
    "pop" => Some(Token::Pop),
    _ if mnemonic.len() == 3 => match mnemonic.split_at(2) {
      ("ad", hex) => parse_hex(&hex).and_then(Size::new).map(Token::AdS),
      ("su", hex) => parse_hex(&hex).and_then(Size::new).map(Token::SuS),
      ("if", hex) => parse_hex(&hex).and_then(Size::new).map(Token::IfS),
      ("sw", hex) => parse_hex(&hex).and_then(Size::new).map(Token::SwS),
      ("ro", hex) => parse_hex(&hex).and_then(Size::new).map(Token::RoS),
      ("or", hex) => parse_hex(&hex).and_then(Size::new).map(Token::OrS),
      ("an", hex) => parse_hex(&hex).and_then(Size::new).map(Token::AnS),
      ("xo", hex) => parse_hex(&hex).and_then(Size::new).map(Token::XoS),
      ("xn", hex) => parse_hex(&hex).and_then(Size::new).map(Token::XnS),
      ("ld", hex) => parse_hex(&hex).and_then(Ofst::new).map(Token::LdO),
      ("st", hex) => parse_hex(&hex).and_then(Ofst::new).map(Token::StO),
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
    Ok(Instruction::Psh(Imm(imm))) => Token::XXX(imm),
    Ok(Instruction::Add(Size(0x01))) => Token::Add,
    Ok(Instruction::Add(size)) => Token::AdS(size),
    Ok(Instruction::Sub(Size(0x01))) => Token::Sub,
    Ok(Instruction::Sub(size)) => Token::SuS(size),
    Ok(Instruction::Iff(Size(0x01))) => Token::Iff,
    Ok(Instruction::Iff(size)) => Token::IfS(size),
    Ok(Instruction::Swp(Size(0x01))) => Token::Swp,
    Ok(Instruction::Swp(size)) => Token::SwS(size),
    Ok(Instruction::Rot(Size(0x01))) => Token::Rot,
    Ok(Instruction::Rot(size)) => Token::RoS(size),
    Ok(Instruction::Orr(Size(0x01))) => Token::Orr,
    Ok(Instruction::Orr(size)) => Token::OrS(size),
    Ok(Instruction::And(Size(0x01))) => Token::And,
    Ok(Instruction::And(size)) => Token::AnS(size),
    Ok(Instruction::Xor(Size(0x01))) => Token::Xor,
    Ok(Instruction::Xor(size)) => Token::XoS(size),
    Ok(Instruction::Xnd(Size(0x01))) => Token::Xnd,
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
    Ok(Instruction::Clc) => Token::Clc,
    Ok(Instruction::Sec) => Token::Sec,
    Ok(Instruction::Flc) => Token::Flc,
    Ok(Instruction::Nop) => Token::Nop,
    Ok(Instruction::Pop) => Token::Pop,
    Ok(Instruction::Phn(Nimm(nimm))) => Token::XXX(nimm),
    Err(opcode) => Token::AtDD(opcode),
  }
}

impl std::fmt::Display for File {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    use path_clean::PathClean;
    use std::path::Path;
    write!(
      f,
      "@{}",
      Path::new(&self.0).clean().to_str().unwrap().to_string()
    )
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
    write!(f, "{}:{}:{}", self.0, self.1 + 1, self.2 + 1)
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

pub fn reflexive_transitive_closure<T: Clone + Ord>(graph: &mut BTreeMap<T, BTreeSet<T>>) {
  // brute-force reflexive transitive closure
  let original_graph = graph.clone();
  for parent in original_graph.keys() {
    let mut stack: Vec<T> = vec![parent.clone()];
    let mut visited: BTreeSet<T> = BTreeSet::new();

    let children = graph.get_mut(parent).unwrap_or_else(|| unreachable!());
    children.insert(parent.clone()); // reflexive closure

    while let Some(parent) = stack.pop() {
      for child in original_graph.get(&parent).unwrap_or(&BTreeSet::new()) {
        if !visited.contains(child) {
          stack.push(child.clone());
          visited.insert(child.clone());
          children.insert(child.clone()); // transitive closure
        }
      }
    }
  }
}
