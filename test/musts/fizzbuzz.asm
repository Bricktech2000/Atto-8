@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm

# input any of `0123456789:;<=>?@ABC...` to start.
# input `0` displays all numbers

main! !nop
  loop:
    !block_getc !char.digit_zero !char.sub :fizzbuzz !call
    !char.line_feed !putc
  :loop !jmp

  str_num: @00 @00 @00 @00 # enough for null-terminated `"255"`
  str_fizz: @46 @69 @7A @7A @00 # "Fizz"
  str_fizzbuzz: @46 @69 @7A @7A # "FizzBuzz"
  str_buzz: @42 @75 @7A @7A @00 # "Buzz"

  ptrs:
    # note how bit `0b01` being set indicates divisibility by `3`
    # and bit `0b10` being set indicates divisibility by `5`
    :str_num @data # ptrs[0b00]
    :str_fizz @data # ptrs[0b01]
    :str_buzz @data # ptrs[0b10]
    :str_fizzbuzz @data # ptrs[0b11]

  fizzbuzz: # fizzbuzz(u8 n)
    x00 for_i:
      # convert `n` to decimal string onto the stack
      !char.null !u8.ld1 !u8.to_dec
      # move string from the stack to `str_num`
      :str_num for_c:
        swp !z ld1 !u8.sta inc
      :for_c !bcc pop

      x00 # default offset: `0b00`
        # if `n` is divisible by `3`, set bit `0b01`
        ld1 x05 !mod !zr shl @dyn
        # if `n` is divisible by `5`, set bit `0b10`
        ld1 x03 !mod !zr shl @dyn
      # load string pointer from `ptrs` and print it
      :ptrs add lda !puts

      !char.line_feed !putc

    # increment `i` and loop until `i == n`
    inc ld0 ld3 !eq :for_i !bcc pop
  !rt1
