const CTRL_NONE: u32 = 0b00000000000000000000000000000000;
const IP_TO_ADDR: u32 = 0b00000000000000000000000000000001;
const MEM_TO_DATA: u32 = 0b00000000000000000000000000000010;
const DATA_TO_IR: u32 = 0b00000000000000000000000000000100;
const ZERO_TO_OL: u32 = 0b00000000000000000000000000001000;
const ZERO_TO_SC: u32 = 0b00000000000000000000000000010000;
const INCIP_TO_IP: u32 = 0b00000000000000000000000000100000;
const DECSP_TO_SP: u32 = 0b00000000000000000000000001000000;
const OP_TO_ADDR: u32 = 0b00000000000000000000000010000000;
const IR_TO_DATA: u32 = 0b00000000000000000000000100000000;
const DATA_TO_MEM: u32 = 0b00000000000000000000001000000000;
const SZIR_TO_OL: u32 = 0b00000000000000000000010000000000;
const DATA_TO_AL: u32 = 0b00000000000000000000100000000000;
const DATA_TO_BL: u32 = 0b00000000000000000001000000000000;
const INCSP_TO_SP: u32 = 0b00000000000000000010000000000000;
const SUM_TO_DATA: u32 = 0b00000000000000000100000000000000;
const DATA_TO_IP: u32 = 0b00000000000000001000000000000000;

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

  let microcode_image = build_microcode_image();

  let mc = Microcomputer {
    mem: memory_image,
    mp: Microprocessor {
      ctrl: microcode_image,
      sc: 0x00,
      ip: 0x00,
      sp: 0x00,
      ir: 0x00,
      al: 0x00,
      bl: 0x00,
      ol: 0x00,

      ip_to_addr: false,
      data_to_ir: false,
      zero_to_ol: false,
      zero_to_sc: false,
      incip_to_ip: false,
      decsp_to_sp: false,
      op_to_addr: false,
      ir_to_data: false,
      szir_to_ol: false,
      data_to_al: false,
      data_to_bl: false,
      incsp_to_sp: false,
      sum_to_data: false,
      data_to_ip: false,
    },
    clk: Clock::Low,
    rst: true, // reset microcomputer on startup
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
  sp: u8,
  ir: u8,
  al: u8,
  bl: u8,
  ol: u8,

  ip_to_addr: bool,
  data_to_ir: bool,
  zero_to_ol: bool,
  zero_to_sc: bool,
  incip_to_ip: bool,
  decsp_to_sp: bool,
  op_to_addr: bool,
  ir_to_data: bool,
  szir_to_ol: bool,
  data_to_al: bool,
  data_to_bl: bool,
  incsp_to_sp: bool,
  sum_to_data: bool,
  data_to_ip: bool,
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
    mc.rst = false; // reset is only active for one cycle
    std::thread::sleep(std::time::Duration::from_millis(100));
  }
}

