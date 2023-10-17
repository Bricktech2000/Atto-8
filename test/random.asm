@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm
@ lib/display.asm
@ misc/common/common.asm

main!
  pop pop !display_buffer sts

  !rand_seed # rand_seed

  # seed rand by incrementing until keypress
  wait: inc !getc !char.is_null :wait !bcs

  loop:
    !rand x13 ld1 :print_byte.min !call
    !rand x12 ld1 :print_byte.min !call
    !block_any
  :loop !jmp

  !print_byte.min.def

  !display_buffer @org
    # @EE @EC @EC @AA @8A @AE # PRNG
    @C4 @CC @EE @AA @AA @AC @00 @00 # RAND
    @FF @FF @00 @00                 # ----
    @00 @00 @00 @00 @00 @00 @00 @00 # empty
    @05 @40 @00 @00                 # ...
