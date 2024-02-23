@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm

main! !nop
  loop:
    # !char.null !block_getc !u8.to_dec !char.space !stack_puts
    !block_getc !char.commercial_at !hex_putc
    # !getc !char.to_lower clc !putc
    # !getc !char.to_upper clc !putc
  :loop !jmp
