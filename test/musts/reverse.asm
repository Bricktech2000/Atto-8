@ lib/core.asm
@ lib/types.asm
@ lib/stdio.asm

main! !nop
  loop:
    !stack_gets !stack_puts
  :loop !jmp

