@ lib/core.asm
@ lib/types.asm
@ lib/stdio.asm

main!
  !char.null loop: !char.pop
    !getc !char.null xor :loop !bcs
    !u8.to_chars !putc !putc
    !char.space !putc
    !here !wait_null
  !char.null :loop !jmp
