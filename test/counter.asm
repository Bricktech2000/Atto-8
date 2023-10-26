@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/display.asm

main!
  pop pop !display_buffer sts

  !u64.0 loop:
    x00 ld8 :display_byte.min !call
    x01 ld7 :display_byte.min !call
    x08 ld6 :display_byte.min !call
    x09 ld5 :display_byte.min !call
    x10 ld4 :display_byte.min !call
    x11 ld3 :display_byte.min !call
    x18 ld2 :display_byte.min !call
    x19 ld1 :display_byte.min !call
  # :display_byte.min sets the carry flag
  !u64.0 !u64.add :loop !jmp

  !display_byte.min.def
