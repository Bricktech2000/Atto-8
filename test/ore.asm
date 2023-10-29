@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm

main!
  x00 x06 !popcnt # 0x02
  x00 xE5 !popcnt # 0x05
  x05 :fib !call # 0x05
  x0A :fib !call # 0x37
  x0D :fib !call # 0xE9
  x0C :collatz !call # 0x09
  :str_Atto-8 ld0 !str_Atto-8.len dec :sort !call :str_Atto-8 !puts # -8Aott
  !char.carriage_return !putc !char.line_feed !putc
  x41 x42 x43 x04 :hanoi !call # ...
  # x41 x42 x43 x08 :hanoi !call # ...
  !hlt


  !fib.def
  !collatz.def
  !hanoi.def
  !sort.def

  str_Atto-8: @41 @74 @74 @6F @2D @38 @00 str_Atto-8.end:

str_Atto-8.len! :str_Atto-8.end :str_Atto-8 sub @const

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
      ld2 shl # 2 * n
      ld3 shr @dyn neg # -n / 2
      ld1 iff ad4 @dyn # `n += CF ? 2*n+CF : -n/2`
      x04 !eq # if `3 * n + 1 == 4` then `n == 1`
    inc .for_s !bcc
  # return* steps
  st1 !rt0

hanoi.def!
  str. src. @20 @20 @2D @3E @20 dst. @20 @0D @0A @00
  hanoi: clc # hanoi(u8 n, char dst, char via, char src)
    x00 xo2 @dyn .break !bcs
    ld4 ld3 ld5 ld4 dec :hanoi !call # `n - 1` from `src` to `via`
    ld4 .src sta ld2 .dst sta .str !puts.min # `1` from `src` to `dst`
    ld3 ld5 ld4 ld4 dec :hanoi !call # `n - 1` from `via` to `dst`
  break. !rt4