fn tick(mc: &mut Microcomputer) {
  let mp = &mut mc.mp;

  // clock
  match mc.clk {
    Clock::Rising => mc.clk = Clock::High,
    Clock::High => mc.clk = Clock::Falling,
    Clock::Falling => mc.clk = Clock::Low,
    Clock::Low => mc.clk = Clock::Rising,
  };
  if mc.rst {
    mc.clk = Clock::Low;
  }

  // step counter
  if let Clock::Falling = mc.clk {
    mp.sc = mp.sc.wrapping_add(1);
  }
  if mp.zero_to_sc {
    mp.sc = 0x00;
  }
  if mc.rst {
    mp.sc = 0x00;
  }

  // control logic
  let control_word = mp.ctrl[mp.ir as usize | (mp.sc as usize) << 8];
  mp.ip_to_addr = control_word & IP_TO_ADDR != 0;
  mc.mem_to_data = control_word & MEM_TO_DATA != 0;
  mp.data_to_ir = control_word & DATA_TO_IR != 0;
  mp.zero_to_ol = control_word & ZERO_TO_OL != 0;
  mp.zero_to_sc = control_word & ZERO_TO_SC != 0;
  mp.incip_to_ip = control_word & INCIP_TO_IP != 0;
  mp.decsp_to_sp = control_word & DECSP_TO_SP != 0;
  mp.op_to_addr = control_word & OP_TO_ADDR != 0;
  mp.ir_to_data = control_word & IR_TO_DATA != 0;
  mc.data_to_mem = control_word & DATA_TO_MEM != 0;
  mp.szir_to_ol = control_word & SZIR_TO_OL != 0;
  mp.data_to_al = control_word & DATA_TO_AL != 0;
  mp.data_to_bl = control_word & DATA_TO_BL != 0;
  mp.incsp_to_sp = control_word & INCSP_TO_SP != 0;
  mp.sum_to_data = control_word & SUM_TO_DATA != 0;
  mp.data_to_ip = control_word & DATA_TO_IP != 0;
  if control_word == CTRL_NONE {
    panic!(
      "Error: Unimplemented instruction: {:02X}#{:02X}",
      mp.ir, mp.sc
    );
  }

  // instruction pointer
  if mp.ip_to_addr {
    mc.addr = mp.ip;
  }
  if let Clock::Rising = mc.clk {
    if mp.data_to_ip {
      mp.ip = mc.data;
    }
    if mp.incip_to_ip {
      mp.ip = mp.ip.wrapping_add(1);
    }
  }
  if mc.rst {
    mp.ip = 0x00;
  }

  // instruction register
  if mp.ir_to_data {
    mc.data = mp.ir;
  }
  if let Clock::Rising = mc.clk {
    if mp.data_to_ir {
      mp.ir = mc.data;
    }
  }
  if mc.rst {
    mp.ir = 0x00;
  }

  // stack pointer
  if mp.op_to_addr {
    mc.addr = mp.sp.wrapping_add(mp.ol);
  }
  if let Clock::Rising = mc.clk {
    if mp.incsp_to_sp {
      mp.sp = mp.sp.wrapping_add(1);
    }
    if mp.decsp_to_sp {
      mp.sp = mp.sp.wrapping_sub(1);
    }
  }
  if mc.rst {
    mp.sp = 0x00;
  }

  // A latch
  if let Clock::Rising = mc.clk {
    if mp.data_to_al {
      mp.al = mc.data;
    }
  }
  if mc.rst {
    mp.al = 0x00;
  }

  // B latch
  if let Clock::Rising = mc.clk {
    if mp.data_to_bl {
      mp.bl = mc.data;
    }
  }
  if mc.rst {
    mp.bl = 0x00;
  }

  // offset latch
  if let Clock::Rising = mc.clk {
    if mp.szir_to_ol {
      mp.ol = 1 << (mp.ir & 0b00000011); // decode_size
    }
  }
  if mp.zero_to_ol {
    mp.ol = 0x00;
  }
  if mc.rst {
    mp.ol = 0x00;
  }

  // memory
  if mc.mem_to_data {
    mc.data = mc.mem[mc.addr as usize];
  }
  if let Clock::Rising = mc.clk {
    if mc.data_to_mem {
      mc.mem[mc.addr as usize] = mc.data;
    }
  }

  // arithmetic logic unit
  if mp.sum_to_data {
    mc.data = mp.al.wrapping_add(mp.bl);
  };
}

fn build_microcode_image() -> [u32; 0x10000] {
  let mut microcode_image = [CTRL_NONE; 0x10000];

  // fetch
  for instruction in 0x00..=0xFF {
    microcode_image[instruction | 0x00 << 8] |=
      IP_TO_ADDR | MEM_TO_DATA | DATA_TO_IR | ZERO_TO_OL | INCIP_TO_IP;
  }

  // psh
  for instruction in 0x00..=0x7F {
    microcode_image[instruction | 0x01 << 8] |= DECSP_TO_SP;
    microcode_image[instruction | 0x02 << 8] |= OP_TO_ADDR | IR_TO_DATA | DATA_TO_MEM;
    microcode_image[instruction | 0x03 << 8] |= ZERO_TO_SC;
  }

  // phn
  for instruction in 0xF0..=0xFF {
    microcode_image[instruction | 0x01 << 8] |= DECSP_TO_SP;
    microcode_image[instruction | 0x02 << 8] |= OP_TO_ADDR | IR_TO_DATA | DATA_TO_MEM;
    microcode_image[instruction | 0x03 << 8] |= ZERO_TO_SC;
  }

  // nop
  microcode_image[0xE8 | 0x01 << 8] |= ZERO_TO_SC;

  // add
  for instruction in 0x80..=0x83 {
    microcode_image[instruction | 0x01 << 8] |= OP_TO_ADDR | MEM_TO_DATA | DATA_TO_AL | SZIR_TO_OL;
    microcode_image[instruction | 0x02 << 8] |= OP_TO_ADDR | MEM_TO_DATA | DATA_TO_BL;
    microcode_image[instruction | 0x03 << 8] |=
      OP_TO_ADDR | SUM_TO_DATA | DATA_TO_MEM | INCSP_TO_SP;
    microcode_image[instruction | 0x04 << 8] |= ZERO_TO_SC;
  }

  // sti
  microcode_image[0xE3 | 0x01 << 8] |= OP_TO_ADDR | MEM_TO_DATA | DATA_TO_IP | INCSP_TO_SP;
  microcode_image[0xE3 | 0x02 << 8] |= ZERO_TO_SC;

  microcode_image
}

