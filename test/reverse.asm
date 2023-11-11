@ lib/core.asm
@ lib/types.asm
@ lib/stdio.asm

main!
  nop @dyn loop:
    !stack_gets !stack_puts
  :loop !jmp

