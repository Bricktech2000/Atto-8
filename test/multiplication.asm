@ lib/microprocessor/core.asm
@ lib/microprocessor/types.asm

main!
  # xA0 !i4f4 xA8 !i4f4 !c4f4m4f4 x66 !i4f4 x5F !i4f4 !c4f4m4f4 !c4f4m4f4.mul
  x12 x45 !i8f8 x52 x55 !i8f8 !c8f8m8f8 x2F xB9 !i8f8 x21 x05 !i8f8 !c8f8m8f8 !c8f8m8f8.mul
  !hlt

  # !u8.mul.def
  # !i8.mul.def
  # !i4f4.mul.def
  !u16.mul.def
  !i16.mul.def
  !i8f8.mul.def
