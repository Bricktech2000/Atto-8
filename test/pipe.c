#include <stdio.h>

inline char to_upper(char c) {
  asm { clc !char.to_upper }
}

int main(void) {
  puts("*pipe*\r\n");

  while (1) {
    putc(to_upper(getc()));
  }
}
