@ lib/core.asm
@ lib/stdlib.asm
@ lib/types.asm
@ lib/display.asm

# currently missing `size` and `pos` from C implementation

main!
  pop pop !display_buffer sts

  x00 !u4u4 for_xy: dec
    # !u4u4.ld0 !u4u4.snd x03 rot x40 sub !i4f4 # c_re
    !u4u4.ld0 !u4u4.snd x01 rot x10 sub !i4f4 # c_re (zoomed)
    xFB !i4f4 # pos_re
    # add # reduces size but not idiomatic
    clc !i4f4.add

    # !u4u4.ld1 !u4u4.fst x03 rot x40 sub !i4f4 # c_im
    !u4u4.ld1 !u4u4.fst x01 rot x10 sub !i4f4 # c_im (zoomed)
    x00 !i4f4 # pos_im
    # add # reduces size but not idiomatic
    clc !i4f4.add

    !is_in_set @dyn x00 shl @dyn
    ld1 !display_buffer !bit_addr !store_bit
  !z !here :for_xy swp iff !jmp


  !u8.mul.def
  !i8.mul.def
  !i4f4.mul.def

is_in_set! clc # bool b = is_in_set(c4f4m4f4 c)
  !i4f4.0 # z_re
  !i4f4.0 # z_im
  x10 for_i: dec
    !i4f4.ld1+1 !i4f4.ld0 !i4f4.mul # z_re2
    !i4f4.ld1+1 !i4f4.ld0 !i4f4.mul # z_im2
    !i4f4.ld1 !i4f4.ld1 !i4f4.add x18 !i4f4 !i4f4.sub @dyn !i4f4.pop :ret !bcc
    !i4f4.ld3+1 !i4f4.ld3+1 !i4f4.mul shl clc !i4f4.ld4+2 !i4f4.add clc !i4f4.st2+1
    !i4f4.sub clc !i4f4.ld4+1 !i4f4.add !i4f4.st1+1
  !z :for_i !bcc
  x00 x00
ret:
  pop pop pop pop pop pop pop
