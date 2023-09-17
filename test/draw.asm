@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm
@ lib/display.asm
@ lib/controller.asm

main!
  pop pop !display_buffer sts

  # xF0 # rand_seed

  x77 !u4u4 # xy_pos
  x00 !u8 # xy_vel

  loop:
    # input = getc()
    !getc
    # input = (1 << rand()) & 0x0F
    # ld2 !rand.min st2 ld2 x01 swp rot x0F and

    # default: `xy_vel`
    swp !primary_to_delta st0
    # xy_pos += xy_vel
    clc ld0 ad2

    # invert pixel at xy_pos
    !u4u4.ld1 !display_buffer !bit_addr !flip_bit

    x20 !delay
  :loop !jmp
