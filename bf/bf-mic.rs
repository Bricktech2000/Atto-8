#[path = "../misc/common/common.rs"]
mod common;
use common::*;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 2 {
    println!("BF-Mic: Usage: bf-mic <microcode image file>");
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
        .map(|error| format!("Mic: Error: {}", error))
        .collect::<Vec<String>>()
        .join("\n");

      println!("{}", errors);
      std::process::exit(1);
    }
  }

  println!("BF-Mic: Done");
}

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
    ($control_word:expr; $n:expr) => {
      vec![$control_word.clone(); $n].concat()
    };
  }

  // convenience function for printing instruction clocks
  #[allow(dead_code)]
  fn print_len<T>(seq: Vec<T>) -> Vec<T> {
    println!("{}", seq.len());
    seq
  }

  let sp_xl = ControlWord! {sp_data, data_xl};
  let ip_alxl = ControlWord! {ip_data, data_al, data_xl};
  let nand_ylzl = ControlWord! {nand_data, data_yl, data_zl};
  let sum_spal = ControlWord! {sum_data, data_sp, data_al};
  let cinsum_ip = ControlWord! {set_cin, sum_data, data_ip};
  let nand_mem = ControlWord! {nand_data, data_mem};
  let mem_xl = ControlWord! {mem_data, data_xl};
  let sum_mem = ControlWord! {sum_data, data_mem};
  let set_yl = ControlWord! {data_yl};
  let sp_al = ControlWord! {sp_data, data_al};
  let sp_alxl = ControlWord! {sp_data, data_al, data_xl};
  let mem_ylzl = ControlWord! {mem_data, data_yl, data_zl};
  let cinsum_mem = ControlWord! {set_cin, sum_data, data_mem};
  let set_ylzl = ControlWord! {data_yl, data_zl};
  let nand_yl = ControlWord! {nand_data, data_yl};
  let cinsum_sp = ControlWord! {set_cin, sum_data, data_sp};
  let nand_ylcf = ControlWord! {nand_data, data_yl, data_cf};
  let cinsum_al = ControlWord! {set_cin, sum_data, data_al};
  let nand_zlcf = ControlWord! {nand_data, data_zl, data_cf};
  let mem_yl = ControlWord! {mem_data, data_yl};
  let sum_xl = ControlWord! {sum_data, data_xl};
  let set_xlylzl = ControlWord! {data_xl, data_yl, data_zl};
  let clr_sc = ControlWord! {clr_sc};
  let nand_xlylcf = ControlWord! {nand_data, data_xl, data_yl, data_cf};
  let nand_xlylzl = ControlWord! {nand_data, data_xl, data_yl, data_zl};
  let nand_il = ControlWord! {nand_data, data_il};
  let sum_ip = ControlWord! {sum_data, data_ip};
  let nand_ylal = ControlWord! {nand_data, data_yl, data_al};
  let set_al = ControlWord! {data_al};
  let set_mem = ControlWord! {data_mem};
  let set_alylzl = ControlWord! {data_al, data_yl, data_zl};
  let nand_xlyl = ControlWord! {nand_data, data_xl, data_yl};
  let sum_memyl = ControlWord! {sum_data, data_mem, data_yl};
  let cinsum_memyl = ControlWord! {set_cin, sum_data, data_mem, data_yl};

  let noop = seq![ControlWord! {}];
  let clr_yl = seq![set_ylzl, nand_yl];
  let set_cf = seq![set_xlylzl, set_xlylzl, nand_xlylcf];
  let clr_cf = seq![set_xlylzl, nand_xlylzl, nand_zlcf];
  let nfetch = seq![ip_alxl, cinsum_ip, mem_ylzl, nand_il];
  let walk = seq![set_al, mem_yl, sum_ip];

  let microcode: [[[Result<ControlWord, TickTrap>; 0x20]; 0x02]; 0x80] = [[[(); 0x20]; 0x02]; 0x80]
    .iter()
    .enumerate()
    .map(|(opcode, rest)| (opcode as u8 | 0x80, rest)) // ignore `psh`s as they will be mapped to `phn`s by `sim`
    .map(|(opcode, rest)| {
      rest
        .iter()
        .enumerate()
        .map(|(carry, rest)| (carry != 0, rest))
        .map(|(carry, rest)| {
          rest
            .iter()
            .enumerate()
            .map(|(step, rest)| (step as usize, rest))
            .map(|(step, rest)| {
              let () = rest;
              // when encountering instructions `[` and `]`, the microcode must walk to the matching
              // instruction. address `0x01` stores the current nesting level to support nested loops.
              // when walking right the nesting level is positive and when walking left it is negative.
              // address `0xFF` stores the walking direction. when walking right the walking direction
              // is `0x01` and when walking left it is `0xFF`. this information can be derived from the
              // nesting level, but doing so would be inconvenient. `CF` stores whether or not we are
              // in a walking state. this information can be derived from the nesting level, but doing
              // so would be inconvenient.

              // the most significant bit of an opcode must be set before it is loaded into `IL`. `nfetch`
              // fetches `*IP`, inverts all its bits with a boolean `not`, and stores the result in `IL`.
              // since brainfuck is 7-bit ASCII, this ensures the most significant bit of `IL` is always
              // set. since the opcode was `not`ed, below we use `match !opcode` instead of `match opcode`
              let seq = match !opcode {
                b'>' => {
                  seq![
                    nfetch,
                    set_ylzl, //
                    match carry {
                      true => seq![walk, clr_yl],
                      false => seq![
                        sp_xl, sum_spal, // SP--
                        nand_yl
                      ],
                    }
                  ]
                }

                b'<' => {
                  seq![
                    nfetch,
                    clr_yl, //
                    match carry {
                      true => seq![walk, clr_yl],
                      false => seq![
                        sp_alxl, cinsum_sp // SP++
                      ],
                    }
                  ]
                }

                b'+' => {
                  seq![
                    nfetch,
                    clr_yl, //
                    match carry {
                      true => seq![walk, clr_yl],
                      false => seq![
                        sp_al, mem_xl, cinsum_mem // *SP++
                      ],
                    }
                  ]
                }

                b'-' => {
                  seq![
                    nfetch,
                    set_ylzl, //
                    match carry {
                      true => seq![walk, clr_yl],
                      false => seq![
                        sp_al, mem_xl, sum_mem, // *SP--
                        nand_yl
                      ],
                    }
                  ]
                }

                b'.' => {
                  seq![
                    nfetch,
                    set_ylzl, //
                    match carry {
                      true => seq![walk, clr_yl],
                      false => seq![
                        sp_al, mem_xl, nand_ylal, sum_mem // *SP -> *0x00
                      ],
                    }
                  ]
                }

                b',' => {
                  seq![
                    nfetch,
                    set_ylzl, //
                    match carry {
                      true => seq![walk, clr_yl],
                      false => seq![
                        nand_ylal, mem_xl, sp_al, sum_mem // *0x00 -> *SP
                      ],
                    }
                  ]
                }

                b'[' => {
                  seq![
                    nfetch,
                    set_alylzl,
                    nand_xlyl, //
                    match carry {
                      true => seq![
                        cinsum_al,
                        mem_xl,       // *0x01 -> XL
                        cinsum_memyl  // XL + 1 -> *0x01
                      ], // if walking, increment nesting level
                      false => seq![
                        noop, cinsum_al, sum_memyl // 0x00 -> *0x01
                      ], // else, reset nesting level to 0x00
                    },
                    nand_yl,
                    nand_ylcf, // *0x01 == 0x00 -> CF
                    match carry {
                      true => seq![clr_cf],  // if nesting level is 0x00, we're done walking
                      false => seq![set_cf], // else, we're walking
                    }, // !CF -> CF
                    match carry {
                      true => seq![noop, noop, noop, noop, noop, noop, noop, noop], // if we're walking, no-op
                      false => seq![
                        set_al, cinsum_mem, // 0x01 -> *0xFF
                        cinsum_al, cinsum_mem, // 0x01 -> *0x01
                        sp_al, mem_ylzl, nand_ylzl, nand_ylcf // *SP == 0x00 -> CF
                      ], // if we're done walking, set walking direction to right, set nesting level to 0x01, and check `*SP == 0x00`
                    },
                    match carry {
                      true => seq![ip_alxl, set_yl, sum_xl, walk], // if we're walking, perform a walk step
                      false => seq![noop, noop, noop, noop, noop, noop],
                    },
                    clr_yl
                  ]
                }

                b']' => {
                  seq![
                    nfetch,
                    set_alylzl,
                    nand_xlyl, //
                    match carry {
                      true => seq![
                        cinsum_al, mem_xl, // *0x01 -> XL
                        set_yl, sum_memyl // XL - 1 -> *0x01
                      ], // if we're walking, decrement nesting level
                      false => seq![
                        noop, noop, cinsum_al, sum_memyl // 0x00 -> *0x01
                      ], // else, reset nesting level to 0x00
                    },
                    nand_yl,
                    nand_ylcf, // *0x01 == 0x00 -> CF
                    // note that carry was not inverted here. for the next few lines,
                    // the carry bit represents `!walking` instead of `walking`
                    match carry {
                      false => seq![noop, noop, noop, noop, noop, noop, noop, noop, noop], // if we're walking, no-op
                      true => seq![
                        set_alylzl, set_mem, // 0xFF -> *0xFF
                        nand_xlyl, cinsum_al, nand_mem, // 0xFF -> *0x01
                        sp_al, mem_ylzl, nand_ylzl, nand_ylcf // *SP == 0x00 -> CF
                      ], // if we're done walking, set walking direction to left, set nesting level to 0xFF, and check `*SP != 0x00`
                    },
                    match carry {
                      false => seq![ip_alxl, set_yl, sum_xl, walk], // if we're walking, perform a walk step
                      true => seq![noop, noop, noop, noop, noop, noop],
                    },
                    // invert carry again so it represents `walking` instead of `!walking`
                    match carry {
                      true => seq![clr_cf],  // if nesting level is 0x00, we're done walking
                      false => seq![set_cf], // else, we're walking
                    } // !CF -> CF
                  ]
                }

                b'#' => {
                  seq![
                    nfetch,
                    clr_yl, //
                    match carry {
                      true => seq![walk, clr_yl],
                      false => vec![Err(TickTrap::DebugRequest)],
                    }
                  ]
                }

                b'\0' => {
                  seq![
                    nfetch, clr_yl, sum_ip // IP--
                  ]
                }

                _ => seq![
                  nfetch,
                  clr_yl, //
                  match carry {
                    true => seq![walk, clr_yl],
                    false => vec![],
                  }
                ],
              };

              let pre = seq![seq, clr_sc];
              let post = seq![clr_sc];
              match 0x20usize.overflowing_sub(pre.len() + post.len()) {
                (padding, false) => seq![pre, vec![Err(TickTrap::MicrocodeFault); padding], post],
                (wrapped, true) => {
                  if step == 0x00 {
                    errors.push(Error(format!(
                      "Microcode for opcode `{:02X}` with carry `{:01X}` overflows by {} steps",
                      opcode,
                      carry as u8,
                      wrapped.wrapping_neg()
                    )));
                  }
                  vec![Err(TickTrap::MicrocodeFault); 0x20]
                }
              }
              .get(step)
              .copied()
              .unwrap()
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
