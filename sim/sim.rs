fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 3 {
    println!("Usage: sim <memory image file> <microcode image file>");
    std::process::exit(1);
  }

  let memory_image_file: &String = &args[1];

  let memory_image = std::fs::read(memory_image_file)
    .unwrap_or_else(|_| {
      println!("Sim: Error: Unable to read file `{}`", memory_image_file);
      std::process::exit(1);
    })
    .try_into()
    .unwrap_or_else(|_| {
      println!(
        "Sim: Error: Memory image `{}` has incorrect size",
        memory_image_file
      );
      std::process::exit(1);
    });

  let microcode_image_file: &String = &args[2];

  let microcode_image = std::fs::read(microcode_image_file)
    .unwrap_or_else(|_| {
      println!("Sim: Error: Unable to read file `{}`", microcode_image_file);
      std::process::exit(1);
    })
    .chunks(2)
    .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
    .collect::<Vec<u16>>()
    .try_into()
    .unwrap_or_else(|_| {
      println!(
        "Sim: Error: Microcode image `{}` has incorrect size",
        microcode_image_file
      );
      std::process::exit(1);
    });

  let mut mc = Microcomputer {
    mem: memory_image,
    stdin: 0x00,
    stdout: 0x00,
    mp: Microprocessor {
      ip: 0x00,
      sp: 0x00,
      cf: false,
      il: 0x00,
      sc: 0x00,
      al: 0x00,
      xl: 0x00,
      yl: 0x00,
      zl: 0x00,

      ctrl: ControlWord::default(),
      size_data: Signal::Inactive,
      ofst_data: Signal::Inactive,
      set_cin: Signal::Inactive,
      cout_cf: Signal::Inactive,
      zero_cf: Signal::Inactive,
      ones_data: Signal::Inactive,

      ones: 0x00,
      size: 0x00,
      ofst: 0x00,
      sum: 0x00,
      nand: 0x00,
      cin: false,
      cout: false,
      zero: false,

      mic: microcode_image,
    },

    clk: Clock::Low,
    rst: Reset::Asserted, // reset microcomputer on startup
    addr: 0x00,
    data: 0x00,
    read: Signal::Inactive,
    wrt: Signal::Inactive,
  };

  tick(&mut mc).unwrap_or_else(|_| {
    println!("Sim: Error: Tick trap during reset sequence");
    std::process::exit(1);
  });
  mc.rst = Reset::Deasserted;

  simulate(mc, 1000000);
}

const MEM_SIZE: usize = 0x100;
const MIC_SIZE: usize = 0x80 * 0x02 * 0x20;
const MICROCODE_FAULT_MAGIC: u16 = -1i16 as u16;
const ILLEGAL_OPCODE_MAGIC: u16 = -2i16 as u16;
const DEBUG_REQUEST_MAGIC: u16 = -3i16 as u16;

struct Microcomputer {
  mem: [u8; MEM_SIZE], // memory
  stdin: u8,           // standard input
  stdout: u8,          // standard output
  mp: Microprocessor,  // microprocessor

  clk: Clock,   // clock
  rst: Reset,   // reset
  addr: u8,     // address bus
  data: u8,     // data bus
  read: Signal, // memory read
  wrt: Signal,  // memory write
}

struct Microprocessor {
  ip: u8,   // instruction pointer
  sp: u8,   // stack pointer
  cf: bool, // carry flag
  il: u8,   // instruction latch
  sc: u8,   // step counter
  al: u8,   // address latch
  xl: u8,   // X latch
  yl: u8,   // Y latch
  zl: u8,   // Z latch

  ctrl: ControlWord, // control word derivation
  size_data: Signal, // size-to-data derivation
  ofst_data: Signal, // offset-to-data derivation
  set_cin: Signal,   // set-to-carry-in derivation
  cout_cf: Signal,   // carry-out-to-carry-flag derivation
  zero_cf: Signal,   // zero-to-carry-flag derivation
  ones_data: Signal, // ones-to-data derivation

  ones: u8,   // ones derivation
  size: u8,   // size derivation
  ofst: u8,   // offset derivation
  sum: u8,    // sum derivation
  nand: u8,   // not-and derivation
  cin: bool,  // sum carry-in derivation
  cout: bool, // sum carry-out derivation
  zero: bool, // nand is-zero derivation

