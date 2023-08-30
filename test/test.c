char space() {
  return ' ';
}

int foo() {
  return 42;
  space();
}

int main() {
  // return 0;

  // return 1 + 2;

  // return ~2 + (3 + 4) * 5;

  // return 1 || 0 && 2;

  // return 2 == 4 >= 2;

  // return 2 > 1 == 4 >= 2;

  asm ('A' + 32) { !putc }
  asm ('B', ' ') { add !putc }
  asm ('C' + space()) { !putc }
  asm ('\r') { !putc }
  asm ((int)'\n') { !putc }
  // asm ('\a') { !putc }

  1 + 2 == 3;
  return foo() + (char)1;
  return 2 > 1 == 4 >= 2;

  asm () {
    !display_buffer sts

    xF0 // rand_seed

    loop:
      !rand ld0 !display_buffer !bit_addr !flip_bit
      x10 !stall
    :loop !jmp
  }
}
