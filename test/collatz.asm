@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm

# input any of `0123456789:;<=>?@ABC...` to start.
# inputs `0?GNSW` overflow `u8`s

main! !nop
  loop:
    !char.null !block_getc !char.digit_zero !char.sub :collatz !call
    !char.left_parenthesis !putc !u8.to_dec !stack_puts !char.right_parenthesis !putc
    !char.carriage_return !putc !char.line_feed !putc
  :loop !jmp

  collatz: clc # u8 steps = collatz(u8 n)
    ld1 ad2 # double input so it gets printed
    x00 for_s:
      ld2 shl # 2 * n
      ld3 shr @dyn neg # -n / 2
      ld1 iff ad4 @dyn clc # `n += CF ? 2*n+CF : -n/2`
      !char.null ld4 !u8.to_dec !stack_puts !char.space !putc
      x04 !eq # if `3 * n + 1 == 4` then `n == 1`
    inc :for_s !bcc
  # return* steps
  st1 !rt0
