@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm

# input any of `0123456789:;<=>?@ABC...` to start.
# inputs after `9` take longer than a second to compute

main! !nop
  !char.latin_capital_letter_a !char.latin_capital_letter_b !char.latin_capital_letter_c
  loop:
    !block_getc !char.digit_zero !char.sub :hanoi !call !char.pop
    !char.line_feed !putc
  :loop !jmp

  hanoi: # (u8 n, char dst, char via, char src) = hanoi(u8 n, char dst, char via, char src)
    x00 xo2 @dyn :ret !bcs # stack is `ret, n, dst, via, src`
    sw4 ld0 sw4 sw2 st0 dec # stack is now `n - 1, via, dst, src, ret`
    :hanoi !call # move `n - 1` disks from `src` to `via`
    ld0 !char.digit_zero !char.add @dyn :n sta ld3 :src sta ld2 :dst sta
    :str_move !puts.min # move `1` disk from `src` to `dst`
    ld1 sw4 st1 ld2 sw2 st2 # stack is now `n - 1, dst, src, via, ret`
    :hanoi !call # move `n - 1` disks from `via` to `dst`
    inc sw2 sw4 ld2 sw2 st2 # stack is now `ret, n, dst, via, src`
  ret: !ret

  str_move: @23 n: @58 @20 @7C @20 src: @58 @20 @2D @3E @20 dst: @58 @0A @00 # "#X | X -> X\n"