impl std::fmt::Display for Microcomputer {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "Microcomputer:")?;
    writeln!(f, "  Memory:")?;
    for y in 0..0x10 {
      write!(f, "    ")?;
      for x in 0..0x10 {
        write!(f, "{:02X} ", self.mem[(y << 0x04 | x) as usize])?;
      }
      writeln!(f)?;
    }
    writeln!(f, "  Microprocessor:")?;
    writeln!(f, "    Microcode:")?;
    writeln!(f, "      [...]")?;
    writeln!(f, "    Registers:")?;
    writeln!(f, "      SC: {:02X}", self.mp.sc)?;
    writeln!(f, "      IP: {:02X}", self.mp.ip)?;
    writeln!(f, "      SP: {:02X}", self.mp.sp)?;
    writeln!(f, "      IR: {:02X}", self.mp.ir)?;
    writeln!(f, "      AL: {:02X}", self.mp.al)?;
    writeln!(f, "      BL: {:02X}", self.mp.bl)?;
    writeln!(f, "      OL: {:02X}", self.mp.ol)?;
    writeln!(f, "    Control Signals:")?;
    writeln!(f, "      IP_TO_ADDR: {:01X}", self.mp.ip_to_addr as u8)?;
    writeln!(f, "      DATA_TO_IR: {:01X}", self.mp.data_to_ir as u8)?;
    writeln!(f, "      ZERO_TO_OL: {:01X}", self.mp.zero_to_ol as u8)?;
    writeln!(f, "      ZERO_TO_SC: {:01X}", self.mp.zero_to_sc as u8)?;
    writeln!(f, "      INCIP_TO_IP: {:01X}", self.mp.incip_to_ip as u8)?;
    writeln!(f, "      DECSP_TO_SP: {:01X}", self.mp.decsp_to_sp as u8)?;
    writeln!(f, "      OP_TO_ADDR: {:01X}", self.mp.op_to_addr as u8)?;
    writeln!(f, "      IR_TO_DATA: {:01X}", self.mp.ir_to_data as u8)?;
    writeln!(f, "      SZIR_TO_OL: {:01X}", self.mp.szir_to_ol as u8)?;
    writeln!(f, "      DATA_TO_AL: {:01X}", self.mp.data_to_al as u8)?;
    writeln!(f, "      DATA_TO_BL: {:01X}", self.mp.data_to_bl as u8)?;
    writeln!(f, "      INCSP_TO_SP: {:01X}", self.mp.incsp_to_sp as u8)?;
    writeln!(f, "      SUM_TO_DATA: {:01X}", self.mp.sum_to_data as u8)?;
    writeln!(f, "      DATA_TO_IP: {:01X}", self.mp.data_to_ip as u8)?;
    writeln!(f, "  Clock State: {}", self.clk)?;
    writeln!(f, "  Reset State: {:01X}", self.rst as u8)?;
    writeln!(f, "  Data Bus: {:02X}", self.data)?;
    writeln!(f, "  Address Bus: {:02X}", self.addr)?;
    writeln!(f, "  Control Signals:")?;
    writeln!(f, "    DATA_TO_MEM: {:01X}", self.data_to_mem as u8)?;
    writeln!(f, "    MEM_TO_DATA: {:01X}", self.mem_to_data as u8)?;
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
