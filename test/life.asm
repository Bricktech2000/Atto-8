@ lib/microprocessor/core.asm
@ lib/microprocessor/memory.asm
@ lib/microcomputer/display.asm

# to count neighbours, front buffer is read from and back buffer is written to.
# back buffer is copied to front buffer at the end of each iteration.
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
  pop !back_buffer sts

  loop:
    # copy back buffer to front buffer.
    !front_buffer !back_buffer sub @const !back_buffer !front_buffer :memcpy !call
    # loop through every cell
    x00 for_xy: dec
      x00 # allocate neighbour count

      # count neighbours
      :neighbours_end :neighbours sub @const for_dxdy: dec
        # neighbour_addr = *(neighbours + dxdy) ++ for_xy
        # neighbour_value = load_bit(bit_addr(neighbour_addr, &FRONT_BUFFER))
        !front_buffer :neighbours ld2 add lda ld4 adn !bit_addr !load_bit clc
        # neighbour_count += neighbour_value
        ld2 add st1
      buf :for_dxdy !bcc pop

      # apply rules outlined above
      ld0 x04 xor pop :ignore !bcs
      ld0 x03 xor pop x00 x01 iff !back_buffer ld3 !bit_addr !store_bit
      ignore:

      pop # pop neighbour count
    buf :for_xy !bcc pop
  :loop !jmp

  neighbours:
    dFF dF0 dF1 d0F d00 d01 d1F d10 d11
  neighbours_end:

  !memcpy_def

  # !glider
  # !blinker
  # !r-pentomino
  # !lightweight_spaceship
  !heavyweight_spaceship
  # !copperhead
  # !diehard
  # !compact_pulsar


blinker!
  !back_buffer x0C add @org
  d07 d00

glider!
  !back_buffer x0C add @org
  d07 d00
  d01 d00
  d02 d00

diehard!
  # already advanced 2 generations
  !back_buffer x0C add @org
  d30 d80
  d31 dC0

r-pentomino!
  !back_buffer x0C add @org
  d06 d00
  d0C d00
  d04 d00

lightweight_spaceship!
  !back_buffer x0A add @org
  d00 d09
  d00 d10
  d00 d11
  d00 d1E

heavyweight_spaceship!
  !back_buffer x0A add @org
  d00 d0C
  d00 d21
  d00 d40
  d00 d41
  d00 d7E

compact_pulsar!
  # pattern that turns into a pulsar
  !back_buffer x0C add @org
  d07 dC0
  d08 d40
  d07 dC0

copperhead!
  !back_buffer x08 add @org
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
