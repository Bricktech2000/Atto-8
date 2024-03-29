@ lib/core.asm
@ lib/stdlib.asm
@ lib/display.asm
@ misc/common/common.asm

main!
  pop pop !display_buffer sts

  !rand_seed # rand_seed

  loop:
    !notched clc
    # !continuous
    !rand ld0 !display_buffer !bit_addr !flip_bit
  :loop !jmp

  !display_buffer @org
    !fade
    # !fade_proper
    # !atto_-8
    # !hllo_wrld
    # !avatar
    # !mushroom
    # !yin_yang
    # !music_notes
    # !checkerboard

continuous! x10 !stall
notched! !z x00 xFF iff !delay

fade! x0C !pad
  # characters A and F moved closer together
  @74 @CE @6E @AC @4A @CE @00 @00 # FADE

fade_proper! x0C !pad
  @E4 @CE @CE @AC @8A @CE @00 @00 # FADE
