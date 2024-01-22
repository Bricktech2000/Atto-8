@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm

main! !nop
  loop:
    !block_getc
    !u8.to_hex !putc !putc !char.space !putc
    # !char.null swp !u8.to_dec !stack_puts !char.space !putc
    # !char.to_lower !putc
    # !char.to_upper !putc
  :loop !jmp