  mic: [u16; MIC_SIZE], // microcode read-only memory
}

// TODO copied from `emu.rs`
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
enum TickTrap {
  MicrocodeFault,
  IllegalOpcode,
  DebugRequest,
}

// TODO copied from `mic.rs`
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Default)]
struct ControlWord {
  clr_sc: Signal,       // clear to step counter
  data_il: Signal,      // data bus to instruction latch
  size_and_cin: Signal, // size and carry-in
  ofst_and_cf: Signal,  // offset and carry-flag

  ip_data: Signal, // instruction pointer to data bus
  data_ip: Signal, // data bus to instruction pointer

  sp_data: Signal, // stack pointer to data bus
  data_sp: Signal, // data bus to stack pointer

  data_al: Signal,  // data bus to address latch
  mem_data: Signal, // data bus to memory
  data_mem: Signal, // memory to data bus

  data_xl: Signal,   // data bus to X latch
  data_yl: Signal,   // data bus to Y latch
  data_zl: Signal,   // data bus to Z latch
  sum_data: Signal,  // sum to data bus
  nand_data: Signal, // nand to data bus
}

// TODO copied from `mic.rs`
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Default)]
enum Signal {
  #[default]
  Inactive,
  Active,
}

enum Clock {
  Rising,
  High,
  Falling,
  Low,
}

enum Reset {
  Asserted,
  Deasserted,
}

// TODO copied from `emu.rs`

fn simulate(mut mc: Microcomputer, clock_speed: u128) {
  use std::collections::VecDeque;

  let mut start_time = std::time::Instant::now();
  let mut next_print_time = std::time::Instant::now();
  let mut current_clocks = 0;
  let mut status_line = "".to_string();
  let mut stdout_string = "".to_string();
  let mut stdin_queue = VecDeque::new();
  let mut controller_input = [None; 8];
  let mut debug_mode = false;
  let mut show_state = false;

  // this call will switch the termital to raw mode
  let input_channel = spawn_input_channel();

  mc.stdin = mc.mem[0x00];
  mc.mem[0x00] = 0x00; // controller input

  loop {
    if debug_mode {
      'until_valid: loop {
        match input_channel.recv() {
          Ok(console::Key::Del) => {
            stdout_string = "".to_string();
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
        stdout_string = "".to_string();
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

        controller_input = keys
          .iter()
          .map(|k| (k == &key).then_some(std::time::Instant::now()))
          .zip(controller_input.iter())
          .map(|(next, curr)| next.or(*curr))
          .collect::<Vec<_>>()
          .try_into()
          .unwrap();

        stdin_queue.push_back(match key {
          console::Key::Char(c) => c as u8,
          console::Key::Backspace => 0x08,
          console::Key::Enter => 0x0A,
          console::Key::Tab => 0x09,
          console::Key::Del => 0x7F,
          _ => 0x00,
        });
      }

      Err(TryRecvError::Empty) => (),
      Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
    }

    match tick(&mut mc) {
      Ok(clocks) => {
        current_clocks += clocks;
      }
      Err(tick_trap) => {
        debug_mode = true;
        status_line = match tick_trap {
          TickTrap::MicrocodeFault => format!("Microcode fault"),
          TickTrap::IllegalOpcode => format!("Illegal opcode"),
          TickTrap::DebugRequest => format!("Debug request"),
        }
      }
    };

    let timestamp_threshold = std::time::Duration::from_millis(200);
    controller_input = controller_input
      .iter()
      .map(|timestamp| timestamp.and_then(|t| (t.elapsed() < timestamp_threshold).then_some(t)))
      .collect::<Vec<_>>()
      .try_into()
      .unwrap();

    mc.mem[0x00] = controller_input
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

    // was stdout written to?
    if mc.stdout != 0x00 {
      stdout_string.push(mc.stdout as char);
      mc.stdout = 0x00;
    }

    // was stdin read from?
    if mc.stdin == 0x00 {
      mc.stdin = stdin_queue.pop_front().unwrap_or(0x00);
    }

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
          render_display_buffer(&mc.mem[0xE0..0x100].try_into().unwrap()),
          render_controller_input(&mc.mem[0x00..0x01].try_into().unwrap())
        );
      }
      print!("\r\n");
      print!("{}", stdout_string);
      use std::io::Write;
      std::io::stdout().flush().unwrap();
    }
  }
}

