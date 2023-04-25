fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 3 {
    println!("Usage: dasm <image file> <assembly file>");
    std::process::exit(1);
  }

  let image_file: &String = &args[1];
  let assembly_file: &String = &args[2];

  let memory: Vec<u8> = match std::fs::read(image_file) {
    Ok(source) => source,
    Err(_) => {
      println!("Error: Unable to read file: {}", image_file);
      std::process::exit(1);
    }
  };

  match memory.try_into() {
    Ok(slice) => {
      std::fs::write(assembly_file, disassemble(slice, "main")).unwrap();
      println!("Done.");
    }
    Err(_) => {
      println!("Error: Memory image has incorrect size");
      std::process::exit(1);
    }
  };
}

fn disassemble(memory: [u8; 0x100], entry_point: &str) -> String {
  format!(
    "{}!\n{}",
    entry_point,
    memory
      .iter()
      .map(|instruction| {
        match (instruction & 0b10000000) >> 7 {
          0b0 => {
            let immediate = instruction; // decode_immediate
            format!("x{:02X}  @dyn", immediate)
          }

          0b1 => {
            match (instruction & 0b01000000) >> 6 {
              0b0 => {
                // (arithmetic and logic)
                let size = 1 << (instruction & 0b00000011); // decode_size
                let opcode = (instruction & 0b00111100) >> 2;

                match opcode {
                  0x0 => format!("add{:01X} @dyn", size),
                  0x1 => format!("adc{:01X} @dyn", size),

                  0x2 => format!("sub{:01X} @dyn", size),
                  0x3 => format!("sbc{:01X} @dyn", size),

                  0x4 => format!("shf{:01X} @dyn", size),
                  0x5 => format!("sfc{:01X} @dyn", size),

                  0x6 => format!("rot{:01X} @dyn", size),

                  0x7 => format!("iff{:01X} @dyn", size),

                  0x8 => format!("orr{:01X} @dyn", size),

                  0x9 => format!("and{:01X} @dyn", size),

                  0xA => format!("xor{:01X} @dyn", size),

                  0xB => format!("xnd{:01X} @dyn", size),

                  _ => match (opcode, instruction & 0b00000011) {
                    // (size used as part of opcode)
                    (0xC, 0b00) => format!("adn  @dyn"),

                    (0xC, 0b01) => format!("sbn  @dyn"),

                    (0xC, 0b10) => format!("inc  @dyn"),

                    (0xC, 0b11) => format!("dec  @dyn"),

                    (0xD, 0b00) => format!("neg  @dyn"),

                    (0xD, 0b10) => format!("not  @dyn"),

                    (0xD, 0b11) => format!("buf  @dyn"),

                    (0b1110, 0b11) => format!("dBB"),

                    _ => {
                      format!("d{:02X}      ", instruction)
                    }
                  },
                }
              }

              0b1 => {
                match (instruction & 0b00110000) >> 4 {
                  0b00 => {
                    let offset = instruction & 0b00001111; // decode_offset
                    format!("ld{:01X}  @dyn", offset)
                  }

                  0b01 => {
                    let offset = instruction & 0b00001111; // decode_offset
                    format!("st{:01X}  @dyn", offset)
                  }

                  0b10 => {
                    // (clock and flags and stack)
                    match instruction & 0b00001111 {
                      0x0 => format!("nop  @dyn"),

                      0x1 => format!("clc  @dyn"),

                      0x2 => format!("sec  @dyn"),

                      0x3 => format!("flc  @dyn"),

                      0x4 => format!("swp  @dyn"),

                      0x5 => format!("pop  @dyn"),

                      0x8 => format!("lda  @dyn"),

                      0x9 => format!("sta  @dyn"),

                      0xA => format!("ldi  @dyn"),

                      0xB => format!("sti  @dyn"),

                      0xC => format!("lds  @dyn"),

                      0xD => format!("sts  @dyn"),

                      _ => {
                        format!("d{:02X}      ", instruction)
                      }
                    }
                  }

                  0b11 => {
                    let immediate = instruction; // decode_immediate
                    format!("x{:02X}  @dyn", immediate)
                  }

                  _ => unreachable!(),
                }
              }

              _ => unreachable!(),
            }
          }

          _ => unreachable!(),
        }
      })
      .enumerate()
      .zip(memory.iter())
      .map(|((index, mnemonic), byte)| format!("{} # x{:02X} @org d{:02X}", mnemonic, index, byte))
      .collect::<Vec<String>>()
      .join("\n")
  )
}
