fn main() {
  let args: Vec<String> = std::env::args().collect();
  // if args.len() != 3 {
  //   println!("Usage: sim <memory image file> <microcode image file>");
  //   std::process::exit(1);
  // }

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

  // let microcode_image_file: &String = &args[2];
  //
  // let microcode = std::fs::read(microcode_image_file)
  //   .unwrap_or_else(|_| {
  //     println!("Sim: Error: Unable to read file `{}`", microcode_image_file);
  //     std::process::exit(1);
  //   })
  //   .try_into()
  //   .unwrap_or_else(|_| {
  //     println!(
  //       "Sim: Error: Microcode image `{}` has incorrect size",
  //       microcode_image_file
  //     );
  //     std::process::exit(1);
  //   });

  let microcode_image = build_microcode_image();

  let mc = Microcomputer {
    mem: memory_image,
    clk: Clock::Low,
    rst: Reset::Asserted, // reset microcomputer on startup
    addr: 0x00,
    data: 0x00,
    read: Signal::Inactive,
    wrt: Signal::Inactive,
    mp: Microprocessor {
      rom: microcode_image,
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
      imm: 0x00,
      size: 0x00,
      ofst: 0x00,
      sum: 0x00,
      nand: 0x00,
    },
  };

  simulate(mc);
}

const MAX_STEPS: usize = 0x20;
const MEM_SIZE: usize = 0x100;
type MicrocodeImage = [[[Result<ControlWord, TickTrap>; MAX_STEPS]; MEM_SIZE]; 2];

#[derive(Clone, Copy, Debug)]
struct Microcomputer {
  mem: [u8; MEM_SIZE], // memory
  clk: Clock,          // clock state
  rst: Reset,          // reset state
  addr: u8,            // address bus
  data: u8,            // data bus
  read: Signal,        // memory read
  wrt: Signal,         // memory write
  mp: Microprocessor,  // microprocessor
}

#[derive(Clone, Copy, Debug)]
struct Microprocessor {
  rom: MicrocodeImage, // microcode read-only memory
  ip: u8,              // instruction pointer
  sp: u8,              // stack pointer
  cf: bool,            // carry flag
  il: u8,              // instruction latch
  sc: u8,              // step counter
  al: u8,              // address latch
  xl: u8,              // X latch
  yl: u8,              // Y latch
  zl: u8,              // Z latch
  ctrl: ControlWord,   // control word
  imm: u8,             // immediate derivation
  size: u8,            // size derivation
  ofst: u8,            // offset derivation
  sum: u8,             // sum derivation
  nand: u8,            // not-and derivation
}

#[derive(Clone, Copy, Default, Debug)]
struct ControlWord {
  cin_sum: Signal,
  cout_cf: Signal,

  data_il: Signal,
  zero_sc: Signal,
  size_data: Signal,
  ofst_data: Signal,

  ip_data: Signal,
  data_ip: Signal,

  sp_data: Signal,
  data_sp: Signal,

  data_al: Signal,
  mem_data: Signal,
  data_mem: Signal,

  data_xl: Signal,
  data_yl: Signal,
  data_zl: Signal,
  sum_data: Signal,
  nand_data: Signal,
}

#[derive(Clone, Copy, Debug)]
enum TickTrap {
  Todo, // TODO remove
  StepOverflow,
  DebugRequest,
  IllegalOpcode(u8),
}

#[derive(Clone, Copy, Debug)]
enum Clock {
  Rising,
  High,
  Falling,
  Low,
}

#[derive(Clone, Copy, Debug)]
enum Reset {
  Asserted,
  Deasserted,
}

#[derive(Clone, Copy, Default, Debug)]
enum Signal {
  Active,
  #[default]
  Inactive,
}

