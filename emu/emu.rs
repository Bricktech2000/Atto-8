fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 2 {
    println!("Emu: Usage: emu <memory image file>");
    std::process::exit(1);
  }

  let memory_image_file: &String = &args[1];

  let image = std::fs::read(memory_image_file)
    .unwrap_or_else(|_| {
      println!("Emu: Error: Unable to read file `{}`", memory_image_file);
      std::process::exit(1);
    })
    .try_into()
    .unwrap_or_else(|_| {
      println!(
        "Emu: Error: Memory image `{}` has incorrect size",
        memory_image_file,
      );
      std::process::exit(1);
    });

  let mc = Microcomputer {
    mem: image,
    stdin: 0x00,
    stdout: 0x00,
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
  stdin: u8,          // standard input
  stdout: u8,         // standard output
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
  let mut stdout = "".to_string();
  let mut debug_mode = false;

  // this call will switch the termital to raw mode
  let input_channel = spawn_input_channel();

  mc.stdin = mc.mem[0x00];
  mc.mem[0x00] = 0x00; // d-pad

  loop {
    if debug_mode {
      'until_valid: loop {
        match input_channel.recv() {
          Ok(console::Key::Escape) => {
            debug_mode = false;
            break 'until_valid;
          }

          Ok(console::Key::Tab) => {
            status_line = "Single stepped".to_string();
            break 'until_valid;
          }

          _ => continue 'until_valid,
        }
      }

      // conceptually hacky but does the job
      start_time = std::time::Instant::now();
      current_clocks = 0;
    }

    use std::sync::mpsc::TryRecvError;
    match input_channel.try_recv() {
      Ok(console::Key::Escape) => {
        debug_mode = true;
        status_line = "Force debug".to_string();
      }

      Ok(key) => {
        let d_pad_lo = match key {
          console::Key::ArrowUp => 0b0001,
          console::Key::ArrowDown => 0b0010,
          console::Key::ArrowLeft => 0b0100,
          console::Key::ArrowRight => 0b1000,
          _ => 0b0000,
        };
        let d_pad_hi = match key {
          console::Key::PageUp => 0b0001,
          console::Key::PageDown => 0b0010,
          console::Key::Home => 0b0100,
          console::Key::End => 0b1000,
          _ => 0b0000,
        };

        mc.mem[0x00] = d_pad_lo | (d_pad_hi << 4);
        mc.stdin = match key {
          console::Key::Char(c) => c as u8,
          console::Key::Enter => 0x0A,
          console::Key::Backspace => 0x08,
          _ => 0x00,
        };
      }

      Err(TryRecvError::Empty) => (),
      Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
    }

    let (clocks, tick_trap) = tick(&mut mc);
    current_clocks += clocks;

    if let Some(tick_trap) = tick_trap {
      debug_mode = true;
      status_line = match tick_trap {
        TickTrap::DebugRequest => format!("Debug request"),
        TickTrap::UnknownInstruction(instruction) => {
          format!("Unknown instruction `{:#04X}`", instruction)
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
      status_line = if -realtime_ratio > realtime_tolerance {
        format!("Emulation behind by {:.0}%", -realtime_ratio * 100.0)
      } else if realtime_ratio > realtime_tolerance {
        format!("Emulation ahead by {:.0}%", realtime_ratio * 100.0)
      } else {
        format!("Emulation on time")
      };
    }

    if mc.stdout != 0x00 {
      stdout.push(mc.stdout as char);
      mc.stdout = 0x00;
    }

    // print at most 30 times per second
    if next_print_time <= std::time::Instant::now() || debug_mode {
      next_print_time += std::time::Duration::from_millis(1000 / 30);

      print!("\x1B[2J"); // clear screen
      print!("\x1B[1;1H"); // move cursor to top left
      print!("{}", mc);
      print!("\r\n");
      print!("{:32}\r\n", status_line);
      print!("\r\n");
      print!("_______________________________________________\r\n");
      print!("{}", stdout);
      use std::io::Write;
      std::io::stdout().flush().unwrap();
    }
  }
}

fn tick(mc: &mut Microcomputer) -> (u128, Option<TickTrap>) {
  let mp = &mut mc.mp;

  macro_rules! mem_read {
    ($address:expr) => {{
      let address = $address;
      if address == 0x00 {
        if mc.stdin != 0x00 {
          let stdin = mc.stdin;
          mc.stdin = 0x00;
          stdin
        } else {
          mc.mem[0x00] // d-pad
        }
      } else {
        mc.mem[address as usize]
      }
    }};
  }

  macro_rules! mem_write {
    ($address:expr, $value:expr) => {{
      let address = $address;
      let value = $value;
      if address == 0x00 {
        mc.stdout = value;
      } else {
        mc.mem[address as usize] = value;
      }
    }};
  }

  macro_rules! push {
    ($value:expr) => {{
      let value = $value;
      mp.sp = mp.sp.wrapping_sub(1);
      mem_write!(mp.sp, value);
    }};
  }

  macro_rules! pop {
    () => {{
      let value = mem_read!(mp.sp);
      mp.sp = mp.sp.wrapping_add(1);
      value
    }};
  }

  let instruction = mem_read!(mp.ip);
  mp.ip = mp.ip.wrapping_add(1);

  match (instruction & 0b10000000) >> 7 {
    0b0 => {
      // psh
      let immediate = instruction; // decode_immediate
      push!(immediate);
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
              let a = pop!();
              let b = mem_read!(size_pointer);
              mem_write!(size_pointer, (b as u16 + a as u16 + mp.cf as u16) as u8);
              mp.cf = (b as u16 + a as u16 + mp.cf as u16) > 0xFF;
              (0x04, None)
            }

            0x1 => {
              // sub
              let a = pop!();
              let b = mem_read!(size_pointer);
              mem_write!(size_pointer, (b as i16 - a as i16 - mp.cf as i16) as u8);
              mp.cf = (b as i16 - a as i16 - mp.cf as i16) < 0x00;
              (0x04, None)
            }

            0x4 => {
              // iff
              let a = pop!();
              let b = mem_read!(size_pointer);
              mem_write!(size_pointer, if mp.cf { a } else { b });
              (0x04, None)
            }

            0x5 => {
              // rot
              let a = pop!();
              let b = mem_read!(size_pointer);
              let shifted = if a as i8 >= 0 {
                (b as u16).wrapping_shl(a as u32)
              } else {
                (b as u16).wrapping_shr(a.wrapping_neg() as u32)
              };
              mem_write!(size_pointer, (shifted & 0xFF) as u8 | (shifted >> 8) as u8);
              (0x04, None)
            }

            0x8 => {
              // orr
              let a = pop!();
              let b = mem_read!(size_pointer);
              mem_write!(size_pointer, a | b);
              mp.cf = mem_read!(size_pointer) == 0x00;
              (0x04, None)
            }

            0x9 => {
              // and
              let a = pop!();
              let b = mem_read!(size_pointer);
              mem_write!(size_pointer, a & b);
              mp.cf = mem_read!(size_pointer) == 0x00;
              (0x04, None)
            }

            0xA => {
              // xor
              let a = pop!();
              let b = mem_read!(size_pointer);
              mem_write!(size_pointer, a ^ b);
              mp.cf = mem_read!(size_pointer) == 0x00;
              (0x04, None)
            }

            0xB => {
              // xnd
              let _ = pop!();
              mem_write!(size_pointer, 0x00);
              mp.cf = mem_read!(size_pointer) == 0x00;
              (0x04, None)
            }

            _ => match (opcode, instruction & 0b00000011) {
              // (size used as part of opcode)
              (0xC, 0b00) => {
                // inc
                push!(pop!().wrapping_add(1));
                (0x04, None)
              }

              (0xC, 0b01) => {
                // dec
                push!(pop!().wrapping_sub(1));
                (0x04, None)
              }

              (0xC, 0b10) => {
                // neg
                push!(pop!().wrapping_neg());
                (0x04, None)
              }

              (0xD, 0b00) => {
                // shl
                let a = pop!();
                push!(a.wrapping_shl(1) | (mp.cf as u8));
                mp.cf = a & 0b10000000 != 0x00;
                (0x04, None)
              }

              (0xD, 0b01) => {
                // shr
                let a = pop!();
                push!(a.wrapping_shr(1) | (mp.cf as u8) << 7);
                mp.cf = a & 0b00000001 != 0x00;
                (0x04, None)
              }

              (0xD, 0b10) => {
                // not
                push!(!pop!());
                mp.cf = mem_read!(mp.sp) == 0x00;
                (0x04, None)
              }

              (0xD, 0b11) => {
                // buf
                push!(pop!());
                mp.cf = mem_read!(mp.sp) == 0x00;
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
                  push!(mem_read!(offset_pointer));
                  (0x04, None)
                }

                0b1 => {
                  // sto
                  let offset = instruction & 0b00001111; // decode_offset
                  let offset_pointer = mp.sp.wrapping_add(offset).wrapping_add(1);
                  mem_write!(offset_pointer, pop!());
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
                      push!(mem_read!(pop!()));
                      (0x04, None)
                    }

                    0x1 => {
                      // sta
                      let a = pop!();
                      let b = pop!();
                      mem_write!(b, a);
                      (0x04, None)
                    }

                    0x2 => {
                      // ldi
                      push!(mp.ip);
                      (0x04, None)
                    }

                    0x3 => {
                      // sti
                      mp.ip = pop!();
                      (0x04, None)
                    }

                    0x4 => {
                      // lds
                      push!(mp.sp);
                      (0x04, None)
                    }

                    0x5 => {
                      // sts
                      mp.sp = pop!();
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
                      let a = pop!();
                      let b = pop!();
                      push!(a);
                      push!(b);
                      (0x04, None)
                    }

                    0xD => {
                      // pop
                      let _ = pop!();
                      (0x04, None)
                    }

                    _ => (0x04, Some(TickTrap::UnknownInstruction(instruction))),
                  }
                }

                0b1 => {
                  // phn
                  let immediate = instruction; // decode_immediate
                  push!(immediate);
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
fn spawn_input_channel() -> Receiver<console::Key> {
  let stdout = console::Term::stdout();

  let (tx, rx) = mpsc::channel::<console::Key>();
  std::thread::spawn(move || loop {
    if let Ok(key) = stdout.read_key() {
      tx.send(key).unwrap();
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
          _ => unreachable!(),
        };
      }
      fmt += &col_right;
      fmt += "\r\n";
    }

    fmt += &line_bottom;
    fmt += "\r\n";

    fn bit_to_str(d_pad: u8, bit: u8) -> &'static str {
      match d_pad >> bit & 0x01 {
        0b0 => "\u{2591}\u{2591}",
        0b1 => "\u{2588}\u{2588}",
        _ => unreachable!(),
      }
    }

    let d_pad = self.mem[0x00];

    fmt += &format!(
      "    {}      {}    \r\n",
      bit_to_str(d_pad, 0),
      bit_to_str(d_pad, 4),
    );
    fmt += &format!(
      "  {}  {}  {}  {}  \r\n",
      bit_to_str(d_pad, 2),
      bit_to_str(d_pad, 3),
      bit_to_str(d_pad, 6),
      bit_to_str(d_pad, 7),
    );
    fmt += &format!(
      "    {}      {}    \r\n",
      bit_to_str(d_pad, 1),
      bit_to_str(d_pad, 5),
    );

    fmt += "\r\n";
    fmt += "MEM\r\n";

    for y in 0..0x10 {
      for x in 0..0x10 {
        let address: u8 = (y << 0x04 | x) as u8;
        fmt += &format!(
          "{:02X}{}",
          self.mem[address as usize],
          if address == self.mp.sp.wrapping_sub(1) {
            if self.mp.cf {
              "/"
            } else {
              "|"
            }
          } else if address == self.mp.ip.wrapping_sub(1) {
            "["
          } else if address == self.mp.ip {
            "]"
          } else {
            " "
          }
        );
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
      "IP  SP  CF\r\n{:02X}  {:02X}   {:01X}\r\n",
      self.ip, self.sp, self.cf as u8,
    )
  }
}
