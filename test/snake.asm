@ lib/microprocessor/bit.asm
@ lib/microprocessor/core.asm
@ lib/microprocessor/prng.asm
@ lib/microcomputer/input.asm
@ lib/microcomputer/display.asm
@ lib/microcomputer/delay.asm

# currently a few bytes short, stack runs into program data

main!
  pop !front_buffer sts
  !reset_input

  x77 # head_pos
  xF0 # head_vel
  x77 # tail_pos
  xF0 # tail_vel

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
      # vel = (input & 0b1010) ? 0x01 : 0x0F
      ld0 x0A and pop x01 x0F iff
      # rot = (input & 0b0011) ? 0x04 : 0x00
      ld1 x03 and pop x04 x00 iff
      # head_vel = vel << rot
      rot st4
      # reset the input buffer
      !reset_input
    # pop input
    ignore: pop

    x02 for_head_twice: dec
      # head_pas += head_vel
      ld5 ld5 adn st5
      # if head_pos == food_pos, spawn a new food
      ld1 x11 orr ld6 xor pop :food !bcs
      # compute bit_addr of head_pos
      !front_buffer ld6 !bit_addr
      # game over if pixel at head_pos is set
      ld1 ld1 !load_bit buf pop :game_over !bcc
      # set pixel at head_pos
      !set_bit
    buf :for_head_twice !bcc clc pop

    # figure out where the tail is headed
    :directions_end :directions sub @const for_dir: dec
      :directions ld1 add lda ld3
        !front_buffer ld6 ld3 adn !bit_addr !load_bit
      buf pop iff st2
    buf :for_dir !bcc pop

    x02 for_tail_twice: dec
      # tail_pas += tail_vel
      ld3 ld3 adn st3
      # clear pixel at tail_pos
      !front_buffer ld4 !bit_addr !clear_bit
    buf :for_tail_twice !bcc pop

    # sleep
    # x40 !delay
  :loop !jmp

  game_over:
    # !hlt

  directions:
    d01 d0F d10 dF0
  directions_end:
