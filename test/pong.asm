@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm
@ lib/display.asm
@ lib/controller.asm

main!
  # we're putting the bottom of the stack two bytes away from the display buffer
  # because the logic that draws the paddles may overwrite two bytes before or after
  # the display buffer to allow paddle bounds checking to be more efficient
  pop pop !display_buffer dec dec @const sts

  x07 !u8 # paddle_a
  x07 !u8 # paddle_b
  x70 !u4f4 # x_pos
  x08 !i4f4 # x_vel
  x70 !u4f4 # y_pos
  x03 !i4f4 # y_vel

  loop:
    # x_pos += x_vel
    !u8u8.ld1 !u4f4.add !u4f4.st3
    # y_pos += y_vel
    !u8u8.ld0 !u4f4.add !u4f4.st1

    # load (x_pos >> 4, y_pos >> 4) onto the stack
    !u4f4.ld3 !u4f4.in !u4f4.ld2 !u4f4.in x04 rot orr clc !u4u4
      # if (y_pos ^ y_pos << 1) & 0xE0 == 0 { y_vel = -y_vel }
      # this checks if y_pos & 0xF0 is either 0x00 of 0xF0
      !u4f4.ld2 !u4f4.ld0 shl xor xE0 and pop
      !u4f4.ld1 !u4f4.ld0 !u4f4.neg !u4f4.iff !u4f4.st1 clc
      # if (x_pos ^ x_pos << 1) & 0xE0 != 0 { goto ignore_check }
      # this only checks paddle bounces if the ball is on either side of the screen
      !u4f4.ld4 !u4f4.ld0 shl xor xE0 and pop
      :ignore_check !bcc
        # x_vel = -x_vel
        !u4f4.ld3 !u4f4.neg !u4f4.st3
        # if the byte in memory where the ball sits is not 0, game over
        !display_buffer !u4u4.ld1 xF8 and x03 !rneg @const rot add lda
        !is_zero :game_over !bcs
      ignore_check:

      # loop through paddles
      x00 for_paddle:
        # byte_to_store = paddle ? 0x80 : 0x01
        !check_zero x80 x01 iff
        # base_addr = DISPLAY_BUFFER + (paddle ? 2 * paddle_a : 2 * paddle_b + 1)
        !display_buffer !u8.ld9 !u8.ld9 iff ld0 add add
          # draw paddle using `byte_to_store`
          x00 ld1 x04 sub sta
          ld1 ld1 x02 sub sta
          ld1 ld1 x02 add sta
          x00 ld1 x04 add sta
        sta

        # paddle_pos = paddle ? paddle_a : paddle_b
        !check_zero !u8.ld7 !u8.ld7 iff
          # get user input and conditionally swap nibbles depending on `paddle`
          !getc x04 x00 iff rot
          # compute `paddle_vel` based on user input
          x00 # default: `0x00`
            !primary_up xo2 xFF iff !primary_up xo2
            !primary_down xo2 x01 iff # !primary_down xo2
          clc !u8.st0
        # if ((paddle_pos + paddle_vel) & 0x0F != 0x00) paddle_pos += paddle_vel
        # this prevents the paddles from going out of bounds
        !u8.ld1 !u8.add x0F and swp iff
        # store `paddle_pos` to either `paddle_a` or `paddle_b` depending on `paddle`
        x00 xo2 @dyn lds x08 neg @const sub @dyn sta
      not :for_paddle !bcc pop

    # compute bit_addr of (x_pos >> 4, y_pos >> 4) and consume (x_pos >> 4, y_pos >> 4)
    !display_buffer !bit_addr
      # draw pixel at bit_addr then stall
      !u8u8.ld0 !set_bit
      x08 magic_label: !delay
    # erase pixel at bit_addr and consume bit_addr
    !clear_bit
  :loop !jmp

  game_over:
    # stall then move ball closer to the center and resume game
    !u4u4.pop x30 xo4
    x00 x00 !u8u8 # dummy `bit_addr` that will be consumed after `:magic_label`
    xFF # argument to `!delay`
    :magic_label !jmp # cheaper than calling `!delay` again
