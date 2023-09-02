@ lib/core.asm
@ lib/types.asm
@ lib/string.asm
@ lib/stdio.asm

# - `+[>,.<]` pipes stdin to stdout
# - `+++++++.` sends a bell to stdout
# - `++++[->++++++++<]>[.+]` prints all ASCII characters
# - `>+[,[[>,]<[.<]>[-]]+]` reverses a string
# - `+[,[---------[-[++++++++++.[-]]],]+]` strips newlines and tabs
# - `++++++++++[>++++++++++>+<<-]>[>.<-]` clears the screen
# - `+++++[>+++++++++<-]+[,[[>--.++>+<<-]>+.->[<.>-]<<,]+]` converts text to brainfuck code that prints it
# - `>+[,[>>>++++++++[<[<++>-]<+[>+<-]<-[-[-<]>]>[-<]<,>>>-]<.[-]<<]+]` converts a binary string to ascii text
# - `----[---->+<]>++.--[----->+<]>+..-----.[->+++++<]>++.+++++++++++.` prints _Atto-8_ to stdout

main!
  !interpreter
  # !compiler


compiler!
  :main !jmp
  code_buffer: x40 !pad !hlt

  main:
    !stdout # for call into `:code_buffer` way later
    :code_buffer # current end of the `dst` string
    # wait for input from `stdin` and preserve character
    !char.null wait: !char.pop !getc !char.null xor :wait !bcs
    loop:
      # echo back character
      ld0 !putc

      :_ # default is an empty string
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
        ld1 lda !char.null xor
        ld1 sta
      inc swp inc swp @dyn :for_c !bcc dec st2 pop
    # loop while `stdin` is not empty
    !char.pop !getc !char.null xor :loop !bcc !char.pop

    # compute and substitute magic values
    :code_buffer for_b:
      ld0 lda :default
        ![_magic xo2 :[_magic iff ![_magic xo2
        !]_magic xo2 :]_magic iff !]_magic xo2
        !char.null xo2
      st0 !jmp default:
    inc :for_b !bcc pop

    !char.carriage_return !putc
    !char.line_feed !putc

    # execute compiled brainfuck program
    # `head` and `!stdout` are already on the stack
    x00 :code_buffer !jmp

    [_magic:
      # create offset from current address and store to memory containing `!pad_magic`
      ld0 x03 add ld1 dec sta
      # save current address onto the stack for `:]_magic` later
      ld0
    :default !jmp

    ]_magic:
      # compute offset to previous current address and save into memory contaning `!]_magic`
      ld1 dec dec ld1 sta
      # pop previous current address from stack and store into it an offset to current address
      swp ld1 inc inc swp sta
    :default !jmp

  # expects `*head` on top of the stack, followed by `head`, followed by `!stdout`
  # top of stack is written to `*head` only when `'<'` or `'>'` are encountered
  >: ld1 sta inc ld0 lda !char.null
  <: ld1 sta dec ld0 lda !char.null
  +: inc !char.null
  -: dec !char.null
  # `!stdin` and `!stdout` are `'\0'` which is also `!char.null`
  .: ld0 ld3 !fputc !char.null
  ,: ld2 !fgetc st0 !char.null
  [: buf !pad_magic ![_magic iff !jmp !char.null
  ]: !]_magic !jmp !char.null
  _: !char.null

[_magic! @FF
]_magic! @FE
pad_magic! @FD


interpreter!
  pop pop :source_buffer sts

  while:
    :source_buffer !gets.min
    :source_buffer !puts.min
  :source_buffer lda buf pop :while !bcs
  !char.space !putc
  !char.latin_capital_letter_o !putc
  !char.latin_capital_letter_k !putc
  !char.carriage_return !putc
  !char.line_feed !putc

  xFF # head
  :source_buffer loop:
    ld0 lda # load source character
    # > <
    ld2 # default is current head
      !char.greater_than_sign xo2 x00 sub @dyn !char.greater_than_sign xo2
      !char.less_than_sign xo2 x00 add @dyn !char.less_than_sign xo2
    st2
    # + -
    ld2 lda # default is current cell value
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
      ld1 lda buf pop :got_neither !bcc
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
      buf :for_c !bcc # 0x00 is left on the stack
      # we're at a right bracket if and only if we're coming from a left bracket.
      # if we're at a right bracket, increment head to skip over the right bracket
      ld1 inc lda !char.right_square_bracket xor pop add @dyn
    # loop if current source char is not null
    got_neither:
  inc !here :loop swp iff !jmp

  x06 !pad source_buffer:
