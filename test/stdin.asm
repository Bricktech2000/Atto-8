@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm

main!
  !char.null loop: !char.pop
    !getc !char.check_null :loop !bcs
    !u8.to_chars !putc !putc !char.space !putc
    # !char.to_lower !putc
    # !char.to_upper !putc
    !here !wait_null
  !char.null :loop !jmp
