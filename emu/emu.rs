use std::collections::VecDeque;

#[path = "../misc/common/common.rs"]
mod common;
use common::*;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 2 {
    println!("Emu: Usage: emu <memory image file>");
    std::process::exit(1);
  }

  let memory_image_file: &String = &args[1];

  let memory_image: [u8; common::MEM_SIZE] = std::fs::read(memory_image_file)
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

    let opcode = mem_read!(mp.ip);
    mp.ip = mp.ip.wrapping_add(1);

    let instruction = common::opcode_to_instruction(opcode).map_err(|_| TickTrap::IllegalOpcode)?;

    match instruction {
      Instruction::Psh(imm) => {
        let imm = 0b00000000 | imm;
        push!(imm);
        Ok(10)
      }

      Instruction::Add(size) => {
        let size = mp.sp.wrapping_add(size);
        let a = pop!();
        let b = mem_read!(size);
        let sum = b as u16 + a as u16 + mp.cf as u16;
        mem_write!(size, sum as u8);
        mp.cf = sum > 0xFF;
        Ok(16)
      }

      Instruction::Sub(size) => {
        let size = mp.sp.wrapping_add(size);
        let a = pop!();
        let b = mem_read!(size);
        let diff = b as i16 - a as i16 - mp.cf as i16;
        mem_write!(size, diff as u8);
        mp.cf = diff < 0x00;
        Ok(16)
      }

      Instruction::Iff(size) => {
        let size = mp.sp.wrapping_add(size);
        let a = pop!();
        let b = mem_read!(size);
        mem_write!(size, if mp.cf { a } else { b });
        Ok(15)
      }

      Instruction::Rot(size) => {
        let size = mp.sp.wrapping_add(size);
        let a = pop!();
        let b = mem_read!(size);
        let shifted = (b as u16) << a % 8;
        mem_write!(size, (shifted & 0xFF) as u8 | (shifted >> 8) as u8);
        mp.cf = false;
        Ok(19 * (a as u128 + 1))
      }

      Instruction::Orr(size) => {
        let size = mp.sp.wrapping_add(size);
        let a = pop!();
        let b = mem_read!(size);
        mem_write!(size, a | b);
        mp.cf = mem_read!(size) == 0x00;
        Ok(16)
      }

      Instruction::And(size) => {
        let size = mp.sp.wrapping_add(size);
        let a = pop!();
        let b = mem_read!(size);
        mem_write!(size, a & b);
        mp.cf = mem_read!(size) == 0x00;
        Ok(13)
      }

      Instruction::Xor(size) => {
        let size = mp.sp.wrapping_add(size);
        let a = pop!();
        let b = mem_read!(size);
        mem_write!(size, a ^ b);
        mp.cf = mem_read!(size) == 0x00;
        Ok(24)
      }

      Instruction::Xnd(size) => {
        let size = mp.sp.wrapping_add(size);
        let _ = pop!();
        mem_write!(size, 0x00);
        mp.cf = mem_read!(size) == 0x00;
        Ok(9)
      }

      Instruction::Inc => {
        push!(pop!().wrapping_add(1));
        Ok(6)
      }

      Instruction::Dec => {
        push!(pop!().wrapping_sub(1));
        Ok(8)
      }

      Instruction::Neg => {
        push!(pop!().wrapping_neg());
        Ok(11)
      }

      Instruction::Shl => {
        let a = pop!();
        push!(a.wrapping_shl(1) | (mp.cf as u8));
        mp.cf = a & 0b10000000 != 0x00;
        Ok(9)
      }

      Instruction::Shr => {
        let a = pop!();
        push!(a.wrapping_shr(1) | (mp.cf as u8) << 7);
        mp.cf = a & 0b00000001 != 0x00;
        Ok(16)
      }

      Instruction::Not => {
        push!(!pop!());
        mp.cf = mem_read!(mp.sp) == 0x00;
        Ok(8)
      }

      Instruction::Buf => {
        push!(pop!());
        mp.cf = mem_read!(mp.sp) == 0x00;
        Ok(9)
      }

      Instruction::Dbg => Err(TickTrap::DebugRequest),

      Instruction::Ldo(ofst) => {
        let ofst = mp.sp.wrapping_add(ofst);
        push!(mem_read!(ofst));
        Ok(14)
      }

      Instruction::Sto(ofst) => {
        let ofst = mp.sp.wrapping_add(ofst).wrapping_add(1);
        mem_write!(ofst, pop!());
        Ok(13)
      }

      Instruction::Lda => {
        push!(mem_read!(pop!()));
        Ok(9)
      }

      Instruction::Sta => {
        mem_write!(pop!(), pop!());
        Ok(15)
      }

      Instruction::Ldi => {
        push!(mp.ip);
        Ok(9)
      }

      Instruction::Sti => {
        mp.ip = pop!();
        Ok(6)
      }

      Instruction::Lds => {
        push!(mp.sp);
        Ok(10)
      }

      Instruction::Sts => {
        mp.sp = pop!();
        Ok(5)
      }

      Instruction::Nop => Ok(3),

      Instruction::Clc => {
        mp.cf = false;
        Ok(6)
      }

      Instruction::Sec => {
        mp.cf = true;
        Ok(6)
      }

      Instruction::Flc => {
        mp.cf = !mp.cf;
        Ok(6)
      }

      Instruction::Swp => {
        let a = pop!();
        let b = pop!();
        push!(a);
        push!(b);
        Ok(15)
      }

      Instruction::Pop => {
        let _ = pop!();
        Ok(5)
      }

      Instruction::Phn(imm) => {
        let imm = 0b11110000 | imm;
        push!(imm);
        Ok(10)
      }
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
