#include <stdio.h>

int main() {
  putc('*');
  putc('P');
  putc('I');
  putc('P');
  putc('E');
  putc('*');
  putc('\r');
  putc('\n');

  asm { loop: }
  putc(getc());
  asm { :loop !jmp }
}
