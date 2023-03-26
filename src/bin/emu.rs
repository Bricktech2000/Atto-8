fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 2 {
    println!("Usage: emu <filename>");
    std::process::exit(1);
  }

  println!("Emulating CPU...");

  let memory: Vec<u8> = std::fs::read(&args[1]).expect("Unable to read file");
  emulate(
    memory.try_into().expect("Slice with incorrect length"),
    1000,
  );

  println!("");
  println!("CPU halted.");
}

fn emulate(memory: [u8; 0x100], clock: u64) {
  let mut memory = memory.clone();
  let mut stack_pointer: u8 = 0x00; // CPU stack pointer
  let mut work_pointer: u8 = 0x00; // CPU work pointer
  let mut instruction_pointer: u8 = 0x00; // CPU instruction pointer
  let mut carry_flag: bool = false; // CPU carry flag
  let mut debug_flag: bool = false; // CPU debug flag

  // clear screen
  print!("\x1B[2J");

  loop {
    let instruction: u8 = memory[instruction_pointer as usize];
    instruction_pointer = instruction_pointer.wrapping_add(1);

    // roughly 4 clock cycles per instruction
    std::thread::sleep(std::time::Duration::from_millis(1000 * 4 / clock));

    if !debug_flag {
      // move cursor to top left
      print!("\x1B[1;1H");
    }
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
      "IP  {:#04x}\nSP  {:#04x}\nWP  {:#04x}\nCF  {}",
      instruction_pointer, stack_pointer, work_pointer, carry_flag
    );

    if debug_flag {
      use std::io::{stdin, Read};
      stdin().read(&mut [0]).unwrap();
    }

    match instruction {
      0x80 => {
        // nop
      }
      0x81 => {
        // hlt
        break;
      }
      0x88 => {
        // dbg
        debug_flag = true;
      }
      0x83 => {
        // clc
        carry_flag = false;
      }
      0x84 => {
        // sec
        carry_flag = true;
      }
      0x85 => {
        // flc
        carry_flag = !carry_flag;
      }
      0xA0 => {
        // inc
        let a = memory[work_pointer as usize];
        memory[work_pointer as usize] = a.wrapping_add(1);
      }
      0xA1 => {
        // dec
        let a = memory[work_pointer as usize];
        memory[work_pointer as usize] = a.wrapping_sub(1);
      }
      0xA2 => {
        // add
        let a = memory[stack_pointer as usize];
        memory[stack_pointer as usize] = 0x00;
        stack_pointer = stack_pointer.wrapping_add(1);
        work_pointer = work_pointer.wrapping_add(1);
        let b = memory[work_pointer as usize];

        memory[work_pointer as usize] = (b as u16 + a as u16 + carry_flag as u16) as u8;
        carry_flag = (b as u16 + a as u16 + carry_flag as u16) > 0xFF;
      }
      0xA3 => {
        // sub
        let a = memory[stack_pointer as usize];
        memory[stack_pointer as usize] = 0x00;
        stack_pointer = stack_pointer.wrapping_add(1);
        work_pointer = work_pointer.wrapping_add(1);
        let b = memory[work_pointer as usize];

        memory[work_pointer as usize] = (b as i16 - a as i16 - carry_flag as i16) as u8;
        carry_flag = (b as i16 - a as i16 - carry_flag as i16) > 0xFF;
      }
      0xA4 => {
        // rol
        let a = memory[work_pointer as usize];
        memory[work_pointer as usize] = a << 1 | carry_flag as u8;
        carry_flag = a & 0x80 != 0;
      }
      0xA5 => {
        // ror
        let a = memory[work_pointer as usize];
        memory[work_pointer as usize] = a >> 1 | carry_flag as u8;
        carry_flag = a & 0x01 != 0;
      }
      0xA6 => {
        // oor
        let a = memory[stack_pointer as usize];
        memory[stack_pointer as usize] = 0x00;
        stack_pointer = stack_pointer.wrapping_add(1);
        work_pointer = work_pointer.wrapping_add(1);
        let b = memory[work_pointer as usize];

        memory[work_pointer as usize] = a | b;
        carry_flag = memory[work_pointer as usize] == 0x00;
      }
      0xA7 => {
        // and
        let a = memory[stack_pointer as usize];
        memory[stack_pointer as usize] = 0x00;
        stack_pointer = stack_pointer.wrapping_add(1);
        work_pointer = work_pointer.wrapping_add(1);
        let b = memory[work_pointer as usize];

        memory[work_pointer as usize] = a & b;
        carry_flag = memory[work_pointer as usize] == 0x00;
      }
      0xA8 => {
        // xor
        let a = memory[stack_pointer as usize];
        memory[stack_pointer as usize] = 0x00;
        stack_pointer = stack_pointer.wrapping_add(1);
        work_pointer = work_pointer.wrapping_add(1);
        let b = memory[work_pointer as usize];

        memory[work_pointer as usize] = a ^ b;
        carry_flag = memory[work_pointer as usize] == 0x00;
      }
      0xA9 => {
        // xnd
        memory[stack_pointer as usize] = 0x00;
        stack_pointer = stack_pointer.wrapping_add(1);
        work_pointer = work_pointer.wrapping_add(1);

        memory[work_pointer as usize] = 0;
        carry_flag = memory[work_pointer as usize] == 0x00;
      }
      0xAA => {
        // not
        let a = memory[work_pointer as usize];

        memory[work_pointer as usize] = !a;
        carry_flag = memory[work_pointer as usize] == 0x00;
      }
      0x90 => {
        // iif
        let a = memory[stack_pointer as usize];
        memory[stack_pointer as usize] = 0x00;
        stack_pointer = stack_pointer.wrapping_add(1);
        work_pointer = work_pointer.wrapping_add(1);
        let b = memory[stack_pointer as usize];

        memory[stack_pointer as usize] = if carry_flag { a } else { b };
        carry_flag = false;
      }
      0x91 => {
        // swp
        let a = memory[stack_pointer as usize];
        stack_pointer = stack_pointer.wrapping_add(1);
        work_pointer = work_pointer.wrapping_add(1);
        let b = memory[work_pointer as usize];

        memory[work_pointer as usize] = a;
        stack_pointer = stack_pointer.wrapping_sub(1);
        work_pointer = work_pointer.wrapping_sub(1);
        memory[work_pointer as usize] = b;
      }
      0x92 => {
        // dup
        let a = memory[work_pointer as usize];
        stack_pointer = stack_pointer.wrapping_sub(1);
        work_pointer = work_pointer.wrapping_sub(1);

        memory[stack_pointer as usize] = a;
      }
      0x93 => {
        // str
        let a = memory[stack_pointer as usize];
        memory[stack_pointer as usize] = 0x00;
        stack_pointer = stack_pointer.wrapping_add(1);
        work_pointer = work_pointer.wrapping_add(1);

        memory[work_pointer as usize] = a;
      }
      0x94 => {
        // pop
        memory[stack_pointer as usize] = 0x00;
        stack_pointer = stack_pointer.wrapping_add(1);
        work_pointer = work_pointer.wrapping_add(1);
      }
      _ if instruction & 0b11000000 == 0b00000000 || instruction & 0b11000000 == 0b11000000 => {
        // xXX
        let a = instruction;
        stack_pointer = stack_pointer.wrapping_sub(1);
        work_pointer = work_pointer.wrapping_sub(1);
        memory[stack_pointer as usize] = a;
      }
      0x95 => {
        // xXX
        let a = memory[instruction_pointer as usize];
        instruction_pointer = instruction_pointer.wrapping_add(1);
        stack_pointer = stack_pointer.wrapping_sub(1);
        work_pointer = work_pointer.wrapping_sub(1);

        memory[stack_pointer as usize] = a;
      }
      _ if instruction & 0b11000000 == 0b01000000 => {
        // @WW
        let a = instruction & 0b00111111;
        work_pointer = stack_pointer.wrapping_add(a as u8);
      }
      0x96 => {
        // ldi
        stack_pointer = stack_pointer.wrapping_sub(1);
        work_pointer = work_pointer.wrapping_sub(1);
        memory[stack_pointer as usize] = instruction_pointer;
      }
      0x97 => {
        // sti
        instruction_pointer = memory[stack_pointer as usize];
        memory[stack_pointer as usize] = 0x00;
        stack_pointer = stack_pointer.wrapping_add(1);
        work_pointer = work_pointer.wrapping_add(1);
      }
      0x98 => {
        // ldw
        stack_pointer = stack_pointer.wrapping_sub(1);
        work_pointer = work_pointer.wrapping_sub(1);
        memory[stack_pointer as usize] = work_pointer;
      }
      0x99 => {
        // stw
        work_pointer = memory[stack_pointer as usize];
        memory[stack_pointer as usize] = 0x00;
        stack_pointer = stack_pointer.wrapping_add(1);
        // work_pointer = work_pointer.wrapping_add(1);
      }
      0x9A => {
        // lds
        stack_pointer = stack_pointer.wrapping_sub(1);
        work_pointer = work_pointer.wrapping_sub(1);
        memory[stack_pointer as usize] = stack_pointer;
      }
      0x9B => {
        // sts
        stack_pointer = memory[stack_pointer as usize];
        memory[stack_pointer as usize] = 0x00;
        // stack_pointer = stack_pointer.wrapping_add(1);
        // work_pointer = work_pointer.wrapping_add(1);
      }
      _ => {
        panic!("Unknown instruction: {:#04x}", instruction);
      }
    }
  }
}

fn print_display(display_buffer: &[u8; 0x20]) {
  let mut display_buffer_string: String = String::new();
  let line: String = std::iter::repeat("-").take(0x10).collect::<String>();
  let line_top: String = ".-".to_owned() + &line + "-.\n";
  let line_bottom: String = "'-".to_owned() + &line + "-'\n";
  let col_left: String = "| ".to_string();
  let col_right: String = " |".to_string();

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
  let mut memory_string: String = String::new();

  for y in 0..0x10 {
    for x in 0..0x10 {
      memory_string += &format!("{:02x} ", memory[(y << 0x04 | x) as usize]);
    }
    memory_string.push('\n');
  }
  println!("{}", memory_string);
}
