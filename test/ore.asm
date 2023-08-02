@ lib/microprocessor/core.asm
@ lib/microprocessor/types.asm
@ lib/microprocessor/string.asm

main!
  x05 :fib !call # 0x05
  x0A :fib !call # 0x37
  x0D :fib !call # 0xE9
  :str_abc :strlen !call # 0x03
  :str_abcdef :strlen !call # 0x06
  :str_ac :str_ab :strcmp !call # 0x01
  :str_ab :str_abc :strcmp !call # 0x9D
  :str_abc :str_abc :strcmp !call # 0x00
  :str_abcdef :str_buf :strcpy !call #
  :str_buf :strlen !call # 0x06
  x06 :str_abcdef :str_buf :memcmp !call # 0x00
  x03 :str_ac :str_ab :memcmp !call # 0x01
  !hlt

  str_ab: d61 d62 d00
  str_ac: d61 d63 d00
  str_abc: d61 d62 d63 d00
  str_abcdef: d61 d62 d63 d64 d65 d66 d00
  str_buf: dCC dCC dCC dCC dCC dCC dCC dCC

  !strlen.def
  !strcpy.def
  !strcmp.def
  !memcmp.def

  fib: clc # u8 f = fib(u8 n)
    x00 x01 # initial values
    for_n:
      ld1 add swp # assume no overflow
    x01 su4 :for_n !bcc
    # return*
    st2 pop !rt0
