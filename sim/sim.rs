const IP_TO_ADDR: u32 = 0b00000000000000000000000000000001;
const MEM_TO_DATA: u32 = 0b00000000000000000000000000000010;
const DATA_TO_IR: u32 = 0b00000000000000000000000000000100;
const ZERO_TO_OR: u32 = 0b00000000000000000000000000001000;
const ZERO_TO_SC: u32 = 0b00000000000000000000000000010000;
const INCIP_TO_IP: u32 = 0b00000000000000000000000000100000;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  // if args.len() != 3 {
  //   println!("Usage: sim <memory image file> <microcode image file>");
  //   std::process::exit(1);
  // }

  let memory_image_file: &String = &args[1];

  let memory_image = std::fs::read(memory_image_file)
    .unwrap_or_else(|_| {
      println!("Error: Unable to read file: {}", memory_image_file);
      std::process::exit(1);
    })
    .try_into()
    .unwrap_or_else(|_| {
      println!("Error: Memory image has incorrect size");
      std::process::exit(1);
    });

  // let microcode_image_file: &String = &args[2];
  //
  // let microcode = std::fs::read(microcode_image_file)
  //   .unwrap_or_else(|_| {
  //     println!("Error: Unable to read file: {}", microcode_image_file);
  //     std::process::exit(1);
  //   })
  //   .try_into()
  //   .unwrap_or_else(|_| {
  //     println!("Error: Microcode image has incorrect size");
  //     std::process::exit(1);
  //   });

  let mut microcode_image = [0x0000; 0x10000];

  // fetch
  for instruction in 0..0x100 {
    microcode_image[instruction | 0x00 << 8] |= IP_TO_ADDR | MEM_TO_DATA | DATA_TO_IR | ZERO_TO_OR;
    microcode_image[instruction | 0x01 << 8] |= INCIP_TO_IP;
  }

  // psh
  for immediate in 0..0x7F {
    let instruction = immediate; // encode_immediate
    microcode_image[instruction | 0x01 << 8] |= 0x00; // TODO
  }

  // nop
  microcode_image[0xE8 | 0x01 << 8] |= ZERO_TO_SC;

  let mc = Microcomputer {
    mem: memory_image,
    mp: Microprocessor {
      ctrl: microcode_image,
      sc: 0x00,
      ip: 0x00,
      ir: 0x00,
      ar: 0x00,
      br: 0x00,

      ip_to_addr: false,
      data_to_ir: false,
      zero_to_or: false,
      zero_to_sc: false,
      incip_to_ip: false,
    },
    clk: Clock::High,
    rst: false,
    data: 0x00,
    addr: 0x00,

    data_to_mem: false,
    mem_to_data: false,
  };

  simulate(mc);
}

#[derive(Clone, Copy, Debug)]
struct Microcomputer {
  mem: [u8; 0x100],
  mp: Microprocessor,
  clk: Clock,
  rst: bool,
  data: u8,
  addr: u8,

  data_to_mem: bool,
  mem_to_data: bool,
}

#[derive(Clone, Copy, Debug)]
struct Microprocessor {
  ctrl: [u32; 0x10000],
  sc: u8,
  ip: u8,
  ir: u8,
  ar: u8,
  br: u8,

  ip_to_addr: bool,
  data_to_ir: bool,
  zero_to_or: bool,
  zero_to_sc: bool,
  incip_to_ip: bool,
}

#[derive(Clone, Copy, Debug)]
enum Clock {
  Rising,
  High,
  Falling,
  Low,
}

fn simulate(mut mc: Microcomputer) {
  loop {
    println!("{}", mc);
    tick(&mut mc);
  }
}

