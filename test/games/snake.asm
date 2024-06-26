@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm
@ lib/display.asm
@ lib/controller.asm

main!
  pop pop !display_buffer sts

  x37 !u4u4 # tail_pos
  x27 !u4u4 # head_pos
  x10 !u8 # head_vel

  x26 # rand_seed
      # food_pos = rand_seed | 0x11

  x00 not @const # frame parity

  food:
    ld1
      # increment score
      x00 ad8 @dyn # clears carry
      # `!rand`, but with no `inc` and hand-tuned `!rand_seed` and `!rand_bits`
      # that maximize cycle length while ensuring food never spawns on a wall
      shl x01 x00 iff xor
    st1

  loop: not
    x14 !delay

    # default: `head_vel`
    !u8.ld2 !getc !primary_to_delta
    # on even frame parity, ignore user input
    !u8.ld3 ld2 !zr !u8.iff !u8.st2
    # head_pos += head_vel
    clc ld2 ad4

    # not enough memory to avoid snake wrapping around
    # !u4u4.ld3 !u4u4.fst !zr !here !bcs
    # !u4u4.ld3 !u4u4.snd !zr !here !bcs

    # draw food at `food_pos`
    ld1 x11 orr ld0 !display_buffer !bit_addr !set_bit # bleeds `food_pos` onto the stack

    # if head_pos == food_pos, spawn new food
    !u4u4.ld4 !eq :food !bcs # consumes `food_pos` from the stack
    # compute bit_addr of head_pos
    !u4u4.ld3 !display_buffer !bit_addr
    # game over if pixel at head_pos is set
    !u8u8.ld0 !load_bit !zr !here !bcc
    # set pixel at head_pos
    !set_bit

    # clear pixel at tail_pos
    !u4u4.ld4 !display_buffer !bit_addr !clear_bit clc

    # figure out where the tail is headed
    !u4u4.ld4 # default: tail_pos
    !directions.len for_dir: dec
      # compute test_pos = tail_pos + direction
      :directions ld1 add !u8.lda !u4u4.ld7 !u8.add
      # load pixel at test_pos
      !u4u4.ld0 !display_buffer !bit_addr !load_bit
      # store test_pos if pixel is set
      !nzr if2
    !z :for_dir !bcc pop

    # save computed `tail_pos`
    !u4u4.st4

    # not enough memory to check if food spawned on snake body.
    # therefore, if food_pos == tail_pos, generate new food somewhere else.
    # otherwise, the algorithm above confuses food for the snake body.
    ld1 x11 orr !u4u4.ld5 !eq :food !bcs
  :loop !jmp

  directions:
    @F0 !i4i4
    @10 !i4i4
    @FF !i4i4
    @01 !i4i4
  directions.end:

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

directions.len! :directions.end :directions sub @const
