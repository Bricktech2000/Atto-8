@ lib/microprocessor/core.asm
@ lib/microprocessor/math.asm
@ lib/microprocessor/memory.asm
@ lib/microcomputer/time.asm
@ lib/microcomputer/input.asm
@ lib/microcomputer/display.asm

main!
  pop !front_buffer sts

  # xF0 # prng_seed

  x77 # xy_pos
  x00 # xy_vel

  loop:
    # xy_pos += xy_vel
    ld1 ld1 !adn st1
    # invert pixel at xy_pos
    !front_buffer ld2 !bit_addr !flip_bit
    # sleep
    x03 !delay_long
    # input = *INPUT_BUFFER
    !input_buffer lda
    # input = (1 << prng()) & 0x0F
    # ld2 !prng_minimal st2 x01 ld3 rot x0F and
    # ignore if input is empty
    x0F and :ignore !bcs
    # vel = (input & 0b1010) ? 0x0F : 0x01
    ld0 x0A and pop x01 x0F iff
    # rot = (input & 0b0011) ? 0x04 : 0x00
    ld1 x03 and pop x04 x00 iff
    # xy_vel = vel << rot
    rot st1
    # reset the input buffer
    !reset_input
    # pop input
    ignore: pop
  :loop !jmp
