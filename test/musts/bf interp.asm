@ lib/core.asm
@ lib/types.asm
@ lib/stdio.asm

# brainfuck interpreter; interprets brainfuck straight from source without preprocessing.
# only has to store one byte per source character, allowing for running larger brainfuck
# programs, at the expense of performance
#
# most programs from /bf/test/ can be pasted in directly. note the following:
# - `,` is non-blocking; if no input is currently available, '\0' is returned
# - cells are 8-bit unsigned integers, wrapping on overflow and underflow
# - writing beyond the start of the tape will result in undefined behavior
# - unbalanced brackets in the source code will result in undefined behavior

main!
  pop pop :source_buffer sts

  # :source_buffer :source_buffer :getline !call
  :source_buffer :getline.min !call
  !'\n' !putc

  xFF # head
  :source_buffer loop:
    ld0 lda # load source character
    # > <
    ld2 # default: current head
      !'>' xo2 x00 sub @dyn !'>' xo2
      !'<' xo2 x00 add @dyn !'<' xo2
    st2
    # + -
    ld2 lda # default: current cell value
      !'+' xo2 x00 add @dyn !'+' xo2
      !'-' xo2 x00 sub @dyn !'-' xo2
    ld3 sta
    # . ,
    ld2 lda !'.' xo2 ld3 !stdout iff !fputc !'.' xor
    ld2 !',' xo2 !stdin iff !fgetc !',' xo2 ld3 sta
    # [ ]
    :other
      !'[' xo2 :'[' iff !'[' xo2
      !']' xo2 :']' iff !']' xo2
    st0 !jmp
    '[':
      # ignore if value at head is non-zero
      ld1 lda !zr :other !bcc
    ']':
      x00 # nesting level
      for_c:
        ld1 lda
          # decrement nesting level when encountering left square bracket
          !'[' xor x00 su2 @dyn !'[' xor
          # increment nesting level when encountering right square bracket
          !']' xor x00 ad2 @dyn # !']' xor
        pop
        # increment or decrement head depending on sign of nesting level
        shl @dyn ld1 dec ld2 inc iff st1 shr @dyn
      # loop if nesting level is non-zero
      !z :for_c !bcc # bleed `0x00`
      # we're at a right bracket if and only if we're coming from a left bracket.
      # if we're at a right bracket, increment head to skip over the right bracket
      ld1 inc lda !']' !eq add @dyn
    # loop if current source char is not null
    other:
  inc !here !bcs :loop !jmp

  # !getline.def
  !getline.min.def

  x06 !pad source_buffer:
