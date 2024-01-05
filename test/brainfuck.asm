@ lib/core.asm
@ lib/types.asm
@ lib/string.asm
@ lib/stdlib.asm
@ lib/stdio.asm

# most programs from `/bf/test/` can be pasted into this interpreter and compiler directly.
# note the following:
# - `,` is non-blocking; if no input is currently available, `'\0'` is returned
# - `CRLF` is used for printing newlines. sending `LF` will not return the carriage
# - cells are 8-bit unsigned integers, wrapping on overflow and underflow
# - writing beyond the start of the tape will result in undefined behavior
# - unbalancedd brackets in the source code will result in undefined behavior

main!
  !interpreter
  # !compiler


compiler!
  :main !jmp
  code_buffer: x40 !pad !hlt

  main:
    !stdout # for call into `:code_buffer` way later
    :code_buffer # current end of the `dst` string
    # wait for input
    !block_getc
    loop:
      # echo back character
      !char.ld0 !putc

      :_ # default: an empty string
        !char.greater_than_sign xo2 :> iff !char.greater_than_sign xo2
        !char.less_than_sign xo2 :< iff !char.less_than_sign xo2
        !char.plus_sign xo2 :+ iff !char.plus_sign xo2
        !char.hyphen_minus xo2 :- iff !char.hyphen_minus xo2
        !char.full_stop xo2 :. iff !char.full_stop xo2
        !char.comma xo2 :, iff !char.comma xo2
        !char.left_square_bracket xo2 :[ iff !char.left_square_bracket xo2
        !char.right_square_bracket xo2 :] iff !char.right_square_bracket xo2

      # `strcpy`, but keeps track of the end of the `dst` string
      # both more performant and smaller in size than `strcat`
      ld2 for_c:
        # loop if *dst != '\xFF'
        ld1 lda !z
        ld1 sta
      inc swp inc swp @dyn :for_c !bcc dec st2 pop
    # loop while `stdin` is not empty
    !char.pop !getc !char.check_null :loop !bcc !char.pop

    # compute and substitute sentinel values
    :code_buffer for_b: inc
      ld0 lda :for_b
        ![_sentinel xo2 :[_sentinel iff ![_sentinel xo2
        !]_sentinel xo2 :]_sentinel iff !]_sentinel xo2
        !null_sentinel xo2 :break iff !null_sentinel xo2
      st0 !jmp
    break: pop

    !char.carriage_return !putc
    !char.line_feed !putc

    # execute compiled brainfuck program.
    # `head` and `!stdout` are already on the stack
    x00 :code_buffer !jmp

    [_sentinel:
      # create offset from current address and store to memory containing `!pad_sentinel`
      ld0 x03 add ld1 dec sta
      # save current address onto the stack for `:]_sentinel` later
      ld0
    :for_b !jmp

    ]_sentinel:
      # compute offset to previous current address and save into memory contaning `!]_sentinel`
      ld1 dec dec ld1 sta
      # pop previous current address from stack and store into it an offset to current address
      swp ld1 inc inc swp sta
    :for_b !jmp

  # expects `*head` on top of the stack, followed by `head`, followed by `!stdout`
  # top of stack is written to `*head` only when `'<'` or `'>'` are encountered
  >: ld1 sta inc ld0 lda !null_sentinel
  <: ld1 sta dec ld0 lda !null_sentinel
  +: inc !null_sentinel
  -: dec !null_sentinel
  # `!stdin` and `!stdout` are `'\0'` which is also `!null_sentinel`. to avoid null bytes
  # within the string, we load `'\0'` from the stack using `ldo` instead
  .: ld0 ld3 !fputc !null_sentinel
  ,: ld2 !fgetc st0 !null_sentinel
  [: !z !pad_sentinel ![_sentinel iff !jmp !null_sentinel
  ]: !]_sentinel !jmp _: !null_sentinel

[_sentinel! @FF
]_sentinel! @FE
pad_sentinel! @FD
null_sentinel! @00


interpreter!
  pop pop :source_buffer sts

  while:
    :source_buffer !gets.min
    :source_buffer !puts.min
  :source_buffer lda !zr :while !bcs
  !char.space !putc
  !char.latin_capital_letter_o !putc
  !char.latin_capital_letter_k !putc
  !char.carriage_return !putc
  !char.line_feed !putc

  xFF # head
  :source_buffer loop:
    ld0 lda # load source character
    # > <
    ld2 # default: current head
      !char.greater_than_sign xo2 x00 sub @dyn !char.greater_than_sign xo2
      !char.less_than_sign xo2 x00 add @dyn !char.less_than_sign xo2
    st2
    # + -
    ld2 lda # default: current cell value
      !char.plus_sign xo2 x00 add @dyn !char.plus_sign xo2
      !char.hyphen_minus xo2 x00 sub @dyn !char.hyphen_minus xo2
    ld3 sta
    # . ,
    ld2 lda !char.full_stop xo2 ld3 !stdout iff !fputc !char.full_stop xor
    ld2 !char.comma xo2 !stdin iff !fgetc !char.comma xo2 ld3 sta
    # [ ]
    :got_neither
      !char.left_square_bracket xo2 :got_left_square_bracket iff !char.left_square_bracket xo2
      !char.right_square_bracket xo2 :got_right_square_bracket iff !char.right_square_bracket xo2
    st0 !jmp
    got_left_square_bracket:
      # ignore if value at head is non-zero
      ld1 lda !zr :got_neither !bcc
    got_right_square_bracket:
      x00 # nesting level
      for_c:
        ld1 lda
          # decrement nesting level when encountering left square bracket
          !char.left_square_bracket xor x00 su2 @dyn !char.left_square_bracket xor
          # increment nesting level when encountering right square bracket
          !char.right_square_bracket xor x00 ad2 @dyn # !char.right_square_bracket xor
        pop
        # increment or decrement head depending on sign of nesting level
        shl ld1 dec ld2 inc iff st1 shr
        # loop if nesting level is non-zero
      !z :for_c !bcc # bleed `0x00`
      # we're at a right bracket if and only if we're coming from a left bracket.
      # if we're at a right bracket, increment head to skip over the right bracket
      ld1 inc lda !char.right_square_bracket !eq add @dyn
    # loop if current source char is not null
    got_neither:
  inc !here !bcs :loop !jmp

  x06 !pad source_buffer:
