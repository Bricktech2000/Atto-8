@ ../../lib/core.asm
@ ../../lib/prng.asm
@ ../../lib/nibble.asm
@ ../../lib/microcomputer/input.asm
@ ../../lib/microcomputer/display.asm
@ ../../lib/microcomputer/hex_chars.asm
@ ../../lib/microcomputer/print_byte.asm

main!
  !front_buffer !alloc_buffer
  !reset_input

  # seed PRNG by incrementing until keypress
  :seed x00 wait: inc :wait !branch_input sta

  loop:
    x13 :seed :prng !call :print_byte !call
    x12 :seed :prng !call :print_byte !call
    !wait_input
  :loop sti

  !prng

  !hex_chars_minimal
  !print_byte_minimal

  seed: d00

  !front_buffer @org
    # dEE dEC dEC dAA d8A dAE # PRNG
    dE4 dEC dCE dAA dAA dAC # RAND
    d00 d00 dFF dFF         # ----
    d00 d00 d00 d00 d00 d00 d00 d00 # (empty lines)
    # dEA d00 dA4 d00 dEA d00 # 0X
    d00 d00 d05 d40 # ...
