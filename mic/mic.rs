fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 2 {
    println!("Emu: Usage: mic <microcode image file>");
    std::process::exit(1);
  }

  let microcode_image_file: &String = &args[1];

  let microcode_image = compile_microcode();

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

  println!("Mic: Done");
}

const MEM_SIZE: usize = 0x100;
const MIC_SIZE: usize = 2 * 0x20 * MEM_SIZE;

// TODO copied from `emu.rs`
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
enum TickTrap {
  MicrocodeFault,
  DebugRequest,
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

fn compile_microcode() -> [u16; MIC_SIZE] {
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

  // TODO assumes YL = 0x00
  let fetch = seq![ip_alxl, mem_ilzl, cinsum_ip];
  let zero_yl = seq![ones_ylzl, nand_yl];
  let zero_sc = seq![ControlWord! {zero_sc}];
  let psh = seq![
    fetch, //
    // instruction is in ZL
    sp_xl, ones_yl, sum_spal, // SP-- -> AL
    nand_zl, nand_mem, // IL -> *AL
    zero_yl, zero_sc //
  ];
  let pop = seq![
    fetch, //
    sp_xl, cinsum_sp, // SP++
    zero_sc    //
  ];
  let sec = seq![
    fetch, //
    ones_ylzl, ones_ylzl, nand_ylcf, // 1 -> CF
    zero_sc    //
  ];
  let clc = seq![
    fetch, //
    ones_ylzl, nand_ylzl, nand_zlcf, // 0 -> CF
    zero_sc    //
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
              match (instruction & 0b10000000) >> 7 {
                0b0 => {
                  // psh
                  seq![psh]
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
                          nand_yl,
                          zero_sc //
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
                          },
                          zero_sc //
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
                          zero_yl,
                          zero_sc //
                        ]
                      }

                      0x5 => {
                        // rot
                        // TODO
                        seq![fetch, vec![Err(TickTrap::MicrocodeFault)]]
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
                          zero_yl, zero_sc //
                        ]
                      }

                      0x9 => {
                        // and
                        seq![
                          fetch, //
                          sp_alxl, cinsum_sp, // SP++
                          mem_zl,    // *SP -> ZL
                          size_yl, sum_al, // SP + SIZE -> AL
                          ones_yl, nand_zl, nand_xl,   // ZL -> XL
                          mem_zl,    // *AL -> ZL
                          cinsum_yl, // XL -> YL
                          nand_ylzl, nand_memcf, // ~(YL NAND ZL) -> *AL
                          zero_yl, zero_sc //
                        ]
                      }

                      0xA => {
                        // xor
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
                          zero_yl, zero_sc //
                        ]
                      }

                      0xB => {
                        // xnd
                        // TODO
                        seq![fetch, vec![Err(TickTrap::MicrocodeFault)]]
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
                          seq![
                            fetch, //
                            ones_ylzl, nand_xl, // 0x00 -> XL
                            sp_al, mem_ylzl, nand_ylzl, cinsum_mem, // 0x00 - *SP -> *SP
                            zero_yl, zero_sc //
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
                            nand_yl,
                            zero_sc //
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
                            nand_yl,
                            zero_sc //
                          ]
                        }

                        (0xD, 0b10) => {
                          // not
                          seq![
                            fetch, //
                            sp_al, mem_ylzl, nand_memcf, // ~*SP -> *SP
                            zero_yl, zero_sc //
                          ]
                        }

                        (0xD, 0b11) => {
                          // buf
                          seq![
                            fetch, //
                            sp_al, mem_ylzl, nand_ylzl, nand_memcf, // *SP -> *SP
                            zero_yl, zero_sc //
                          ]
                        }

                        (0b1110, 0b11) => {
                          // dbg
                          seq![fetch, vec![Err(TickTrap::DebugRequest)], zero_sc]
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
                              zero_yl, zero_sc //
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
                              zero_yl,
                              zero_sc //
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
                                  sp_al, sum_mem, // XL -> *SP
                                  zero_sc  //
                                ]
                              }

                              0x1 => {
                                // sta
                                seq![
                                  fetch, //
                                  sp_alxl, cinsum_sp, mem_zl, // *SP++ -> ZL
                                  sp_alxl, cinsum_sp, mem_xl, // *SP++ -> XL
                                  ones_yl, nand_zl, nand_al, // ZL -> AL
                                  zero_yl, sum_mem, // ZL -> *AL
                                  zero_sc  //
                                ]
                              }

                              0x2 => {
                                // ldi
                                // TODO
                                seq![fetch, vec![Err(TickTrap::MicrocodeFault)]]
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
                                seq![
                                  fetch, //
                                  sp_al, mem_sp,  // *SP -> SP
                                  zero_sc  //
                                ]
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
                                seq![clc]
                              }

                              0xA => {
                                // sec
                                seq![sec]
                              }

                              0xB => {
                                // flc
                                match carry {
                                  true => seq![clc],
                                  false => seq![sec],
                                }
                              }

                              0xC => {
                                // swp
                                seq![
                                  fetch, //
                                  sp_al, mem_zl, // *SP -> ZL
                                  sp_xl, cinsum_al, mem_xl, // *(SP + 1) -> *XL
                                  sp_al, sum_mem, // XL -> *SP
                                  sp_xl, cinsum_al, ones_yl, nand_zl,
                                  nand_mem, // ZL -> *(SP + 1)
                                  zero_yl, zero_sc //
                                ]
                              }

                              0xD => {
                                // pop
                                seq![pop]
                              }

                              _ => seq![fetch, vec![Err(TickTrap::IllegalOpcode(instruction))]],
                            }
                          }

                          0b1 => {
                            // phn
                            seq![psh]
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
              }
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
    .map(|control_word| control_word.unwrap_or_default().into())
    .collect::<Vec<_>>()
    .try_into()
    .unwrap();

  microcode_image
}

impl From<ControlWord> for u16 {
  fn from(control_word: ControlWord) -> Self {
    let control_word = unsafe {
      std::mem::transmute::<ControlWord, [u8; std::mem::size_of::<ControlWord>()]>(control_word)
    };

    control_word
      .iter()
      .fold(0, |acc, &byte| (acc << 1) | byte as u16)
  }
}
