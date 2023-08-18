@ lib/microprocessor/core.asm
@ lib/microprocessor/types.asm
@ lib/microprocessor/string.asm
@ lib/microcomputer/stdio.asm
@ lib/microprocessor/stdlib.asm

main!
  # x05 :fib !call # 0x05
  # x0A :fib !call # 0x37
  # x0D :fib !call # 0xE9
  # x0C :collatz !call # 0x0A
  # x41 x42 x43 x04 :hanoi !call
  # x41 x42 x43 x08 :hanoi !call
  # x63 :str_abcdef :strchr !call :strlen !call # 0x04
  # xCC :str_abcdef :strchr !call # 0x00
  # :str_abc :strlen !call # 0x03
  # :str_abcdef :strlen !call # 0x06
  # :str_abcdef :str_buf :strcpy !call :str_buf :strlen !call # 0x06
  # :str_ac :str_ab :strcmp !call # 0x01
  # :str_ab :str_abc :strcmp !call # 0x9D
  # :str_abc :str_abc :strcmp !call # 0x00
  # x06 x63 :str_abcdef :memchr !call :strlen !call # 0x04
  # x06 xCC :str_abcdef :memchr !call # 0x00
  # x06 :str_abcdef :str_buf :memcmp !call # 0x00
  # x03 :str_ac :str_ab :memcmp !call # 0x01
  # :str_hello_world ld0 ld0 :strlen !call :sort !call !puts
  !hlt


  # str_ab: @61 @62 @00
  # str_ac: @61 @63 @00
  # str_abc: @61 @62 @63 @00
  # str_abcdef: @61 @62 @63 @64 @65 @66 @00
  # str_hello_world: @48 @65 @6C @6C @6F @20 @57 @6F @72 @6C @64 @21 @00
  # str_buf: @CC @CC @CC @CC @CC @CC @CC @CC

  # !fib.def
  # !collatz.def
  # !hanoi.def
  # !strchr.def
  # !strlen.def
  # !strcpy.def
  # !strcmp.def
  # !memchr.def
  # !memset.def
  # !memcpy.def
  # !memcmp.def
  # !sort.def

fib.def!
  fib: clc # u8 f = fib(u8 n)
    x00 x01 # initial values
    for_n.
      ld1 add swp # assume no overflow
    x01 su4 .for_n !bcc
    # return*
    st2 pop !rt0

collatz.def!
  collatz: clc # u8 steps = collatz(u8 n)
    x00 for_s.
      ld2 ld0 shl add inc
      ld3 shr ld1 iff st3
      x04 xor pop # if `3 * n + 1 == 4` then `n == 1`
    inc .for_s !bcc
  # return* steps
  st1 !rt0

hanoi.def!
  str_arrow. @20 @2D @3E @20 @00
  hanoi: clc # hanoi(u8 n, char dst, char via, char src)
    x00 xo2 .break !bcs
    ld4 ld3 ld5 ld4 dec :hanoi !call # `n - 1` from `src` to `via`
    ld4 !putc .str_arrow !puts ld2 !putc
    !char.carriage_return !putc !char.line_feed !putc
    ld3 ld5 ld4 ld4 dec :hanoi !call # `n - 1` from `via` to `dst`
  break. !rt4
