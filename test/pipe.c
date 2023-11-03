#include <stdio.h>

int main() {
  puts("*PIPE*\r\n");

  asm { loop: }
  putc(getc());
  asm { :loop !jmp }
}
