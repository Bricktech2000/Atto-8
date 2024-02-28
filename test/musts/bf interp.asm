@ lib/core.asm
@ lib/types.asm
@ lib/stdio.asm

# brainfuck interpreter; interprets brainfuck straight from source without preprocessing.
# only has to store one byte per source character, allowing for running larger brainfuck
# programs, at the expense of performance
#
# most programs from `/bf/test/` can be pasted in directly. note the following:
# - `,` is non-blocking; if no input is currently available, `'\0'` is returned
# - cells are 8-bit unsigned integers, wrapping on overflow and underflow
# - writing beyond the start of the tape will result in undefined behavior
# - unbalanced brackets in the source code will result in undefined behavior

main!
  pop pop :source_buffer sts

  # :source_buffer :source_buffer :getline !call
  :source_buffer :getline.min !call
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
        shl @dyn ld1 dec ld2 inc iff st1 shr @dyn
      # loop if nesting level is non-zero
      !z :for_c !bcc # bleed `0x00`
      # we're at a right bracket if and only if we're coming from a left bracket.
      # if we're at a right bracket, increment head to skip over the right bracket
      ld1 inc lda !char.right_square_bracket !eq add @dyn
    # loop if current source char is not null
    got_neither:
  inc !here !bcs :loop !jmp

  # !getline.def
  !getline.min.def

  x06 !pad source_buffer:
