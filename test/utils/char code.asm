@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm

main! !nop
  loop:
    # !'\0' !block_getc !u8.to_dec !'\s' !stack_puts
    !block_getc !'@' !hex_putc
    # !getc !char.to_lower clc !putc
    # !getc !char.to_upper clc !putc
  :loop !jmp
