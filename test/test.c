char getc() {
  int ret;
  asm { !getc st0 } // call into `stdio.asm`
  return ret;
}

void putc(char c) {
  asm { ld1 !putc } // call into `stdio.asm`
  return;
}

// #define EMPTY
// #include EMPTY<file.h>
// EMPTY #include <file.h>

#define ADDITION 1 + TWO
#define TWO 2

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

  // asm ('A' + 32) { !putc }
  // asm ('B', ' ') { add !putc }
  // asm ('C' + space()) { !putc }
  // asm ('\r') { !putc }
  // asm ((int)'\n') { !putc }
  // asm ('\a') { !putc }

  ADDITION == 3;
  return foo() + (char)1;
  return 2 > 1 == 4 >= 2;
}

#define CORE "lib/core.asm"

asm {
  #include CORE
  #include <lib/stdio.asm>
}