fn tick(mc: &mut Microcomputer) -> Result<u128, TickTrap> {
  let mp = &mut mc.mp;

  // TODO copied from `emu.rs`
  macro_rules! mem_read {
    ($address:expr) => {{
      let address = $address;
      if address == 0x00 {
        // was stdin written to?
        if mc.stdin != 0x00 {
          let stdin = mc.stdin;
          // TODO move somewhere better
          if let Clock::Rising = mc.clk {
            mc.stdin = 0x00;
          }
          stdin
        } else {
          mc.mem[0x00] // controller input
        }
      } else {
        mc.mem[address as usize]
      }
    }};
  }

  // TODO copied from `emu.rs`
  macro_rules! mem_write {
    ($address:expr, $value:expr) => {{
      let address = $address;
      let value = $value;
      if address == 0x00 {
        // was stdout read from?
        if mc.stdout == 0x00 {
          // TODO move somewhere better
          if let Clock::Rising = mc.clk {
            mc.stdout = value;
          }
        } else {
          panic!("attempt to write to `stdout` more than once within one tick");
        }
      } else {
        mc.mem[address as usize] = value;
      }
    }};
  }

  // clock
  match mc.clk {
    Clock::Rising => mc.clk = Clock::High,
    Clock::High => mc.clk = Clock::Falling,
    Clock::Falling => mc.clk = Clock::Low,
    Clock::Low => mc.clk = Clock::Rising,
  };
  if let Reset::Asserted = mc.rst {
    mc.clk = Clock::Low;
  }

  // step counter
  if let Clock::Falling = mc.clk {
    if true {
      mp.sc = mp.sc.wrapping_add(1) & 0x20 - 1;
    }
  }

  // microcode and control logic
  let il = match mp.il & 0x80 {
    0b0 => mp.il | 0xF0, // map `psh` to `phn` as both have equivalent microcode
    _ => mp.il,          // not `psh`; pass through
  };
  mp.ctrl = u16_into_result(
    mp.mic[(il as usize & 0x80 - 1) * 0x02 * 0x20 | mp.cf as usize * 0x20 | mp.sc as usize],
  )?;
  mp.ones_data = match (
    mp.ctrl.ip_data,
    mp.ctrl.sp_data,
    mp.ctrl.mem_data,
    mp.ctrl.size_and_cin,
    mp.ctrl.ofst_and_cf,
    mp.ctrl.sum_data,
    mp.ctrl.nand_data,
  ) {
    (
      Signal::Inactive,
      Signal::Inactive,
      Signal::Inactive,
      Signal::Inactive,
      Signal::Inactive,
      Signal::Inactive,
      Signal::Inactive,
    ) => Signal::Active,
    _ => Signal::Inactive,
  };
  (mp.size_data, mp.set_cin) = match mp.ctrl.sum_data {
    Signal::Inactive => (mp.ctrl.size_and_cin, Signal::Inactive),
    Signal::Active => (Signal::Inactive, mp.ctrl.size_and_cin),
  };
  (mp.ofst_data, mp.cout_cf, mp.zero_cf) = match (mp.ctrl.sum_data, mp.ctrl.nand_data) {
    (Signal::Inactive, Signal::Inactive) => {
      (mp.ctrl.ofst_and_cf, Signal::Inactive, Signal::Inactive)
    }
    (Signal::Active, Signal::Inactive) => (Signal::Inactive, mp.ctrl.ofst_and_cf, Signal::Inactive),
    (Signal::Inactive, Signal::Active) => (Signal::Inactive, Signal::Inactive, mp.ctrl.ofst_and_cf),
    _ => return Err(TickTrap::MicrocodeFault),
  };

  // ones
  mp.ones = 0xFF;
  if let Signal::Active = mp.ones_data {
    mc.data = mp.ones;
  }

  // instruction latch and step counter
  if let Clock::Rising = mc.clk {
    if let Signal::Active = mp.ctrl.data_il {
      mp.il = mc.data;
    }
  }
  if let Signal::Active = mp.ctrl.clr_sc {
    mp.sc = 0x00; // asynchronous
  }
  if let Signal::Active = mp.size_data {
    mc.data = mp.size;
  }
  if let Signal::Active = mp.ofst_data {
    mc.data = mp.ofst;
  }
  if let Reset::Asserted = mc.rst {
    mp.il = 0x00;
    mp.sc = 0x00;
  }
  mp.size = 1 << (mp.il & 0b00000011); // decode_size
  mp.ofst = mp.il & 0b00001111; // decode_ofst

  // instruction pointer
  if let Clock::Rising = mc.clk {
    if let Signal::Active = mp.ctrl.data_ip {
      mp.ip = mc.data;
    }
  }
  if let Signal::Active = mp.ctrl.ip_data {
    mc.data = mp.ip;
  }
  if let Reset::Asserted = mc.rst {
    mp.ip = 0x00;
  }

  // stack pointer
  if let Clock::Rising = mc.clk {
    if let Signal::Active = mp.ctrl.data_sp {
      mp.sp = mc.data;
    }
  }
  if let Signal::Active = mp.ctrl.sp_data {
    mc.data = mp.sp;
  }
  if let Reset::Asserted = mc.rst {
    mp.sp = 0x00;
  }

  // carry flag
  if let Clock::Rising = mc.clk {
    if let Signal::Active = mp.cout_cf {
      mp.cf = mp.cout;
    }
    if let Signal::Active = mp.zero_cf {
      mp.cf = mp.zero;
    }
  }
  if let Reset::Asserted = mc.rst {
    mp.cf = false;
  }

  // address latch and memory
  mc.addr = mp.al;
  mc.read = mp.ctrl.mem_data;
  mc.wrt = mp.ctrl.data_mem;
  if let Clock::Rising = mc.clk {
    if let Signal::Active = mp.ctrl.data_al {
      mp.al = mc.data;
    }
    if let Signal::Active = mc.wrt {
      mem_write!(mc.addr, mc.data);

      // TODO remove
      // mc.mem[mc.addr as usize] = mc.data;
    }
  }
  if let Signal::Active = mc.read {
    mc.data = mem_read!(mc.addr);

    // TODO remove
    // mc.data = mc.mem[mc.addr as usize];
  }
  if let Reset::Asserted = mc.rst {
    mp.al = 0x00;
  }

  // X latch and Y latch and Z latch
  let sum = mp.xl as u16 + mp.yl as u16 + mp.cin as u16;
  let nand = !(mp.yl & mp.zl);
  mp.sum = sum as u8;
  mp.cout = sum > 0xFF;
  mp.nand = nand;
  mp.zero = nand == 0x00;
  mp.cin = match mp.ctrl.size_and_cin {
    Signal::Active => true,
    Signal::Inactive => false,
  };
  if let Clock::Rising = mc.clk {
    if let Signal::Active = mp.ctrl.data_xl {
      mp.xl = mc.data;
    }
    if let Signal::Active = mp.ctrl.data_yl {
      mp.yl = mc.data;
    }
    if let Signal::Active = mp.ctrl.data_zl {
      mp.zl = mc.data;
    }
  }
  if let Signal::Active = mp.ctrl.sum_data {
    mc.data = mp.sum;
  }
  if let Signal::Active = mp.ctrl.nand_data {
    mc.data = mp.nand;
  }
  if let Reset::Asserted = mc.rst {
    mp.xl = 0x00;
    mp.yl = 0x00;
    mp.zl = 0x00;
  }

  Ok(match mc.clk {
    Clock::Rising => 1,
    _ => 0,
  })
}

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

