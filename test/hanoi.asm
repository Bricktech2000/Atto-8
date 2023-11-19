@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm

# input any of `0123456789:;<=>?@ABC...` to start.
# anything after `Q` causes a stack overflow

main! !nop
  loop:
    !char.latin_capital_letter_a !char.latin_capital_letter_b !char.latin_capital_letter_c
    !block_getc !char.digit_zero !char.sub :hanoi !call
    !char.line_feed !putc
  :loop !jmp

  hanoi: # hanoi(u8 n, char dst, char via, char src)
    x00 xo2 @dyn :break !bcs
    ld4 ld3 ld5 ld4 dec :hanoi !call # `n - 1` from `src` to `via`
    ld4 :src sta ld2 :dst sta :str !puts.min # `1` from `src` to `dst`
    ld3 ld5 ld4 ld4 dec :hanoi !call # `n - 1` from `via` to `dst`
  break: !rt4

  str: src: @20 @20 @2D @3E @20 dst: @20 @0D @0A @00
