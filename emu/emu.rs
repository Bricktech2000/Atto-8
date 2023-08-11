fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 2 {
    println!("Emu: Usage: emu <memory image file>");
    std::process::exit(1);
  }

  let memory_image_file: &String = &args[1];

  let memory_image = std::fs::read(memory_image_file)
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
    mem: memory_image,
    stdin: 0x00,
    stdout: 0x00,
    mp: Microprocessor {
      ip: 0x00,
      sp: 0x00,
      cf: false,
    },
  };

  emulate(mc, 100000);
}

const MEM_SIZE: usize = 0x100;

struct Microcomputer {
  mem: [u8; MEM_SIZE], // memory
  stdin: u8,           // standard input
  stdout: u8,          // standard output
  mp: Microprocessor,  // microprocessor
}

struct Microprocessor {
  ip: u8,   // instruction pointer
  sp: u8,   // stack pointer
  cf: bool, // carry flag
}

#[allow(dead_code)]
enum TickTrap {
  DebugRequest,
  MicrocodeFault,
  IllegalOpcode(u8),
}

fn emulate(mut mc: Microcomputer, clock_speed: u128) {
  use std::collections::VecDeque;

  let mut start_time = std::time::Instant::now();
  let mut next_print_time = std::time::Instant::now();
  let mut current_clocks = 0;
  let mut status_line = "".to_string();
  let mut stdout_string = "".to_string();
  let mut stdin_queue = VecDeque::new();
  let mut controller_input = [None; 8];
  let mut debug_mode = false;
  let mut show_state = false;

  // this call will switch the termital to raw mode
  let input_channel = spawn_input_channel();

  mc.stdin = mc.mem[0x00];
  mc.mem[0x00] = 0x00; // controller input

  loop {
    if debug_mode {
      'until_valid: loop {
        match input_channel.recv() {
          Ok(console::Key::Del) => {
            stdout_string = "".to_string();
            break 'until_valid;
          }

          Ok(console::Key::Tab) => {
            status_line = "Single stepped".to_string();
            break 'until_valid;
          }

          Ok(console::Key::Escape) => {
            debug_mode = !debug_mode;
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
      Ok(console::Key::Del) => {
        stdout_string = "".to_string();
      }

      Ok(console::Key::Tab) => {
        show_state = !show_state;
      }

      Ok(console::Key::Escape) => {
        debug_mode = !debug_mode;
        status_line = "Force debug".to_string();
      }

      Ok(key) => {
        let keys = [
          console::Key::ArrowUp,
          console::Key::ArrowDown,
          console::Key::ArrowLeft,
          console::Key::ArrowRight,
          console::Key::PageUp,
          console::Key::PageDown,
          console::Key::Home,
          console::Key::End,
        ];

        controller_input = keys
          .iter()
          .map(|k| (k == &key).then_some(std::time::Instant::now()))
          .zip(controller_input.iter())
          .map(|(next, curr)| next.or(*curr))
          .collect::<Vec<_>>()
          .try_into()
          .unwrap();

        stdin_queue.push_back(match key {
          console::Key::Char(c) => c as u8,
          console::Key::Backspace => 0x08,
          console::Key::Enter => 0x0A,
          console::Key::Tab => 0x09,
          console::Key::Del => 0x7F,
          _ => 0x00,
        });
      }

      Err(TryRecvError::Empty) => (),
      Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
    }

    match tick(&mut mc) {
      Ok(clocks) => {
        current_clocks += clocks;
      }
      Err(tick_trap) => {
        debug_mode = true;
        status_line = match tick_trap {
          TickTrap::DebugRequest => format!("Debug request"),
          TickTrap::MicrocodeFault => format!("Microcode fault"),
          TickTrap::IllegalOpcode(instruction) => {
            format!("Illegal opcode `{:02X}`", instruction)
          }
        }
      }
    };

    let timestamp_threshold = std::time::Duration::from_millis(200);
    controller_input = controller_input
      .iter()
      .map(|timestamp| timestamp.and_then(|t| (t.elapsed() < timestamp_threshold).then_some(t)))
      .collect::<Vec<_>>()
      .try_into()
      .unwrap();

    mc.mem[0x00] = controller_input
      .iter()
      .enumerate()
      .fold(0x00, |acc, (index, timestamp)| {
        acc | ((timestamp.is_some() as u8) << index)
      });

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

    // was stdout written to?
    if mc.stdout != 0x00 {
      stdout_string.push(mc.stdout as char);
      mc.stdout = 0x00;
    }

    // was stdin read from?
    if mc.stdin == 0x00 {
      mc.stdin = stdin_queue.pop_front().unwrap_or(0x00);
    }

    // print at most 30 times per second
    if next_print_time <= std::time::Instant::now() || debug_mode {
      next_print_time += std::time::Duration::from_millis(1000 / 30);

      print!("\x1B[2J"); // clear screen
      print!("\x1B[1;1H"); // move cursor to top left
      print!("{}\r\n", status_line);
      print!("\r\n");
      if show_state || debug_mode {
        print!("{}", mc);
      } else {
        print!(
          "{}\r\n{}",
          render_display_buffer(&mc.mem[0xE0..0x100].try_into().unwrap()),
          render_controller_input(&mc.mem[0x00..0x01].try_into().unwrap())
        );
      }
      print!("\r\n");
      print!("{}", stdout_string);
      use std::io::Write;
      std::io::stdout().flush().unwrap();
    }
  }
}

fn tick(mc: &mut Microcomputer) -> Result<u128, TickTrap> {
  let mp = &mut mc.mp;

  macro_rules! mem_read {
    ($address:expr) => {{
      let address = $address;
      if address == 0x00 {
        // was stdin written to?
        if mc.stdin != 0x00 {
          let stdin = mc.stdin;
          mc.stdin = 0x00;
          stdin
        } else {
          mc.mem[0x00] // controller input
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
        // was stdout read from?
        if mc.stdout == 0x00 {
          mc.stdout = value;
        } else {
          panic!("attempt to write to `stdout` more than once within one tick");
        }
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
      let imm = instruction; // decode_imm
      push!(imm);
      Ok(4)
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
              let sum = b as u16 + a as u16 + mp.cf as u16;
              mem_write!(size_pointer, sum as u8);
              mp.cf = sum > 0xFF;
              Ok(4)
            }

            0x1 => {
              // sub
              let a = pop!();
              let b = mem_read!(size_pointer);
              let diff = b as i16 - a as i16 - mp.cf as i16;
              mem_write!(size_pointer, diff as u8);
              mp.cf = diff < 0x00;
              Ok(4)
            }

            0x4 => {
              // iff
              let a = pop!();
              let b = mem_read!(size_pointer);
              mem_write!(size_pointer, if mp.cf { a } else { b });
              Ok(4)
            }

            0x5 => {
              // rot
              let a = pop!();
              let b = mem_read!(size_pointer);
              let shifted = (b as u16) << a % 8;
              mem_write!(size_pointer, (shifted & 0xFF) as u8 | (shifted >> 8) as u8);
              Ok(4)
            }

            0x8 => {
              // orr
              let a = pop!();
              let b = mem_read!(size_pointer);
              mem_write!(size_pointer, a | b);
              mp.cf = mem_read!(size_pointer) == 0x00;
              Ok(4)
            }

            0x9 => {
              // and
              let a = pop!();
              let b = mem_read!(size_pointer);
              mem_write!(size_pointer, a & b);
              mp.cf = mem_read!(size_pointer) == 0x00;
              Ok(4)
            }

            0xA => {
              // xor
              let a = pop!();
              let b = mem_read!(size_pointer);
              mem_write!(size_pointer, a ^ b);
              mp.cf = mem_read!(size_pointer) == 0x00;
              Ok(4)
            }

            0xB => {
              // xnd
              let _ = pop!();
              mem_write!(size_pointer, 0x00);
              mp.cf = mem_read!(size_pointer) == 0x00;
              Ok(4)
            }

            _ => match (opcode, instruction & 0b00000011) {
              // (size used as part of opcode)
              (0xC, 0b00) => {
                // inc
                push!(pop!().wrapping_add(1));
                Ok(4)
              }

              (0xC, 0b01) => {
                // dec
                push!(pop!().wrapping_sub(1));
                Ok(4)
              }

              (0xC, 0b10) => {
                // neg
                push!(pop!().wrapping_neg());
                Ok(4)
              }

              (0xD, 0b00) => {
                // shl
                let a = pop!();
                push!(a.wrapping_shl(1) | (mp.cf as u8));
                mp.cf = a & 0b10000000 != 0x00;
                Ok(4)
              }

              (0xD, 0b01) => {
                // shr
                let a = pop!();
                push!(a.wrapping_shr(1) | (mp.cf as u8) << 7);
                mp.cf = a & 0b00000001 != 0x00;
                Ok(4)
              }

              (0xD, 0b10) => {
                // not
                push!(!pop!());
                mp.cf = mem_read!(mp.sp) == 0x00;
                Ok(4)
              }

              (0xD, 0b11) => {
                // buf
                push!(pop!());
                mp.cf = mem_read!(mp.sp) == 0x00;
                Ok(4)
              }

              (0b1110, 0b11) => {
                // dbg
                Err(TickTrap::DebugRequest)
              }

              _ => Err(TickTrap::IllegalOpcode(instruction)),
            },
          }
        }

        0b1 => {
          match (instruction & 0b00100000) >> 5 {
            0b0 => {
              // (offset operations)
              let ofst = instruction & 0b00001111; // decode_ofst

              match (instruction & 0b00010000) >> 4 {
                0b0 => {
                  // ldo
                  let ofst_pointer = mp.sp.wrapping_add(ofst);
                  push!(mem_read!(ofst_pointer));
                  Ok(4)
                }

                0b1 => {
                  // sto
                  let ofst_pointer = mp.sp.wrapping_add(ofst).wrapping_add(1);
                  mem_write!(ofst_pointer, pop!());
                  Ok(4)
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
                      Ok(4)
                    }

                    0x1 => {
                      // sta
                      mem_write!(pop!(), pop!());
                      Ok(4)
                    }

                    0x2 => {
                      // ldi
                      push!(mp.ip);
                      Ok(4)
                    }

                    0x3 => {
                      // sti
                      mp.ip = pop!();
                      Ok(4)
                    }

                    0x4 => {
                      // lds
                      push!(mp.sp);
                      Ok(4)
                    }

                    0x5 => {
                      // sts
                      mp.sp = pop!();
                      Ok(4)
                    }

                    0x8 => {
                      // nop
                      Ok(4)
                    }

                    0x9 => {
                      // clc
                      mp.cf = false;
                      Ok(4)
                    }

                    0xA => {
                      // sec
                      mp.cf = true;
                      Ok(4)
                    }

                    0xB => {
                      // flc
                      mp.cf = !mp.cf;
                      Ok(4)
                    }

                    0xC => {
                      // swp
                      let a = pop!();
                      let b = pop!();
                      push!(a);
                      push!(b);
                      Ok(4)
                    }

                    0xD => {
                      // pop
                      let _ = pop!();
                      Ok(4)
                    }

                    _ => Err(TickTrap::IllegalOpcode(instruction)),
                  }
                }

                0b1 => {
                  // phn
                  let imm = instruction; // decode_imm
                  push!(imm);
                  Ok(4)
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

impl std::fmt::Display for Microcomputer {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}\r\n{}\r\n{}\r\n{}",
      self.mp,
      render_memory(&self.mem, self.mp.ip, self.mp.sp, self.mp.cf),
      render_display_buffer(self.mem[0xE0..0x100].try_into().unwrap()),
      render_controller_input(self.mem[0x00..0x01].try_into().unwrap())
    )
  }
}

impl std::fmt::Display for Microprocessor {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "IP  SP  CF\r\n{:02X}  {:02X}  {:01X} \r\n",
      self.ip, self.sp, self.cf as u8,
    )
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

fn render_memory(memory: &[u8; MEM_SIZE], ip: u8, sp: u8, cf: bool) -> String {
  let mut fmt: String = "".to_string();

  fmt += "MEM\r\n";
  for y in 0..0x10 {
    for x in 0..0x10 {
      let address: u8 = (y << 0x04 | x) as u8;
      fmt += &format!(
        "{:02X}{}",
        memory[address as usize],
        if address == sp.wrapping_sub(1) {
          if cf {
            "/"
          } else {
            "|"
          }
        } else if address == ip.wrapping_sub(1) {
          "["
        } else if address == ip {
          "]"
        } else {
          " "
        }
      );
    }
    fmt += "\r\n";
  }

  fmt
}

fn render_display_buffer(display_buffer: &[u8; 0x20]) -> String {
  let mut fmt = "".to_string();

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

  fmt
}

fn render_controller_input(controller_input: &[u8; 0x01]) -> String {
  let mut fmt = "".to_string();

  fn bit_to_str(controller_input: &[u8; 0x01], bit: u8) -> &'static str {
    match controller_input[0x00] >> bit & 0x01 {
      0b0 => "\u{2591}\u{2591}",
      0b1 => "\u{2588}\u{2588}",
      _ => unreachable!(),
    }
  }

  fmt += &format!(
    "    {}      {}    \r\n",
    bit_to_str(controller_input, 0),
    bit_to_str(controller_input, 4),
  );
  fmt += &format!(
    "  {}  {}  {}  {}  \r\n",
    bit_to_str(controller_input, 2),
    bit_to_str(controller_input, 3),
    bit_to_str(controller_input, 6),
    bit_to_str(controller_input, 7),
  );
  fmt += &format!(
    "    {}      {}    \r\n",
    bit_to_str(controller_input, 1),
    bit_to_str(controller_input, 5),
  );

  fmt
}
