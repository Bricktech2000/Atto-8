@ lib/microprocessor/core.asm
@ lib/microprocessor/types.asm
@ lib/microprocessor/stdlib.asm
@ lib/microcomputer/stdio.asm
@ lib/microcomputer/display.asm

main!
  pop pop !display_buffer sts

  x00 # prng_seed

  # seed prng by incrementing until keypress
  wait: inc :wait !wait_char

  loop:
    !prng x13 ld1 :print_byte !call
    !prng x12 ld1 :print_byte !call
    !here !wait_char
  :loop !jmp

  !print_byte_min_def

  seed: d00

  !display_buffer @org
    # dEE dEC dEC dAA d8A dAE # PRNG
    dE4 dEC dCE dAA dAA dAC # RAND
    d00 d00 dFF dFF         # ----
    d00 d00 d00 d00 d00 d00 d00 d00 # (empty lines)
    # dEA d00 dA4 d00 dEA d00 # 0X
    d00 d00 d05 d40 # ...
