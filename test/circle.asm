@ lib/microprocessor/core.asm
@ lib/microprocessor/types.asm
@ lib/microprocessor/stdlib.asm
@ lib/microprocessor/string.asm
@ lib/microcomputer/stdio.asm
@ lib/microcomputer/display.asm

# controls:
#
# - Primary Up -- move circle up
# - Primary Down -- move circle down
# - Primary Left -- move circle left
# - Primary Right -- move circle right
# - Secondary Up -- increase circle radius
# - Secondary Down -- decrease circle radius
# - Secondary Left -- do nothing
# - Secondary Right -- do nothing

main!
  pop pop !display_buffer sts

  x88 !u4u4 # (cx, cy)
  x00 !u8 # r

  # pop previous controller state and fall through
  loop:
    # clear display then draw circle
    !display_buffer_len x00 !display_buffer :memset !call
    ld1 ld1 inc !draw_circle

    x30 !delay

    # wait until a controller button is pressed
    x00 wait: pop !getc !char.null xor :wait !bcs
    x00 # 0x00 as default
      !secondary_up xo2 x01 iff !secondary_up xo2
      !secondary_down xo2 xFF iff !secondary_down xo2
    ad2 x07 an2
    x00 # 0x00 as default
      !primary_up xo2 xF0 iff !primary_up xo2
      !primary_down xo2 x10 iff !primary_down xo2
      !primary_left xo2 xFF iff !primary_left xo2
      !primary_right xo2 x01 iff !primary_right xo2
    st0 ad2
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
    !u4u4.ld1 xF0 and x04 rot !u8.add
    # push (x - 1, y)
    !u4u4.ld1 x01 !u8.sub # x will not underflow
    # push t1 - x
    !u4u4.ld1 !u4u4.ld3 x0F and !u8.sub # will set carry if t1 - x < 0
    # (x, t1) = t1 - x < 0 ? (x, t1) : (x - 1, t1 - x)
    flc !u8u8.iff clc
  # loop while x >= y
  ld1 x04 rot ld2 sub pop .while !bcc
  # return*
  pop pop pop
