fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 2 {
    println!("Usage: emu <filename>");
    std::process::exit(1);
  }

  let filename: &String = &args[1];

  let memory: Vec<u8> = match std::fs::read(filename) {
    Ok(source) => source,
    Err(_) => {
      println!("Error: Unable to read file: {}", filename);
      std::process::exit(1);
    }
  };

  match memory.try_into() {
    Ok(slice) => {
      emulate(slice, 2000000);
    }
    Err(_) => {
      println!("Error: Memory image has incorrect size");
      std::process::exit(1);
    }
  };
}

fn emulate(memory: [u8; 0x100], clock: u128) {
  let mut memory = memory.clone();
  let mut stack_pointer: u8 = 0x00; // SP
  let mut instruction_pointer: u8 = 0x00; // IP
  let mut carry_flag: bool = false; // CF

  let mut debug_flag: bool = false; // whether to print debug info
  let mut input_flag: bool = false; // whether to read input buffer
  let mut debug_status: String = "Emulating microcomputer...".to_string();
  let mut now = std::time::Instant::now();
  let start = std::time::Instant::now();

  // clear screen
  print!("\x1B[2J");

  // this call will switch termital to raw mode
  let input_channel = spawn_input_channel();

  for i in 0u128.. {
    let instruction: u8 = memory[instruction_pointer as usize];
    instruction_pointer = instruction_pointer.wrapping_add(1);

    // roughly 4 clock cycles per instruction
    if start.elapsed().as_millis() < i * 1000 * 4 / clock {
      // assuming instruction emulation is instant, for 1ms to be enough to correct for
      // the offset, it must be the case that the clock runs at more than 1000Hz
      std::thread::sleep(std::time::Duration::from_millis(1));
    }

    // only print 30 times per second
    if now.elapsed().as_millis() > 1000 / 30 || debug_flag {
      now = std::time::Instant::now();

      // move cursor to top left
      print!("\x1B[1;1H");

      print_display(&memory[0xE0..0x100].try_into().unwrap());
      print!("RAM\r\n");
      print_memory(&memory.clone().try_into().unwrap());
      print!(
        "IP {:8}\r\nSP {:8}\r\nCF {:8}\r\n",
        format!("{:02x}", instruction_pointer),
        format!("{:02x}", stack_pointer),
        carry_flag,
      );

      print!("\r\n{:26}", debug_status);
      use std::io::{stdout, Write};
      stdout().flush().unwrap();
    }

    if debug_flag {
      // use any key to single step
      // use '\n' to skip to next breakpoint
      loop {
        match input_channel.recv() {
          Ok('\n') => {
            debug_flag = false;
            debug_status = "Continuing emulation...".to_string();
            break;
          }
          Ok(' ') => {
            debug_status = "Single Stepped.".to_string();
            break;
          }
          _ => {}
        }
      }
    }

    if input_flag {
      use std::sync::mpsc::TryRecvError;
      match input_channel.try_recv() {
        Ok(character) => {
          let lo_nibble: u8 = match character {
            'w' => 0b0001,
            's' => 0b0010,
            'a' => 0b0100,
            'd' => 0b1000,
            _ => 0b0000,
          };
          let hi_nibble: u8 = match character {
            'i' => 0b0001,
            'k' => 0b0010,
            'j' => 0b0100,
            'l' => 0b1000,
            _ => 0b0000,
          };
          memory[0x00] |= lo_nibble | (hi_nibble << 4);
        }
        Err(TryRecvError::Empty) => {}
        Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
      }
    }

    match (instruction & 0b10000000) >> 7 {
      0b0 => {
        // psh
        let immediate = instruction; // decode_immediate
        stack_pointer = stack_pointer.wrapping_sub(1);
        memory[stack_pointer as usize] = immediate;
      }
      0b1 => {
        match (instruction & 0b01000000) >> 6 {
          0b0 => {
            // (arithmetic and logic)
            let size = 1 << (instruction & 0b00000011); // decode_size
            let opcode = (instruction & 0b00111100) >> 2;
            let size_pointer = stack_pointer.wrapping_add(size);

            match opcode {
              0x0 | 0x1 => {
                // add, adc
                if opcode == 0x0 {
                  carry_flag = false;
                }

                let a = memory[stack_pointer as usize];
                memory[stack_pointer as usize] = 0x00;
                stack_pointer = stack_pointer.wrapping_add(1);
                let b = memory[size_pointer as usize];

                memory[size_pointer as usize] = (b as u16 + a as u16 + carry_flag as u16) as u8;
                carry_flag = (b as u16 + a as u16 + carry_flag as u16) > 0xFF;
              }

              0x2 | 0x3 => {
                // sub, sbc
                if opcode == 0x2 {
                  carry_flag = false;
                }

                let a = memory[stack_pointer as usize];
                memory[stack_pointer as usize] = 0x00;
                stack_pointer = stack_pointer.wrapping_add(1);
                let b = memory[size_pointer as usize];

                memory[size_pointer as usize] = (b as i16 - a as i16 - carry_flag as i16) as u8;
                carry_flag = (b as i16 - a as i16 - carry_flag as i16) < 0x00;
              }

              0x4 | 0x5 => {
                // shf, sfc
                if opcode == 0x4 {
                  carry_flag = false;
                }

                if opcode == 0x5 {
                  todo!();
                }

                let a = memory[stack_pointer as usize];
                memory[stack_pointer as usize] = 0x00;
                stack_pointer = stack_pointer.wrapping_add(1);
                let b = memory[size_pointer as usize];

                let shifted = if a as i8 >= 0 {
                  (b as u16).wrapping_shl(a as u32)
                } else {
                  (b as u16).wrapping_shr(a.wrapping_neg() as u32)
                } | carry_flag as u16;

                memory[size_pointer as usize] = shifted as u8;
                carry_flag = shifted > 0xFF;
              }

              0x6 => {
                // rot
                let a = memory[stack_pointer as usize];
                memory[stack_pointer as usize] = 0x00;
                stack_pointer = stack_pointer.wrapping_add(1);
                let b = memory[size_pointer as usize];

                let shifted = if a as i8 >= 0 {
                  (b as u16).wrapping_shl(a as u32)
                } else {
                  (b as u16).wrapping_shr(a.wrapping_neg() as u32)
                };

                memory[size_pointer as usize] = (shifted & 0xFF) as u8 | (shifted >> 8) as u8;
                carry_flag = shifted > 0xFF;
              }

              0x7 => {
                // iff
                let a = memory[stack_pointer as usize];
                memory[stack_pointer as usize] = 0x00;
                stack_pointer = stack_pointer.wrapping_add(1);
                let b = memory[stack_pointer as usize];

                memory[stack_pointer as usize] = if carry_flag { a } else { b };
                carry_flag = false;
              }

              0x8 => {
                // orr
                let a = memory[stack_pointer as usize];
                memory[stack_pointer as usize] = 0x00;
                stack_pointer = stack_pointer.wrapping_add(1);
                let b = memory[size_pointer as usize];

                memory[size_pointer as usize] = a | b;
                carry_flag = memory[size_pointer as usize] == 0x00;
              }

              0x9 => {
                // and
                let a = memory[stack_pointer as usize];
                memory[stack_pointer as usize] = 0x00;
                stack_pointer = stack_pointer.wrapping_add(1);
                let b = memory[size_pointer as usize];

                memory[size_pointer as usize] = a & b;
                carry_flag = memory[size_pointer as usize] == 0x00;
              }

              0xA => {
                // xor
                let a = memory[stack_pointer as usize];
                memory[stack_pointer as usize] = 0x00;
                stack_pointer = stack_pointer.wrapping_add(1);
                let b = memory[size_pointer as usize];

                memory[size_pointer as usize] = a ^ b;
                carry_flag = memory[size_pointer as usize] == 0x00;
              }

              0xB => {
                // xnd
                memory[stack_pointer as usize] = 0x00;
                stack_pointer = stack_pointer.wrapping_add(1);

                memory[size_pointer as usize] = 0;
                carry_flag = memory[size_pointer as usize] == 0x00;
              }

              _ => match (opcode, instruction & 0b00000011) {
                // (size used as part of opcode)
                (0xC, 0b00) => {
                  // adn
                  let a = memory[stack_pointer as usize];
                  memory[stack_pointer as usize] = 0x00;
                  stack_pointer = stack_pointer.wrapping_add(1);
                  let b = memory[stack_pointer as usize];

                  memory[stack_pointer as usize] = (b.wrapping_add(a) & 0b00001111)
                    | ((b & 0b11110000).wrapping_add(a & 0b11110000));
                }

                (0xC, 0b01) => {
                  // sbn
                  let a = memory[stack_pointer as usize];
                  memory[stack_pointer as usize] = 0x00;
                  stack_pointer = stack_pointer.wrapping_add(1);
                  let b = memory[stack_pointer as usize];

                  memory[stack_pointer as usize] = (b.wrapping_sub(a) & 0b00001111)
                    | ((b & 0b11110000).wrapping_sub(a & 0b11110000));
                }

                (0xC, 0b10) => {
                  // inc
                  let a = memory[stack_pointer as usize];
                  memory[stack_pointer as usize] = a.wrapping_add(1);
                }

                (0xC, 0b11) => {
                  // dec
                  let a = memory[stack_pointer as usize];
                  memory[stack_pointer as usize] = a.wrapping_sub(1);
                }

                (0xD, 0b00) => {
                  // neg
                  let a = memory[stack_pointer as usize];
                  memory[stack_pointer as usize] = a.wrapping_neg();
                }

                (0xD, 0b10) => {
                  // not
                  let a = memory[stack_pointer as usize];

                  memory[stack_pointer as usize] = !a;
                  carry_flag = memory[stack_pointer as usize] == 0x00;
                }

                (0xD, 0b11) => {
                  // buf
                  let a = memory[stack_pointer as usize];

                  memory[stack_pointer as usize] = a;
                  carry_flag = memory[stack_pointer as usize] == 0x00;
                }

                (0b1110, 0b11) => {
                  // dbg

                  debug_flag = true;
                  debug_status = "Debug Request.".to_string();
                }

                _ => {
                  debug_flag = true;
                  debug_status = format!("Unknown instruction: {:#04x}", instruction);
                }
              },
            }
          }

          0b1 => {
            match (instruction & 0b00110000) >> 4 {
              0b00 => {
                // ldo
                let offset = instruction & 0b00001111; // decode_offset
                let offset_pointer = stack_pointer.wrapping_add(offset);

                stack_pointer = stack_pointer.wrapping_sub(1);
                let a = memory[offset_pointer as usize];

                memory[stack_pointer as usize] = a;
              }

              0b01 => {
                // sto
                let a = memory[stack_pointer as usize];
                memory[stack_pointer as usize] = 0x00;
                stack_pointer = stack_pointer.wrapping_add(1);

                let offset = instruction & 0b00001111; // decode_offset
                let offset_pointer = stack_pointer.wrapping_add(offset);

                memory[offset_pointer as usize] = a;
              }

              0b10 => {
                // (clock and flags and stack)
                match instruction & 0b00001111 {
                  0x0 => {
                    // nop
                  }

                  0x1 => {
                    // clc
                    carry_flag = false;
                  }

                  0x2 => {
                    // sec
                    carry_flag = true;
                  }

                  0x3 => {
                    // flc
                    carry_flag = !carry_flag;
                  }

                  0x4 => {
                    // swp
                    let a = memory[stack_pointer as usize];
                    stack_pointer = stack_pointer.wrapping_add(1);
                    let b = memory[stack_pointer as usize];

                    memory[stack_pointer as usize] = a;
                    stack_pointer = stack_pointer.wrapping_sub(1);
                    memory[stack_pointer as usize] = b;
                  }

                  0x5 => {
                    // pop
                    memory[stack_pointer as usize] = 0x00;
                    stack_pointer = stack_pointer.wrapping_add(1);
                  }

                  0x8 => {
                    // lda
                    let a = memory[stack_pointer as usize];

                    memory[stack_pointer as usize] = memory[a as usize];
                  }

                  0x9 => {
                    // sta
                    let a = memory[stack_pointer as usize];
                    memory[stack_pointer as usize] = 0x00;
                    stack_pointer = stack_pointer.wrapping_add(1);
                    let b = memory[stack_pointer as usize];
                    memory[stack_pointer as usize] = 0x00;
                    stack_pointer = stack_pointer.wrapping_add(1);

                    memory[b as usize] = a;

                    if a == 0x00 && b == 0x00 {
                      input_flag = true;
                    }
                  }

                  0xA => {
                    // ldi
                    stack_pointer = stack_pointer.wrapping_sub(1);
                    memory[stack_pointer as usize] = instruction_pointer;
                  }

                  0xB => {
                    // sti
                    instruction_pointer = memory[stack_pointer as usize];
                    memory[stack_pointer as usize] = 0x00;
                    stack_pointer = stack_pointer.wrapping_add(1);
                  }

                  0xC => {
                    // lds
                    stack_pointer = stack_pointer.wrapping_sub(1);
                    memory[stack_pointer as usize] = stack_pointer;
                  }

                  0xD => {
                    // sts
                    let a = memory[stack_pointer as usize];
                    memory[stack_pointer as usize] = 0x00;

                    stack_pointer = a;
                  }

                  _ => {
                    debug_flag = true;
                    debug_status = format!("Unknown instruction: {:#04x}", instruction);
                  }
                }
              }

              0b11 => {
                // phn

                let immediate = instruction; // decode_immediate
                stack_pointer = stack_pointer.wrapping_sub(1);
                memory[stack_pointer as usize] = immediate;
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
  let line_top: &str = &format!(".-{}-.\r\n", line);
  let line_bottom: &str = &format!("'-{}-'\r\n", line);
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
    display_buffer_string += "\r\n";
  }

  display_buffer_string += &line_bottom;
  print!("{}\r\n", display_buffer_string);
}

fn print_memory(memory: &[u8; 0x100]) {
  let mut memory_string: String = "".to_string();

  for y in 0..0x10 {
    for x in 0..0x10 {
      memory_string += &format!("{:02x} ", memory[(y << 0x04 | x) as usize]);
    }
    memory_string += "\r\n";
  }
  print!("{}\r\n", memory_string);
}

use std::sync::mpsc;
use std::sync::mpsc::Receiver;
fn spawn_input_channel() -> Receiver<char> {
  let stdout = console::Term::stdout();

  let (tx, rx) = mpsc::channel::<char>();
  std::thread::spawn(move || loop {
    if let Ok(character) = stdout.read_char() {
      tx.send(character).unwrap();
    }
  });

  rx
}
