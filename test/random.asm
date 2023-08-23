@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm
@ lib/display.asm

main!
  pop pop !display_buffer sts

  x00 # rand_seed

  # seed rand by incrementing until keypress
  wait: inc :wait !wait_char

  loop:
    !rand x13 ld1 :print_byte.min !call
    !rand x12 ld1 :print_byte.min !call
    !here !wait_char
  :loop !jmp

  !print_byte.min.def

  !display_buffer @org
    # @EE @EC @EC @AA @8A @AE # PRNG
    @E4 @EC @CE @AA @AA @AC # RAND
    @00 @00 @FF @FF         # ----
    @00 @00 @00 @00 @00 @00 @00 @00 # (empty lines)
    # @EA @00 @A4 @00 @EA @00 # 0X
    @00 @00 @05 @40 # ...
