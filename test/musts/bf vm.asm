@ lib/core.asm
@ lib/types.asm
@ lib/stdio.asm

# brainfuck virtual machine; compiles brainfuck to bytecode into an internal memory
# buffer then runs it through a virtual machine. the bytecode is an array of pointers
# to functions implementing brainfuck commands, with `:[` and `:]` being followed by
# a one-byte operand pointing to the corresponding bracket. this compact representation
# allows for execution of larger brainfuck programs at a higher level of performance
#
# most programs from `/bf/test/` can be pasted in directly. note the following:
# - `,` is non-blocking; if no input is currently available, `'\0'` is returned
# - cells are 8-bit unsigned integers, wrapping on overflow and underflow
# - writing beyond the start of the tape will result in undefined behavior
# - unbalanced brackets in the source code will result in undefined behavior

main!
  pop pop !vm_core sts

  :code_buffer # for call into virtual machine later
  :code_buffer :getline !jmp

    other:
      :_ neg @const # default: no-op
        !'>' xo2 :'>' neg @const iff !'>' xo2
        !'<' xo2 :'<' neg @const iff !'<' xo2
        !'+' xo2 :'+' neg @const iff !'+' xo2
        !'-' xo2 :'-' neg @const iff !'-' xo2
        !'.' xo2 :'.' neg @const iff !'.' xo2
        !',' xo2 :',' neg @const iff !',' xo2
        !'[' xo2 :'[' neg @const iff !'[' xo2
        !']' xo2 :']' neg @const iff !']' xo2
      neg
      # append function pointer to `code_buffer` and increment `dst` pointer
      ld2 sta x01 ad2

      # append `![_sentinel` to `code_buffer`. if `char == '['`, increment `dst` pointer.
      # this way, `code_buffer` will always ends with `![_sentinel`, which is a pointer to
      # the `halt` function, halting the virtual machine at the end of the bytecode
      !char.ld0 !'[' !eq
      ![_sentinel ld2 sta x00 ad2 @dyn

      # if `char == ']'`, resolve jump targets to and from corresponding `']'`
      !char.ld0 !']' !eq :'\0' !bcc
      # find address of corresponding `![_sentinel`. since we resolve jump targets as
      # we go along, the "corresponding `![_sentinel`" is simply the latest `![_sentinel`
      ld1 for_b: dec
        ld0 lda ![_sentinel !eq
      :for_b !bcc # sets carry
      # poke addresses into `code_buffer` as arguments to bytecode instructions `:[` and `:]`
      ld2 inc ld1 sta # for is-zero branch from `[` to after `]`
      dec ld2 sta     # for unconditional jump from `]` to `[`

      # increment `dst` pointer past the argument to `:]`
      x00 ad2 @dyn

      # structure similar to `getline.min`, but compiles to bytecode brainfuck user input
      # into `dst` directly, instead of writing user input to `dst` as-is
    '\0':
      # `char` is either `'\0'` or `other` from above
      # putc(char)
      !putc
  getline: # getline(*dst)
      !getc
    :other
      !'\n' xo2 :'\n' iff !'\n' xo2
      !'\0' xo2 :'\0' iff !'\0' xo2
    !jmp
    '\n':
      # print `char`, which is a `'\n'`
      !putc # bleed `dst`

      # increment `dst` pointer past `![_sentilel`, alias `:halt` function pointer
      inc

      # run brainfuck bytecode through virtual machine. `head` and `!stdout` are
      # already on the stack
      x00 :next !jmp

  code_buffer: # beginning of internal memory buffer

  !vm_core @org
    # expects `*head` on top of the stack, followed by `head`, followed by VM `ip`.
    # top of stack is written to `*head` only when `'<'` and `'>'` are encountered
    '>': ld1 sta inc :<_post !jmp
    '<': ld1 sta dec <_post: ld0 lda :next !jmp
    '+': inc :next !jmp
    '-': dec :next !jmp
    '.': ld0 !putc :next !jmp
    ',': !getc st0 :next !jmp
    # bytecode instructions `:[` and `:]` are followed by a one-byte operand pointing
    # to the corresponding bracket
    '[': !z ld2 inc ld3 lda iff :]_post !jmp
    ']': ld2 lda ]_post: st2 _: :next !jmp

    next: ld2 lda ld3 inc st3 !jmp
    halt: !hlt

# see above for why `![_sentinel` is an alias for `:halt`
[_sentinel! :halt

vm_core! xD0 # largest possible address
