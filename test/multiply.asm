@ lib/microprocessor/core.asm
@ lib/microprocessor/math.asm

main!
  x12 !u8 x34 !u8 :u8.mul !call
  x12 x34 !u16 x56 x78 !u16 :u16.mul !call
  !hlt

  !u8.mul
  !u16.mul
