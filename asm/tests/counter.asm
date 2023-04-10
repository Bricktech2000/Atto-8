@ ../../lib/core.asm
@ ../../lib/text.asm
@ ../../lib/microcomputer/core.asm
@ ../../lib/microcomputer/hex_chars.asm
@ ../../lib/microcomputer/print_byte.asm

main!
  !front_buffer !alloc_buffer

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
  adc8 adc8 adc8 adc8 adc8 adc8 adc8 adc8 :loop sti

  !load_nibble_minimal
  !hex_chars_minimal
  !print_byte_minimal
