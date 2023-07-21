@ lib/microprocessor/core.asm
@ lib/microprocessor/types.asm
@ lib/microcomputer/display.asm

# using display buffer as extra memory. missing `size` and `pos` from C implementation

main!
  pop pop !display_buffer x10 add @const sts

  loop:
    x80 !u4u4 for_xy: dec
      !u4u4.ld0 !u4u4.snd x03 rot clc x40 sub !i4f4 # c_re
      x00 !i4f4 # pos_re
      !i4f4.add
      !u4u4.ld1 !u4u4.fst x03 rot clc x40 sub !i4f4 # c_im
      x07 !i4f4 # pos_im
      !i4f4.add
      !is_in_set x00 shl @dyn
      !display_buffer x10 add ld2 !bit_addr !store_bit
    buf :for_xy !bcc pop
  :loop !jmp


  !u8.mul_def
  !i8.mul_def
  !i4f4.mul_def

is_in_set! clc # bool b = is_in_set(c4f4m4f4 c)
  !i4f4.0 # z_re
  !i4f4.0 # z_im
  x10 for_i: dec
    !i4f4.ld1+1 !i4f4.ld0 !i4f4.mul clc # z_re2
    !i4f4.ld1+1 !i4f4.ld0 !i4f4.mul clc # z_im2
    !i4f4.ld1 !i4f4.ld1 !i4f4.add clc x20 !i4f4 !i4f4.sub !i4f4.pop :ret !bcc clc
    !i4f4.ld3+1 !i4f4.ld3+1 !i4f4.mul clc !i4f4.ld5+1 !i4f4.add clc x01 rot !i4f4.st2+1
    !i4f4.sub clc !i4f4.ld4+1 !i4f4.add !i4f4.st1+1
    # !c4f4m4f4.ld0+1 !c4f4.norm x20 !i4f4 !i4f4.sub !i4f4.pop :ret !bcc
    # !c4f4m4f4.ld0+1 !c4f4.ld0 !c4f4.mul !c4f4.ld1+1 !c4f4.add !c4f4.st0+1
  buf :for_i !bcc
  x00 x00
ret:
  pop pop pop pop pop pop pop
