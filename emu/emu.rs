fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 2 {
    println!("Usage: emu <memory image file>");
    std::process::exit(1);
  }

  let memory_image_file: &String = &args[1];

  let memory_image = std::fs::read(memory_image_file)
    .unwrap_or_else(|_| {
      println!("Error: Unable to read file: {}", memory_image_file);
      std::process::exit(1);
    })
    .try_into()
    .unwrap_or_else(|_| {
      println!("Error: Memory image has incorrect size");
      std::process::exit(1);
    });

  let mc = Microcomputer {
    mem: memory_image,
    mp: Microprocessor {
      sp: 0x00,
      ip: 0x00,
      cf: false,
    },
  };

  emulate(mc, 100000);
}

struct Microcomputer {
  mem: [u8; 0x100],   // memory
  mp: Microprocessor, // microprocessor
}

struct Microprocessor {
  sp: u8,   // stack pointer
  ip: u8,   // instruction pointer
  cf: bool, // carry flag
}

enum TickTrap {
  DebugRequest,
  UnknownInstruction(u8),
}

fn emulate(mut mc: Microcomputer, clock_speed: u128) {
  let mut start_time = std::time::Instant::now();
  let mut next_print_time = std::time::Instant::now();
  let mut current_clocks = 0;
  let mut status_line = "".to_string();
  let mut debug_mode = false;

  print!("\x1B[2J"); // clear screen

  // this call will switch termital to raw mode
  let input_channel = spawn_input_channel();

  loop {
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
        mc.mem[0x00] |= lo_nibble | (hi_nibble << 4);
      }
      Err(TryRecvError::Empty) => (),
      Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
    }

    if debug_mode {
      'until_valid: loop {
        match input_channel.recv() {
          Ok('\n') => {
            debug_mode = false;
            break 'until_valid;
          }
          Ok(' ') => {
            status_line = "Single Stepped.".to_string();
            break 'until_valid;
          }
          _ => continue 'until_valid,
        }
      }
      start_time = std::time::Instant::now();
    }

    let (clocks, tick_trap) = tick(&mut mc);
    current_clocks += clocks;

    if let Some(tick_trap) = tick_trap {
      debug_mode = true;
      status_line = match tick_trap {
        TickTrap::DebugRequest => format!("Debug Request."),
        TickTrap::UnknownInstruction(instruction) => {
          format!("Unknown instruction: {:#04X}", instruction)
        }
      };
    }

    let realtime = std::cmp::max(start_time.elapsed().as_millis(), 1); // prevent division by zero
    let realtime_offset = (1000 * current_clocks / clock_speed) as i128 - realtime as i128;
    let realtime_ratio = realtime_offset as f64 / realtime as f64;
    std::thread::sleep(std::time::Duration::from_millis(
      std::cmp::max(realtime_offset, 0) as u64,
    ));

    if !debug_mode {
      let realtime_tolerance = 0.01;
      status_line = if realtime_ratio < -realtime_tolerance {
        format!("Emulation behind by {:.0}%.", -realtime_ratio * 100.0)
      } else if realtime_ratio > realtime_tolerance {
        format!("Emulation ahead by {:.0}%.", realtime_ratio * 100.0)
      } else {
        format!("Emulation on time.")
      };
    }

    // print at most 30 times per second
    if next_print_time <= std::time::Instant::now() || debug_mode {
      next_print_time += std::time::Duration::from_millis(1000 / 30);

      print!("\x1B[1;1H"); // move cursor to top left
      print!("{}", mc);
      print!("\r\n");
      print!("{:32}\r\n", status_line);
    }
  }
}

