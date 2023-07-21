@ lib/microprocessor/core.asm
@ lib/microprocessor/stdlib.asm
@ lib/microcomputer/display.asm

main!
  pop pop !display_buffer sts

  xF0 # prng_seed

  loop:
    !prng !display_buffer ld1 !bit_addr !flip_bit
    x10 !stall
  :loop !jmp
