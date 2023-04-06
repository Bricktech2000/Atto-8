fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 2 {
    println!("Usage: emu <filename>");
    std::process::exit(1);
  }

  let memory: Vec<u8> =
    std::fs::read(&args[1]).expect(format!("Unable to read file: {}", &args[1]).as_str());

  emulate(
    memory.try_into().expect("Slice with incorrect length"),
    10000,
  );
}

fn emulate(memory: [u8; 0x100], clock: u64) {
  let mut memory = memory.clone();
  let mut stack_pointer: u8 = 0x00; // SP
  let mut instruction_pointer: u8 = 0x00; // IP
  let mut carry_flag: bool = false; // CF

  let mut debug_flag: bool = false; // not an actual flag
  let mut debug_status: &str = "Emulating CPU...";
  let mut now = std::time::Instant::now();

  // clear screen
  print!("\x1B[2J");

  loop {
    let instruction: u8 = memory[instruction_pointer as usize];
    instruction_pointer = instruction_pointer.wrapping_add(1);

    // roughly 4 clock cycles per instruction
    std::thread::sleep(std::time::Duration::from_millis(1000 * 4 / clock));

    // only print 60 times per second
    if now.elapsed().as_millis() > 1000 / 60 || debug_flag {
      now = std::time::Instant::now();

      // move cursor to top left
      print!("\x1B[1;1H");

      print_display(
        &memory[0xE0..0x100]
          .try_into()
          .expect("Slice with incorrect length"),
      );
      println!("RAM");
      print_memory(
        &memory
          .clone()
          .try_into()
          .expect("Slice with incorrect length"),
      );
      println!(
        "IP {:16}\nSP {:16}\nCF {:16}\nDF {:16}",
        format!("{:02x}", instruction_pointer),
        format!("{:02x}", stack_pointer),
        carry_flag,
        debug_flag
      );

      print!("\n{:16}", debug_status);
      use std::io::{stdout, Write};
      stdout().flush().unwrap();

      if debug_flag {
        // use any key to single step
        // use '\n' to skip to next breakpoint
        let stdout = console::Term::buffered_stdout();
        if let Ok(character) = stdout.read_char() {
          if character == '\n' {
            debug_flag = false;
            debug_status = "Continuing...";
          } else {
            debug_status = "Single Step.";
          }
        }
      }
    }

    match instruction & 0b10000000 {
      0b00000000 => {
        // psh
        let immediate = instruction; // decode_immediate
        stack_pointer = stack_pointer.wrapping_sub(1);
        memory[stack_pointer as usize] = immediate;
      }
      0b10000000 => {
        match instruction & 0b01000000 {
          0b01000000 => {
            // (arithmetic and logic)
            let size = 1 << ((instruction & 0b00110000) >> 4); // decode_size
            let size_pointer = stack_pointer.wrapping_add(size);

            match instruction & 0b00001111 {
              0x00 if size == 0x01 => {
                // inc
                let a = memory[stack_pointer as usize];
                memory[stack_pointer as usize] = a.wrapping_add(1);
              }

              0x01 if size == 0x01 => {
                // dec
                let a = memory[stack_pointer as usize];
                memory[stack_pointer as usize] = a.wrapping_sub(1);
              }

              0x02 | 0x03 => {
                // add, adc
                if instruction & 0b00001111 == 0x02 {
                  carry_flag = false;
                }

                let a = memory[stack_pointer as usize];
                memory[stack_pointer as usize] = 0x00;
                stack_pointer = stack_pointer.wrapping_add(1);
                let b = memory[size_pointer as usize];

                memory[size_pointer as usize] = (b as u16 + a as u16 + carry_flag as u16) as u8;
                carry_flag = (b as u16 + a as u16 + carry_flag as u16) > 0xFF;
              }

              0x04 | 0x05 => {
                // sub, sbc
                if instruction & 0b00001111 == 0x04 {
                  carry_flag = false;
                }

                let a = memory[stack_pointer as usize];
                memory[stack_pointer as usize] = 0x00;
                stack_pointer = stack_pointer.wrapping_add(1);
                let b = memory[size_pointer as usize];

                memory[size_pointer as usize] = (b as i16 - a as i16 - carry_flag as i16) as u8;
                carry_flag = (b as i16 - a as i16 - carry_flag as i16) < 0x00;
              }

              0x06 => {
                // shf
                let a = memory[stack_pointer as usize];
                memory[stack_pointer as usize] = 0x00;
                stack_pointer = stack_pointer.wrapping_add(1);
                let b = memory[size_pointer as usize];

                let shifted = if a & 0b00001000 == 0 {
                  (b as u16) << (a & 0b00001111) as u16
                } else {
                  (b as u16) >> ((a ^ 0b00001111) + 1) as u16
                };

                memory[size_pointer as usize] = shifted as u8;
                // TODO: set carry flag
              }

              0x07 => {
                // rot
                let a = memory[stack_pointer as usize];
                memory[stack_pointer as usize] = 0x00;
                stack_pointer = stack_pointer.wrapping_add(1);
                let b = memory[size_pointer as usize];

                let shifted = if a & 0b00001000 == 0 {
                  (b as u16) << (a & 0b00001111) as u16
                } else {
                  (b as u16) >> ((a ^ 0b00001111) + 1) as u16
                };

                // TODO: use carry flag
                memory[size_pointer as usize] = (shifted & 0xFF) as u8 | (shifted >> 8) as u8;
                // TODO: set carry flag
              }

              0x08 => {
                // orr
                let a = memory[stack_pointer as usize];
                memory[stack_pointer as usize] = 0x00;
                stack_pointer = stack_pointer.wrapping_add(1);
                let b = memory[size_pointer as usize];

                memory[size_pointer as usize] = a | b;
                carry_flag = memory[size_pointer as usize] == 0x00;
              }

              0x09 => {
                // and
                let a = memory[stack_pointer as usize];
                memory[stack_pointer as usize] = 0x00;
                stack_pointer = stack_pointer.wrapping_add(1);
                let b = memory[size_pointer as usize];

                memory[size_pointer as usize] = a & b;
                carry_flag = memory[size_pointer as usize] == 0x00;
              }

              0x0A => {
                // xor
                let a = memory[stack_pointer as usize];
                memory[stack_pointer as usize] = 0x00;
                stack_pointer = stack_pointer.wrapping_add(1);
                let b = memory[size_pointer as usize];

                memory[size_pointer as usize] = a ^ b;
                carry_flag = memory[size_pointer as usize] == 0x00;
              }

              0x0B => {
                // xnd
                memory[stack_pointer as usize] = 0x00;
                stack_pointer = stack_pointer.wrapping_add(1);

                memory[size_pointer as usize] = 0;
                carry_flag = memory[size_pointer as usize] == 0x00;
              }

              0x0C if instruction & 0b00110000 == 0b00000000 => {
                // not
                let a = memory[stack_pointer as usize];

                memory[stack_pointer as usize] = !a;
                carry_flag = memory[stack_pointer as usize] == 0x00;
              }

              0x0C if instruction & 0b00110000 == 0b00010000 => {
                // ntp
                let a = memory[stack_pointer as usize];

                memory[stack_pointer as usize] = !a;
              }

              0x0D if instruction & 0b00110000 == 0b00000000 => {
                // buf
                let a = memory[stack_pointer as usize];

                memory[stack_pointer as usize] = a;
                carry_flag = memory[stack_pointer as usize] == 0x00;
              }

              0x0D if instruction & 0b00110000 == 0b00010000 => {
                // bfp
                let a = memory[stack_pointer as usize];

                memory[stack_pointer as usize] = a;
              }

              0x0E => {
                // iff
                let a = memory[stack_pointer as usize];
                memory[stack_pointer as usize] = 0x00;
                stack_pointer = stack_pointer.wrapping_add(1);
                let b = memory[stack_pointer as usize];

                memory[stack_pointer as usize] = if carry_flag { a } else { b };
                carry_flag = false;
              }

              _ => panic!("Unknown instruction: {:#04x}", instruction),
            }
          }

          0b00000000 => {
            match instruction & 0b00110000 {
              0b00000000 => {
                // ldo
                let offset = instruction & 0b00001111; // decode_offset
                let offset_pointer = stack_pointer.wrapping_add(offset);

                stack_pointer = stack_pointer.wrapping_sub(1);
                let a = memory[offset_pointer as usize];

                memory[stack_pointer as usize] = a;
              }

              0b00010000 => {
                // sto
                let a = memory[stack_pointer as usize];
                memory[stack_pointer as usize] = 0x00;
                stack_pointer = stack_pointer.wrapping_add(1);

                let offset = instruction & 0b00001111; // decode_offset
                let offset_pointer = stack_pointer.wrapping_add(offset);

                memory[offset_pointer as usize] = a;
              }

              0b00100000 => {
                // (clock and flags)
                match instruction & 0b00001111 {
                  0x00 => {
                    // nop
                  }

                  0x0A => {
                    // dbg
                    debug_flag = true;
                    debug_status = "Debug Request.";
                  }

                  0x01 => {
                    // clc
                    carry_flag = false;
                  }

                  0x02 => {
                    // sec
                    carry_flag = true;
                  }

                  0x03 => {
                    // flc
                    carry_flag = !carry_flag;
                  }

                  _ => panic!("Unknown instruction: {:#04x}", instruction),
                }
              }

              0b00110000 => {
                // (stack and memory)
                match instruction & 0b00001111 {
                  0x00 => {
                    // swp
                    let a = memory[stack_pointer as usize];
                    stack_pointer = stack_pointer.wrapping_add(1);
                    let b = memory[stack_pointer as usize];

                    memory[stack_pointer as usize] = a;
                    stack_pointer = stack_pointer.wrapping_sub(1);
                    memory[stack_pointer as usize] = b;
                  }

                  0x01 => {
                    // pop
                    memory[stack_pointer as usize] = 0x00;
                    stack_pointer = stack_pointer.wrapping_add(1);
                  }

                  0x02 => {
                    // xXX (Push Next)
                    let a = memory[instruction_pointer as usize];
                    instruction_pointer = instruction_pointer.wrapping_add(1);
                    stack_pointer = stack_pointer.wrapping_sub(1);

                    memory[stack_pointer as usize] = a;
                  }

                  0x08 => {
                    // lda
                    let a = memory[stack_pointer as usize];

                    memory[stack_pointer as usize] = memory[a as usize];
                  }

                  0x09 => {
                    // sta
                    let a = memory[stack_pointer as usize];
                    memory[stack_pointer as usize] = 0x00;
                    stack_pointer = stack_pointer.wrapping_add(1);
                    let b = memory[stack_pointer as usize];
                    memory[stack_pointer as usize] = 0x00;
                    stack_pointer = stack_pointer.wrapping_add(1);

                    memory[b as usize] = a;
                  }

                  0x0A => {
                    // ldi
                    stack_pointer = stack_pointer.wrapping_sub(1);
                    memory[stack_pointer as usize] = instruction_pointer;
                  }

                  0x0B => {
                    // sti
                    instruction_pointer = memory[stack_pointer as usize];
                    memory[stack_pointer as usize] = 0x00;
                    stack_pointer = stack_pointer.wrapping_add(1);
                  }

                  0x0C => {
                    // lds
                    stack_pointer = stack_pointer.wrapping_sub(1);
                    memory[stack_pointer as usize] = stack_pointer;
                  }

                  0x0D => {
                    // sts
                    let a = memory[stack_pointer as usize];
                    memory[stack_pointer as usize] = 0x00;

                    stack_pointer = a;
                  }

                  _ => panic!("Unknown instruction: {:#04x}", instruction),
                }
              }
              _ => unreachable!(),
            }
          }
          _ => unreachable!(),
        }
      }
      _ => unreachable!(),
    }
  }
}

