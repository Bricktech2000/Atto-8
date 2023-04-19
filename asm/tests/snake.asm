@ ../../lib/bit.asm
@ ../../lib/core.asm
@ ../../lib/microcomputer/input.asm
@ ../../lib/microcomputer/display.asm
# @ ../../lib/microcomputer/delay.asm

main!
  !front_buffer !alloc_buffer
  !reset_input

  x77 # head_pos
  xF0 # head_vel
  x77 # tail_pos
  xF0 # tail_vel

  loop:
    x02 for_head_twice: dec
    # head_pas += head_vel
    ld4 ld4 adn st4
    # compute bit_addr of head_pos
    !front_buffer ld5 !bit_addr
    # game over if pixel at head_pos is set
    ld1 ld1 !load_bit buf pop :game_over !bcc
    # set pixel at head_pos
    !set_bit
    buf :for_head_twice !bcc pop

    :directions_end :directions sub @const for_dir: dec
    :directions ld1 add lda ld2
    !front_buffer ld5 :directions ld5 add lda adn !bit_addr !load_bit
    buf pop iff st1
    buf :for_dir !bcc pop

    x02 for_tail_twice: dec
    # tail_pas += tail_vel
    ld2 ld2 adn st2
    # clear pixel at tail_pos
    !front_buffer ld3 !bit_addr !clear_bit
    buf :for_tail_twice !bcc pop

    # sleep
    # x40 !delay_long

    # input = *INPUT_BUFFER
    !input_buffer lda
    # ignore if input is empty
    x0F and :ignore !bcs
    # vel = (input & 0b1010) ? 0x01 : 0x0F
    ld0 x0A and pop x01 x0F iff
    # rot = (input & 0b0011) ? 0x04 : 0x00
    ld1 x03 and pop x04 x00 iff
    # head_vel = vel << rot
    rot st3
    # reset the input buffer
    !reset_input
    # pop input
    ignore: pop
  :loop sti

  game_over:
    !hlt

  directions:
    d01 d0F d10 dF0
  directions_end:
