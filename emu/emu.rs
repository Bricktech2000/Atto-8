fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 2 {
    println!("Usage: emu <image file>");
    std::process::exit(1);
  }

  let image_file: &String = &args[1];

  let memory: Vec<u8> = match std::fs::read(image_file) {
    Ok(source) => source,
    Err(_) => {
      println!("Error: Unable to read file: {}", image_file);
      std::process::exit(1);
    }
  };

  match memory.try_into() {
    Ok(slice) => {
      emulate(slice, 100000);
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
  let mut input_ored: [u8; 0x01] = [memory[0]]; // input buffer values since last frame ORed together
  let mut now = std::time::Instant::now();
  let mut start = std::time::Instant::now();

  // clear screen
  print!("\x1B[2J");

  // this call will switch termital to raw mode
  let input_channel = spawn_input_channel();

  for i in 0u128.. {
    let instruction: u8 = memory[instruction_pointer as usize];
    instruction_pointer = instruction_pointer.wrapping_add(1);

    let elapsed = std::cmp::max(start.elapsed().as_millis(), 1); // prevent division by zero
    let realtime_offset = elapsed as i128 - i as i128 * 1000 * 4 / clock as i128; // roughly 4 clock cycles per instruction
    let realtime_ratio = realtime_offset as f64 / elapsed as f64;
    if realtime_offset < 0 {
      std::thread::sleep(std::time::Duration::from_millis(-realtime_offset as u64));
    }

    // only print 60 times per second
    if now.elapsed().as_millis() > 1000 / 60 || debug_flag {
      now = std::time::Instant::now();

      // move cursor to top left
      print!("\x1B[1;1H");

      print_display(&memory[0xE0..0x100].try_into().unwrap());
      print!("\r\n");
      print_input(&input_ored.try_into().unwrap());
      print!("\r\n");
      print!("RAM\r\n");
      print_memory(&memory.clone().try_into().unwrap());
      print!(
        "IP {:8}\r\nSP {:8}\r\nCF {:8}\r\n",
        format!("{:02X}", instruction_pointer),
        format!("{:02X}", stack_pointer),
        format!("{:01b}", carry_flag as u8)
      );

      let realtime_tolerance = 0.01;
      print!("\r\n");
      print!(
        "{:32}\r\n",
        if realtime_ratio > realtime_tolerance {
          format!("Emulation behind by {:.0}%", realtime_ratio * 100.0)
        } else if realtime_ratio < -realtime_tolerance {
          format!("Emulation ahead by {:.0}%", -realtime_ratio * 100.0)
        } else {
          format!("Emulation on time")
        }
      );
      print!("{:32}\r\n", debug_status);

      input_ored = [memory[0x00]];
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

      start = std::time::Instant::now();
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
          input_ored = [input_ored[0x00] | memory[0x00]];
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
              // TODO
              0x0 | 0x1 => {
                // add

                let a = memory[stack_pointer as usize];
                memory[stack_pointer as usize] = 0x00;
                stack_pointer = stack_pointer.wrapping_add(1);
                let b = memory[size_pointer as usize];

                memory[size_pointer as usize] = (b as u16 + a as u16 + carry_flag as u16) as u8;
                carry_flag = (b as u16 + a as u16 + carry_flag as u16) > 0xFF;
              }

              // TODO
              0x2 | 0x3 => {
                // sub

                let a = memory[stack_pointer as usize];
                memory[stack_pointer as usize] = 0x00;
                stack_pointer = stack_pointer.wrapping_add(1);
                let b = memory[size_pointer as usize];

                memory[size_pointer as usize] = (b as i16 - a as i16 - carry_flag as i16) as u8;
                carry_flag = (b as i16 - a as i16 - carry_flag as i16) < 0x00;
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
              }

              0x7 => {
                // iff
                let a = memory[stack_pointer as usize];
                memory[stack_pointer as usize] = 0x00;
                stack_pointer = stack_pointer.wrapping_add(1);
                let b = memory[stack_pointer as usize];

                memory[stack_pointer as usize] = if carry_flag { a } else { b };
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
                  debug_status = format!("Unknown instruction: {:#04X}", instruction);
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
                // (carry and flags and stack)
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

                  0xE => {
                    // shl
                    let a = memory[stack_pointer as usize];

                    memory[stack_pointer as usize] = a.wrapping_shl(1) | (carry_flag as u8);
                    carry_flag = a & 0b10000000 != 0;
                  }

                  0xF => {
                    // shr
                    let a = memory[stack_pointer as usize];

                    memory[stack_pointer as usize] = a.wrapping_shr(1) | ((carry_flag as u8) << 7);
                    carry_flag = a & 0b00000001 != 0;
                  }

                  _ => {
                    debug_flag = true;
                    debug_status = format!("Unknown instruction: {:#04X}", instruction);
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

// https://en.wikipedia.org/wiki/Block_Elements

fn print_display(display_buffer: &[u8; 0x20]) {
  let mut display_buffer_string: String = "".to_string();
  let line_top: &str = "\u{25aa}                \u{25aa}\r\n";
  let line_bottom: &str = "\u{25aa}                \u{25aa}\r\n";
  let col_left: &str = " ";
  let col_right: &str = " ";

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
  print!("{}", display_buffer_string);
}

fn print_input(input_buffer: &[u8; 0x01]) {
  fn bit_to_str(input_buffer: &[u8; 0x01], bit: u8) -> &str {
    match input_buffer[0] >> bit & 0x01 {
      0b0 => "\u{2591}\u{2591}",
      0b1 => "\u{2588}\u{2588}",
      _ => unreachable!(),
    }
  }

  print!(
    "    {}      {}    \r\n",
    bit_to_str(input_buffer, 0),
    bit_to_str(input_buffer, 4),
  );
  print!(
    "  {}  {}  {}  {}  \r\n",
    bit_to_str(input_buffer, 2),
    bit_to_str(input_buffer, 3),
    bit_to_str(input_buffer, 6),
    bit_to_str(input_buffer, 7),
  );
  print!(
    "    {}      {}    \r\n",
    bit_to_str(input_buffer, 1),
    bit_to_str(input_buffer, 5),
  );
}

fn print_memory(memory: &[u8; 0x100]) {
  let mut memory_string: String = "".to_string();

  for y in 0..0x10 {
    for x in 0..0x10 {
      memory_string += &format!("{:02X} ", memory[(y << 0x04 | x) as usize]);
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