fn u16_into_result(u16: u16) -> Result<ControlWord, TickTrap> {
  match u16 {
    MICROCODE_FAULT_MAGIC => Err(TickTrap::MicrocodeFault),
    ILLEGAL_OPCODE_MAGIC => Err(TickTrap::IllegalOpcode),
    DEBUG_REQUEST_MAGIC => Err(TickTrap::DebugRequest),
    control_word => Ok(control_word.into()),
  }
}

impl std::fmt::Display for Microcomputer {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}\r\n{}\r\n{}\r\n{}\r\n{}",
      self.mp,
      format!(
        "CLK  RST  ADDR  DATA  READ  WRT\r\n{}  {}  {:02X}    {:02X}    {}    {}\r\n",
        self.clk, self.rst, self.addr, self.data, self.read, self.wrt
      ),
      render_memory(&self.mem, self.mp.ip, self.mp.sp, self.mp.cf),
      render_display_buffer(self.mem[0xE0..0x100].try_into().unwrap()),
      render_controller_input(self.mem[0x00..0x01].try_into().unwrap())
    )
  }
}

impl std::fmt::Display for Microprocessor {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}\r\n{}\r\n{}\r\n{}",
      format!(
        "IP  SP  CF  IL  SC  AL  XL  YL  ZL\r\n{:02X}  {:02X}  {:01X}   {:02X}  {:02X}  {:02X}  {:02X}  {:02X}  {:02X}\r\n",
        self.ip, self.sp, self.cf as u8, self.il, self.sc, self.al, self.xl, self.yl, self.zl
      ),
      format!(
        "CTRL  {} {} {} {} {} {} {} {}  {} {} {}\r\n      {} {} {} {} {} {} {} {}  {} {} {}\r\n",
        self.ctrl.clr_sc,
        self.ctrl.data_il,
        self.ctrl.size_and_cin,
        self.ctrl.ofst_and_cf,
        self.ctrl.ip_data,
        self.ctrl.data_ip,
        self.ctrl.sp_data,
        self.ctrl.data_sp,
        self.size_data,
        self.ofst_data,
        self.set_cin,
        self.ctrl.data_al,
        self.ctrl.mem_data,
        self.ctrl.data_mem,
        self.ctrl.data_xl,
        self.ctrl.data_yl,
        self.ctrl.data_zl,
        self.ctrl.sum_data,
        self.ctrl.nand_data,
        self.cout_cf,
        self.zero_cf,
        self.ones_data,
      ),
      format!(
        "ONES  SIZE  OFST  SUM  NAND  CIN  COUT  ZERO\r\n{:02X}    {:02X}    {:02X}    {:02X}   {:02X}    {:01X}    {:01X}     {:01X}\r\n",
        self.ones, self.size, self.ofst, self.sum, self.nand, self.cin as u8, self.cout as u8, self.zero as u8,
      ),
      format!(
        "MIC  ({} B)\r\n",
        self.mic.len()
      ),
    )
  }
}

