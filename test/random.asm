@ lib/microprocessor/core.asm
@ lib/microprocessor/math.asm
@ lib/microprocessor/memory.asm
@ lib/microcomputer/text.asm
@ lib/microcomputer/input.asm
@ lib/microcomputer/display.asm

main!
  pop !front_buffer sts
  !reset_input

  x00 # prng_seed

  # seed prng by incrementing until keypress
  wait: inc :wait !branch_input !reset_input

  loop:
    !prng x13 ld1 :print_byte !call
    !prng x12 ld1 :print_byte !call
    !wait_input
  :loop !jmp

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
