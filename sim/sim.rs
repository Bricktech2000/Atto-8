use std::collections::VecDeque;

#[path = "../misc/common/common.rs"]
mod common;
use common::*;

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

  let mc = Microcomputer {
    mem: memory_image,
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
      pull: Signal::Inactive,

      ones: 0x00,
      sum: 0x00,
      nand: 0x00,
      cin: false,
      cout: false,
      zero: false,

      mic: microcode_image,
    },

    clk: Clock::Low,
    rst: Reset::Deasserted,
    addr: 0x00,
    data: 0x00,
    read: Signal::Inactive,
    wrt: Signal::Inactive,
  };

  common::execute(mc, 1000000);
}

struct Microcomputer {
  mem: [u8; common::MEM_SIZE], // memory
  mp: Microprocessor,          // microprocessor

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
  pull: Signal,      // pull-up derivation

  ones: u8,   // ones derivation
  sum: u8,    // sum derivation
  nand: u8,   // not-and derivation
  cin: bool,  // sum carry-in derivation
  cout: bool, // sum carry-out derivation
  zero: bool, // nand is-zero derivation

  mic: [u16; common::MIC_SIZE], // microcode derivation
}

pub enum Clock {
  Rising,
  High,
  Falling,
  Low,
}

pub enum Reset {
  Asserted,
  Deasserted,
}

impl Tickable for Microcomputer {
  fn reset(
    &mut self,
    stdin: &mut VecDeque<u8>,
    stdout: &mut VecDeque<u8>,
    display: &mut [u8; common::DISPLAY_BUFFER_LEN],
    controller: &mut u8,
  ) {
    self.rst = Reset::Asserted;
    self
      .tick(stdin, stdout, display, controller)
      .unwrap_or_else(|_| {
        panic!("Tick trap during reset sequence");
      });
    self.rst = Reset::Deasserted;
  }

