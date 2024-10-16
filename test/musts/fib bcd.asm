@ lib/core.asm
@ lib/types.asm
@ lib/stdio.asm

# Fibonacci sequence using arbitrary-precision integers in packed BCD. runs out of memory after
# around 850 Fibonacci numbers. the two previous numbers of the sequence are stored intertwined
# in `digit_stack`; lo nibbles store `fib(n-1)` and hi nibbles store `fib(n-2)`. `digit_stack`
# grows downward, from least significant digit to most significant digit. we perform arithmetic
# in BCD because if we performed native bytewise arithmetic, we'd need an additional buffer for
# the arbitrary-precision `divmod`s to convert to decimal for printing.

main!
  pop pop :digit_stack_limit sts

  !digit_stack.bot dec # `digit_stack.top`, pointer to the most significant digit

  loop:
    # print the BCD digits in lo nibbles, starting from `digit_stack.top`
    ld0 for_d:
      ld0 lda x0F and
      !'0' orr !putc # works for radices up to 10
      # clc !u4.to_hex !putc # fixes radices above 10
    inc !z :for_d !bcc # bleed `0x00` as `carry`
    !'\n' !putc

    !digit_stack.bot for_byte: dec
      ld0 lda
        ld0 x0F and # BCD digit from lo nibble
        x04 ro2 ld1 x0F and clc # BCD digit from hi nibble
        add ld3 add # sum then add in `carry`
        # if sum is at least `radix`, subtract `radix` and set `carry`. otherwise, clear `carry`
        ld0 !radix neg @const add @dyn iff
        x00 shl @dyn st3
      # store the updated byte, whose hi nibble is the original lo nibble and whose lo nibble
      # is the sum we just computed. this is the usual `(a, b) = (b, a + b)` that computes the
      # Fibonacci sequence
      xF0 an2 orr ld1 sta
    # loop until one past `digit_stack.top`, for potential carry out
    ld0 ld3 !gt :for_byte !bcc # bleeds `(digit_stack.top, 0x00)`
    # if we have a carry out, decrement `digit_stack.top` to make room for the new BCD digit
    lda !eq xFF add @dyn

    # halt when `digit_stack.top` is about to run into the bottom of the hardware stack, located
    # at `digit_stack_limit`. these four extra bytes for the check cost us 20 Fibonacci numbers
    x00 xo2 @dyn !here !bcc
  :loop !jmp

  x07 !pad digit_stack_limit:

  !digit_stack.bot dec @org
    x01 # initial values `(0, 1)`

digit_stack.bot! x00 # `0x00` as it's easily recognizable through `!z`

# the smaller the radix, the more bits we waste. for radices above 10, remember to uncomment
# the `!u4.to_hex` fix above. a radix above 16 won't work because it breaks the assumption that
# we can pack two digits in one byte
# radix! x02
# radix! x03
# radix! x05
# radix! x06
# radix! x08
radix! x0A
# radix! x0C
# radix! x10
