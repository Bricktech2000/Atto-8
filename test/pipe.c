// TODO implement linker
asm {
  #include "lib/core.asm"
  #include "lib/stdio.asm"

  main!
    !main.ref !call !hlt
    !main.def
    !putc.def
    !getc.def

  getc.ref! :getc
  putc.ref! :putc
}

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