fn print_display(display_buffer: &[u8; 0x20]) {
  let mut display_buffer_string: String = "".to_string();
  let line: &str = &"-".repeat(0x10);
  let line_top: &str = &format!(".-{}-.\n", line);
  let line_bottom: &str = &format!("'-{}-'\n", line);
  let col_left: &str = "| ";
  let col_right: &str = " |";

  display_buffer_string += &line_top;
  for y in (0..0x10).step_by(2) {
    display_buffer_string += &col_left;
    for x in 0..0x10 {
      let mut pixel_pair = 0;
      for y2 in 0..2 {
        let address: u8 = (x >> 0x03) | ((y + y2) << 0x01);
        let pixel = display_buffer[address as usize] >> (0x07 - (x & 0x07)) & 0x01;
        pixel_pair |= pixel << y2;
      }
      // https://en.wikipedia.org/wiki/Block_Elements
      display_buffer_string += match pixel_pair {
        0b00 => " ",
        0b01 => "\u{2580}",
        0b10 => "\u{2584}",
        0b11 => "\u{2588}",
        _ => "?",
      };
    }
    display_buffer_string += &col_right;
    display_buffer_string.push('\n');
  }

  display_buffer_string += &line_bottom;
  println!("{}", display_buffer_string);
}

fn print_memory(memory: &[u8; 0x100]) {
  let mut memory_string: String = "".to_string();

  for y in 0..0x10 {
    for x in 0..0x10 {
      memory_string += &format!("{:02x} ", memory[(y << 0x04 | x) as usize]);
    }
    memory_string.push('\n');
  }
  println!("{}", memory_string);
}
