@ lib/core.asm
@ lib/types.asm
@ lib/stdio.asm

main! !nop
  loop:
    !getc !putc
  :loop !jmp
