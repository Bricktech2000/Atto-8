#[path = "../misc/common/common.rs"]
mod common;
use common::{ControlWord, Signal, TickTrap};

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 2 {
    println!("Mic: Usage: mic <microcode image file>");
    std::process::exit(1);
  }

  let mut errors: Vec<Error> = vec![];
  let microcode_image_file: &String = &args[1];

  let microcode_image = build_microcode(&mut errors);

  match errors[..] {
    [] => {
      std::fs::write::<&String, [u8; 2 * common::MIC_SIZE]>(
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

struct Error(String);

fn build_microcode(errors: &mut Vec<Error>) -> [u16; common::MIC_SIZE] {
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

  // convenience function for printing instruction clocks
  #[allow(dead_code)]
  fn print_len<T>(seq: Vec<T>) -> Vec<T> {
    println!("{}", seq.len());
    seq
  }

  let sp_xl = ControlWord! {sp_data, data_xl};
  let ofst_yl = ControlWord! {ofst_and_cf, data_yl};
  let size_yl = ControlWord! {size_and_cin, data_yl};
  let ip_alxl = ControlWord! {ip_data, data_al, data_xl};
  let mem_ilzl = ControlWord! {mem_data, data_il, data_zl};
  let nand_ylzl = ControlWord! {nand_data, data_yl, data_zl};
  let sum_spal = ControlWord! {sum_data, data_sp, data_al};
  let cinsum_ip = ControlWord! {size_and_cin, sum_data, data_ip};
  let nand_mem = ControlWord! {nand_data, data_mem};
  let mem_zl = ControlWord! {mem_data, data_zl};
  let mem_xl = ControlWord! {mem_data, data_xl};
  let sum_mem = ControlWord! {sum_data, data_mem};
  let set_yl = ControlWord! {data_yl};
  let sp_al = ControlWord! {sp_data, data_al};
  let sp_alxl = ControlWord! {sp_data, data_al, data_xl};
  let sum_al = ControlWord! {sum_data, data_al};
  let nand_xl = ControlWord! {nand_data, data_xl};
  let mem_ylzl = ControlWord! {mem_data, data_yl, data_zl};
  let nand_zl = ControlWord! {nand_data, data_zl};
  let cinsum_yl = ControlWord! {size_and_cin, sum_data, data_yl};
  let cinsum_mem = ControlWord! {size_and_cin, sum_data, data_mem};
  let set_ylzl = ControlWord! {data_yl, data_zl};
  let sp_xlzl = ControlWord! {sp_data, data_xl, data_zl};
  let set_zl = ControlWord! {data_zl};
  let nand_yl = ControlWord! {nand_data, data_yl};
  let mem_ip = ControlWord! {mem_data, data_ip};
  let cinsum_sp = ControlWord! {size_and_cin, sum_data, data_sp};
  let nand_memcf = ControlWord! {nand_data, data_mem, ofst_and_cf};
  let mem_sp = ControlWord! {mem_data, data_sp};
  let nand_ylcf = ControlWord! {nand_data, data_yl, ofst_and_cf};
  let mem_xlyl = ControlWord! {mem_data, data_xl, data_yl};
  let nand_al = ControlWord! {nand_data, data_al};
  let cinsum_spxl = ControlWord! {size_and_cin, sum_data, data_sp, data_xl};
  let cinsum_al = ControlWord! {size_and_cin, sum_data, data_al};
  let nand_zlcf = ControlWord! {nand_data, data_zl, ofst_and_cf};
  let cinsum_xlcf = ControlWord! {size_and_cin, sum_data, data_xl, ofst_and_cf};
  let sum_xlcf = ControlWord! {sum_data, data_xl, ofst_and_cf};
  let cinsum_xlylcf = ControlWord! {size_and_cin, sum_data, data_xl, data_yl, ofst_and_cf};
  let sum_xlylcf = ControlWord! {sum_data, data_xl, data_yl, ofst_and_cf};
  let mem_yl = ControlWord! {mem_data, data_yl};
  let cinsum_zl = ControlWord! {size_and_cin, sum_data, data_zl};
  let sum_memcf = ControlWord! {sum_data, data_mem, ofst_and_cf};
  let sum_zl = ControlWord! {sum_data, data_zl};
  let cinsum_spal = ControlWord! {size_and_cin, sum_data, data_sp, data_al};
  let yl_mem = ControlWord! {data_yl, data_mem};
  let set_xlylzl = ControlWord! {data_xl, data_yl, data_zl};
  let ip_mem = ControlWord! {ip_data, data_mem};
  let clr_sc = ControlWord! {clr_sc};

  let noop = seq![ControlWord! {}];
  let fetch = seq![ip_alxl, cinsum_ip, mem_ilzl];
  let clr_yl = seq![set_ylzl, nand_yl];
  let set_cf = seq![set_ylzl, set_ylzl, nand_ylcf];
  let clr_cf = seq![set_ylzl, nand_ylzl, nand_zlcf];

  let microcode: [[[Result<ControlWord, TickTrap>; 0x20]; 0x02]; 0x80] = [[[(); 0x20]; 0x02]; 0x80]
    .iter()
    .enumerate()
    .map(|(instruction, rest)| (instruction as u8 | 0x80, rest)) // map `psh` to `phn` as both have equivalent microcode
    .map(|(instruction, rest)| {
      rest
        .iter()
        .enumerate()
        .map(|(carry, rest)| (carry != 0, rest))
        .map(|(carry, rest)| {
          rest
            .iter()
            .enumerate()
            .map(|(step, rest)| (step as usize, rest))
            .map(|(step, _rest)| {
              let seq = match (instruction & 0b10000000) >> 7 {
                0b0 => {
                  // psh
                  unreachable!()
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
                          set_yl,
                          nand_zl,
                          nand_yl, // ZL -> YL
                          mem_xl,  // *AL -> XL
                          match carry {
                            true => seq![cinsum_xlcf], // XL + YL -> XL
                            false => seq![sum_xlcf],   // XL + YL -> XL
                          },
                          set_ylzl,
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
                          set_yl,
                          nand_yl, // ~ZL -> YL
                          mem_xl,  // *AL -> XL
                          match carry {
                            true => seq![sum_xlcf],     // XL - YL -> XL
                            false => seq![cinsum_xlcf], // XL - YL -> XL
                          },
                          set_ylzl,
                          cinsum_mem, // XL -> *AL
                          match carry {
                            true => seq![nand_ylzl, nand_zlcf], // 0 -> CF
                            false => seq![noop, nand_ylcf],     // 1 -> CF
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
                          set_yl,
                          match carry {
                            true => seq![nand_zl, nand_xl], // ZL -> xL
                            false => seq![noop, noop],      // no-op
                          },
                          cinsum_mem, // XL -> *AL
                          clr_yl
                        ]
                      }

                      0x5 => {
                        // rot
                        seq![
                          match carry {
                            true => seq![set_yl, nand_zl, nand_mem],
                            false => seq![fetch],
                          }, // continuation of match below
                          sp_xl,
                          size_yl,
                          sum_al, // SP + SIZE -> AL
                          mem_xlyl,
                          sum_xlcf, // *AL + *AL -> XL
                          clr_yl,
                          match carry {
                            true => seq![cinsum_zl], // XL + 1 -> ZL
                            false => seq![sum_zl],   // XL -> ZL
                          },
                          sp_al,
                          mem_xl,
                          set_yl,
                          sum_memcf, // *SP - 1 -> *SP
                          match carry {
                            // not done. store shifted value
                            true => seq![sp_xl, size_yl, sum_al, noop], // ZL -> *(SP + SIZE)
                            // done. ignore shifted value, pop counter, fetch next instruction
                            false => seq![clr_yl, sp_xl, cinsum_sp], // SP++
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
                          set_yl, nand_xl, // ~ZL -> XL
                          mem_zl, nand_zl,    // ~*AL -> ZL
                          cinsum_yl,  // XL -> YL
                          nand_memcf, // YL NAND ZL -> *AL
                          clr_yl      //
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
                          clr_yl      //
                        ]
                      }

                      0xA => {
                        // xor
                        seq![
                          fetch, //
                          sp_alxl, cinsum_sp, // SP++
                          mem_zl,    // *SP -> ZL
                          size_yl, sum_al, // SP + SIZE -> AL
                          set_yl, nand_zl, nand_xl, // ZL -> XL
                          mem_zl, nand_zl,   // ~*AL -> ZL
                          cinsum_yl, // XL -> YL
                          nand_xl,   // YL NAND ZL -> XL
                          // ---
                          set_zl, nand_yl, // ~YL -> YL
                          mem_zl,  // *AL -> ZL
                          nand_zl, // YL NAND ZL -> ZL
                          set_yl, cinsum_yl,  // XL -> YL
                          nand_memcf, // ~(YL NAND ZL) -> *AL
                          clr_yl      //
                        ]
                      }

                      0xB => {
                        // xnd
                        seq![
                          fetch, //
                          sp_xl,
                          cinsum_spal, // SP++
                          yl_mem,      // 0x00 -> SP
                          set_xlylzl,  // 0xFF -> XL; 0xFF -> YL; 0xFF -> ZL
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
                            set_ylzl, sum_mem, // XL + 0xFF -> *SP
                            nand_yl  //
                          ]
                        }

                        (0xC, 0b10) => {
                          // neg
                          seq![
                            fetch, //
                            set_ylzl, nand_xl, // 0x00 -> XL
                            sp_al, mem_ylzl, nand_ylzl, cinsum_mem, // 0x00 - *SP -> *SP
                            clr_yl      //
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
                            set_ylzl,
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
                            set_ylzl,
                            cinsum_mem, // XL -> *SP
                            nand_yl
                          ]
                        }

                        (0xD, 0b10) => {
                          // not
                          seq![
                            fetch, //
                            sp_al, mem_ylzl, nand_memcf, // ~*SP -> *SP
                            clr_yl      //
                          ]
                        }

                        (0xD, 0b11) => {
                          // buf
                          seq![
                            fetch, //
                            sp_al, mem_ylzl, nand_ylzl, nand_memcf, // *SP -> *SP
                            clr_yl      //
                          ]
                        }

                        (0b1110, 0b11) => {
                          // dbg
                          seq![fetch, vec![Err(TickTrap::DebugRequest)]]
                        }

                        _ => seq![fetch, vec![Err(TickTrap::IllegalOpcode)]],
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
                              sp_xl, set_yl, sum_spal, // SP-- -> AL
                              nand_zl, nand_mem, // ZL -> *AL
                              clr_yl    //
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
                              set_yl,
                              nand_zl,
                              nand_mem, // ZL -> *AL
                              clr_yl
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
                                  set_yl, nand_zl, nand_al, // ZL -> AL
                                  clr_yl, sum_mem // ZL -> *AL
                                ]
                              }

                              0x2 => {
                                // ldi
                                seq![
                                  fetch, //
                                  sp_xl, set_yl, sum_spal, // SP-- -> AL
                                  ip_mem,   // IP -> *AL
                                  clr_yl    //
                                ]
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
                                  sp_xlzl, set_yl, sum_spal, // SP -> ZL; SP-- -> AL
                                  nand_zl, nand_mem, // ZL -> *AL
                                  clr_yl    //
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
                                seq![fetch, clr_cf]
                              }

                              0xA => {
                                // sec
                                seq![fetch, set_cf]
                              }

                              0xB => {
                                // flc
                                match carry {
                                  true => seq![fetch, clr_cf],
                                  false => seq![fetch, set_cf],
                                }
                              }

                              0xC => {
                                // swp
                                seq![
                                  fetch, //
                                  sp_al, mem_zl, // *SP -> ZL
                                  sp_xl, cinsum_al, mem_xl, // *(SP + 1) -> *XL
                                  set_yl, nand_zl, nand_mem, // ZL -> *(SP + 1)
                                  sp_al, cinsum_mem, // XL -> *SP
                                  clr_yl      //
                                ]
                              }

                              0xD => {
                                // pop
                                seq![
                                  fetch,
                                  sp_xl, cinsum_sp // SP++
                                ]
                              }

                              _ => vec![Err(TickTrap::IllegalOpcode)],
                            }
                          }

                          0b1 => {
                            // phn
                            seq![
                              fetch, // instruction is in ZL
                              sp_xl, set_yl, sum_spal, // SP-- -> AL
                              nand_zl, nand_mem, // IL -> *AL
                              clr_yl    //
                            ]
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
              let seq = seq![seq, clr_sc];
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

  let microcode_image: [u16; common::MIC_SIZE] = microcode
    .concat()
    .concat()
    .into_iter()
    .map(common::result_into_u16)
    .collect::<Vec<_>>()
    .try_into()
    .unwrap();

  microcode_image
}
