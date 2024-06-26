@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm
@ lib/display.asm
@ misc/common/common.asm

main!
  pop pop !display_buffer sts
  x00 # previous char

  !rand_seed # rand_seed

  x00 !u4f4 # x_pos
  x04 !i4f4 # x_vel
  x80 !u4f4 # y_pos
  xF0 !i4f4 # y_vel

  loop:
    # set y_vel to flap_vel if no button was pressed previously but some button is pressed now
    !getc !char.check_null !i4f4.ld1 !flap_vel !i4f4.ld3 !i4f4.iff ld8 !zr !i4f4.iff !i4f4.st1 st5
    # compute bit_addr of (BIRD_POS, y_pos)
    !display_buffer !u4f4.ld2 !u4f4.in clc !u4f4.shl orr x07 !bird_pos sub @const
    # clear pixel at (BIRD_POS, y_pos)
    !clear_bit clc

    # x_pos += x_vel
    !u8u8.ld1 !u4f4.add !u4f4.st3
    # y_vel += !y_accel
    !y_accel !u4f4.add
    # y_pos += y_vel
    !u8u8.ld0 !u4f4.add !u4f4.st1

    # if x_pos % 0x10 == 0, rotate entire screen left by 1 pixel
    !u4f4.ld3 x0F !cl :ignore_shift !bcc
      !display_buffer for_addr:
        ld0 inc lda shl @dyn pop # load carry
        ld0 lda shl ld1 sta inc
        ld0 lda shl ld1 sta inc
      !z :for_addr !bcc pop
    ignore_shift:

    # compute bit_addr of (BIRD_POS, y_pos)
    !display_buffer !u4f4.ld2 !u4f4.in clc !u4f4.shl orr x07 !bird_pos sub @const
    # if pixel at (BIRD_POS, y_pos) is set, game over
    !u8u8.ld0 !load_bit !z :game_over !bcc pop
    # set pixel at (BIRD_POS, y_pos)
    !set_bit

    # if x_pos % 0x80 == 0, generate a new pipe
    !u4f4.ld3 x7F !cl :ignore_pipe !bcc
      # fill right side of the screen with 0x01
      x0C for_i: dec
        x01 !display_buffer ld2 x01 rot orr inc sta
      !z :for_i !bcc pop
      # remove a few pixels at a random height
      ld4 !rand st4 ld4
      x00 swp # for `sta` below
      x01 orr x0F and !display_buffer x04 add @const add
      x00 ld1 x02 add sta
      x00 ld1 x02 sub sta
      sta
    ignore_pipe:

    x07 !delay
  :loop !jmp

  game_over:
    # blink pixel at (BIRD_POS, y_pos)
    blink: pop
      !u8u8.ld0 !flip_bit
      # `!delay`, but with optimized conditional jump
      x20 delay: x1F !stall x01 sub @dyn :delay !bcc
    :blink !jmp

  !display_buffer @org
    !deep_void
    !day_desert
    # !night_desert

bird_pos! x02 # from left edge of the screen
y_accel! x01 !i4f4 # gravity
flap_vel! xF8 !i4f4 # upward velocity when flapping
