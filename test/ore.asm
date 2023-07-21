@ lib/microprocessor/core.asm
@ lib/microprocessor/types.asm
@ lib/microprocessor/string.asm

main!
  x05 :fib !call
  x0A :fib !call
  x0D :fib !call
  :str_abc :strlen !call
  :str_abcdef :strlen !call
  :str_ac :str_ab :strcmp !call
  :str_ab :str_abc :strcmp !call
  :str_abc :str_abc :strcmp !call
  !hlt

  str_ab: d61 d62 d00
  str_ac: d61 d63 d00
  str_abc: d61 d62 d63 d00
  str_abcdef: d61 d62 d63 d64 d65 d66 d00

  !strlen_def
  !strcmp_def

  fib: clc # u8 f = fib(u8 n)
    x00 x01 # initial values
    for_n:
      ld1 add swp # assume no overflow
    x01 su4 :for_n !bcc
    # return*
    st2 pop !rt0

