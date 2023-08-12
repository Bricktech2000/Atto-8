fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 2 {
    println!("Emu: Usage: mic <microcode image file>");
    std::process::exit(1);
  }

  let mut errors: Vec<Error> = vec![];
  let microcode_image_file: &String = &args[1];

  let microcode_image = compile_microcode(&mut errors);

  match errors[..] {
    [] => {
      std::fs::write::<&String, [u8; 2 * MIC_SIZE]>(
        microcode_image_file,
        microcode_image
          .iter()
          .flat_map(|&word| word.to_le_bytes().to_vec())
          .collect::<Vec<u8>>()
          .try_into()
          .unwrap(),
      )
      .unwrap();
    }

    _ => {
      let errors = errors
        .iter()
        .map(|error| format!("Mic: Error: {}", error.0))
        .collect::<Vec<String>>()
        .join("\n");

      println!("{}", errors);
      std::process::exit(1);
    }
  }

  println!("Mic: Done");
}

const MEM_SIZE: usize = 0x100;
const MIC_SIZE: usize = 2 * 0x20 * MEM_SIZE;
const MICROCODE_FAULT_TOMBSTONE: u16 = 0xFFF0;
const DEBUG_REQUEST_TOMBSTONE: u16 = 0xFFF1;
const ILLEGAL_OPCODE_TOMBSTONE: u16 = 0xFFF2;

// TODO copied from `emu.rs`
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
enum TickTrap {
  DebugRequest,
  MicrocodeFault,
  IllegalOpcode(u8),
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Default)]
struct ControlWord {
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

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Default)]
enum Signal {
  #[default]
  Inactive,
  Active,
}

struct Error(String);

