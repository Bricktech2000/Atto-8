@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm

# input any of `0123456789:;<=>?@ABC...` to start.
# anything after `<` overflows `u8`s

main!
  nop @dyn loop:
    !block_getc !char.digit_zero !char.sub :fib !call !u8.pop
    !char.carriage_return !putc !char.line_feed !putc
  :loop !jmp

fib: clc # u8 f = fib(u8 n)
  x00 x01 # initial values
  for_n:
    ld1 add swp # assume no overflow
    ld1 !u8.to_dec !stack_puts !char.space !putc clc
  x01 su4 :for_n !bcc
  # return*
  st2 pop !rt0
