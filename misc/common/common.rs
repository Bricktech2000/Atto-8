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

impl std::fmt::Display for Signal {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Signal::Active => write!(f, "HI"),
      Signal::Inactive => write!(f, "LO"),
    }
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