fn compile_microcode(errors: &mut Vec<Error>) -> [u16; MIC_SIZE] {
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

  // TODO document why every instruction must end with YL = 0x00
  // TODO document sum_data && ofst_data is sum_data && sum_cf
  // TODO document nand_data && ofst_data is nand_data && nand_cf
  // TODO document sum_data && size_data is sum_data && cin_sum

  // TODO document `sim` expects memory around SP to behave normally, otherwise UB

  // TODO order
  let default = ControlWord! {};
  let sp_xl = ControlWord! {sp_data, data_xl};
  let ofst_yl = ControlWord! {ofst_data, data_yl};
  let size_yl = ControlWord! {size_data, data_yl};
  let ip_alxl = ControlWord! {ip_data, data_al, data_xl};
  let mem_ilzl = ControlWord! {mem_data, data_il, data_zl};
  let nand_ylzl = ControlWord! {nand_data, data_yl, data_zl};
  let sum_spal = ControlWord! {sum_data, data_sp, data_al};
  let cinsum_ip = ControlWord! {size_data, sum_data, data_ip};
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
  let cinsum_yl = ControlWord! {size_data, sum_data, data_yl};
  let cinsum_mem = ControlWord! {size_data, sum_data, data_mem};
  let ones_ylzl = ControlWord! {data_yl, data_zl};
  let sp_xlzl = ControlWord! {sp_data, data_xl, data_zl};
  let ones_zl = ControlWord! {data_zl};
  let nand_yl = ControlWord! {nand_data, data_yl};
  let mem_ip = ControlWord! {mem_data, data_ip};
  let cinsum_sp = ControlWord! {size_data, sum_data, data_sp};
  let nand_memcf = ControlWord! {nand_data, data_mem, ofst_data};
  let mem_sp = ControlWord! {mem_data, data_sp};
  let nand_ylcf = ControlWord! {nand_data, data_yl, ofst_data};
  let mem_xlyl = ControlWord! {mem_data, data_xl, data_yl};
  let nand_al = ControlWord! {nand_data, data_al};
  let cinsum_spxl = ControlWord! {size_data, sum_data, data_sp, data_xl};
  let cinsum_al = ControlWord! {size_data, sum_data, data_al};
  let nand_zlcf = ControlWord! {nand_data, data_zl, ofst_data};
  let cinsum_xlcf = ControlWord! {size_data, sum_data, data_xl, ofst_data};
  let sum_xlcf = ControlWord! {sum_data, data_xl, ofst_data};
  let cinsum_xlylcf = ControlWord! {size_data, sum_data, data_xl, data_yl, ofst_data};
  let sum_xlylcf = ControlWord! {sum_data, data_xl, data_yl, ofst_data};
  let mem_yl = ControlWord! {mem_data, data_yl};
  let cinsum_zl = ControlWord! {size_data, sum_data, data_zl};
  let sum_memcf = ControlWord! {sum_data, data_mem, ofst_data};
  let sum_zl = ControlWord! {sum_data, data_zl};
  let cinsum_spal = ControlWord! {size_data, sum_data, data_sp, data_al};
  let yl_mem = ControlWord! {data_yl, data_mem};
  let ones_xlylzl = ControlWord! {data_xl, data_yl, data_zl};

  // TODO assumes YL = 0x00
  let fetch = seq![ip_alxl, cinsum_ip, mem_ilzl];
  let zero_yl = seq![ones_ylzl, nand_yl];
  let zero_sc = seq![ControlWord! {zero_sc}];
  let psh = seq![
    // instruction is in ZL
    sp_xl, ones_yl, sum_spal, // SP-- -> AL
    nand_zl, nand_mem, // IL -> *AL
    zero_yl   //
  ];
  let pop = seq![
    sp_xl, cinsum_sp // SP++
  ];
  let sec = seq![
    ones_ylzl, ones_ylzl, nand_ylcf // 1 -> CF
  ];
  let clc = seq![
    ones_ylzl, nand_ylzl, nand_zlcf // 0 -> CF
  ];

  let microcode: [[[Result<ControlWord, TickTrap>; MEM_SIZE]; 0x20]; 2] = [[[(); MEM_SIZE]; 0x20];
    2]
    .iter()
    .enumerate()
    .map(|(carry, rest)| (carry != 0, rest))
    .map(|(carry, rest)| {
      rest
        .iter()
        .enumerate()
        .map(|(step, rest)| (step as usize, rest))
        .map(|(step, rest)| {
          rest
            .iter()
            .enumerate()
            .map(|(instruction, rest)| (instruction as u8, rest))
            .map(|(instruction, _)| {
              let seq = match (instruction & 0b10000000) >> 7 {
                0b0 => {
                  // psh
                  seq![fetch, psh]
                }
                0b1 => match (instruction & 0b01000000) >> 6 {
                  0b0 => {
                    // (arithmetic and logic)
                    let opcode = (instruction & 0b00111100) >> 2;

                    match opcode {
                      0x0 => {
                        // add
                        seq![
                          fetch, //
                          sp_alxl,
                          cinsum_sp, // SP++
                          mem_zl,    // *SP -> ZL
                          size_yl,
                          sum_al, // SP + SIZE -> AL
                          ones_yl,
                          nand_zl,
                          nand_yl, // ZL -> YL
                          mem_xl,  // *AL -> XL
                          match carry {
                            true => seq![cinsum_xlcf], // XL + YL -> XL
                            false => seq![sum_xlcf],   // XL + YL -> XL
                          },
                          ones_ylzl,
                          cinsum_mem, // XL -> *AL
                          nand_yl
                        ]
                      }

                      0x1 => {
                        // sub
                        seq![
                          fetch, //
                          sp_alxl,
                          cinsum_sp, // SP++
                          mem_zl,    // *SP -> ZL
                          size_yl,
                          sum_al, // SP + SIZE -> AL
                          ones_yl,
                          nand_yl, // ~ZL -> YL
                          mem_xl,  // *AL -> XL
                          match carry {
                            true => seq![sum_xlcf],     // XL - YL -> XL
                            false => seq![cinsum_xlcf], // XL - YL -> XL
                          },
                          ones_ylzl,
                          cinsum_mem, // XL -> *AL
                          match carry {
                            true => seq![nand_ylzl, nand_zlcf], // 0 -> CF
                            false => seq![default, nand_ylcf],  // 1 -> CF
                          }
                        ]
                      }

                      0x4 => {
                        // iff
                        seq![
                          fetch, //
                          sp_alxl,
                          cinsum_sp, // SP++
                          mem_zl,    // *SP -> ZL
                          size_yl,
                          sum_al, // SP + SIZE -> AL
                          mem_xl, // *AL -> XL
                          ones_yl,
                          match carry {
                            true => seq![nand_zl, nand_xl],  // ZL -> xL
                            false => seq![default, default], // no-op
                          },
                          cinsum_mem, // XL -> *AL
                          zero_yl
                        ]
                      }

                      0x5 => {
                        // rot
                        // TODO document that `rot` clears carry
                        seq![
                          match carry {
                            true => seq![ones_yl, nand_zl, nand_mem],
                            false => seq![fetch],
                          }, // continuation of match below
                          sp_xl,
                          size_yl,
                          sum_al, // SP + SIZE -> AL
                          mem_xlyl,
                          sum_xlcf, // *AL + *AL -> XL
                          zero_yl,
                          match carry {
                            true => seq![cinsum_zl], // XL + 1 -> ZL
                            false => seq![sum_zl],   // XL -> ZL
                          },
                          sp_al,
                          mem_xl,
                          ones_yl,
                          sum_memcf, // *SP - 1 -> *SP
                          match carry {
                            // not done. store shifted value
                            true => seq![sp_xl, size_yl, sum_al, default], // ZL -> *(SP + SIZE)
                            // done. ignore shifted value, pop counter, fetch next instruction
                            false => seq![zero_yl, sp_xl, cinsum_sp], // SP++
                          }
                        ]
                      }

                      0x8 => {
                        // orr
                        seq![
                          fetch, //
                          sp_alxl, cinsum_sp, // SP++
                          mem_zl,    // *SP -> ZL
                          size_yl, sum_al, // SP + SIZE -> AL
                          ones_yl, nand_xl, // ~ZL -> XL
                          mem_zl, nand_zl,    // ~*AL -> ZL
                          cinsum_yl,  // XL -> YL
                          nand_memcf, // YL NAND ZL -> *AL
                          zero_yl     //
                        ]
                      }

                      0x9 => {
                        // and
                        seq![
                          fetch, //
                          sp_alxl, cinsum_sp, // SP++
                          mem_zl,    // *SP -> ZL
                          size_yl, sum_al, // SP + SIZE -> AL
                          mem_yl, // *AL -> YL
                          nand_ylzl, nand_memcf, // ~(YL NAND ZL) -> *AL
                          zero_yl     //
                        ]
                      }

                      0xA => {
                        // xor
                        // seq![fetch, vec![Err(TickTrap::DebugRequest)]]

                        // seq![
                        //   ones_ylzl, nand_ylzl, default, //
                        //   // TODO get to fit in 0x10 steps
                        //   ip_alxl, mem_xl, sum_il, // fetch `and` instruction
                        //   sp_alxl, mem_zl, // *SP -> ZL
                        //   size_yl, cinsum_al, // (SP + 1) + SIZE -> AL
                        //   mem_ylxl,  // *AL -> YL; *AL -> XL
                        //   nand_mem,  // YL NAND ZL -> *AL
                        //   ones_yl, cinsum_zl, nand_xl, // ~XL -> XL
                        //   sp_al, mem_zl, nand_zl,   // ~*SP -> ZL
                        //   cinsum_yl, // XL -> YL
                        //   nand_mem,  // YL NAND ZL -> *AL
                        //   ones_yl    //
                        //
                        //              // ones_zl, nand_yl, // ~YL -> YL
                        //              // mem_zl,  // *AL -> ZL
                        //              // nand_zl, // YL NAND ZL -> ZL
                        //              // ones_yl, cinsum_yl,  // XL -> YL
                        //              // nand_memcf, // ~(YL NAND ZL) -> *AL
                        //              // zero_yl     //
                        // ]

                        seq![
                          fetch, //
                          sp_alxl, cinsum_sp, // SP++
                          mem_zl,    // *SP -> ZL
                          size_yl, sum_al, // SP + SIZE -> AL
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
                          zero_yl     //
                        ]
                      }

                      0xB => {
                        // xnd
                        seq![
                          fetch, //
                          sp_xl,
                          cinsum_spal, // SP++
                          yl_mem,      // 0x00 -> SP
                          ones_xlylzl, // 0xFF -> XL; 0xFF -> YL; 0xFF -> ZL
                          sum_xlcf,    // 1 -> CF
                          nand_yl
                        ]
                      }

                      _ => match (opcode, instruction & 0b00000011) {
                        // (size used as part of opcode)
                        (0xC, 0b00) => {
                          // inc
                          seq![
                            fetch, //
                            sp_al, mem_xl,     // *SP -> XL
                            cinsum_mem  // XL + 1 -> *SP
                          ]
                        }

                        (0xC, 0b01) => {
                          // dec
                          seq![
                            fetch, //
                            sp_al, mem_xl, // *SP -> XL
                            ones_ylzl, sum_mem, // XL + 0xFF -> *SP
                            nand_yl  //
                          ]
                        }

                        (0xC, 0b10) => {
                          // neg
                          seq![
                            fetch, //
                            ones_ylzl, nand_xl, // 0x00 -> XL
                            sp_al, mem_ylzl, nand_ylzl, cinsum_mem, // 0x00 - *SP -> *SP
                            zero_yl     //
                          ]
                        }

                        (0xD, 0b00) => {
                          // shl
                          seq![
                            fetch, //
                            sp_al,
                            mem_xlyl, // *SP -> XL; *SP -> YL
                            match carry {
                              true => seq![cinsum_xlcf], // XL + XL -> XL
                              false => seq![sum_xlcf],   // XL + XL -> XL
                            },
                            ones_ylzl,
                            cinsum_mem, // XL -> *SP
                            nand_yl
                          ]
                        }

                        (0xD, 0b01) => {
                          // shr
                          seq![
                            fetch, //
                            sp_al,
                            mem_xlyl, // *SP -> XL; *SP -> YL
                            match carry {
                              true => std::iter::repeat(seq![cinsum_xlylcf])
                                .take(8)
                                .flatten()
                                .collect::<Vec<_>>(), // XL + XL -> XL; XL + XL -> YL
                              false => std::iter::repeat(seq![sum_xlylcf])
                                .take(8)
                                .flatten()
                                .collect::<Vec<_>>(), // XL + XL -> XL; XL + XL -> YL
                            },
                            ones_ylzl,
                            cinsum_mem, // XL -> *SP
                            nand_yl
                          ]
                        }

                        (0xD, 0b10) => {
                          // not
                          seq![
                            fetch, //
                            sp_al, mem_ylzl, nand_memcf, // ~*SP -> *SP
                            zero_yl     //
                          ]
                        }

                        (0xD, 0b11) => {
                          // buf
                          seq![
                            fetch, //
                            sp_al, mem_ylzl, nand_ylzl, nand_memcf, // *SP -> *SP
                            zero_yl     //
                          ]
                        }

                        (0b1110, 0b11) => {
                          // dbg
                          seq![fetch, vec![Err(TickTrap::DebugRequest)]]
                        }

                        _ => seq![fetch, vec![Err(TickTrap::IllegalOpcode(instruction))]],
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
                              sp_xl, ofst_yl, sum_al, // SP + OFST -> AL
                              mem_zl, // *AL -> ZL
                              sp_xl, ones_yl, sum_spal, // SP-- -> AL
                              nand_zl, nand_mem, // ZL -> *AL
                              zero_yl   //
                            ]
                          }

                          0b1 => {
                            // sto
                            seq![
                              fetch, //
                              sp_alxl,
                              mem_zl, // *SP -> ZL
                              cinsum_spxl,
                              ofst_yl,
                              sum_al, // ++SP + OFST -> AL
                              ones_yl,
                              nand_zl,
                              nand_mem, // ZL -> *AL
                              zero_yl
                            ]
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
                                seq![
                                  fetch, //
                                  sp_al, mem_xl, // *SP -> XL
                                  sum_al, mem_xl, // *XL -> XL
                                  sp_al, sum_mem // XL -> *SP
                                ]
                              }

                              0x1 => {
                                // sta
                                seq![
                                  fetch, //
                                  sp_alxl, cinsum_sp, mem_zl, // *SP++ -> ZL
                                  sp_alxl, cinsum_sp, mem_xl, // *SP++ -> XL
                                  ones_yl, nand_zl, nand_al, // ZL -> AL
                                  zero_yl, sum_mem // ZL -> *AL
                                ]
                              }

                              0x2 => {
                                // ldi
                                // TODO
                                vec![Err(TickTrap::DebugRequest)]
                              }

                              0x3 => {
                                // sti
                                seq![
                                  fetch, //
                                  sp_alxl, cinsum_sp, // SP++
                                  mem_ip     // *SP -> IP
                                ]
                              }

                              0x4 => {
                                // lds
                                seq![
                                  fetch, //
                                  sp_xlzl, ones_yl, sum_spal, // SP -> ZL; SP-- -> AL
                                  nand_zl, nand_mem, // ZL -> *AL
                                  zero_yl   //
                                ]
                              }

                              0x5 => {
                                // sts
                                seq![
                                  fetch, //
                                  sp_al, mem_sp // *SP -> SP
                                ]
                              }

                              0x8 => {
                                // nop
                                seq![fetch]
                              }

                              0x9 => {
                                // clc
                                seq![fetch, clc]
                              }

                              0xA => {
                                // sec
                                seq![fetch, sec]
                              }

                              0xB => {
                                // flc
                                match carry {
                                  true => seq![fetch, clc],
                                  false => seq![fetch, sec],
                                }
                              }

                              0xC => {
                                // swp
                                seq![
                                  fetch, //
                                  sp_al, mem_zl, // *SP -> ZL
                                  sp_xl, cinsum_al, mem_xl, // *(SP + 1) -> *XL
                                  ones_yl, nand_zl, nand_mem, // ZL -> *(SP + 1)
                                  sp_al, cinsum_mem, // XL -> *SP
                                  zero_yl     //
                                ]
                              }

                              0xD => {
                                // pop
                                seq![fetch, pop]
                              }

                              _ => vec![Err(TickTrap::IllegalOpcode(instruction))],
                            }
                          }

                          0b1 => {
                            // phn
                            seq![fetch, psh]
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
              };
              let seq = seq![seq, zero_sc];
              let rest = seq.get(0x20..seq.len() - 1).unwrap_or(&[]);
              if step == 0x00 && rest.len() > 0 {
                errors.push(Error(format!(
                  "Microcode for instruction `{:02X}` with carry `{:01X}` overflows by {} steps",
                  instruction,
                  carry as u8,
                  rest.len()
                )));
              }
              seq
                .get(step)
                .copied()
                .unwrap_or(Err(TickTrap::MicrocodeFault))
            })
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
    .unwrap();

  let microcode_image: [u16; MIC_SIZE] = microcode
    .concat()
    .concat()
    .iter()
    .map(|result| {
      result
        .map(|control_word| control_word.into())
        .unwrap_or_else(|trap| trap.into())
    })
    .collect::<Vec<_>>()
    .try_into()
    .unwrap();

  microcode_image
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

impl Into<u16> for TickTrap {
  fn into(self) -> u16 {
    match self {
      TickTrap::DebugRequest => DEBUG_REQUEST_TOMBSTONE,
      TickTrap::MicrocodeFault => MICROCODE_FAULT_TOMBSTONE,
      TickTrap::IllegalOpcode(_) => ILLEGAL_OPCODE_TOMBSTONE,
    }
  }
}
