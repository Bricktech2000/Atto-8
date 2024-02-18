@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm

main! !nop
  loop:
    !block_getc
    # !char.null swp !u8.to_dec !stack_puts !char.space !putc
    !char.commercial_at !hex_putc
    # !char.to_lower !putc
    # !char.to_upper !putc
  :loop !jmp