impl std::fmt::Display for Clock {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let underline = "\u{005F}";
    let rising = "/";
    let overline = "\u{203E}";
    let falling = "\\";
    match self {
      Clock::Rising => write!(f, "{}{}{}", underline, rising, overline),
      Clock::High => write!(f, "{}{}{}", overline, overline, overline),
      Clock::Falling => write!(f, "{}{}{}", overline, falling, underline),
      Clock::Low => write!(f, "{}{}{}", underline, underline, underline),
    }
  }
}

impl std::fmt::Display for Reset {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Reset::Asserted => write!(f, "[#]"),
      Reset::Deasserted => write!(f, "[ ]"),
    }
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

// TODO copied from `emu.rs`

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

fn render_memory(memory: &[u8; MEM_SIZE], ip: u8, sp: u8, cf: bool) -> String {
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

fn render_display_buffer(display_buffer: &[u8; 0x20]) -> String {
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
        let pixel = display_buffer[address as usize] >> (0x07 - (x & 0x07)) & 0x01;
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

fn render_controller_input(controller_input: &[u8; 0x01]) -> String {
  let mut fmt = "".to_string();

  fn bit_to_str(controller_input: &[u8; 0x01], bit: u8) -> &'static str {
    match controller_input[0x00] >> bit & 0x01 {
      0b0 => "\u{2591}\u{2591}",
      0b1 => "\u{2588}\u{2588}",
      _ => unreachable!(),
    }
  }

  fmt += &format!(
    "    {}      {}    \r\n",
    bit_to_str(controller_input, 0),
    bit_to_str(controller_input, 4),
  );
  fmt += &format!(
    "  {}  {}  {}  {}  \r\n",
    bit_to_str(controller_input, 2),
    bit_to_str(controller_input, 3),
    bit_to_str(controller_input, 6),
    bit_to_str(controller_input, 7),
  );
  fmt += &format!(
    "    {}      {}    \r\n",
    bit_to_str(controller_input, 1),
    bit_to_str(controller_input, 5),
  );

  fmt
}
