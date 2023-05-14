@ lib/microprocessor/bit.asm
@ lib/microprocessor/core.asm
@ lib/microcomputer/delay.asm
@ lib/microcomputer/input.asm
@ lib/microcomputer/display.asm

main!
  pop !front_buffer sts
  !reset_input

  x07 # paddle_a
  x07 # paddle_b
  x70 # x_pos
  x70 # y_pos
  x08 # x_vel
  x03 # y_vel

  loop:
    # erase pixel at (x_pos >> 4, y_pos >> 4)
    !front_buffer ld4 xF0 and x04 !ror ld4 xF0 and orr !bit_addr !clear_bit
    # x_pos, y_pos += x_vel, y_vel
    ld3 ld2 add st3
    ld2 ld1 add st2
    # load (x_pos >> 4, y_pos >> 4) onto the stack
    ld3 xF0 and x04 !ror ld3 xF0 and orr
    # if (y_pos ^ y_pos << 1) & 0xE0 == 0 { y_vel = -y_vel }
    # this checks if y_pos & 0xF0 is either 0x00 of 0xF0
    ld3 ld0 x01 rot xor xE0 and pop ld1 ld0 neg iff st1
    # ld3 buf pop ld1 ld0 neg iff st1
    # if (x_pos ^ x_pos << 1) & 0xE0 != 0 { goto ignore_check }
    # this only checks paddle bounces if the ball is on either side of the screen
    ld4 ld0 x01 rot xor xE0 and pop :ignore_check !bcc
    # if the byte in memory where the ball is is 0, game over
    !front_buffer ld1 x03 !ror clc add lda buf pop :game_over !bcs
    # otherwise, x_vel = -x_vel
    ld2 neg st2
    ignore_check:

    !front_buffer ld6 x01 rot clc add inc
    ld0 x04 sub x00 sta
    ld0 x02 sub x01 sta
    ld0         x01 sta
    ld0 x02 add x01 sta
    ld0 x04 add x00 sta
    pop
    !input_buffer lda
    ld0 x03 and pop :check_next !bcs
    ld6 dec ld7 inc ld2 x01 and pop iff st6
    check_next:
    pop !reset_input

    # draw pixel at (x_pos >> 4, y_pos >> 4)
    !front_buffer ld1 !bit_addr !set_bit

    # pop (x_pos >> 4, y_pos >> 4) from the stack
    pop

    x7F !delay

  :loop !jmp

  game_over:
    !hlt

  # !front_buffer @org
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
