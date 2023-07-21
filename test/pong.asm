@ lib/microprocessor/core.asm
@ lib/microprocessor/types.asm
@ lib/microprocessor/stdlib.asm
@ lib/microcomputer/stdio.asm
@ lib/microcomputer/display.asm

# missing second paddle, paddle bounds checks and ball bounce randomization

main!
  pop pop !display_buffer sts

  x07 !u8 # paddle_a
  x07 !u8 # paddle_b
  x70 !u4f4 # x_pos
  x08 !i4f4 # x_vel
  x70 !u4f4 # y_pos
  x03 !i4f4 # y_vel

  loop:
    # erase pixel at (x_pos >> 4, y_pos >> 4)
    !display_buffer !u4f4.ld4 !u4f4.in !u4f4.ld3 !u4f4.in x04 rot orr !u4u4 !bit_addr !clear_bit

    # x_pos += x_vel
    !u8u8.ld1 !u4f4.add !u4f4.st3
    # y_pos += y_vel
    !u8u8.ld0 !u4f4.add !u4f4.st1

    # load (x_pos >> 4, y_pos >> 4) onto the stack
    !u4f4.ld3 !u4f4.in !u4f4.ld2 !u4f4.in x04 rot orr !u4u4
      # if (y_pos ^ y_pos << 1) & 0xE0 == 0 { y_vel = -y_vel }
      # this checks if y_pos & 0xF0 is either 0x00 of 0xF0
      !u4f4.ld2 !u4f4.ld0 x01 rot xor xE0 and pop !u4f4.ld1 !u4f4.ld0 !u4f4.neg !u4f4.iff !u4f4.st1
      # if (x_pos ^ x_pos << 1) & 0xE0 != 0 { goto ignore_check }
      # this only checks paddle bounces if the ball is on either side of the screen
      !u4f4.ld4 !u4f4.ld0 x01 rot xor xE0 and pop :ignore_check !bcc
      # if the byte in memory where the ball is not 0, game over
      !display_buffer !u4u4.ld1 x03 !ror clc add lda buf pop :game_over !bcs
      # otherwise, x_vel = -x_vel
      !u4f4.ld3 !u4f4.neg !u4f4.st3
      ignore_check:

      # draw paddle centered at paddle_b
      !display_buffer !u8.ld6 x01 rot clc add inc
      ld0 x04 sub x00 sta
      ld0 x02 sub x01 sta
      ld0         x01 sta
      ld0 x02 add x01 sta
      ld0 x04 add x00 sta
      pop

      # check for input and move paddle_b
      !getchar
      ld0 x03 and pop :check_next !bcs
      !u8.ld6 x01 !u8.sub !u8.ld7 x01 !u8.add ld2 x01 and pop iff !u8.st6
      check_next: pop

      # draw pixel at (x_pos >> 4, y_pos >> 4)
      !display_buffer ld1 !bit_addr !set_bit
    # pop (x_pos >> 4, y_pos >> 4) from the stack
    !u4u4.pop

    x7F !stall

  :loop !jmp

  game_over:
    !hlt

  # !display_buffer @org
  # d80 d00
  # d80 d00
  # d80 d00
  # d80 d00
  # d80 d00
  # d80 d00
  # d80 d00
  # d80 d00
  # d80 d00
  # d80 d00
  # d80 d00
  # d80 d00
  # d80 d00
  # d80 d00
  # d80 d00
  # d80 d00
