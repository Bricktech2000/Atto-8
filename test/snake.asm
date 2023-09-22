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

  xCD # rand_seed
      # food_pos = rand_seed | 0x11

  x00 not @const # frame parity

  food:
    ld1
      # not enough memory to increment score
      # x00 ad8 @dyn # clears carry
      # `!rand.min`, but with hand-tuned `rand_seed` and `!rand_bits` that
      # maximizes cycle length while ensuring food never spawns on a wall
      shl x01 x00 iff xor
    st1

  loop: not
    x12 !delay

    # default: `head_vel`
    !getc !u8.ld3 !primary_to_delta st0
    # on even frame parity, ignore user input
    !u8.ld3 ld2 !is_zero !u8.iff !u8.st2
    # head_pos += head_vel
    clc ld2 ad4

    # not enough memory to avoid snake wrapping around
    # !u4u4.ld3 !u4u4.fst !is_zero !here !bcs
    # !u4u4.ld3 !u4u4.snd !is_zero !here !bcs

    # draw food at `food_pos`
    ld1 x11 orr ld0 !display_buffer !bit_addr !set_bit # bleeds `food_pos` onto the stack

    # if head_pos == food_pos, spawn new food
    !u4u4.ld4 xor pop :food !bcs # consumes `food_pos` from the stack
    # compute bit_addr of head_pos
    !u4u4.ld3 !display_buffer !bit_addr
    # game over if pixel at head_pos is set
    !u8u8.ld0 !load_bit !is_zero !here !bcc
    # set pixel at head_pos
    !set_bit

    # clear pixel at tail_pos
    !u4u4.ld4 !display_buffer !bit_addr !clear_bit clc

    # figure out where the tail is headed
    !u4u4.ld4 # default: tail_pos
    !directions_len for_dir: dec
      # compute test_pos = tail_pos + direction
      :directions ld1 add !u8.lda !u4u4.ld7 !u8.add
      # load pixel at test_pos
      !u4u4.ld0 !display_buffer !bit_addr !load_bit
      # store test_pos if pixel is set
      shr @dyn pop if2
    !check_zero :for_dir !bcc pop

    # save computed `tail_pos`
    !u4u4.st4

    # not enough memory to check if food spawned on snake body.
    # therefore, if food_pos == tail_pos, generate new food somewhere else.
    # otherwise, the algorithm above confuses food for the snake body.
    ld1 x11 orr !u4u4.ld5 xor pop
  :loop :food iff !jmp

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
