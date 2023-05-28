fn main() {
  let args: Vec<String> = std::env::args().collect();
  // if args.len() != 3 {
  //   println!("Usage: sim <image file> <microcode file>");
  //   std::process::exit(1);
  // }

  let image_file: &String = &args[1];

  let memory = std::fs::read(image_file)
    .unwrap_or_else(|_| {
      println!("Error: Unable to read file: {}", image_file);
      std::process::exit(1);
    })
    .try_into()
    .unwrap_or_else(|_| {
      println!("Error: Memory image has incorrect size");
      std::process::exit(1);
    });

  // let microcode_file: &String = &args[2];
  //
  // let microcode = std::fs::read(microcode_file)
  //   .unwrap_or_else(|_| {
  //     println!("Error: Unable to read file: {}", image_file);
  //     std::process::exit(1);
  //   })
  //   .try_into()
  //   .unwrap_or_else(|_| {
  //     println!("Error: Microcode has incorrect size");
  //     std::process::exit(1);
  //   });

  let microcode = [0x00; 0x10];

  simulate(memory, microcode, 100000);
}

fn simulate(memory: [u8; 0x10], microcode: [u8; 0x10], clock: u128) {
  println!("sim");

  let mut microcomputer = Microcomputer {
    cpu: Microprocessor {
      sc: StepCounter(0x00),
      ip: InstructionPointer(0x00),
      ir: Register(0x00),
      ctrl: ControlLogic(microcode),
      ar: Register(0x00),
      br: Register(0x00),

      ip_to_addr: ControlSignal(false),
      data_to_ir: ControlSignal(false),
      zero_to_or: ControlSignal(false),
      zero_to_sr: ControlSignal(false),
    },
    clk: Clock::High,
    rst: Reset::Low,
    data: Bus(0x00),
    addr: Bus(0x00),
    mem: Memory(memory),

    data_to_mem: ControlSignal(false),
    mem_to_data: ControlSignal(false),
  };

  loop {
    microcomputer.tick();
  }
}

#[derive(Debug)]
struct Microcomputer {
  cpu: Microprocessor,
  clk: Clock,
  rst: Reset,
  data: Bus,
  addr: Bus,
  mem: Memory,

  data_to_mem: ControlSignal,
  mem_to_data: ControlSignal,
}

impl Microcomputer {
  fn tick(&mut self) {
    println!("{:#?}", self);
    println!("-----------------------------------------------------------------------------------------------");

    self.cpu.tick(
      &self.clk,
      &self.rst,
      &mut self.data,
      &mut self.addr,
      &mut self.data_to_mem,
      &mut self.mem_to_data,
    );
    self.mem.tick(
      &self.data_to_mem,
      &self.mem_to_data,
      &self.addr,
      &mut self.data,
    );
    self.clk.tick(&self.rst);
  }
}

#[derive(Debug)]
struct Microprocessor {
  sc: StepCounter,
  ip: InstructionPointer,
  ir: Register,
  ctrl: ControlLogic,
  ar: Register,
  br: Register,

  ip_to_addr: ControlSignal,
  data_to_ir: ControlSignal,
  zero_to_or: ControlSignal,
  zero_to_sr: ControlSignal,
}

impl Microprocessor {
  fn tick(
    &mut self,
    clk: &Clock,
    rst: &Reset,
    data: &mut Bus,
    addr: &mut Bus,
    data_to_mem: &mut ControlSignal,
    mem_to_data: &mut ControlSignal,
  ) {
    self
      .sc
      .tick(clk, rst, &ControlSignal(true), &ControlSignal(false));
    self.ip.tick(clk, rst, addr, &self.ip_to_addr);
    self.ctrl.tick(
      clk,
      rst,
      &self.sc,
      &self.ir,
      &mut self.ip_to_addr,
      mem_to_data,
      &mut self.data_to_ir,
      &mut self.zero_to_or,
      &mut self.zero_to_sr,
    );
  }
}