fn tick(mc: &mut Microcomputer) {
  let mp = &mut mc.mp;

  // clock
  match mc.rst {
    true => mc.clk = Clock::High,
    false => match mc.clk {
      Clock::Rising => mc.clk = Clock::High,
      Clock::High => mc.clk = Clock::Falling,
      Clock::Falling => mc.clk = Clock::Low,
      Clock::Low => mc.clk = Clock::Rising,
    },
  };

  // instruction pointer
  match mc.rst {
    true => mp.ip = 0x00,
    false => {
      match mp.ip_to_addr {
        true => mc.addr = mp.ip,
        false => (),
      }
      match mc.clk {
        Clock::Rising => match mp.incip_to_ip {
          true => mp.ip += 1,
          false => (),
        },
        Clock::High => (),
        Clock::Falling => (),
        Clock::Low => (),
      }
    }
  };

  // instruction register
  match mc.rst {
    true => mp.ir = 0x00,
    false => match mc.clk {
      Clock::Rising => match mp.data_to_ir {
        true => mp.ir = mc.data,
        false => (),
      },
      Clock::High => (),
      Clock::Falling => (),
      Clock::Low => (),
    },
  };

  // step counter
  match mc.rst {
    true => mp.sc = 0x00,
    false => match mc.clk {
      Clock::Rising => match mp.zero_to_sc {
        true => mp.sc = 0x00,
        false => mp.sc = mp.sc.wrapping_add(1),
      },
      Clock::High => (),
      Clock::Falling => (),
      Clock::Low => (),
    },
  };

  // memory
  match (mc.data_to_mem, mc.mem_to_data) {
    (false, false) => (),
    (false, true) => mc.data = mc.mem[mc.addr as usize],
    (true, false) => mc.mem[mc.addr as usize] = mc.data,
    (true, true) => {
      panic!("Error: simultaneous read and write to memory")
    }
  };

  // control logic
  match mc.clk {
    Clock::Rising => (),
    Clock::High => (),
    Clock::Falling => {
      let control_word = mp.ctrl[mp.ir as usize | (mp.sc as usize) << 8];

      mp.ip_to_addr = control_word & IP_TO_ADDR != 0;
      mc.mem_to_data = control_word & MEM_TO_DATA != 0;
      mp.data_to_ir = control_word & DATA_TO_IR != 0;
      mp.zero_to_or = control_word & ZERO_TO_OR != 0;
      mp.zero_to_sc = control_word & ZERO_TO_SC != 0;
      mp.incip_to_ip = control_word & INCIP_TO_IP != 0;

      if control_word == 0x0000 {
        panic!(
          "Error: Unimplemented instruction: {:02X}#{:02X}",
          mp.ir, mp.sc
        );
      }
    }
    Clock::Low => (),
  };
}

impl std::fmt::Display for Microcomputer {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "Microcomputer:")?;
    writeln!(f, "  Microprocessor:")?;
    writeln!(f, "    Registers:")?;
    writeln!(f, "      SC: {:02X}", self.mp.sc)?;
    writeln!(f, "      IP: {:02X}", self.mp.ip)?;
    writeln!(f, "      IR: {:02X}", self.mp.ir)?;
    writeln!(f, "      AR: {:02X}", self.mp.ar)?;
    writeln!(f, "      BR: {:02X}", self.mp.br)?;
    writeln!(f, "    Signals:")?;
    writeln!(f, "      IP_TO_ADDR: {:02X}", self.mp.ip_to_addr as u8)?;
    writeln!(f, "      DATA_TO_IR: {:02X}", self.mp.data_to_ir as u8)?;
    writeln!(f, "      ZERO_TO_OR: {:02X}", self.mp.zero_to_or as u8)?;
    writeln!(f, "      ZERO_TO_SC: {:02X}", self.mp.zero_to_sc as u8)?;
    writeln!(f, "      INCIP_TO_IP: {:02X}", self.mp.incip_to_ip as u8)?;
    writeln!(f, "  Clock: {}", self.clk)?;
    writeln!(f, "  Reset: {:02X}", self.rst as u8)?;
    writeln!(f, "  Data: {:02X}", self.data)?;
    writeln!(f, "  Address: {:02X}", self.addr)?;
    writeln!(f, "  Signals:")?;
    writeln!(f, "    DATA_TO_MEM: {:02X}", self.data_to_mem as u8)?;
    writeln!(f, "    MEM_TO_DATA: {:02X}", self.mem_to_data as u8)?;
    Ok(())
  }
}

impl std::fmt::Display for Clock {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Clock::Rising => write!(f, "Rising"),
      Clock::High => write!(f, "High"),
      Clock::Falling => write!(f, "Falling"),
      Clock::Low => write!(f, "Low"),
    }
  }
}
