@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm
@ lib/display.asm
@ lib/controller.asm

main!
  pop pop !display_buffer sts

  xE7 !u4u4 # tail_pos
  xE7 !u4u4 # head_pos
  xF0 !u8 # head_vel

  # seeds determined through trial and error
  x15 # rand_seed, 14 steps
  # x2A # rand_seed, 13 steps
  # x54 # rand_seed, 12 steps
  # food_pos = rand_seed | 0x11

  x00 not @const # frame parity

  food:
    ld1 !rand.min st1

  loop: not
    x12 !delay

    # draw food at food_pos
    ld1 x11 orr !display_buffer !bit_addr !set_bit

    # default: `head_vel`
    !getc !u8.ld0+3 !primary_to_delta st0
    # on even frame parity, ignore user input
    !u8.ld0+3 ld2 !is_zero !u8.iff !u8.st0+2
    # head_pos += head_vel
    clc ld2 ad4

    # not enough memory to avoid snake wrapping around
    # !u4u4.ld1+2 !u4u4.fst !is_zero :game_over !bcs
    # !u4u4.ld1+2 !u4u4.snd !is_zero :game_over !bcs

    # # if head_pos == food_pos, spawn a new food
    ld1 x11 orr !u4u4.ld1+3 xor pop :food !bcs
    # compute bit_addr of head_pos
    !u4u4.ld1+2 !display_buffer !bit_addr
    # game over if pixel at head_pos is set
    !u8u8.ld0 !load_bit !is_zero :game_over !bcc
    # set pixel at head_pos
    !set_bit

    # clear pixel at tail_pos
    !u4u4.ld4 !display_buffer !bit_addr !clear_bit clc

    # figure out where the tail is headed
    !u4u4.ld4 # default: tail_pos
    !directions_len for_dir: dec
      :directions ld1 add !u8.lda !u4u4.ld7 !u8.add
      !u4u4.ld0 !display_buffer !bit_addr !load_bit
      shr @dyn pop if2
    !check_zero :for_dir !bcc pop

    # save computed `tail_pos`
    !u4u4.st4

  :loop !jmp

  game_over:
    !hlt

  directions:
    @F0 !i4i4
    @10 !i4i4
    @FF !i4i4
    @01 !i4i4
  directions_end:

  !display_buffer @org
    # not enough memory to avoid snake wrapping around.
    # i4i4 addition breaks when wrapping horizontally.
    # this one-pixel border prevents wrapping by making
    # the snake believe it has run into its body
    @00 @00
    @7F @FF
    @40 @01
    @40 @01
    @40 @01
    @40 @01
    @40 @01
    @40 @01
    @40 @01
    @40 @01
    @40 @01
    @40 @01
    @40 @01
    @40 @01
    @40 @01
    @7F @FF

directions_len! :directions_end :directions sub @const
