@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm

# input any of `0123456789:;<=>?@ABC...` to start.
# use `0` to display all numbers

main! !nop
  loop:
    !block_getc !char.digit_zero !char.sub :fizzbuzz !call
    !char.carriage_return !putc !char.line_feed !putc
  :loop !jmp

  # we use the syntax `:label` in `ptrs` below, which makes the
  # assembler generate a push instruction for the address of `label`.
  # the opcode for pushing a byte `b` is `b` if `b < 0x80`, and so
  # we must ensure all labels below are less than `0x80`
  str_num: @00 @00 @00 @00 # enough for `"255\0"`
  str_fizz: @46 @69 @7A @7A @00
  str_fizzbuzz: @46 @69 @7A @7A # fall through to `str_buzz`
  str_buzz: @42 @75 @7A @7A @00
  !here x80 not and @org # assert location counter less than `0x80`

  ptrs:
    # note how bit `0b01` being set indicates divisibility by `3`
    # and bit `0b10` being set indicates divisibility by `5`
    :str_num # ptrs[0b00]
    :str_fizz # ptrs[0b01]
    :str_buzz # ptrs[0b10]
    :str_fizzbuzz # ptrs[0b11]

  fizzbuzz: # fizzbuzz(u8 n)
    x00 for_i:
      # convert `n` to decimal string onto the stack
      !u8.ld0 !u8.to_dec
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

      !char.carriage_return !putc !char.line_feed !putc

    # increment `i` and loop until `i == n`
    inc ld0 ld3 !eq :for_i !bcc pop
  !rt1
