@ lib/core.asm
@ lib/types.asm
@ lib/string.asm
@ lib/display.asm

# to count neighbors, display buffer is read from and back buffer is written to.
# back buffer is copied to display buffer at the end of each iteration.
#
# rules used:
#
# ```rust
# let next_state = match neighbor_count {
#   3 => State::Alive,
#   4 => current_state,
#   _ => State::Dead,
# }
# ```

main!
  pop pop !back_buffer sts

  loop:
    # copy back buffer to display buffer
    !display_buffer !back_buffer sub @const !back_buffer !display_buffer :memcpy !call
    # loop through every cell
    x00 !u4u4 for_xy: dec
      x00 # allocate neighbor count

      # count neighbors
      !neighbors_len for_dxdy: dec
        # neighbor_addr = *(neighbors + dxdy) + for_xy
        :neighbors ld1 add !i4i4.lda !u4u4.ld3 !i4i4.add
        # neighbor_value = load_bit(bit_addr(&DISPLAY_BUFFER, neighbor_addr))
        !display_buffer !bit_addr !load_bit clc
        # neighbor_count += neighbor_value
        ad2
      buf :for_dxdy !bcc pop

      # apply rules outlined above
      x04 xor :ignore !bcs x04 xor
      x03 xor x00 shl @dyn
      !u4u4.ld2 !back_buffer !bit_addr !store_bit
      ignore:

      pop # pop neighbor count
    buf :for_xy !bcc pop
  :loop !jmp

  neighbors:
    @FF !i4i4
    @F0 !i4i4
    @F1 !i4i4
    @0F !i4i4
    @00 !i4i4
    @01 !i4i4
    @1F !i4i4
    @10 !i4i4
    @11 !i4i4
  neighbors_end:

  !memcpy.def

  !back_buffer @org
    # !glider
    # !blinker
    # !r-pentomino
    # !lightweight_spaceship
    !heavyweight_spaceship
    # !copperhead
    # !diehard
    # !compact_pulsar

neighbors_len! :neighbors_end :neighbors sub @const
back_buffer! xC0 @const


glider!
  x0C !pad
  @07 @00
  @01 @00
  @02 @00

blinker!
  x0C !pad
  @07 @00

diehard!
  # already advanced 2 generations
  x0C !pad
  @30 @80
  @31 @C0

r-pentomino!
  x0C !pad
  @06 @00
  @0C @00
  @04 @00

lightweight_spaceship!
  x0A !pad
  @00 @09
  @00 @10
  @00 @11
  @00 @1E

heavyweight_spaceship!
  x0A !pad
  @00 @0C
  @00 @21
  @00 @40
  @00 @41
  @00 @7E

compact_pulsar!
  # pattern that turns into a pulsar
  x0C !pad
  @07 @C0
  @08 @40
  @07 @C0

copperhead!
  x08 !pad
  @06 @60
  @01 @80
  @01 @80
  @0A @50
  @08 @10
  @00 @00
  @08 @10
  @06 @60
  @03 @C0
  @00 @00
  @01 @80
  @01 @80