  fn tick(
    &mut self,
    stdin: &mut VecDeque<u8>,
    stdout: &mut VecDeque<u8>,
    display: &mut [u8; common::DISPLAY_BUFFER_LEN],
    controller: &mut u8,
  ) -> Result<u128, TickTrap> {
    let mp = &mut self.mp;

    // clock
    match self.clk {
      Clock::Rising => self.clk = Clock::High,
      Clock::High => self.clk = Clock::Falling,
      Clock::Falling => self.clk = Clock::Low,
      Clock::Low => self.clk = Clock::Rising,
    };
    if let Reset::Asserted = self.rst {
      self.clk = Clock::Low;
    }

    // step counter
    if let Clock::Falling = self.clk {
      if true {
        mp.sc = mp.sc.wrapping_add(1) & 0x20 - 1;
      }
    }

    // microcode and control logic
    let il = match mp.il & 0x80 {
      0b0 => mp.il | 0xF0, // map `psh` to `phn` as both have equivalent microcode
      _ => mp.il,          // not `psh`; pass through
    };
    let il = il & 0x80 - 1; // ignore `psh`s as they have been mapped to `phn`s
    mp.ctrl = common::u16_into_result(
      mp.mic[il as usize * 0x02 * 0x20 | mp.cf as usize * 0x20 | mp.sc as usize],
    )?;
    let active_count = [
      mp.ctrl.ip_data,
      mp.ctrl.sp_data,
      mp.ctrl.mem_data,
      mp.ctrl.sum_data,
      mp.ctrl.nand_data,
    ]
    .into_iter()
    .filter(|s| match s {
      Signal::Active => true,
      Signal::Inactive => false,
    })
    .count();
    mp.pull = match active_count {
      0 => Ok(Signal::Active),
      1 => Ok(Signal::Inactive),
      _ => Err(TickTrap::BusFault),
    }?;

    // ones
    mp.ones = 0xFF;
    if let Signal::Active = mp.pull {
      self.data = mp.ones;
    }

    // instruction latch and step counter
    if let Clock::Rising = self.clk {
      if let Signal::Active = mp.ctrl.data_il {
        mp.il = self.data;
      }
    }
    if let Signal::Active = mp.ctrl.clr_sc {
      mp.sc = 0x00; // asynchronous
    }
    if let Reset::Asserted = self.rst {
      mp.il = 0x00;
      mp.sc = 0x00;
    }

    // instruction pointer
    if let Clock::Rising = self.clk {
      if let Signal::Active = mp.ctrl.data_ip {
        mp.ip = self.data;
      }
    }
    if let Signal::Active = mp.ctrl.ip_data {
      self.data = mp.ip;
    }
    if let Reset::Asserted = self.rst {
      mp.ip = 0x00;
    }

    // stack pointer
    if let Clock::Rising = self.clk {
      if let Signal::Active = mp.ctrl.data_sp {
        mp.sp = self.data;
      }
    }
    if let Signal::Active = mp.ctrl.sp_data {
      self.data = mp.sp;
    }
    if let Reset::Asserted = self.rst {
      mp.sp = 0x00;
    }

    // carry flag
    if let Clock::Rising = self.clk {
      if let Signal::Active = mp.ctrl.data_cf {
        mp.cf = match (mp.ctrl.sum_data, mp.ctrl.nand_data) {
          (Signal::Active, Signal::Inactive) => Ok(mp.cout),
          (Signal::Inactive, Signal::Active) => Ok(mp.zero),
          _ => Err(TickTrap::BusFault),
        }?;
      }
    }
    if let Reset::Asserted = self.rst {
      mp.cf = false;
    }

    // address latch and memory
    self.addr = mp.al;
    self.read = mp.ctrl.mem_data;
    self.wrt = mp.ctrl.data_mem;
    if let Clock::Rising = self.clk {
      if let Signal::Active = mp.ctrl.data_al {
        mp.al = self.data;
      }
      if let Signal::Active = self.wrt {
        // stdout
        if self.addr == 0x00 {
          stdout.push_back(self.data);
        }
        // display
        if self.addr as usize & common::DISPLAY_BUFFER == common::DISPLAY_BUFFER {
          display[self.addr as usize & !common::DISPLAY_BUFFER] = self.data
        }
        self.mem[self.addr as usize] = self.data;
      }
    }
    if let Signal::Active = self.read {
      // stdin and controller
      if self.addr == 0x00 {
        self.data = *stdin.front().unwrap_or(controller);
        if let Clock::Rising = self.clk {
          stdin.pop_front();
        }
      } else {
        self.data = self.mem[self.addr as usize];
      }
    }
    if let Reset::Asserted = self.rst {
      mp.al = 0x00;
      stdin.push_back(self.mem[0x00]);
      display.copy_from_slice(
        &self.mem[common::DISPLAY_BUFFER..common::DISPLAY_BUFFER + common::DISPLAY_BUFFER_LEN],
      );
    }

    // X latch and Y latch and Z latch
    let sum = mp.xl as u16 + mp.yl as u16 + mp.cin as u16;
    let nand = !(mp.yl & mp.zl);
    mp.sum = sum as u8;
    mp.cout = sum > 0xFF;
    mp.nand = nand;
    mp.zero = nand == 0x00;
    mp.cin = match mp.ctrl.set_cin {
      Signal::Active => true,
      Signal::Inactive => false,
    };
    if let Clock::Rising = self.clk {
      if let Signal::Active = mp.ctrl.data_xl {
        mp.xl = self.data;
      }
      if let Signal::Active = mp.ctrl.data_yl {
        mp.yl = self.data;
      }
      if let Signal::Active = mp.ctrl.data_zl {
        mp.zl = self.data;
      }
    }
    if let Signal::Active = mp.ctrl.sum_data {
      self.data = mp.sum;
    }
    if let Signal::Active = mp.ctrl.nand_data {
      self.data = mp.nand;
    }
    if let Reset::Asserted = self.rst {
      mp.xl = 0x00;
      mp.yl = 0x00;
      mp.zl = 0x00;
    }

    Ok(match self.clk {
      Clock::Rising => 1,
      _ => 0,
    })
  }
}

impl std::fmt::Display for Microcomputer {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}\r\n{}\r\n{}",
      self.mp,
      format!(
        "CLK  RST  ADDR  DATA  READ  WRT\r\n{}  {}  {:02X}    {:02X}    {}    {}\r\n",
        self.clk, self.rst, self.addr, self.data, self.read, self.wrt
      ),
      common::render_memory(&self.mem, self.mp.ip, self.mp.sp, self.mp.cf),
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
        "CTRL  {} {} {} {} {} {} {} {}\r\n      {} {} {} {} {} {} {} {}\r\n",
        self.ctrl.clr_sc,
        self.ctrl.data_il,
        self.ctrl.data_cf,
        self.ctrl.ip_data,
        self.ctrl.data_ip,
        self.ctrl.sp_data,
        self.ctrl.data_sp,
        self.ctrl.data_al,
        self.ctrl.mem_data,
        self.ctrl.data_mem,
        self.ctrl.data_xl,
        self.ctrl.data_yl,
        self.ctrl.data_zl,
        self.ctrl.set_cin,
        self.ctrl.sum_data,
        self.ctrl.nand_data,
      ),
      format!(
        "PULL  ONES  SUM  NAND  CIN  COUT  ZERO\r\n{}    {:02X}    {:02X}   {:02X}    {:01X}    {:01X}     {:01X}\r\n",
        self.pull, self.ones, self.sum, self.nand, self.cin as u8, self.cout as u8, self.zero as u8,
      ),
      format!(
        "MIC  ({:#X} words)\r\n",
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
