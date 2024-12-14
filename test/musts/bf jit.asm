@ lib/core.asm
@ lib/types.asm
@ lib/stdio.asm

# brainfuck just-in-time compiler; compiles brainfuck to machine code into an internal
# memory buffer then transfers control over to the compiled program. allows for a high
# level of performance, at the expense of maximum program size, as the compiled machine
# code is generally larger than the brainfuck source
#
# most programs from /bf/test/ can be pasted in directly. note the following:
# - `,` is non-blocking; if no input is currently available, '\0' is returned
# - cells are 8-bit unsigned integers, wrapping on overflow and underflow
# - writing beyond the start of the tape will result in undefined behavior
# - unbalanced brackets in the source code will result in undefined behavior

main!
  !stdout # for call into `:code_buffer` later
  code_buffer: # beginning of internal memory buffer
  :code_buffer :getline !jmp

  # reserve `!code_buffer.len` bytes for `code_buffer`. moreover, assert that the entire
  # code buffer is located before address `0x80`, ensuring jump targets can be pushed with
  # a single instruction
  :code_buffer !code_buffer.len add x80 not and @org

    other:
      :_ neg @const # default: an empty string
        !'>' xo2 :'>' neg @const iff !'>' xo2
        !'<' xo2 :'<' neg @const iff !'<' xo2
        !'+' xo2 :'+' neg @const iff !'+' xo2
        !'-' xo2 :'-' neg @const iff !'-' xo2
        !'.' xo2 :'.' neg @const iff !'.' xo2
        !',' xo2 :',' neg @const iff !',' xo2
        !'[' xo2 :'[' neg @const iff !'[' xo2
        !']' xo2 :']' neg @const iff !']' xo2
      neg

      # `strcpy`, but keeps track of the end of the `dst` string.
      # both more performant and smaller in size than `strcat`
      ld2 for_c:
        # loop if *dst != '\0'
        ld1 !char.lda !char.check_null
        ld1 !char.sta
      inc swp inc swp @dyn :for_c !bcc dec st2 pop

      # if `char == ']'`, resolve jump targets to and from corresponding `]`
      !char.ld0 !']' !eq :'\0' !bcc
      # find address of corresponding `!loop_sentinel`. since we resolve jump targets as
      # we go along, the "corresponding `!loop_sentinel`" is simply the latest `!loop_sentinel`
      ld1 for_b: dec
        ld0 lda !on # equivalent to `ld0 lda !loop_sentinel !eq`
      :for_b !bcc # sets carry
      # poke addresses into `code_buffer` at reserved locations
      ld0 dec ld3 dec dec sta # for unconditional jump from `]` to `[`
      ld2 ld1 inc sta         # for is-zero branch from `[` to after `]`
      ld0 x03 ad2 @dyn sta    # for is-non-zero branch from `[` to after `[`

      # structure similar to `getline.min`, but compiles to machine code brainfuck user input
      # into `dst` directly, instead of writing user input to `dst` as-is
    '\0':
      # `char` is either '\0' or `other` from above
      # putc(char)
      !putc
  getline: # getline(*dst)
      !getc
    :other
      !'\n' xo2 :'\n' iff !'\n' xo2
      !'\0' xo2 :'\0' iff !'\0' xo2
    !jmp
    '\n':
      # print `char`, which is a '\n'
      !putc # bleed `dst`

      # construct and append `!hlt` instruction to end of `code_buffer`
      ld0 ld0 sta inc
      :halt lda ld1 sta inc

      # transfer control to compiled brainfuck program. `head` and `!stdout` are
      # already on the stack
      x00 :code_buffer !jmp

  # expects `*head` on top of the stack, followed by `head`, followed by `!stdout`.
  # top of stack is written to `*head` only when `<` and `>` are encountered
  '>': ld1 sta inc ld0 lda @00
  '<': ld1 sta dec ld0 lda @00
  '+': inc @00
  '-': dec @00
  # `!stdin` and `!stdout` are '\0' which is also `@00`. to avoid null bytes
  # within the string, we load '\0' from the stack using `ldo` instead
  '.': ld0 ld3 !fputc @00
  ',': ld2 !fgetc st0 @00
  # we use `!dbg` as jump targets to trigger a trap if not properly overwritten
  '[': !z !loop_sentinel !dbg iff halt: !jmp @00
  ']': !dbg !jmp _: @00

loop_sentinel! @FF # `0xFF` because it's easily recognizable through a `not` instruction with `!on`

code_buffer.len! x60 # largest possible length