fn simulate(mut mc: Microcomputer) {
  loop {
    println!("{}", mc);
    match tick(&mut mc) {
      Ok(_clocks) => (), // TODO use meaningfully
      Err(tick_trap) => match tick_trap {
        TickTrap::Todo => {
          panic!("Unimplemented instruction")
        }
        TickTrap::StepOverflow => {
          panic!("Step counter overflow")
        }
        TickTrap::DebugRequest => {
          panic!("Debug request")
        }
        TickTrap::IllegalOpcode(opcode) => {
          panic!("Illegal opcode `0x{:02X}`", opcode)
        }
      },
    }
    mc.rst = Reset::Deasserted; // reset is only active for one cycle

    // let mut input = String::new();
    // std::io::stdin().read_line(&mut input).unwrap();
    // assert_eq!(input, "\n");

    std::thread::sleep(std::time::Duration::from_millis(5));
  }
}

fn tick(mc: &mut Microcomputer) -> Result<u128, TickTrap> {
  let mp = &mut mc.mp;

  // control logic
  mp.ctrl = mp.rom[mp.cf as usize][mp.il as usize][mp.sc as usize]?;

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

  // ones
  if let (
    Signal::Inactive,
    Signal::Inactive,
    Signal::Inactive,
    Signal::Inactive,
    Signal::Inactive,
    Signal::Inactive,
    Signal::Inactive,
  ) = (
    mp.ctrl.ip_data,
    mp.ctrl.sp_data,
    mp.ctrl.mem_data,
    mp.ctrl.size_data,
    mp.ctrl.ofst_data,
    mp.ctrl.sum_data,
    mp.ctrl.nand_data,
  ) {
    mc.data = 0xFF;
  }

  // instruction latch and step counter
  if let Clock::Rising = mc.clk {
    if let Signal::Active = mp.ctrl.data_il {
      mp.il = mc.data;
    }
  }
  if let Clock::Falling = mc.clk {
    if true {
      mp.sc = mp.sc.wrapping_add(1);
    }
  }
  if let Signal::Active = mp.ctrl.zero_sc {
    mp.sc = 0x00; // asynchronous
  }
  if let Signal::Active = mp.ctrl.size_data {
    mc.data = mp.size;
  }
  if let Signal::Active = mp.ctrl.ofst_data {
    mc.data = mp.ofst;
  }
  if let Reset::Asserted = mc.rst {
    mp.il = 0x00;
    mp.sc = 0x00;
  }
  mp.imm = mp.il & 0b01111111; // decode_imm
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
    if let Signal::Active = mp.ctrl.cout_cf {
      if let Signal::Active = mp.ctrl.sum_data {
        mp.cf = mp.sum == 0x00; // TODO
      }
      if let Signal::Active = mp.ctrl.nand_data {
        mp.cf = mp.nand == 0x00;
      }
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
  }
  if let Signal::Active = mc.wrt {
    mc.mem[mc.addr as usize] = mc.data; // asynchronous
  }
  if let Signal::Active = mc.read {
    mc.data = mc.mem[mc.addr as usize];
  }
  if let Reset::Asserted = mc.rst {
    mp.al = 0x00;
  }

  // X latch and Y latch and Z latch
  mp.sum = mp
    .xl
    .wrapping_add(mp.yl)
    .wrapping_add(match mp.ctrl.cin_sum {
      Signal::Active => 0x01,
      Signal::Inactive => 0x00,
    });
  mp.nand = !(mp.yl & mp.zl);
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
    Clock::Rising => 0x01,
    _ => 0x00,
  })
}

fn build_microcode_image() -> MicrocodeImage {
  // sets specified fields to `true` and wraps to ensure compatibility with `seq!`
  macro_rules! ControlWord {
    ($($field:ident),*) => {
      vec![Ok::<ControlWord, TickTrap>(ControlWord {
        $($field: Signal::Active,)*
        ..ControlWord::default()
      })]
    };
  }

  // automatically concatenates `Vec`s of control words
  macro_rules! seq {
    ($($control_word:expr),*) => {
      vec![$($control_word.clone(),)*].concat()
    };
  }

  // TODO order
  let sp_xl = ControlWord! {sp_data, data_xl};
  let ofst_yl = ControlWord! {ofst_data, data_yl};
  let size_yl = ControlWord! {size_data, data_yl};
  let ip_alxl = ControlWord! {ip_data, data_al, data_xl};
  let mem_il = ControlWord! {mem_data, data_il};
  let nand_ylzl = ControlWord! {nand_data, data_yl, data_zl};
  let sum_spal = ControlWord! {sum_data, data_sp, data_al};
  let cinsum_ip = ControlWord! {cin_sum, sum_data, data_ip};
  let nand_mem = ControlWord! {nand_data, data_mem};
  let mem_zl = ControlWord! {mem_data, data_zl};
  let mem_xl = ControlWord! {mem_data, data_xl};
  let sum_mem = ControlWord! {sum_data, data_mem};
  let ones_yl = ControlWord! {data_yl};
  let sp_al = ControlWord! {sp_data, data_al};
  let sp_alxl = ControlWord! {sp_data, data_al, data_xl};
  let sum_al = ControlWord! {sum_data, data_al};
  let nand_xl = ControlWord! {nand_data, data_xl};
  let mem_ylzl = ControlWord! {mem_data, data_yl, data_zl};
  let nand_zl = ControlWord! {nand_data, data_zl};
  let cinsum_yl = ControlWord! {cin_sum, sum_data, data_yl};
  let cinsum_mem = ControlWord! {cin_sum, sum_data, data_mem};
  let ones_ylzl = ControlWord! {data_yl, data_zl};
  let sp_xlzl = ControlWord! {sp_data, data_xl, data_zl};
  let ones_zl = ControlWord! {data_zl};
  let nand_yl = ControlWord! {nand_data, data_yl};
  let mem_ip = ControlWord! {mem_data, data_ip};
  let cinsum_sp = ControlWord! {cin_sum, sum_data, data_sp};
  let nand_memcf = ControlWord! {nand_data, data_mem, cout_cf};
  let cinsum_spal = ControlWord! {cin_sum, sum_data, data_sp, data_al};

  // assumes YL = 0x00
  let fetch = seq![ip_alxl, mem_il, cinsum_ip];
  let zero_yl = seq![ones_ylzl, nand_yl];
  let zero_sc = seq![ControlWord! {zero_sc}];

  [[[(); MAX_STEPS]; MEM_SIZE]; 2]
    .iter()
    .enumerate()
    .map(|(carry, rest)| (carry != 0, rest))
    .map(|(carry, rest)| {
      rest
        .iter()
        .enumerate()
        .map(|(instruction, rest)| (instruction as u8, rest))
        .map(|(instruction, _)| match (instruction & 0b10000000) >> 7 {
          0b0 => {
            // psh
            seq![
              fetch,  //
              mem_zl, // save IL
              sp_xl, ones_yl, sum_spal, // SP-- -> AL
              nand_zl, nand_mem, // IL -> *AL
              zero_yl, zero_sc //
            ]
          }
          0b1 => match (instruction & 0b01000000) >> 6 {
            0b0 => {
              // (arithmetic and logic)
              let opcode = (instruction & 0b00111100) >> 2;

              match opcode {
                0x0 => {
                  // add
                  // TODO carry
                  seq![
                    fetch, //
                    sp_alxl, cinsum_sp, // SP++
                    mem_zl,    // *SP -> ZL
                    size_yl, sum_al, // SP + S -> AL
                    ones_yl, nand_zl, nand_yl, // ZL -> YL
                    mem_xl,  // *AL -> XL
                    sum_mem, // XL SUM YL -> *AL
                    zero_yl, zero_sc //
                  ]
                }

                0x1 => {
                  // sub
                  seq![fetch, vec![Err(TickTrap::Todo)]]
                }

                0x4 => {
                  // iff
                  match carry {
                    true => seq![
                      fetch, //
                      sp_alxl,
                      mem_zl,      // *SP -> ZL
                      cinsum_spal, // SP++
                      ones_yl,
                      nand_zl,
                      nand_mem, // ZL -> *AL
                      zero_yl,
                      zero_sc //
                    ],
                    false => seq![
                      fetch, //
                      sp_alxl, cinsum_sp, // SP++
                      zero_sc    //
                    ],
                  }
                }

                0x5 => {
                  // rot
                  seq![fetch, vec![Err(TickTrap::Todo)]]
                }

                0x8 => {
                  // orr
                  seq![
                    fetch, //
                    sp_alxl, cinsum_sp, // SP++
                    mem_zl,    // *SP -> ZL
                    size_yl, sum_al, // SP + S -> AL
                    ones_yl, nand_xl, // ~ZL -> XL
                    mem_zl, nand_zl,   // ~*AL -> ZL
                    cinsum_yl, // XL -> YL
                    nand_mem,  // YL NAND ZL -> *AL
                    zero_yl, zero_sc //
                  ]
                }

                0x9 => {
                  // and
                  seq![fetch, vec![Err(TickTrap::Todo)]]
                }

                0xA => {
                  // xor
                  seq![
                    fetch, //
                    sp_alxl, cinsum_sp, // SP++
                    mem_zl,    // *SP -> ZL
                    size_yl, sum_al, // SP + S -> AL
                    ones_yl, nand_zl, nand_xl, // ZL -> XL
                    mem_zl, nand_zl,   // ~*AL -> ZL
                    cinsum_yl, // XL -> YL
                    nand_xl,   // YL NAND ZL -> XL
                    // ---
                    ones_zl, nand_yl, // ~YL -> YL
                    mem_zl,  // *AL -> ZL
                    nand_zl, // YL NAND ZL -> ZL
                    ones_yl, cinsum_yl,  // XL -> YL
                    nand_memcf, // ~(YL NAND ZL) -> *AL
                    zero_yl, zero_sc //
                  ]
                }

                0xB => {
                  // xnd
                  seq![fetch, vec![Err(TickTrap::Todo)]]
                }

                _ => match (opcode, instruction & 0b00000011) {
                  // (size used as part of opcode)
                  (0xC, 0b00) => {
                    // inc
                    seq![
                      fetch, //
                      sp_al, mem_xl,     // *SP -> XL
                      cinsum_mem, // XL + 1 -> *SP
                      zero_sc     //
                    ]
                  }

                  (0xC, 0b01) => {
                    // dec
                    seq![
                      fetch, //
                      sp_al, mem_xl, // *SP -> XL
                      ones_ylzl, sum_mem, // XL + 0xFF -> *SP
                      nand_yl, zero_sc //
                    ]
                  }

                  (0xC, 0b10) => {
                    // neg
                    seq![fetch, vec![Err(TickTrap::Todo)]]
                  }

                  (0xD, 0b00) => {
                    // shl
                    seq![fetch, vec![Err(TickTrap::Todo)]]
                  }

                  (0xD, 0b01) => {
                    // shr
                    seq![fetch, vec![Err(TickTrap::Todo)]]
                  }

                  (0xD, 0b10) => {
                    // not
                    seq![
                      fetch, //
                      sp_al, mem_ylzl, nand_mem, // ~*SP -> *SP
                      zero_yl, zero_sc //
                    ]
                  }

                  (0xD, 0b11) => {
                    // buf
                    seq![
                      fetch, //
                      sp_al, mem_ylzl, nand_ylzl, // *SP -> TODO carry
                      zero_yl, zero_sc //
                    ]
                  }

                  (0b1110, 0b11) => {
                    // dbg
                    seq![fetch, vec![Err(TickTrap::Todo)]]
                  }

                  _ => vec![Err(TickTrap::IllegalOpcode(instruction))],
                },
              }
            }

            0b1 => {
              match (instruction & 0b00100000) >> 5 {
                0b0 => {
                  // (offset operations)
                  match (instruction & 0b00010000) >> 4 {
                    0b0 => {
                      // ldo
                      seq![
                        fetch, //
                        sp_xl, ofst_yl, sum_al, // SP + O -> AL
                        mem_zl, // *AL -> ZL
                        sp_xl, ones_yl, sum_spal, // SP-- -> AL
                        nand_zl, nand_mem, // ZL -> *AL
                        zero_yl, zero_sc //
                      ]
                    }

                    0b1 => {
                      // sto
                      seq![fetch, vec![Err(TickTrap::Todo)]]
                    }

                    _ => unreachable!(),
                  }
                }

                0b1 => {
                  match (instruction & 0b00010000) >> 4 {
                    0b0 => {
                      // (carry and flags and stack)
                      match instruction & 0b00001111 {
                        0x0 => {
                          // lda
                          seq![fetch, vec![Err(TickTrap::Todo)]]
                        }

                        0x1 => {
                          // sta
                          seq![fetch, vec![Err(TickTrap::Todo)]]
                        }

                        0x2 => {
                          // ldi
                          seq![fetch, vec![Err(TickTrap::Todo)]]
                        }

                        0x3 => {
                          // sti
                          seq![
                            fetch, //
                            sp_alxl, cinsum_sp, // SP++
                            mem_ip,    // *SP -> IP
                            zero_sc    //
                          ]
                        }

                        0x4 => {
                          // lds
                          seq![
                            fetch, //
                            sp_xlzl, ones_yl, sum_spal, // SP -> ZL; SP-- -> AL
                            nand_zl, nand_mem, // ZL -> *AL
                            zero_yl, zero_sc //
                          ]
                        }

                        0x5 => {
                          // sts
                          seq![fetch, vec![Err(TickTrap::Todo)]]
                        }

                        0x8 => {
                          // nop
                          seq![
                            fetch,   //
                            zero_sc  //
                          ]
                        }

                        0x9 => {
                          // clc
                          seq![fetch, vec![Err(TickTrap::Todo)]]
                        }

                        0xA => {
                          // sec
                          seq![fetch, vec![Err(TickTrap::Todo)]]
                        }

                        0xB => {
                          // flc
                          seq![fetch, vec![Err(TickTrap::Todo)]]
                        }

                        0xC => {
                          // swp
                          seq![fetch, vec![Err(TickTrap::Todo)]]
                        }

                        0xD => {
                          // pop
                          seq![
                            fetch, //
                            sp_xl, cinsum_sp, // SP++
                            zero_sc    //
                          ]
                        }

                        _ => vec![Err(TickTrap::IllegalOpcode(instruction))],
                      }
                    }

                    0b1 => {
                      // phn
                      seq![fetch, vec![Err(TickTrap::Todo)]]
                    }

                    _ => unreachable!(),
                  }
                }

                _ => unreachable!(),
              }
            }

            _ => unreachable!(),
          },

          _ => unreachable!(),
        })
        .map(|control_sequence| {
          control_sequence
            .iter()
            .chain(std::iter::repeat(&Err(TickTrap::StepOverflow)))
            .take(MAX_STEPS)
            .copied()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
    })
    .collect::<Vec<_>>()
    .try_into()
    .unwrap()
}

impl std::fmt::Display for Microcomputer {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "MC:")?;
    writeln!(f, "  MEM:")?;
    let mut fmt: String = "".to_string();
    for y in 0..0x10 {
      fmt += "    ";
      for x in 0..0x10 {
        let address: u8 = (y << 0x04 | x) as u8;
        fmt += &format!(
          "{:02X}{}",
          self.mem[address as usize],
          if address == self.mp.sp.wrapping_sub(1) {
            if self.mp.cf {
              "/"
            } else {
              "|"
            }
          } else if address == self.mp.ip.wrapping_sub(1) {
            "["
          } else if address == self.mp.ip {
            "]"
          } else {
            " "
          }
        );
      }
      fmt += "\r\n";
    }
    write!(f, "{}", fmt)?;
    writeln!(f, "  CLK: {}", self.clk)?;
    writeln!(f, "  RST: {}", self.rst)?;
    writeln!(f, "  ADDR: {:02X}", self.addr)?;
    writeln!(f, "  DATA: {:02X}", self.data)?;
    writeln!(f, "  READ: {}", self.read)?;
    writeln!(f, "  WRT: {}", self.wrt)?;
    writeln!(f, "  MP:")?;
    writeln!(f, "    ROM:")?;
    writeln!(f, "      [...]")?;
    writeln!(f, "    IP: {:02X}", self.mp.ip)?;
    writeln!(f, "    SP: {:02X}", self.mp.sp)?;
    writeln!(f, "    CF: {:01X}", self.mp.cf as u8)?;
    writeln!(f, "    IL: {:02X}", self.mp.il)?;
    writeln!(f, "    SC: {:02X}", self.mp.sc)?;
    writeln!(f, "    AL: {:02X}", self.mp.al)?;
    writeln!(f, "    XL: {:02X}", self.mp.xl)?;
    writeln!(f, "    YL: {:02X}", self.mp.yl)?;
    writeln!(f, "    ZL: {:02X}", self.mp.zl)?;
    writeln!(f, "    CTRL:")?;
    writeln!(f, "      CIN_SUM: {}", self.mp.ctrl.cin_sum)?;
    writeln!(f, "      COUT_CF: {}", self.mp.ctrl.cout_cf)?;
    writeln!(f, "      DATA_IL: {}", self.mp.ctrl.data_il)?;
    writeln!(f, "      ZERO_SC: {}", self.mp.ctrl.zero_sc)?;
    writeln!(f, "      SIZE_DATA: {}", self.mp.ctrl.size_data)?;
    writeln!(f, "      OFST_DATA: {}", self.mp.ctrl.ofst_data)?;
    writeln!(f, "      IP_DATA: {}", self.mp.ctrl.ip_data)?;
    writeln!(f, "      DATA_IP: {}", self.mp.ctrl.data_ip)?;
    writeln!(f, "      SP_DATA: {}", self.mp.ctrl.sp_data)?;
    writeln!(f, "      DATA_SP: {}", self.mp.ctrl.data_sp)?;
    writeln!(f, "      DATA_AL: {}", self.mp.ctrl.data_al)?;
    writeln!(f, "      MEM_DATA: {}", self.mp.ctrl.mem_data)?;
    writeln!(f, "      DATA_MEM: {}", self.mp.ctrl.data_mem)?;
    writeln!(f, "      DATA_XL: {}", self.mp.ctrl.data_xl)?;
    writeln!(f, "      DATA_YL: {}", self.mp.ctrl.data_yl)?;
    writeln!(f, "      DATA_ZL: {}", self.mp.ctrl.data_zl)?;
    writeln!(f, "      SUM_DATA: {}", self.mp.ctrl.sum_data)?;
    writeln!(f, "      NAND_DATA: {}", self.mp.ctrl.nand_data)?;
    writeln!(f, "    IMM: {:02X}", self.mp.imm)?;
    writeln!(f, "    SIZE: {:02X}", self.mp.size)?;
    writeln!(f, "    OFST: {:02X}", self.mp.ofst)?;
    writeln!(f, "    SUM: {:02X}", self.mp.sum)?;
    writeln!(f, "    NAND: {:02X}", self.mp.nand)?;
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

impl std::fmt::Display for Reset {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Reset::Asserted => write!(f, "Asserted"),
      Reset::Deasserted => write!(f, "Deasserted"),
    }
  }
}

impl std::fmt::Display for Signal {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Signal::Active => write!(f, "Active"),
      Signal::Inactive => write!(f, "Inactive"),
    }
  }
}
