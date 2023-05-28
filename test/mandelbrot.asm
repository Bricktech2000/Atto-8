@ lib/microprocessor/core.asm
@ lib/microprocessor/math.asm
@ lib/microprocessor/memory.asm
@ lib/microcomputer/display.asm

# a few bytes short, never fully tested. missing `scale` factor from C implementation

main!
  pop !front_buffer sts

  loop:
    x00 for_xy: dec
        ld0 x0F and x08 sub !i4f4 x03 !ror
        ld1 x04 rot x0F and x08 sub !i4f4 x03 !ror
      !c4f4
        x0C !i4f4
        x04 !i4f4
      !c4f4
      !c4f4.sub
      !is_in_set
      ld0 x00 shl !store_bit
    buf :for_xy !bcc pop
  :loop !jmp


  !u8.mul_def
  !i8.mul_def
  !i4f4.mul_def

is_in_set! # is_in_set(c4f4 c)
  !c4f4.0 # z
  x10 for_i: dec
    !c4f4.ld0+1 !c4f4.norm x40 !i4f4 !i4f4.sub !i4f4.pop :ret !bcc
    !c4f4.ld0+1 !c4f4.ld0 !c4f4.mul !c4f4.ld1+1 !c4f4.add !c4f4.st0+1
  buf :for_i !bcc pop
ret:
