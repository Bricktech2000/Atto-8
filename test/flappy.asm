@ lib/microprocessor/core.asm
@ lib/microprocessor/types.asm
@ lib/microprocessor/stdlib.asm
@ lib/microcomputer/stdio.asm
@ lib/microcomputer/display.asm

main!
  pop pop !front_buffer sts
  x00 # previous char

  xF0 # prng_seed

  x00 !u4f4 # x_pos
  x02 !i4f4 # x_vel
  x80 !u4f4 # y_pos
  xF0 !i4f4 # y_vel

  loop:
    # set y_vel to flap_vel if no button was pressed previously but some button is pressed now
    !getchar buf !i4f4.ld1 !flap_vel !i4f4.ld3 !i4f4.iff ld8 buf pop !i4f4.iff !i4f4.st1 st5
    # compute bit_addr of (BIRD_POS, y_pos)
    !front_buffer !u4f4.ld2 !u4f4.in clc !u4f4.shl orr x07 !bird_pos sub @const
    # clear pixel at (x_pos, y_pos)
    !clear_bit clc

    # x_pos += x_vel
    !u8u8.ld1 !u4f4.add !u4f4.st3
    # y_vel += !y_accel
    !y_accel !u4f4.add
    # y_pos += y_vel
    !u8u8.ld0 !u4f4.add !u4f4.st1

    # if x_pos % 0x10 == 0, rotate entire screen left by 1 pixel
    !u4f4.ld3 x0F and pop :ignore_shift !bcc
      !front_buffer for_addr:
        ld0 inc lda shl pop # load carry
        ld0 ld0 lda shl sta inc
        ld0 ld0 lda shl sta inc
      buf :for_addr !bcc pop
    ignore_shift:

    # compute bit_addr of (BIRD_POS, y_pos)
    !front_buffer !u4f4.ld2 !u4f4.in clc !u4f4.shl orr x07 !bird_pos sub @const
    # if pixel at (x_pos, y_pos) is set, game over
    !u8u8.ld0 !load_bit buf pop :game_over !bcc
    # set pixel at (x_pos, y_pos)
    !set_bit

    # if x_pos % 0x80 == 0, generate a new pipe
    !u4f4.ld3 x7F and pop :ignore_pipe !bcc
      # fill right side of the screen with 0x01
      x0C for_i: dec
        !front_buffer ld1 x01 rot orr inc x01 sta
      buf :for_i !bcc pop
      # remove a few pixels at a random height
      ld4 !prng_minimal st4 ld4
      x01 orr x0F and !front_buffer x04 add add
      ld0 x02 add x00 sta
      ld0 x02 sub x00 sta
      x00 sta
    ignore_pipe:

    x60 !stall
  :loop !jmp

  game_over:
    # blink pixel at (x_pos, y_pos)
    blink:
      !u8u8.ld0 !flip_bit
      xFF !stall
    :blink !jmp

  !front_buffer @org
  !void
  !light_ground
  # !dark ground

bird_pos! x02 # from left edge of the screen
y_accel! x01 !i4f4 # gravity
flap_vel! xF8 !i4f4 # upward velocity when flapping

void!
  d00 d00 d00 d00 d00 d00 d00 d00
  d00 d00 d00 d00 d00 d00 d00 d00

dark_ground!
  d00 d00 d00 d00 d00 d00 d00 d00
  dFF dFF d02 d04 d90 d80 d24 d4A

light_ground!
  d00 d00 d00 d00 d00 d00 d00 d00
  dFF dFF dFD dFB d6F d7F dDB dB5