fn tick(mc: &mut Microcomputer) -> (u128, Option<TickTrap>) {
  let mp = &mut mc.mp;

  let instruction: u8 = mc.mem[mp.ip as usize];
  mp.ip = mp.ip.wrapping_add(1);

  match (instruction & 0b10000000) >> 7 {
    0b0 => {
      // psh
      let immediate = instruction; // decode_immediate
      mp.sp = mp.sp.wrapping_sub(1);

      mc.mem[mp.sp as usize] = immediate;
      (0x04, None)
    }

    0b1 => {
      match (instruction & 0b01000000) >> 6 {
        0b0 => {
          // (arithmetic and logic)
          let size = 1 << (instruction & 0b00000011); // decode_size
          let opcode = (instruction & 0b00111100) >> 2;
          let size_pointer = mp.sp.wrapping_add(size);

          match opcode {
            0x0 => {
              // add
              let a = mc.mem[mp.sp as usize];
              mc.mem[mp.sp as usize] = 0x00;
              mp.sp = mp.sp.wrapping_add(1);
              let b = mc.mem[size_pointer as usize];

              mc.mem[size_pointer as usize] = (b as u16 + a as u16 + mp.cf as u16) as u8;
              mp.cf = (b as u16 + a as u16 + mp.cf as u16) > 0xFF;
              (0x04, None)
            }

            0x1 => {
              // sub
              let a = mc.mem[mp.sp as usize];
              mc.mem[mp.sp as usize] = 0x00;
              mp.sp = mp.sp.wrapping_add(1);
              let b = mc.mem[size_pointer as usize];

              mc.mem[size_pointer as usize] = (b as i16 - a as i16 - mp.cf as i16) as u8;
              mp.cf = (b as i16 - a as i16 - mp.cf as i16) < 0x00;
              (0x04, None)
            }

            0x4 => {
              // iff
              let a = mc.mem[mp.sp as usize];
              mc.mem[mp.sp as usize] = 0x00;
              mp.sp = mp.sp.wrapping_add(1);
              let b = mc.mem[size_pointer as usize];

              mc.mem[size_pointer as usize] = if mp.cf { a } else { b };
              (0x04, None)
            }

            0x5 => {
              // rot
              let a = mc.mem[mp.sp as usize];
              mc.mem[mp.sp as usize] = 0x00;
              mp.sp = mp.sp.wrapping_add(1);
              let b = mc.mem[size_pointer as usize];

              let shifted = if a as i8 >= 0 {
                (b as u16).wrapping_shl(a as u32)
              } else {
                (b as u16).wrapping_shr(a.wrapping_neg() as u32)
              };

              mc.mem[size_pointer as usize] = (shifted & 0xFF) as u8 | (shifted >> 8) as u8;
              (0x04, None)
            }

            0x8 => {
              // orr
              let a = mc.mem[mp.sp as usize];
              mc.mem[mp.sp as usize] = 0x00;
              mp.sp = mp.sp.wrapping_add(1);
              let b = mc.mem[size_pointer as usize];

              mc.mem[size_pointer as usize] = a | b;
              mp.cf = mc.mem[size_pointer as usize] == 0x00;
              (0x04, None)
            }

            0x9 => {
              // and
              let a = mc.mem[mp.sp as usize];
              mc.mem[mp.sp as usize] = 0x00;
              mp.sp = mp.sp.wrapping_add(1);
              let b = mc.mem[size_pointer as usize];

              mc.mem[size_pointer as usize] = a & b;
              mp.cf = mc.mem[size_pointer as usize] == 0x00;
              (0x04, None)
            }

            0xA => {
              // xor
              let a = mc.mem[mp.sp as usize];
              mc.mem[mp.sp as usize] = 0x00;
              mp.sp = mp.sp.wrapping_add(1);
              let b = mc.mem[size_pointer as usize];

              mc.mem[size_pointer as usize] = a ^ b;
              mp.cf = mc.mem[size_pointer as usize] == 0x00;
              (0x04, None)
            }

            0xB => {
              // xnd
              mc.mem[mp.sp as usize] = 0x00;
              mp.sp = mp.sp.wrapping_add(1);

              mc.mem[size_pointer as usize] = 0;
              mp.cf = mc.mem[size_pointer as usize] == 0x00;
              (0x04, None)
            }

            _ => match (opcode, instruction & 0b00000011) {
              // (size used as part of opcode)
              (0xC, 0b00) => {
                // inc
                let a = mc.mem[mp.sp as usize];
                mc.mem[mp.sp as usize] = a.wrapping_add(1);
                (0x04, None)
              }

              (0xC, 0b01) => {
                // dec
                let a = mc.mem[mp.sp as usize];
                mc.mem[mp.sp as usize] = a.wrapping_sub(1);
                (0x04, None)
              }

              (0xC, 0b10) => {
                // neg
                let a = mc.mem[mp.sp as usize];
                mc.mem[mp.sp as usize] = a.wrapping_neg();
                (0x04, None)
              }

              (0xC, 0b11) => {
                // adn
                let a = mc.mem[mp.sp as usize];
                mc.mem[mp.sp as usize] = 0x00;
                mp.sp = mp.sp.wrapping_add(1);
                let b = mc.mem[mp.sp as usize];

                mc.mem[mp.sp as usize] = (b.wrapping_add(a) & 0b00001111)
                  | ((b & 0b11110000).wrapping_add(a & 0b11110000));
                (0x04, None)
              }

              (0xD, 0b00) => {
                // shl
                let a = mc.mem[mp.sp as usize];

                mc.mem[mp.sp as usize] = a.wrapping_shl(1) | (mp.cf as u8);
                mp.cf = a & 0b10000000 != 0;
                (0x04, None)
              }

              (0xD, 0b01) => {
                // shr
                let a = mc.mem[mp.sp as usize];

                mc.mem[mp.sp as usize] = a.wrapping_shr(1) | ((mp.cf as u8) << 7);
                mp.cf = a & 0b00000001 != 0;
                (0x04, None)
              }

              (0xD, 0b10) => {
                // not
                let a = mc.mem[mp.sp as usize];

                mc.mem[mp.sp as usize] = !a;
                mp.cf = mc.mem[mp.sp as usize] == 0x00;
                (0x04, None)
              }

              (0xD, 0b11) => {
                // buf
                let a = mc.mem[mp.sp as usize];

                mc.mem[mp.sp as usize] = a;
                mp.cf = mc.mem[mp.sp as usize] == 0x00;
                (0x04, None)
              }

              (0b1110, 0b11) => {
                // dbg
                (0x00, Some(TickTrap::DebugRequest))
              }

              _ => (0x00, Some(TickTrap::UnknownInstruction(instruction))),
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
                  let offset = instruction & 0b00001111; // decode_offset
                  let offset_pointer = mp.sp.wrapping_add(offset);

                  mp.sp = mp.sp.wrapping_sub(1);
                  let a = mc.mem[offset_pointer as usize];

                  mc.mem[mp.sp as usize] = a;
                  (0x04, None)
                }

                0b1 => {
                  // sto
                  let a = mc.mem[mp.sp as usize];
                  mc.mem[mp.sp as usize] = 0x00;
                  mp.sp = mp.sp.wrapping_add(1);

                  let offset = instruction & 0b00001111; // decode_offset
                  let offset_pointer = mp.sp.wrapping_add(offset);

                  mc.mem[offset_pointer as usize] = a;
                  (0x04, None)
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
                      let a = mc.mem[mp.sp as usize];

                      mc.mem[mp.sp as usize] = mc.mem[a as usize];
                      (0x04, None)
                    }

                    0x1 => {
                      // sta
                      let a = mc.mem[mp.sp as usize];
                      mc.mem[mp.sp as usize] = 0x00;
                      mp.sp = mp.sp.wrapping_add(1);
                      let b = mc.mem[mp.sp as usize];
                      mc.mem[mp.sp as usize] = 0x00;
                      mp.sp = mp.sp.wrapping_add(1);

                      mc.mem[b as usize] = a;
                      (0x04, None)
                    }

                    0x2 => {
                      // ldi
                      mp.sp = mp.sp.wrapping_sub(1);
                      mc.mem[mp.sp as usize] = mp.ip;
                      (0x04, None)
                    }

                    0x3 => {
                      // sti
                      mp.ip = mc.mem[mp.sp as usize];
                      mc.mem[mp.sp as usize] = 0x00;
                      mp.sp = mp.sp.wrapping_add(1);
                      (0x04, None)
                    }

                    0x4 => {
                      // lds
                      let a = mp.sp;
                      mp.sp = mp.sp.wrapping_sub(1);

                      mc.mem[mp.sp as usize] = a;
                      (0x04, None)
                    }

                    0x5 => {
                      // sts
                      let a = mc.mem[mp.sp as usize];
                      mc.mem[mp.sp as usize] = 0x00;

                      mp.sp = a;
                      (0x04, None)
                    }

                    0x8 => {
                      // nop
                      (0x04, None)
                    }

                    0x9 => {
                      // clc
                      mp.cf = false;
                      (0x04, None)
                    }

                    0xA => {
                      // sec
                      mp.cf = true;
                      (0x04, None)
                    }

                    0xB => {
                      // flc
                      mp.cf = !mp.cf;
                      (0x04, None)
                    }

                    0xC => {
                      // swp
                      let a = mc.mem[mp.sp as usize];
                      mp.sp = mp.sp.wrapping_add(1);
                      let b = mc.mem[mp.sp as usize];

                      mc.mem[mp.sp as usize] = a;
                      mp.sp = mp.sp.wrapping_sub(1);
                      mc.mem[mp.sp as usize] = b;
                      (0x04, None)
                    }

                    0xD => {
                      // pop
                      mc.mem[mp.sp as usize] = 0x00;
                      mp.sp = mp.sp.wrapping_add(1);
                      (0x04, None)
                    }

                    _ => (0x04, Some(TickTrap::UnknownInstruction(instruction))),
                  }
                }

                0b1 => {
                  // phn
                  let immediate = instruction; // decode_immediate
                  mp.sp = mp.sp.wrapping_sub(1);

                  mc.mem[mp.sp as usize] = immediate;
                  (0x04, None)
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

    _ => unreachable!(),
  }
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

impl std::fmt::Display for Microcomputer {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut fmt: String = "".to_string();

    let display_buffer: &[u8; 0x20] = &self.mem[0xE0..0x100].try_into().unwrap();

    // https://en.wikipedia.org/wiki/Block_Elements
    let line_top: &str = "\u{25aa}                \u{25aa}\r\n";
    let line_bottom: &str = "\u{25aa}                \u{25aa}\r\n";
    let col_left: &str = " ";
    let col_right: &str = " ";

    fmt += &line_top;
    for y in (0..0x10).step_by(2) {
      fmt += &col_left;
      for x in 0..0x10 {
        let mut pixel_pair = 0;
        for y2 in 0..2 {
          let address: u8 = (x >> 0x03) | ((y + y2) << 0x01);
          let pixel = display_buffer[address as usize] >> (0x07 - (x & 0x07)) & 0x01;
          pixel_pair |= pixel << y2;
        }
        fmt += match pixel_pair {
          0b00 => " ",
          0b01 => "\u{2580}",
          0b10 => "\u{2584}",
          0b11 => "\u{2588}",
          _ => "?",
        };
      }
      fmt += &col_right;
      fmt += "\r\n";
    }

    fmt += &line_bottom;
    fmt += "\r\n";

    let input_buffer: &[u8; 0x01] = &self.mem[0x00..0x01].try_into().unwrap();

    fn bit_to_str(input_buffer: &[u8; 0x01], bit: u8) -> &str {
      match input_buffer[0] >> bit & 0x01 {
        0b0 => "\u{2591}\u{2591}",
        0b1 => "\u{2588}\u{2588}",
        _ => unreachable!(),
      }
    }

    fmt += &format!(
      "    {}      {}    \r\n",
      bit_to_str(input_buffer, 0),
      bit_to_str(input_buffer, 4),
    );
    fmt += &format!(
      "  {}  {}  {}  {}  \r\n",
      bit_to_str(input_buffer, 2),
      bit_to_str(input_buffer, 3),
      bit_to_str(input_buffer, 6),
      bit_to_str(input_buffer, 7),
    );
    fmt += &format!(
      "    {}      {}    \r\n",
      bit_to_str(input_buffer, 1),
      bit_to_str(input_buffer, 5),
    );

    fmt += "\r\n";
    fmt += "MEM\r\n";

    for y in 0..0x10 {
      for x in 0..0x10 {
        fmt += &format!("{:02X} ", self.mem[(y << 0x04 | x) as usize]);
      }
      fmt += "\r\n";
    }

    fmt += "\r\n";
    fmt += &format!("{}", self.mp);

    write!(f, "{}", fmt)
  }
}

impl std::fmt::Display for Microprocessor {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "IP  SP  CF\r\n{:02X}  {:02X}  {:02X}\r\n",
      self.ip, self.sp, self.cf as u8,
    )
  }
}
