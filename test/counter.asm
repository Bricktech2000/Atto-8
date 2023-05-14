@ lib/microprocessor/core.asm
@ lib/microprocessor/math.asm
@ lib/microprocessor/memory.asm
@ lib/microcomputer/text.asm
@ lib/microcomputer/display.asm

main!
  pop !front_buffer sts

  x00 x00 x00 x00 x00 x00 x00 x00 loop:
    x00 ld8 :print_byte !call
    x01 ld7 :print_byte !call
    x08 ld6 :print_byte !call
    x09 ld5 :print_byte !call
    x10 ld4 :print_byte !call
    x11 ld3 :print_byte !call
    x18 ld2 :print_byte !call
    x19 ld1 :print_byte !call
  x00 x00 x00 x00 x00 x00 x00 x01
  !add_u64 :loop !jmp

  !hex_chars_minimal
  !print_byte_minimal
