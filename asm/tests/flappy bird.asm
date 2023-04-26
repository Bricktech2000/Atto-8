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

  xFF # argument to `delay_long`

  loop:
    !delay_long

    # set y_vel to flap_vel if W is pressed
    !input_buffer lda x01 xor pop ld1 !flap_vel iff st1 !reset_input
    # clear pixel at (x_pos, y_pos)
    !front_buffer ld3 x04 !shr x01 shf add x07 !bird_pos sub @const !clear_bit

    # x_pos += x_vel
    ld4 ld4 add st4
    # y_vel += !y_accel
    ld1 !y_accel add st1
    # y_pos += y_vel
    ld2 ld2 add st2

    # if x_pos % 0x10 == 0, shift entire screen left by 1 pixel
    ld4 x0F and pop :ignore_shift !bcc
    !front_buffer for_addr:
    ld0 lda ld1 inc lda
    x01 x01 shf2 sfc2 x00 adc
    ld2 inc swp sta
    ld1 swp sta
    x02 add :for_addr !bcc pop
    ignore_shift:

    # compute bit_addr of (x_pos, y_pos)
    !front_buffer ld3 x04 !shr x01 shf add x07 !bird_pos sub @const
    # if pixel at (x_pos, y_pos) is set, game over
    ld1 ld1 !load_bit buf pop :game_over !bcc
    # set pixel at (x_pos, y_pos)
    !set_bit

    # if x_pos % 0x80 == 0, generate a new pipe
    ld4 x7F and pop :ignore_pipe !bcc
    # fill right side of the screen with 0x01
    x0C for_i: dec
      ld0 x01 shf xE1 add x01 sta
    buf :for_i !bcc pop
    # remove a few pixels at a random height
    !prng_minimal ld0
    x01 orr x0F and !front_buffer x04 add add
    ld0 x02 add x00 sta
    ld0 x02 sub x00 sta
    x00 sta
    ignore_pipe:

    x08 # argument to `delay_long`
  :loop sti

  game_over:
    !hlt

  !front_buffer @org
  d00 d00 d00 d00 d00 d00 d00 d00
  d00 d00 d00 d00 d00 d00 d00 d00
  d00 d00 d00 d00 d00 d00 d00 d00
  dFF dFF dFD dFB d6F d7F dDB dB5

bird_pos! x02 # from left edge of the screen
y_accel! x01 # gravity
flap_vel! xF8 # upward velocity when flapping
