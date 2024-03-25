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
    stdout: &mut VecDeque<u8>,
    display: &mut [u8; common::DISPLAY_BUFFER_LEN],
    _controller: &mut u8,
  ) {
    self.mp.ip = 0x00;
    self.mp.sp = 0x00;
    self.mp.cf = false;
    stdin.clear();
    stdout.clear();
    stdin.push_back(self.mem[common::STDIO_BUFFER]);
    display.copy_from_slice(
      &self.mem[common::DISPLAY_BUFFER..common::DISPLAY_BUFFER + common::DISPLAY_BUFFER_LEN],
    );
  }

  fn tick(
    &mut self,
    stdin: &mut VecDeque<u8>,
    stdout: &mut VecDeque<u8>,
    display: &mut [u8; common::DISPLAY_BUFFER_LEN],
    controller: &mut u8,
  ) -> Result<u128, TickTrap> {
    let mp = &mut self.mp;

    macro_rules! mem_read {
      ($address:expr) => {{
        let address = $address as usize;
        if address == common::STDIO_BUFFER {
          stdin.pop_front().unwrap_or(*controller)
        } else {
          self.mem[address]
        }
      }};
    }

    macro_rules! mem_write {
      ($address:expr, $value:expr) => {{
        let address = $address as usize;
        let value = $value;
        if address == common::STDIO_BUFFER {
          stdout.push_back(value);
        } else {
          self.mem[address] = value;
        }
        if address & common::DISPLAY_BUFFER == common::DISPLAY_BUFFER {
          display[address & !common::DISPLAY_BUFFER] = value
        }
      }};
    }

    macro_rules! sp_push {
      ($value:expr) => {{
        let value = $value;
        mp.sp = mp.sp.wrapping_sub(1);
        mem_write!(mp.sp, value);
      }};
    }

    macro_rules! sp_pop {
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
      Instruction::Psh(Imm(imm)) => {
        sp_push!(imm);
        Ok(10)
      }

      Instruction::Add(Size(size)) => {
        let addr = mp.sp.wrapping_add(size);
        let res = mem_read!(addr) as u16 + sp_pop!() as u16 + mp.cf as u16;
        mem_write!(addr, res as u8);
        mp.cf = res > 0xFF;
        Ok(14 + size as u128)
      }

      Instruction::Sub(Size(size)) => {
        let addr = mp.sp.wrapping_add(size);
        let res = mem_read!(addr) as u16 - sp_pop!() as u16 - mp.cf as u16;
        mem_write!(addr, res as u8);
        mp.cf = res > 0xFF;
        Ok(14 + size as u128)
      }

      Instruction::Iff(Size(size)) => {
        let addr = mp.sp.wrapping_add(size);
        let top = sp_pop!();
        mem_write!(addr, if mp.cf { top } else { mem_read!(addr) });
        Ok(13 + size as u128)
      }

      Instruction::Swp(Size(size)) => {
        let addr = mp.sp.wrapping_add(size);
        let top = sp_pop!();
        sp_push!(mem_read!(addr));
        mem_write!(addr, top);
        Ok(13 + size as u128)
      }

      Instruction::Rot(Size(size)) => {
        let addr = mp.sp.wrapping_add(size);
        let top = sp_pop!();
        let temp = (mem_read!(addr) as u16) << top % 8;
        let res = (temp & 0xFF) as u8 | (temp >> 8) as u8;
        mem_write!(addr, res);
        mp.cf = false;
        Ok((18 + size as u128) * (top as u128 + 1))
      }

      Instruction::Orr(Size(size)) => {
        let addr = mp.sp.wrapping_add(size);
        let res = sp_pop!() | mem_read!(addr);
        mem_write!(addr, res);
        mp.cf = res == 0x00;
        Ok(14 + size as u128)
      }

      Instruction::And(Size(size)) => {
        let addr = mp.sp.wrapping_add(size);
        let res = sp_pop!() & mem_read!(addr);
        mem_write!(addr, res);
        mp.cf = res == 0x00;
        Ok(11 + size as u128)
      }

      Instruction::Xor(Size(size)) => {
        let addr = mp.sp.wrapping_add(size);
        let res = sp_pop!() ^ mem_read!(addr);
        mem_write!(addr, res);
        mp.cf = res == 0x00;
        Ok(22 + size as u128)
      }

      Instruction::Xnd(Size(size)) => {
        let addr = mp.sp.wrapping_add(size);
        let res = sp_pop!() & 0x00;
        mem_write!(addr, res);
        mp.cf = res == 0x00;
        Ok(8 + size as u128)
      }

      Instruction::Inc => {
        sp_push!(sp_pop!().wrapping_add(1));
        Ok(6)
      }

      Instruction::Dec => {
        sp_push!(sp_pop!().wrapping_sub(1));
        Ok(8)
      }

      Instruction::Neg => {
        sp_push!(sp_pop!().wrapping_neg());
        Ok(11)
      }

      Instruction::Shl => {
        let top = sp_pop!();
        sp_push!(top.wrapping_shl(1) | (mp.cf as u8));
        mp.cf = top & 0b10000000 != 0x00;
        Ok(9)
      }

      Instruction::Shr => {
        let top = sp_pop!();
        sp_push!(top.wrapping_shr(1) | (mp.cf as u8) << 7);
        mp.cf = top & 0b00000001 != 0x00;
        Ok(16)
      }

      Instruction::Not => {
        let res = !sp_pop!();
        sp_push!(res);
        mp.cf = res == 0x00;
        Ok(8)
      }

      Instruction::Buf => {
        let res = sp_pop!();
        sp_push!(res);
        mp.cf = res == 0x00;
        Ok(9)
      }

      Instruction::Dbg => Err(TickTrap::DebugRequest),

      Instruction::Ldo(Ofst(ofst)) => {
        let addr = mp.sp.wrapping_add(ofst);
        sp_push!(mem_read!(addr));
        Ok(12 + ofst as u128)
      }

      Instruction::Sto(Ofst(ofst)) => {
        let top = sp_pop!();
        let addr = mp.sp.wrapping_add(ofst);
        mem_write!(addr, top);
        Ok(11 + ofst as u128)
      }

      Instruction::Lda => {
        sp_push!(mem_read!(sp_pop!()));
        Ok(9)
      }

      Instruction::Sta => {
        mem_write!(sp_pop!(), sp_pop!());
        Ok(15)
      }

      Instruction::Ldi => {
        sp_push!(mp.ip);
        Ok(9)
      }

      Instruction::Sti => {
        mp.ip = sp_pop!();
        Ok(6)
      }

      Instruction::Lds => {
        sp_push!(mp.sp);
        Ok(10)
      }

      Instruction::Sts => {
        mp.sp = sp_pop!();
        Ok(5)
      }

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

      Instruction::Nop => Ok(3),

      Instruction::Pop => {
        mp.sp = mp.sp.wrapping_add(1);
        Ok(5)
      }

      Instruction::Phn(Nimm(nimm)) => {
        sp_push!(nimm);
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
