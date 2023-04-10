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

@../../lib/bit.asm
@../../lib/core.asm
@../../lib/memcpy.asm
@../../lib/microcomputer/core.asm

main!
  !back_buffer !alloc_buffer

  # diehard (advanced 2 generations)
  # !back_buffer x09 add xC2 sta
  # !back_buffer x0B add xC7 sta
  # blinker
  # !back_buffer x0C add x07 sta
  # glider
  # !back_buffer x08 add x07 sta
  # !back_buffer x0A add x01 sta
  # !back_buffer x0C add x02 sta
  # r-pentomino
  # !back_buffer x0B add x06 sta
  # !back_buffer x0D add x0C sta
  # !back_buffer x0F add x04 sta
  # lightweight spaceship
  # !back_buffer x0B add x09 sta
  # !back_buffer x0D add x10 sta
  # !back_buffer x0F add x11 sta
  # !back_buffer x11 add x1E sta
  # name TODO
  # !back_buffer x0C add x1F sta
  # !back_buffer x0E add x21 sta
  # !back_buffer x10 add x1F sta

  loop:
    # copy back buffer to front buffer.
    !front_buffer !back_buffer sub !back_buffer !front_buffer :memcpy !call
    # loop through every cell
    x00 for_xy: dec
      x00 # allocate neighbour count

      # count neighbours
      x02 for_dx: dec
        x20 for_dy: x10 sub
          # neighbour = (for_xy + for_dx & 0x0F) | (for_xy + for_dy & 0xF0)
          !front_buffer ld4 ld3 add x0F and ld5 ld3 add xF0 and orr :load_bit !call
          # neighbour_count += neighbour
          ld3 add st2
        ld0 xF0 xor pop :for_dy !bcc pop
      ld0 xFF xor pop :for_dx !bcc pop

      # apply rules outlined above
      ld0 x04 xor pop :ignore !bcs
      ld0 x03 xor pop x00 x01 iff !back_buffer ld3 :store_bit !call
      ignore:

      pop # pop neighbour count
    buf :for_xy !bcc pop
  :loop sti

  !memcpy
  !bit_addr
  !load_bit
  !store_bit

  # pattern that turns into pulsar
  d00 d00 d00 d00 d00 d00 d00 d00
  d00 d00 d00 d00 d00 d00 d00 d00
  d00 d00 d00 d00 d00 d00 d00 d00
  d00 d00 d00 d00 d00 d00 d00 d00
  d07 dC0 d08 d40 d07 dC0

  # spaceship
  # d00 d00 d00 d00 d00 d00 d00 d00
  # d00 d00 d00 d00 d00 d00 d00 d00
  # d00 d00 d00 d00 d00 d00 d00 d00
  # d06 d60
  # d01 d80
  # d01 d80
  # d0A d50
  # d08 d10
  # d00 d00
  # d08 d10
  # d06 d60
  # d03 dC0
  # d00 d00
  # d01 d80
  # d01 d80
