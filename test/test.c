// char getc() {
//   int ret;
//   asm { !getc st0 } // call into `stdio.asm`
//   return ret;
// }
//
// void putc(char c) {
//   asm { ld1 !putc } // call into `stdio.asm`
//   return;
// }

#define TWO 2
#define ADDITION 1 + TWO

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

asm {
  #include "lib/core.asm"
  #include <lib/stdio.asm>
}