#[derive(Debug)]
enum Clock {
  Rising,
  High,
  Falling,
  Low,
}

impl Clock {
  fn tick(&mut self, rst: &Reset) {
    match rst {
      Reset::High => *self = Clock::High,
      Reset::Low => match self {
        Clock::Rising => *self = Clock::High,
        Clock::High => *self = Clock::Falling,
        Clock::Falling => *self = Clock::Low,
        Clock::Low => *self = Clock::Rising,
      },
    };
  }
}

#[derive(Debug)]
enum Reset {
  High,
  Low,
}

#[derive(Debug)]
struct Register(u8);

#[derive(Debug)]
struct InstructionPointer(u8);

impl InstructionPointer {
  fn tick(&mut self, clk: &Clock, rst: &Reset, addr: &mut Bus, ip_to_addr: &ControlSignal) {
    match rst {
      Reset::High => self.0 = 0x00,
      Reset::Low => match ip_to_addr.0 {
        true => addr.0 = self.0,
        false => match clk {
          Clock::Rising => {}
          Clock::High => {}
          Clock::Falling => {}
          Clock::Low => {}
        },
      },
    };
  }
}

#[derive(Debug)]
struct StepCounter(u8);

impl StepCounter {
  fn tick(&mut self, clk: &Clock, rst: &Reset, increment: &ControlSignal, clear: &ControlSignal) {
    match rst {
      Reset::High => self.0 = 0x00,
      Reset::Low => match clk {
        Clock::Rising => {
          match increment.0 {
            true => self.0 += 1,
            false => {}
          };
          match clear.0 {
            true => self.0 = 0x00,
            false => {}
          };
        }
        Clock::High => {}
        Clock::Falling => {}
        Clock::Low => {}
      },
    };
  }
}

#[derive(Debug)]
struct Bus(u8);

#[derive(Debug)]
struct ControlSignal(bool);

#[derive(Debug)]
struct Memory([u8; 0x10]);

impl Memory {
  fn tick(
    &mut self,
    data_mem: &ControlSignal,
    mem_data: &ControlSignal,
    addr: &Bus,
    data: &mut Bus,
  ) {
    match (data_mem.0, mem_data.0) {
      (false, false) => {}
      (false, true) => data.0 = self.0[addr.0 as usize],
      (true, false) => self.0[addr.0 as usize] = data.0,
      (true, true) => {
        panic!("Error: simultaneous read and write to `MEM`")
      }
    };
  }
}

#[derive(Debug)]
struct ControlLogic([u8; 0x10]);

impl ControlLogic {
  fn tick(
    &self,
    clk: &Clock,
    rst: &Reset,
    sc: &StepCounter,
    ir: &Register,
    ip_to_addr: &mut ControlSignal,
    mem_to_data: &mut ControlSignal,
    data_to_ir: &mut ControlSignal,
    zero_to_or: &mut ControlSignal,
    zero_to_sr: &mut ControlSignal,
  ) {
    match clk {
      Clock::Rising => {}
      Clock::High => {}
      Clock::Falling => {
        (
          ip_to_addr.0,
          mem_to_data.0,
          data_to_ir.0,
          zero_to_or.0,
          zero_to_sr.0,
        ) = match ir.0 {
          0xE8 => match sc.0 {
            0x00 => (true, false, false, false, false),
            0x01 => (false, true, false, false, false),
            0x02 => (false, false, true, false, false),
            0x03 => (false, false, false, true, false),
            0x04 => (false, false, false, true, true),
            _ => panic!(
              "Error: Unimplemented step: {:02X} for instruction: {:02X}",
              sc.0, ir.0
            ),
          },
          _ => match sc.0 {
            0x00 => (true, false, false, false, false),
            0x01 => (false, true, false, false, false),
            0x02 => (false, false, true, false, false),
            0x03 => (false, false, false, true, false),
            _ => panic!("Error: Unimplemented instruction: {:02X}", ir.0),
          },
        }
      }
      Clock::Low => {}
    };
  }
}
