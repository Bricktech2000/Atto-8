#include <stdio.h>

inline char to_upper(char c) {
  asm { clc !char.to_upper }
}

void main(void) {
  puts("*pipe*\r\n");

  while (1) {
    putc(to_upper(getc()));
  }
}
