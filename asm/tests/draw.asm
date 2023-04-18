@ ../../lib/bit.asm
@ ../../lib/core.asm
@ ../../lib/microcomputer/delay.asm
@ ../../lib/microcomputer/input.asm
@ ../../lib/microcomputer/display.asm

main!
  !front_buffer !alloc_buffer
  !reset_input

  x77 # xy_pos
  x00 # xy_vel

  loop:
    # xy_pos += xy_vel
    ld1 ld1 adn st1
    # invert pixel at xy_pos
    !front_buffer ld2 !bit_addr !flip_bit
    # sleep
    x0C :delay_long !call
    # input = *INPUT_BUFFER
    !input_buffer lda
    # input = (1 << prng()) >> 4
    # x01 :seed :prng !call rot x04 !shr
    # ignore if input is empty
    x0F and :ignore !bcs
    # vel = (input & 0b1010) ? 0x0F : 0x01
    ld0 x0A and pop x01 x0F iff
    # rot = (input & 0b0011) ? 0x04 : 0x00
    ld1 x03 and pop x04 x00 iff
    # xy_vel = vel << rot
    rot st1
    # reset the input buffer
    !reset_input
    # pop input
    ignore: pop
  :loop sti

  !delay
  !delay_long
