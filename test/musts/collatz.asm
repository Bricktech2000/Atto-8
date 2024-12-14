@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm

# input any of '0123456789:;<=>?@ABC...' to start.
# inputs '0?GNSW' overflow `u8`s

main! !nop
  loop:
    !'\0' !block_getc !'0' !char.sub :collatz !call
    !'(' !putc !u8.to_dec !stack_puts !')' !putc
    !'\n' !putc
  :loop !jmp

  collatz: clc # u8 steps = collatz(u8 n)
    ld1 ad2 # double input so it gets printed
    x00 for_s:
      ld2 shl # 2 * n
      ld3 shr @dyn neg # -n / 2
      ld1 iff ad4 @dyn clc # `n += CF ? 2*n+CF : -n/2`
      !'\0' ld4 !u8.to_dec !stack_puts !'\s' !putc
      x04 !eq # if `3 * n + 1 == 4` then `n == 1`
    inc :for_s !bcc
  # return* steps
  st1 !rt0
