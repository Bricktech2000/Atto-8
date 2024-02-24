@ lib/core.asm
@ lib/types.asm
@ lib/string.asm
@ lib/stdlib.asm
@ lib/stdio.asm
@ lib/display.asm
@ lib/controller.asm

# controls:
# - Primary Up -- move circle up
# - Primary Down -- move circle down
# - Primary Left -- move circle left
# - Primary Right -- move circle right
# - Secondary Up -- do nothing
# - Secondary Down -- do nothing
# - Secondary Left -- decrease circle radius
# - Secondary Right -- increase circle radius

main!
  pop pop !display_buffer sts

  x88 !u4u4 # (cx, cy)
  x00 !u8 # r

  # pop previous controller state and fall through
  loop:
    # clear display then draw circle
    !display_buffer.len x00 !display_buffer :memset !call
    ld1 ld1 inc !draw_circle # offsets `r` from `0..8` to `1..9`

    x18 !delay

    # wait for controller input
    !block_getc

    x00 ld1 !secondary_to_delta clc ad2
    x07 an2 # clamps `r` to `0..8`
    x00 ld1 !primary_to_delta clc st0 ad2
  :loop !jmp

  !memset.def


# Jesko's method

draw_circle! # draw_circle(u8 r, u4u4 cycy)
  # (x, y) = (r, 0)
  nop !u4u4
  # t1 = r / 16
  x02 !u8 # magic constant 0x02 appears to work well
  while.
    # set_pixels(bid_addr((x, y) + (cx, cy)))
    x00 for_neg_y. # will be 0x00 then 0xF0
      x00 for_neg_x. # will be 0x00 then 0x0F
        x00 for_swap. # will be 0x00 then 0x04
          # load (cx, cy)
          !u4u4.ld2+3
          # compute (x_, x_) = (+-x, +-y) or (+-y, +-x)
          !u4u4.ld2+3 ld2 rot ld4 ld4 xor xor
          # draw pixel at (cx + x_, cy + x_)
          !u4u4.add !display_buffer !bit_addr !set_bit
        x04 xor .for_swap !bcc pop
      x0F xor .for_neg_x !bcc pop
    xF0 xor .for_neg_y !bcc pop
    # y += 1
    x10 dec ad2 # dec because carry was set above
    # t1 += y
    !u4u4.ld1 !u4u4.fst !u8.add
    # push (x - 1, y)
    !u4u4.ld1 x01 !u8.sub # x will not underflow
    # push t1 - x
    !u4u4.ld1 !u4u4.ld3 !u4u4.snd !u8.sub # will set carry if t1 - x < 0
    # (x, t1) = t1 - x < 0 ? (x, t1) : (x - 1, t1 - x)
    flc !u8u8.iff clc
  # loop while x >= y
  ld1 x04 rot ld2 !gt .while !bcc
  # return*
  pop pop pop
