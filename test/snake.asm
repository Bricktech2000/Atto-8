@ lib/microprocessor/core.asm
@ lib/microprocessor/math.asm
@ lib/microprocessor/memory.asm
@ lib/microcomputer/time.asm
@ lib/microcomputer/input.asm
@ lib/microcomputer/display.asm

# a few bytes short, but was working before removing `adn`s and having to add `clc`s.
# has always been missing `!delay` call

main!
  pop !front_buffer sts

  x77 !u4u4 # head_pos
  xF0 !i4i4 # head_vel
  x77 !u4u4 # tail_pos
  xF0 !i4i4 # tail_vel

  xF0 # prng_seed
      # food_pos = prng_seed | 0x11

  x00 food: pop
    !prng_minimal

  loop:
    # draw food at food_ps
    !front_buffer ld1 x11 orr !bit_addr !set_bit

    # input = *INPUT_BUFFER
    !input_buffer lda
    # ignore if input is empty
    x0F and :ignore !bcs
      # vel = (input & 0b1010) ? 0x0F : 0x01
      ld0 x0A and pop x01 x0F iff !i4i4
      # rot = (input & 0b0011) ? 0x04 : 0x00
      ld1 x03 and pop x04 x00 iff
      # head_vel = vel << rot
      rot !i4i4.st4
      # reset the input buffer
      !reset_input
    # pop input
    ignore: pop

    x02 for_head_twice: dec
      # head_pas += head_vel
      !u8u8.ld2 !i4i4.add !u4u4.st5
      # if head_pos == food_pos, spawn a new food
      ld1 x11 orr !u4u4.ld6 xor pop :food !bcs
      # compute bit_addr of head_pos
      !front_buffer !u4u4.ld6 !bit_addr
      # game over if pixel at head_pos is set
      !u8u8.ld0 !load_bit buf pop :game_over !bcc
      # set pixel at head_pos
      !set_bit
    buf :for_head_twice !bcc clc pop

    # figure out where the tail is headed
    :directions_end :directions sub @const for_dir: dec
      :directions ld1 add !i4i4.lda !i4i4.ld3
        !front_buffer !u4u4.ld6 !i4i4.ld3 !i4i4.add !bit_addr !load_bit
      buf pop !i4i4.iff !i4i4.st2
    buf :for_dir !bcc pop

    x02 for_tail_twice: dec
      # tail_pas += tail_vel
      !u8u8.ld1 !i4i4.add !u4u4.st3
      # clear pixel at tail_pos
      !front_buffer !u4u4.ld4 !bit_addr !clear_bit
    buf :for_tail_twice !bcc pop

    # sleep
    # x40 !delay
  :loop !jmp

  game_over:
    # !hlt

  directions:
    d01 !i4i4
    d0F !i4i4
    d10 !i4i4
    dF0 !i4i4
  directions_end:
