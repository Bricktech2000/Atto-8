@ lib/microprocessor/core.asm
@ lib/microprocessor/math.asm
@ lib/microprocessor/memory.asm
@ lib/microcomputer/text.asm
@ lib/microcomputer/display.asm

main!
  pop !front_buffer sts

  !u64.0 loop:
    x00 ld8 :print_byte !call
    x01 ld7 :print_byte !call
    x08 ld6 :print_byte !call
    x09 ld5 :print_byte !call
    x10 ld4 :print_byte !call
    x11 ld3 :print_byte !call
    x18 ld2 :print_byte !call
    x19 ld1 :print_byte !call
  # :print_byte sets the carry flag
  !u64.0 !u64.add :loop !jmp

  !hex_chars_minimal
  !print_byte_minimal
