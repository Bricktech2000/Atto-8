@ lib/microprocessor/core.asm
@ lib/microprocessor/stdlib.asm
@ lib/microcomputer/stdio.asm

main!
  :str_prompt :puts !call

  loop:
    !getchar x01 ld1 x0A xor iff x0D sta !putchar
  :loop !jmp

  !puts_def

  str_prompt: d41 d74 d74 d6F d2D d38 d0D d0A d0A d5C d0D d0A d00
