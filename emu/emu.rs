use std::collections::VecDeque;

#[path = "../misc/common/common.rs"]
mod common;
use common::{TickTrap, Tickable};

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
    mp: Microprocessor {
      ip: 0x00,
      sp: 0x00,
      cf: false,
    },
  };

  common::execute(mc, 1000000);
}

struct Microcomputer {
  mem: [u8; common::MEM_SIZE], // memory
  mp: Microprocessor,          // microprocessor
}

struct Microprocessor {
  ip: u8,   // instruction pointer
  sp: u8,   // stack pointer
  cf: bool, // carry flag
}

impl Tickable for Microcomputer {
  fn reset(
    &mut self,
    stdin: &mut VecDeque<u8>,
    _stdout: &mut VecDeque<u8>,
    display: &mut [u8; 0x20],
    _controller: &mut u8,
  ) {
    stdin.push_back(self.mem[0x00]);
    display.copy_from_slice(&self.mem[common::DISPLAY_BUFFER as usize..]);
  }

  fn tick(
    &mut self,
    stdin: &mut VecDeque<u8>,
    stdout: &mut VecDeque<u8>,
    display: &mut [u8; 0x20],
    controller: &mut u8,
  ) -> Result<u128, TickTrap> {
    let mp = &mut self.mp;

    macro_rules! mem_read {
      ($address:expr) => {{
        let address = $address;
        if address == 0x00 {
          stdin.pop_front().unwrap_or(*controller)
        } else {
          self.mem[address as usize]
        }
      }};
    }

    macro_rules! mem_write {
      ($address:expr, $value:expr) => {{
        let address = $address;
        let value = $value;
        if address == 0x00 {
          stdout.push_back(value);
        }
        if address as usize & common::DISPLAY_BUFFER == common::DISPLAY_BUFFER {
          display[address as usize & !common::DISPLAY_BUFFER] = value
        }
        self.mem[address as usize] = value;
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
        Ok(10)
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
                Ok(16)
              }

              0x1 => {
                // sub
                let a = pop!();
                let b = mem_read!(size_pointer);
                let diff = b as i16 - a as i16 - mp.cf as i16;
                mem_write!(size_pointer, diff as u8);
                mp.cf = diff < 0x00;
                Ok(16)
              }

              0x4 => {
                // iff
                let a = pop!();
                let b = mem_read!(size_pointer);
                mem_write!(size_pointer, if mp.cf { a } else { b });
                Ok(15)
              }

              0x5 => {
                // rot
                let a = pop!();
                let b = mem_read!(size_pointer);
                let shifted = (b as u16) << a % 8;
                mem_write!(size_pointer, (shifted & 0xFF) as u8 | (shifted >> 8) as u8);
                mp.cf = false;
                Ok(19 * (a as u128 + 1))
              }

              0x8 => {
                // orr
                let a = pop!();
                let b = mem_read!(size_pointer);
                mem_write!(size_pointer, a | b);
                mp.cf = mem_read!(size_pointer) == 0x00;
                Ok(16)
              }

              0x9 => {
                // and
                let a = pop!();
                let b = mem_read!(size_pointer);
                mem_write!(size_pointer, a & b);
                mp.cf = mem_read!(size_pointer) == 0x00;
                Ok(13)
              }

              0xA => {
                // xor
                let a = pop!();
                let b = mem_read!(size_pointer);
                mem_write!(size_pointer, a ^ b);
                mp.cf = mem_read!(size_pointer) == 0x00;
                Ok(24)
              }

              0xB => {
                // xnd
                let _ = pop!();
                mem_write!(size_pointer, 0x00);
                mp.cf = mem_read!(size_pointer) == 0x00;
                Ok(9)
              }

              _ => match (opcode, instruction & 0b00000011) {
                // (size used as part of opcode)
                (0xC, 0b00) => {
                  // inc
                  push!(pop!().wrapping_add(1));
                  Ok(6)
                }

                (0xC, 0b01) => {
                  // dec
                  push!(pop!().wrapping_sub(1));
                  Ok(8)
                }

                (0xC, 0b10) => {
                  // neg
                  push!(pop!().wrapping_neg());
                  Ok(11)
                }

                (0xD, 0b00) => {
                  // shl
                  let a = pop!();
                  push!(a.wrapping_shl(1) | (mp.cf as u8));
                  mp.cf = a & 0b10000000 != 0x00;
                  Ok(9)
                }

                (0xD, 0b01) => {
                  // shr
                  let a = pop!();
                  push!(a.wrapping_shr(1) | (mp.cf as u8) << 7);
                  mp.cf = a & 0b00000001 != 0x00;
                  Ok(16)
                }

                (0xD, 0b10) => {
                  // not
                  push!(!pop!());
                  mp.cf = mem_read!(mp.sp) == 0x00;
                  Ok(8)
                }

                (0xD, 0b11) => {
                  // buf
                  push!(pop!());
                  mp.cf = mem_read!(mp.sp) == 0x00;
                  Ok(9)
                }

                (0b1110, 0b11) => {
                  // dbg
                  Err(TickTrap::DebugRequest)
                }

                _ => Err(TickTrap::IllegalOpcode),
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
                    Ok(14)
                  }

                  0b1 => {
                    // sto
                    let ofst_pointer = mp.sp.wrapping_add(ofst).wrapping_add(1);
                    mem_write!(ofst_pointer, pop!());
                    Ok(13)
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
                        Ok(9)
                      }

                      0x1 => {
                        // sta
                        mem_write!(pop!(), pop!());
                        Ok(15)
                      }

                      0x2 => {
                        // ldi
                        push!(mp.ip);
                        Ok(9)
                      }

                      0x3 => {
                        // sti
                        mp.ip = pop!();
                        Ok(6)
                      }

                      0x4 => {
                        // lds
                        push!(mp.sp);
                        Ok(10)
                      }

                      0x5 => {
                        // sts
                        mp.sp = pop!();
                        Ok(5)
                      }

                      0x8 => {
                        // nop
                        Ok(3)
                      }

                      0x9 => {
                        // clc
                        mp.cf = false;
                        Ok(6)
                      }

                      0xA => {
                        // sec
                        mp.cf = true;
                        Ok(6)
                      }

                      0xB => {
                        // flc
                        mp.cf = !mp.cf;
                        Ok(6)
                      }

                      0xC => {
                        // swp
                        let a = pop!();
                        let b = pop!();
                        push!(a);
                        push!(b);
                        Ok(15)
                      }

                      0xD => {
                        // pop
                        let _ = pop!();
                        Ok(5)
                      }

                      _ => Err(TickTrap::IllegalOpcode),
                    }
                  }

                  0b1 => {
                    // phn
                    let imm = instruction; // decode_imm
                    push!(imm);
                    Ok(10)
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
}

impl std::fmt::Display for Microcomputer {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}\r\n{}",
      self.mp,
      common::render_memory(&self.mem, self.mp.ip, self.mp.sp, self.mp.cf),
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
