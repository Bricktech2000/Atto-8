@../../lib/utils/core.asm
@../../lib/utils/text.asm
@../../lib/microcomputer/utils.asm
@../../lib/microcomputer/hex_chars.asm
@../../lib/microcomputer/print_byte.asm

main!
  !front_buffer !alloc_buffer

  x00 x00 loop:
    x00 ld2 :print_byte !call
    x01 ld1 :print_byte !call
  x00 x01 adc2 adc2 :loop sti

  !nibble_addr
  !load_nibble
  !hex_chars_minimal
  !print_byte_minimal
