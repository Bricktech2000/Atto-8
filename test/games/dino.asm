@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm
@ lib/display.asm
@ misc/common/common.asm

main!
  pop pop !display_buffer sts

  !rand_seed # rand_seed

  !0u8u8 # cactus_top, cactus_bot

  x00 !u4f4 # x_pos
  x10 !i4f4 # x_vel
  xB0 !u4f4 # y_pos
  x00 !i4f4 # y_vel

  loop:
    # set y_vel to jump_vel if any button is pressed and y_pos == GROUND_POS
    !getc !char.is_null !jump_vel !i4f4.ld1 !i4f4.iff !u4f4.ld2 !ground_pos !eq !i4f4.iff
    # compute bit_addr of (DINO_POS, y_pos)
    !display_buffer !u4f4.ld2 !u4f4.in !u4f4.shl orr x07 !dino_pos sub @const
    # clear pixel at (DINO_POS, y_pos)
    !clear_bit clc

    # x_pos += x_vel
    !u8u8.ld1 !u4f4.add !u4f4.st3
    # y_vel += !y_accel
    !y_accel !u4f4.add
    # y_pos += y_vel
    !u8u8.ld0 !u4f4.add !u4f4.st1
    # if y_pos > GROUND_POS, (y_pos, y_vel) = (GROUND_POS, 0x00)
    !ground_pos !u4f4.ld2 !gt !ground_pos x00 !i4f4 !u16.iff

    # shift bottom half of screen left by 1 pixel,
    # regardless of x_vel because we're out of memory
    !display_buffer x10 add @const for_addr:
      ld0 inc lda shl @dyn pop # load carry
      ld0 lda shl ld1 sta inc
      ld0 lda shl ld1 sta inc
    !z :for_addr !bcc pop

    # shift in cactus from cactus_top and cactus_bot
    !display_buffer x17 add @const lda
    shr clc !u8.ld5 !u8.shl !u8.st5 shl
    !display_buffer x17 add @const sta
    !display_buffer x15 add @const lda
    shr clc !u8.ld6 !u8.shl !u8.st6 shl
    !display_buffer x15 add @const sta

    # compute bit_addr of (DINO_POS, y_pos)
    !display_buffer !u4f4.ld2 !u4f4.in !u4f4.shl orr x07 !dino_pos sub @const
    # if pixel at (DINO_POS, y_pos) is set, game over
    !u8u8.ld0 !load_bit !zr :game_over !bcc
    # set pixel at (DINO_POS, y_pos)
    !set_bit

    # if x_pos % 0x100 == 0, generate a new cactus
    !u4f4.ld3 !zr :ignore_cactus !bcc
      # generate a pointer to a random cactus
      # the x06 (0b00000110) below requires 4 cacti
      # x0E (0x00001110) could be used for 8 cacti
      ld6 !rand st6 ld6
      x06 and clc :cacti !ofst
      # copy cactus data to cactus_top and cactus_bot
      !u16.lda !u16.st2
    ignore_cactus:

    x07 !delay
  :loop !jmp

  game_over:
    # invert screen
    !display_buffer for_i:
      ld0 lda not ld1 sta
    inc !z !here !bcs :for_i !jmp

  cacti:
  # top bot
    @04 @05 !u8u8 # ______:.
    @10 @50 !u8u8 # __.:____
    @80 @80 !u8u8 # :_______
    @00 @00 !u8u8 # ________

  !display_buffer @org
    # !deep_void
    !planet_and_stars
    # !clouds_and_sun
    # !stars_and_moon
    # !day_desert
    !night_desert

dino_pos! x02 # from left edge of the screen
y_accel! x04 !i4f4 # gravity
jump_vel! xE8 !i4f4 # upward velocity when jumping
ground_pos! xB0 !u8f8 # y_pos of the ground
