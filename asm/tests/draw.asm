@ ../../lib/bit.asm
@ ../../lib/core.asm
@ ../../lib/microcomputer/core.asm
@ ../../lib/microcomputer/delay.asm

main!
  !front_buffer !alloc_buffer
  !reset_input

  x77 # xy_pos
  x00 # xy_vel

  loop:
    # xy_pos += xy_vel
    ld1 ld1 adn st1
    # invert pixel at xy_pos
    !front_buffer ld2 :load_bit !call
    x01 xor !front_buffer ld3 :store_bit !call
    # input = *INPUT_BUFFER
    !input_buffer lda
    # input = (1 << prng()) >> 4
    # x01 :prng !call rot x04 !shr
    # ignore if input is empty
    x0F and :ignore !bcs
    # velocity = (input & 0b1010) ? 0x0F : 0x01
    ld0 x0A and pop x01 x0F iff
    # rot = (input & 0b0011) ? 0x04 : 0x00
    ld1 x03 and pop x04 x00 iff
    # xy_vel = velocity << rot
    rot st1
    # reset the input buffer
    !reset_input
    # pop input
    ignore: pop
    # sleep
    x0C :delay_long !call
  :loop sti

  !delay
  !delay_long
  !bit_addr
  !store_bit
  !load_bit
