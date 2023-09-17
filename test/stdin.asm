@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm

main!
  loop:
    !block_getc
    !u8.to_chars !putc !putc !char.space !putc
    # !char.to_lower !putc
    # !char.to_upper !putc
    !block_null
  :loop !jmp
