@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm

# expects input to use postfix notation and number literals to be decimal. performs all arithmetic
# computations on `u8`s and outputs results as decimal literals.
#
# supports the following operations:
# - '+' addition
# - '-' subtraction
# - '*' multiplication
# - '/' division
# - '%' modulus
#
# status messages are as follows:
# - '!' represents an arithmetic overflow condition
# - '?' represents a syntax error condition
# - ' ' represents successful evaluation

main!
  :stack # `top` of software stack
  !status_success # `status_success` as initial `status`

  !'\0' # dummy character for `:'\n'`

  '\n':
    # print and reset status
    !status_success sw2 !putc

    :stack for_item:
      # print `stack[item]` as decimal, ignoring `0x00` values
      ld0 !u8.lda !z :zero !bcs !'\0' swp !u8.to_dec zero: !'\s' !stack_puts
    # loop while no greater than `top`
    ld0 ld4 !eq inc :for_item !bcc pop

  loop:
    # pop previous character
    !char.pop
    # wait for input
    !block_getc
    # print `stdin` to `stdout`
    !char.ld0 !putc

    :default
      !'\n' xo2 :'\n' iff !'\n' xo2
      !'\s' xo2 :'\s' iff !'\s' xo2
    !jmp default:
    x3A !char.sub clc # map '0'..='9' to 0xF6..=0xFF
    x0A !char.add @dyn :digit !bcs # branch if adding 0x0A wrapped around

    :continue
      # if (top < stack + 2) { status = status_syntax; break; }
      ld3 :stack x02 add !gt :loop !status_syntax if4 iff
      # if (*top != 0x00) { status = status_syntax; break; }
      ld3 !u8.lda !nzr :loop !status_syntax if4 iff
    !jmp continue:
    # `top -= 2` as two arguments are soon to be consumed from software stack.
    ld2 dec dec st2
    # push arguments for operation
    ld2 lda # `*top`
    ld3 inc lda # `*(top + 1)`

    :other
      !'+' x30 sub @const xo4 :'+' iff !'+' x30 sub @const xo4
      !'-' x30 sub @const xo4 :'-' iff !'-' x30 sub @const xo4
      !'*' x30 sub @const xo4 :'*' iff !'*' x30 sub @const xo4
      !'/' x30 sub @const xo4 :'/' iff !'/' x30 sub @const xo4
      !'%' x30 sub @const xo4 :'%' iff # !'%' x30 sub @const xo4
    !jmp

    '+': !u8.add :ret !jmp
    '-': !u8.sub :ret !jmp
    '*': !mul clc :ret !jmp
    '/': !div clc :ret !jmp
    '%': !mod clc :ret !jmp

  digit:
    # push `*top * 10 + digit` for `:ret`
    ld2 !u8.lda !mul_10 !u8.ld1 !u8.add
    # fall through
  ret:
    # store result at `top`. if carry flag is set, report overflow condition
    ld3 !status_overflow if4 !u8.sta
    # clear second argument remaining on software stack
    !0u8 ld3 inc !u8.sta
  :loop !jmp

  other:
    # no match; report syntax error
    pop x02 ad4 pop
    !status_syntax st1
  :loop !jmp

  '\s':
    # increment `top` to make room for new item
    ld2 inc st2 # assumes `top` never overflows
  :loop !jmp

  # software stack
  stack: @00 !u8

status_overflow! !'!'
status_syntax! !'?'
status_success! !'\s'
