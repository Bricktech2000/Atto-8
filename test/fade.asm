@ lib/core.asm
@ lib/stdlib.asm
@ lib/display.asm

main!
  pop pop !display_buffer sts

  xF0 # rand_seed

  loop:
    !rand ld0 !display_buffer !bit_addr !flip_bit
    x10 !stall
  :loop !jmp
