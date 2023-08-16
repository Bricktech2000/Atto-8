@ lib/microprocessor/core.asm
@ lib/microprocessor/types.asm
@ lib/microprocessor/string.asm
@ lib/microcomputer/stdio.asm

# - `+[>,.<]` pipes stdin to stdout
# - `+++++++.` sends a bell to stdout
# - `++++++++[.+]` prints all ASCII characters
# - `>+[,[[>,]<[.<]>[-]]+]` reverses a string
# - `+[,[---------[-[++++++++++.[-]]],]+]` strips newlines and tabs
# - `++++++++++[>++++++++++>+<<-]>[>.<-]` clears the screen
# - `+++++[>+++++++++<-]+[,[[>--.++>+<<-]>+.->[<.>-]<<,]+]` converts text to brainfuck code that prints it
# - `>+[,[>>>++++++++[<[<++>-]<+[>+<-]<-[-[-<]>]>[-<]<,>>>-]<.[-]<<]+]` converts a binary string to ascii text
# - `----[---->+<]>++.--[----->+<]>+..-----.[->+++++<]>++.+++++++++++.` prints _Atto-8_ to stdout

main!
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

  xFF # pointer
  :source_buffer loop:
    ld0 lda # load source character
    # > <
    ld2 # default is current pointer
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
      # ignore if value at pointer is non-zero
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
        # increment or decrement pointer depending on sign of nesting level
        shl ld1 dec ld2 inc iff st1 shr
        # loop if nesting level is non-zero
      buf :for_c !bcc # 0x00 is left on the stack
      # we're at a right bracket if and only if we're coming from a left bracket.
      # if we're at a right bracket, increment pointer to skip over the right bracket
      ld1 inc lda !char.right_square_bracket xor pop add @dyn
    # loop if current source char is not null
    got_neither:
  inc !here :loop swp iff !jmp

  x06 !pad source_buffer:
