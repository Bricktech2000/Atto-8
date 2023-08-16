@ lib/microprocessor/core.asm
@ lib/microprocessor/types.asm
@ lib/microprocessor/string.asm
@ lib/microcomputer/display.asm

# to count neighbours, display buffer is read from and back buffer is written to.
# back buffer is copied to display buffer at the end of each iteration.
#
# rules used:
#
# ```rust
# let next_state = match neighbour_count {
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
      x00 # allocate neighbour count

      # count neighbours
      !neighbours_len for_dxdy: dec
        # neighbour_addr = *(neighbours + dxdy) + for_xy
        :neighbours ld1 add !i4i4.lda !u4u4.ld3 !i4i4.add
        # neighbour_value = load_bit(bit_addr(&DISPLAY_BUFFER, neighbour_addr))
        !display_buffer !bit_addr !load_bit clc
        # neighbour_count += neighbour_value
        ad2
      buf :for_dxdy !bcc pop

      # apply rules outlined above
      x04 xor :ignore !bcs x04 xor
      x03 xor x00 shl @dyn
      !u4u4.ld2 !back_buffer !bit_addr !store_bit
      ignore:

      pop # pop neighbour count
    buf :for_xy !bcc pop
  :loop !jmp

  neighbours:
    dFF !i4i4
    dF0 !i4i4
    dF1 !i4i4
    d0F !i4i4
    d00 !i4i4
    d01 !i4i4
    d1F !i4i4
    d10 !i4i4
    d11 !i4i4
  neighbours_end:

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

neighbours_len! :neighbours_end :neighbours sub @const
back_buffer! xC0 @const


glider!
  x0C !pad
  d07 d00
  d01 d00
  d02 d00

blinker!
  x0C !pad
  d07 d00

diehard!
  # already advanced 2 generations
  x0C !pad
  d30 d80
  d31 dC0

r-pentomino!
  x0C !pad
  d06 d00
  d0C d00
  d04 d00

lightweight_spaceship!
  x0A !pad
  d00 d09
  d00 d10
  d00 d11
  d00 d1E

heavyweight_spaceship!
  x0A !pad
  d00 d0C
  d00 d21
  d00 d40
  d00 d41
  d00 d7E

compact_pulsar!
  # pattern that turns into a pulsar
  x0C !pad
  d07 dC0
  d08 d40
  d07 dC0

copperhead!
  x08 !pad
  d06 d60
  d01 d80
  d01 d80
  d0A d50
  d08 d10
  d00 d00
  d08 d10
  d06 d60
  d03 dC0
  d00 d00
  d01 d80
  d01 d80
