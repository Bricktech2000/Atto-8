@ ../../lib/microprocessor/bit.asm
@ ../../lib/microprocessor/core.asm
@ ../../lib/microprocessor/prng.asm
@ ../../lib/microprocessor/memcpy.asm
@ ../../lib/microcomputer/delay.asm
@ ../../lib/microcomputer/input.asm
@ ../../lib/microcomputer/display.asm

main!
  pop !front_buffer sts
  !reset_input

  x00 # x_pos
  x02 # x_vel
  x80 # y_pos
  xF0 # y_vel

  xF0 # prng_seed

  loop:
    # set y_vel to flap_vel if any button is pressed
    !input_buffer lda buf pop !flap_vel ld2 iff st1 !reset_input
    # clear pixel at (x_pos, y_pos)
    !front_buffer ld3 xF0 and x05 rot orr x07 !bird_pos sub @const !clear_bit clc

    # x_pos += x_vel
    ld4 ld4 add st4
    # y_vel += !y_accel
    ld1 !y_accel add st1
    # y_pos += y_vel
    ld2 ld2 add st2

    # if x_pos % 0x10 == 0, shift entire screen left by 1 pixel
    ld4 x0F and pop :ignore_shift !bcc
    !front_buffer for_addr:
      ld0 inc lda shl pop # load carry
      ld0 ld0 lda shl sta inc
      ld0 ld0 lda shl sta inc
    buf :for_addr !bcc pop
    ignore_shift:

    # compute bit_addr of (x_pos, y_pos)
    !front_buffer ld3 xF0 and x05 rot orr x07 !bird_pos sub @const
    # if pixel at (x_pos, y_pos) is set, game over
    ld1 ld1 !load_bit buf pop :game_over !bcc
    # set pixel at (x_pos, y_pos)
    !set_bit

    # if x_pos % 0x80 == 0, generate a new pipe
    ld4 x7F and pop :ignore_pipe !bcc
    # fill right side of the screen with 0x01
    x0C for_i: dec
      !front_buffer ld1 x01 rot orr inc x01 sta
    buf :for_i !bcc pop
    # remove a few pixels at a random height
    !prng_minimal ld0
    x01 orr x0F and !front_buffer x04 add add
    ld0 x02 add x00 sta
    ld0 x02 sub x00 sta
    x00 sta
    ignore_pipe:

    x60 !delay
  :loop !jmp

  game_over:
    # blink pixel at (x_pos, y_pos)
    blink:
    ld1 ld1 !flip_bit
    xFF !delay
    :blink !jmp

  !front_buffer @org
  d00 d00 d00 d00 d00 d00 d00 d00
  d00 d00 d00 d00 d00 d00 d00 d00
  d00 d00 d00 d00 d00 d00 d00 d00
  dFF dFF dFD dFB d6F d7F dDB dB5

bird_pos! x02 # from left edge of the screen
y_accel! x01 # gravity
flap_vel! xF8 # upward velocity when flapping
