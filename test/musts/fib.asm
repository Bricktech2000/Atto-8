@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm

# input any of `0123456789:;<=>?@ABC...` to start.
# inputs after `=` overflow `u8`s

main! !nop
  loop:
    !block_getc !'0' !char.sub :fib !call !u8.pop
    !'\n' !putc
  :loop !jmp

fib: clc # u8 f = fib(u8 n)
  x00 x01 # initial values
  for_n:
    !'\0' !'\s' ld3 !u8.to_dec !stack_puts clc
    ld1 add swp # assume no overflow
  x01 su4 :for_n !bcc
  # return*
  st2 pop !rt0
