@ lib/microprocessor/core.asm
@ lib/microprocessor/types.asm
@ lib/microprocessor/stdlib.asm
@ lib/microcomputer/stdio.asm
@ lib/microcomputer/display.asm

# a few bytes short, but was working before removing `adn`s and having to add `clc`s.
# has always been missing `!delay` call

main!
  pop !display_buffer sts

  x77 !u4u4 # head_pos
  xF0 !i4i4 # head_vel
  x77 !u4u4 # tail_pos
  xF0 !i4i4 # tail_vel

  xF0 # rand_seed
      # food_pos = rand_seed | 0x11

  x00 food: pop
    !rand.min

  loop:
    # draw food at food_ps
    ld0 x11 orr !display_buffer !bit_addr !set_bit

    # input = getc()
    !getc
    # ignore if input is empty
    x0F and :ignore !bcs
      # vel = (input & 0b1010) ? 0x0F : 0x01
      ld0 !primary_down !primary_right orr and pop x01 x0F iff !i4i4
      # rot = (input & 0b0011) ? 0x04 : 0x00
      ld1 !primary_up !primary_down orr and pop x04 x00 iff
      # head_vel = vel << rot
      rot !i4i4.st4
    # pop input
    ignore: pop

    x02 for_head_twice: dec
      # head_pas += head_vel
      !u8u8.ld2 !i4i4.add !u4u4.st5
      # if head_pos == food_pos, spawn a new food
      ld1 x11 orr !u4u4.ld6 xor pop :food !bcs
      # compute bit_addr of head_pos
      !u4u4.ld5 !display_buffer !bit_addr
      # game over if pixel at head_pos is set
      !u8u8.ld0 !load_bit buf pop :game_over !bcc
      # set pixel at head_pos
      !set_bit
    buf :for_head_twice !bcc clc pop

    # figure out where the tail is headed
    !directions_len for_dir: dec
      :directions ld1 add !i4i4.lda !i4i4.ld3
        !u4u4.ld5 !i4i4.ld2 !i4i4.add !display_buffer !bit_addr !load_bit
      buf pop !i4i4.iff !i4i4.st2
    buf :for_dir !bcc pop

    x02 for_tail_twice: dec
      # tail_pas += tail_vel
      !u8u8.ld1 !i4i4.add !u4u4.st3
      # clear pixel at tail_pos
      !u4u4.ld3 !display_buffer !bit_addr !clear_bit
    buf :for_tail_twice !bcc pop

    # sleep
    # x40 !delay
  :loop !jmp

  game_over:
    # !hlt

  directions:
    @01 !i4i4
    @0F !i4i4
    @10 !i4i4
    @F0 !i4i4
  directions_end:
directions_len! :directions_end :directions sub @const
