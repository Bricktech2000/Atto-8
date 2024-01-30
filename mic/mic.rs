#[path = "../misc/common/common.rs"]
mod common;
use common::*;

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
        .map(|error| format!("Mic: Error: {}", error))
        .collect::<Vec<String>>()
        .join("\n");

      println!("{}", errors);
      std::process::exit(1);
    }
  }

  println!("Mic: Done");
}

fn build_microcode(errors: &mut impl Extend<Error>) -> [u16; common::MIC_SIZE] {
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
  let mem_ilzl = ControlWord! {mem_data, data_il, data_zl};
  let nand_ylzl = ControlWord! {nand_data, data_yl, data_zl};
  let sum_spal = ControlWord! {sum_data, data_sp, data_al};
  let cinsum_ip = ControlWord! {set_cin, sum_data, data_ip};
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
  let cinsum_yl = ControlWord! {set_cin, sum_data, data_yl};
  let cinsum_mem = ControlWord! {set_cin, sum_data, data_mem};
  let set_ylzl = ControlWord! {data_yl, data_zl};
  let sp_xlzl = ControlWord! {sp_data, data_xl, data_zl};
  let set_zl = ControlWord! {data_zl};
  let nand_yl = ControlWord! {nand_data, data_yl};
  let mem_ip = ControlWord! {mem_data, data_ip};
  let cinsum_sp = ControlWord! {set_cin, sum_data, data_sp};
  let nand_memcf = ControlWord! {nand_data, data_mem, data_cf};
  let mem_sp = ControlWord! {mem_data, data_sp};
  let nand_ylcf = ControlWord! {nand_data, data_yl, data_cf};
  let mem_xlyl = ControlWord! {mem_data, data_xl, data_yl};
  let nand_al = ControlWord! {nand_data, data_al};
  let cinsum_spxl = ControlWord! {set_cin, sum_data, data_sp, data_al, data_xl};
  let cinsum_alxl = ControlWord! {set_cin, sum_data, data_al, data_xl};
  let nand_zlcf = ControlWord! {nand_data, data_zl, data_cf};
  let cinsum_xlcf = ControlWord! {set_cin, sum_data, data_xl, data_cf};
  let sum_xlcf = ControlWord! {sum_data, data_xl, data_cf};
  let cinsum_xlylcf = ControlWord! {set_cin, sum_data, data_xl, data_yl, data_cf};
  let sum_xlylcf = ControlWord! {sum_data, data_xl, data_yl, data_cf};
  let mem_yl = ControlWord! {mem_data, data_yl};
  let cinsum_xl = ControlWord! {set_cin, sum_data, data_xl};
  let sum_xl = ControlWord! {sum_data, data_xl};
  let nand_memyl = ControlWord! {nand_data, data_mem, data_yl};
  let set_xlylzl = ControlWord! {data_xl, data_yl, data_zl};
  let ip_mem = ControlWord! {ip_data, data_mem};
  let clr_sc = ControlWord! {clr_sc};
  let nand_xlylcf = ControlWord! {nand_data, data_xl, data_yl, data_cf};
  let nand_xlylzl = ControlWord! {nand_data, data_xl, data_yl, data_zl};

  let noop = seq![ControlWord! {}];
  let fetch = seq![ip_alxl, cinsum_ip, mem_ilzl];
  let clr_yl = seq![set_ylzl, nand_yl];
  let set_cf = seq![set_xlylzl, set_xlylzl, nand_xlylcf];
  let clr_cf = seq![set_xlylzl, nand_xlylzl, nand_zlcf];

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
              let seq = match common::opcode_to_instruction(opcode) {
                Err(_opcode) => seq![fetch, vec![Err(TickTrap::IllegalOpcode)]],
                Ok(instruction) => match instruction {
                  Instruction::Psh(_imm) => {
                    unreachable!()
                  }

                  Instruction::Add(size) => {
                    seq![
                      fetch, //
                      sp_alxl,
                      cinsum_sp,                        // SP++
                      mem_zl,                           // *SP -> ZL
                      seq![cinsum_alxl; size as usize], // SP + SIZE -> AL
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

                  Instruction::Sub(size) => {
                    seq![
                      fetch, //
                      sp_alxl,
                      cinsum_sp,                        // SP++
                      mem_zl,                           // *SP -> ZL
                      seq![cinsum_alxl; size as usize], // SP + SIZE -> AL
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

                  Instruction::Iff(size) => {
                    seq![
                      fetch, //
                      sp_alxl,
                      cinsum_sp,                        // SP++
                      mem_zl,                           // *SP -> ZL
                      seq![cinsum_alxl; size as usize], // SP + SIZE -> AL
                      mem_xl,                           // *AL -> XL
                      set_yl,
                      match carry {
                        true => seq![nand_zl, nand_xl], // ZL -> XL
                        false => seq![noop, noop],      // no-op
                      },
                      cinsum_mem, // XL -> *AL
                      clr_yl
                    ]
                  }

                  Instruction::Swp(size) => {
                    seq![
                      fetch, //
                      sp_alxl,
                      mem_zl,                           // *SP -> ZL
                      seq![cinsum_alxl; size as usize], // SP + SIZE -> AL
                      mem_xl,                           // *AL -> XL
                      set_yl,
                      nand_zl,
                      nand_mem, // ZL -> *AL
                      sp_al,
                      cinsum_mem, // XL -> *SP
                      clr_yl      //
                    ]
                  }

                  Instruction::Rot(size) => {
                    seq![
                      match carry {
                        true => seq![clr_yl, noop],
                        false => seq![fetch],
                      }, // continuation of match below
                      sp_alxl,
                      mem_zl,                           // *SP -> ZL
                      seq![cinsum_alxl; size as usize], // SP + SIZE -> AL
                      mem_xlyl,
                      cinsum_xlcf, // *AL + *AL -> XL; XL++
                      set_yl,
                      match carry {
                        true => seq![cinsum_xl], // --XL + 1 -> XL
                        false => seq![sum_xl],   // --XL -> XL
                      },
                      nand_zl,
                      nand_zlcf, // test ZL == 0x00
                      match carry {
                        // done. ignore shifted value, pop counter, clear carry, fetch next instruction
                        true => seq![clr_yl, sp_alxl, cinsum_sp, clr_cf], // SP++
                        // not done. store shifted value, decrement counter, set carry
                        false => seq![cinsum_mem, sp_al, mem_xl, sum_mem, set_cf], // XL -> *AL; *SP - 1 -> *SP
                      }
                    ]
                  }

                  Instruction::Orr(size) => {
                    seq![
                      fetch, //
                      sp_alxl,
                      cinsum_sp,                        // SP++
                      mem_zl,                           // *SP -> ZL
                      seq![cinsum_alxl; size as usize], // SP + SIZE -> AL
                      set_yl,
                      nand_xl, // ~ZL -> XL
                      mem_zl,
                      nand_zl,    // ~*AL -> ZL
                      cinsum_yl,  // XL -> YL
                      nand_memcf, // YL NAND ZL -> *AL
                      clr_yl      //
                    ]
                  }

                  Instruction::And(size) => {
                    seq![
                      fetch, //
                      sp_alxl,
                      cinsum_sp,                        // SP++
                      mem_zl,                           // *SP -> ZL
                      seq![cinsum_alxl; size as usize], // SP + SIZE -> AL
                      mem_yl,                           // *AL -> YL
                      nand_ylzl,
                      nand_memcf, // ~(YL NAND ZL) -> *AL
                      clr_yl      //
                    ]
                  }

                  Instruction::Xor(size) => {
                    seq![
                      fetch, //
                      sp_alxl,
                      cinsum_sp,                        // SP++
                      mem_zl,                           // *SP -> ZL
                      seq![cinsum_alxl; size as usize], // SP + SIZE -> AL
                      set_yl,
                      nand_zl,
                      nand_xl, // ZL -> XL
                      mem_zl,
                      nand_zl,   // ~*AL -> ZL
                      cinsum_yl, // XL -> YL
                      nand_xl,   // YL NAND ZL -> XL
                      // ---
                      set_zl,
                      nand_yl, // ~YL -> YL
                      mem_zl,  // *AL -> ZL
                      nand_zl, // YL NAND ZL -> ZL
                      set_yl,
                      cinsum_yl,  // XL -> YL
                      nand_memcf, // ~(YL NAND ZL) -> *AL
                      clr_yl      //
                    ]
                  }

                  Instruction::Xnd(size) => {
                    seq![
                      fetch, //
                      sp_alxl,
                      cinsum_sp,                        // SP++
                      seq![cinsum_alxl; size as usize], // SP + SIZE -> AL
                      set_xlylzl,                       // 0xFF -> XL; 0xFF -> YL; 0xFF -> ZL
                      sum_xlcf,                         // 1 -> CF
                      nand_memyl                        // 0x00 -> YL; 0x00 -> *AL
                    ]
                  }

                  Instruction::Inc => {
                    seq![
                      fetch, //
                      sp_al, mem_xl,     // *SP -> XL
                      cinsum_mem  // XL + 1 -> *SP
                    ]
                  }

                  Instruction::Dec => {
                    seq![
                      fetch, //
                      sp_al, mem_xl, // *SP -> XL
                      set_ylzl, sum_mem, // XL + 0xFF -> *SP
                      nand_yl  //
                    ]
                  }

                  Instruction::Neg => {
                    seq![
                      fetch, //
                      set_ylzl, nand_xl, // 0x00 -> XL
                      sp_al, mem_ylzl, nand_ylzl, cinsum_mem, // 0x00 - *SP -> *SP
                      clr_yl      //
                    ]
                  }

                  Instruction::Shl => {
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

                  Instruction::Shr => {
                    seq![
                      fetch, //
                      sp_al,
                      mem_xlyl, // *SP -> XL; *SP -> YL
                      match carry {
                        true => seq![cinsum_xlylcf; 8], // XL + XL -> XL; XL + XL -> YL
                        false => seq![sum_xlylcf; 8],   // XL + XL -> XL; XL + XL -> YL
                      },
                      set_ylzl,
                      cinsum_mem, // XL -> *SP
                      nand_yl
                    ]
                  }

                  Instruction::Not => {
                    seq![
                      fetch, //
                      sp_al, mem_ylzl, nand_memcf, // ~*SP -> *SP
                      clr_yl      //
                    ]
                  }

                  Instruction::Buf => {
                    seq![
                      fetch, //
                      sp_al, mem_ylzl, nand_ylzl, nand_memcf, // *SP -> *SP
                      clr_yl      //
                    ]
                  }

                  Instruction::Dbg => {
                    seq![fetch, vec![Err(TickTrap::DebugRequest)]]
                  }

                  Instruction::Ldo(ofst) => {
                    seq![
                      fetch, //
                      sp_alxl,
                      seq![cinsum_alxl; ofst as usize], // SP + OFST -> AL
                      mem_zl,                           // *AL -> ZL
                      sp_xl,
                      set_yl,
                      sum_spal, // SP-- -> AL
                      nand_zl,
                      nand_mem, // ZL -> *AL
                      clr_yl    //
                    ]
                  }

                  Instruction::Sto(ofst) => {
                    seq![
                      fetch, //
                      sp_alxl,
                      mem_zl, // *SP -> ZL
                      cinsum_spxl,
                      seq![cinsum_alxl; ofst as usize], // ++SP + OFST -> AL
                      set_yl,
                      nand_zl,
                      nand_mem, // ZL -> *AL
                      clr_yl
                    ]
                  }

                  Instruction::Lda => {
                    seq![
                      fetch, //
                      sp_al, mem_xl, // *SP -> XL
                      sum_al, mem_xl, // *XL -> XL
                      sp_al, sum_mem // XL -> *SP
                    ]
                  }

                  Instruction::Sta => {
                    seq![
                      fetch, //
                      sp_alxl, cinsum_sp, mem_zl, // *SP++ -> ZL
                      sp_alxl, cinsum_sp, mem_xl, // *SP++ -> XL
                      set_yl, nand_zl, nand_al, // ZL -> AL
                      clr_yl, sum_mem // ZL -> *AL
                    ]
                  }

                  Instruction::Ldi => {
                    seq![
                      fetch, //
                      sp_xl, set_yl, sum_spal, // SP-- -> AL
                      ip_mem,   // IP -> *AL
                      clr_yl    //
                    ]
                  }

                  Instruction::Sti => {
                    seq![
                      fetch, //
                      sp_alxl, cinsum_sp, // SP++
                      mem_ip     // *SP -> IP
                    ]
                  }

                  Instruction::Lds => {
                    seq![
                      fetch, //
                      sp_xlzl, set_yl, sum_spal, // SP -> ZL; SP-- -> AL
                      nand_zl, nand_mem, // ZL -> *AL
                      clr_yl    //
                    ]
                  }

                  Instruction::Sts => {
                    seq![
                      fetch, //
                      sp_al, mem_sp // *SP -> SP
                    ]
                  }

                  Instruction::Clc => {
                    seq![fetch, clr_cf]
                  }

                  Instruction::Sec => {
                    seq![fetch, set_cf]
                  }

                  Instruction::Flc => match carry {
                    true => seq![fetch, clr_cf],
                    false => seq![fetch, set_cf],
                  },

                  Instruction::Nop => {
                    seq![fetch]
                  }

                  Instruction::Pop => {
                    seq![
                      fetch,
                      sp_xl, cinsum_sp // SP++
                    ]
                  }

                  Instruction::Phn(_nimm) => {
                    seq![
                      fetch, // instruction is in ZL
                      sp_xl, set_yl, sum_spal, // SP-- -> AL
                      nand_zl, nand_mem, // IL -> *AL
                      clr_yl    //
                    ]
                  }
                },
              };

              let pre = seq![seq, clr_sc];
              let post = seq![noop];
              match 0x20usize.overflowing_sub(pre.len() + post.len()) {
                (padding, false) => seq![pre, vec![Err(TickTrap::MicrocodeFault); padding], post],
                (wrapped, true) => {
                  if step == 0x00 {
                    errors.extend([Error(format!(
                      "Microcode for opcode `{:02X}` with carry `{:01X}` overflows by {} steps",
                      opcode,
                      carry as u8,
                      wrapped.wrapping_neg()
                    ))]);
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
