@ lib/microprocessor/core.asm
@ lib/microprocessor/types.asm
@ lib/microcomputer/display.asm

main!
  pop pop !display_buffer sts

  !u64.0 loop:
    x00 ld8 :print_byte.min !call
    x01 ld7 :print_byte.min !call
    x08 ld6 :print_byte.min !call
    x09 ld5 :print_byte.min !call
    x10 ld4 :print_byte.min !call
    x11 ld3 :print_byte.min !call
    x18 ld2 :print_byte.min !call
    x19 ld1 :print_byte.min !call
  # :print_byte.min sets the carry flag
  !u64.0 !u64.add :loop !jmp

  !print_byte.min.def
