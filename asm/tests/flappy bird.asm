@ ../../lib/bit.asm
@ ../../lib/core.asm
@ ../../lib/prng.asm
@ ../../lib/memcpy.asm
@ ../../lib/microcomputer/delay.asm
@ ../../lib/microcomputer/input.asm
@ ../../lib/microcomputer/display.asm

main!
  pop !front_buffer sts
  !reset_input

  x00 # x_pos
  x80 # y_pos
  x02 # x_vel
  xF0 # y_vel

  xFF # argument to `delay_long`

  loop:
    !delay_long

    # set y_vel to flap_vel if W is pressed
    !input_buffer lda x01 xor pop !flap_vel iff !reset_input
    # clear pixel at (x_pos, y_pos)
    !front_buffer ld3 x04 !shr x01 shf add x07 !bird_pos sub @const !clear_bit

    # x_pos += x_vel
    ld3 ld2 add st3
    # y_vel += !y_accel
    ld0 !y_accel add st0
    # y_pos += y_vel
    ld2 ld1 add st2

    # if x_pos % 0x10 == 0, shift entire screen left by 1 pixel
    ld3 x0F and pop :ignore_shift !bcc
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
    ld3 x7F and pop :ignore_pipe !bcc
    # fill right side of the screen with 0x01
    x0C for_i: dec
      ld0 x01 shf xE1 add x01 sta
    buf :for_i !bcc pop
    # remove a few pixels at a random height
    :seed :prng !call
    x01 orr x0F and !front_buffer add
    ld0 x04 add x00 sta
    ld0 x02 add x00 sta
    x00 sta
    ignore_pipe:

    x07 # argument to `delay_long`
  :loop sti

  game_over:
    !hlt

  seed: d80

  !prng_minimal

  !front_buffer @org
  d00 d00 d00 d00 d00 d00 d00 d00
  d00 d00 d00 d00 d00 d00 d00 d00
  d00 d00 d00 d00 d00 d00 d00 d00
  dFF dFF dFD dFB d6F d7F dDB dB5

bird_pos! x02 # from left edge of the screen
y_accel! x01 # gravity
flap_vel! xF8 # upward velocity when flapping
