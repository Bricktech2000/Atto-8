@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm

# expects input to use postfix notation and number literals to be decimal. performs all arithmetic
# computations on `u8`s and outputs results as decimal literals.
#
# supports the following operations:
# - `+` addition
# - `-` subtraction
# - `*` multiplication
# - `/` division
# - `%` modulus
#
# status messages are as follows:
# - `!` represents an arithmetic overflow condition
# - `?` represents a syntax error condition
# - ` ` represents successful evaluation

main!
  :stack # `top` of software stack
  !status_success # `status_success as initial `status`

  !char.null # dummy character for `got_line_feed`

  got_line_feed:
    # print carriage return then print and reset status
    !char.carriage_return !putc
    ld1 !putc !status_success st1

    :stack for_item:
      !char.null # sentinel
      # value = stack[item]
      ld1 !u8.lda
      # while (value != 0)
      while_value: !z :break !bcs
        # (div_10, mod_10) = (value / 10, value % 10)
        !divmod_10
        # char = '0' + mod_10
        !char.digit_zero ad2 # bleeds `char`
      :while_value !jmp break: !u8.pop
      # separate items with spaces
      !char.space
      # print chars in reverse order
      for_char: !putc !char.check_null :for_char !bcc # bleed `0x00`
    # loop while less than `top`
    add @dyn ld3 ld1 !gt :for_item !bcc pop

    # :stack for_item:
    #   # separate items with spaces
    #   !char.space !putc
    #   x00 # sentinel
    #   x01 x0A x64 # place values
    #   # loop through place values
    #   ld4 lda for_place_value:
    #     # subtract place value iteratively and count iterations
    #     xFF for_iteration: inc ld2 su2 :for_iteration !bcc
    #     # print `digit,` where `digit = iteration_count + '0'`
    #     !char.digit_zero dec @const !char.add !putc
    #   # loop until place value is the sentinel
    #   add x00 xo2 @dyn :for_place_value !bcc pop # bleed sentinel
    # # loop while less than `top`
    # add @dyn ld3 ld1 !gt :for_item !bcc magic_label: pop

  loop:
    # pop previous character
    !char.pop
    # wait for input
    !block_getc
    # print `stdin` to `stdout`
    !char.ld0 !putc

    :default
      !char.line_feed xo2 :got_line_feed iff !char.line_feed xo2
      !char.space xo2 :got_space iff !char.space xo2
    !jmp default:
    x3A !char.sub clc # map '0'..='9' to 0xF6..=0xFF
    x0A !char.add @dyn :got_digit !bcs # branch if adding 0x0A wrapped around

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

    :got_other
      !char.plus_sign x30 sub @const xo4 :got_plus_sign iff !char.plus_sign x30 sub @const xo4
      !char.hyphen_minus x30 sub @const xo4 :got_hyphen_minus iff !char.hyphen_minus x30 sub @const xo4
      !char.asterisk x30 sub @const xo4 :got_asterisk iff !char.asterisk x30 sub @const xo4
      !char.solidus x30 sub @const xo4 :got_solidus iff !char.solidus x30 sub @const xo4
      !char.percent_sign x30 sub @const xo4 :got_percent_sign iff # !char.percent_sign x30 sub @const xo4
    !jmp

    got_plus_sign: !u8.add :ret !jmp
    got_hyphen_minus: !u8.sub :ret !jmp
    got_asterisk: !mul clc :ret !jmp
    got_solidus: !div clc :ret !jmp
    got_percent_sign: !mod clc :ret !jmp

  got_digit:
    # push `*top * 10 + digit` for `:ret`
    ld2 !u8.lda !mul_10 !u8.ld1 !u8.add
    # fall through
  ret:
    # store result at `top`. if carry flag is set, report overflow condition
    ld3 !status_overflow if4 !u8.sta
    # clear second argument remaining on software stack
    !u8.0 ld3 inc !u8.sta
  :loop !jmp

  got_other:
    # no match; report syntax error
    !status_syntax st1
  :loop !jmp

  got_space:
    # increment `top` to make room for new item
    ld2 inc st2 # assumes `top` never overflows
  :loop !jmp

  # software stack
  stack: @00 !u8

status_overflow! !char.exclamation_mark
status_syntax! !char.question_mark
status_success! !char.space

divmod_10! # (div_10, mod_10) = divmod_10(n)
  x00 dec @const loop.
    x01 add
    x0A su2 @dyn
  .loop !bcc
  # omit @dyn for optimization
  x0A dec @const ad2