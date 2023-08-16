@ lib/microprocessor/core.asm
@ lib/microprocessor/types.asm
@ lib/microprocessor/stdlib.asm
@ lib/microcomputer/stdio.asm

# AttoMon, the Atto-8 hex monitor
#
# `head` is a pointer; it can be thought of as a "read/write head".
# `buffer` is a one-byte buffer. commands are as follows:
# - typing `/[0-9A-F]{2}/` saves the byte to `buffer`
# - typing `':'` copies the byte from `buffer` into `head`
# - typing `'.'` writes `buffer` to `*head` then increments `head`
# - typing `';'` decrements `head` then prints the byte at `head`
# - typing `'?'` prints the byte at `head` then increments `head`
# - typing `'\b'` swaps the nibbles in `buffer`
# - typing `'\n'` prints `"\r\n"` then prints `head`
# - typing `'!'` jumps program execution to `buffer`
# - typing any other character prints `'\b'`
#
#
# thing to try:
# - `00:45.` prints `E` to `stdout`
# - `B0:01.B0!` moves the stack to the display buffer
# - `F0:F0.E3.F0!` halts the processor at `0xF0`
# - `B0!` or simply `!` on startup warm restarts AttoMon
# - `00:????????????????` prints the first 16 bytes of memory
# - `D5:4F.B0!` restarts AttoMon but prints `OttoMon` instead
# - `E0:CC....33....CC....33....` displays a checkerboard pattern
# - `E0:4E.EE.E4.4A.A4.4E.00.00.` renders the text _ATTO_ on the display
# - `E0:18.B6.4F.B6.3F.B6.E3.20.41.74.74.6F.2D.38.00.E0!` prints _Atto-8_ and returns to AttoMon

main!
  pop pop !user_buffer !jmp # begin execution in `user_buffer` to save memory

  got_colon:
    ld2 st1 # copy `buffer` to `head`
  !char.space :stall_print !jmp

  got_full_stop:
    ld2 ld2 sta x01 ad2 # write `buffer` to `*head` and increment `head`
  !char.space :stall_print !jmp

  got_semicolon:
    x02 su2 # decrement `head`, by two since it is incremented below
  got_question_mark:
    ld1 lda st2 x01 ad2 # copy `*head` to `buffer` for `buffer_print` and increment `head`
  :buffer_print !jmp

  got_hex:
    x0F and # maps 0xFA..0xFF to 0x0A..0x0F
    x04 x0F an4 rot or2 # copy into most significant nibble of `buffer`
    !char.null # previous character was consumed, push dummy character
  got_backspace:
    pop # pop previous character
    x04 ro2 # swap `buffer` nibbles
  :getc_loop !jmp # do not pop previous character again

  # fall through to `buffer_print`
  got_line_feed:
    !char.carriage_return !putc # `'\n'` was just printed, print `'\r'`
    !char.dollar_sign !putc
  got_dollar_sign:
    ld1 st2 # copy `head` to `buffer` for `buffer_print`

  # print `buffer` followed by a space and fall through
  buffer_print:
    x00 for_n:
      x04 ro4 ld3 x0F and clc !u4.to_char !putc
    not :for_n !bcc pop
    !char.space

  # print the character at the top of the stack and fall through
  stall_print:
    x10 !delay # small delay for visual feedback to user
    !putc

  # pop previous character and fall through
  pop_loop:
    pop

  getc_loop:
    !getc
    # ignore empty `stdin`
    !char.null xor :pop_loop !bcs
    # print `stdin` to `stdout`
    !char.ld0 !putc

    :default
      !char.colon xo2 :got_colon iff !char.colon xo2
      !char.full_stop xo2 :got_full_stop iff !char.full_stop xo2
      !char.semicolon xo2 :got_semicolon iff !char.semicolon xo2
      !char.question_mark xo2 :got_question_mark iff !char.question_mark xo2
      !char.backspace xo2 :got_backspace iff !char.backspace xo2
      !char.line_feed xo2 :got_line_feed iff !char.line_feed xo2
      !char.dollar_sign xo2 :got_dollar_sign iff !char.dollar_sign xo2
      !char.exclamation_mark xo2 ld3 iff !char.exclamation_mark xo2
    !jmp default:
    x3A !char.sub clc # map '0'..='9' to 0xF6..=0xFF
    x0A !char.add :got_hex !bcs # branch if adding 0x0A wrapped around
    x11 !char.sub clc # map 'A'..='F' to 0x00..=0x05
    x06 !char.sub :got_hex !bcs # branch if subtracting 0x06 wrapped around
    !char.backspace :stall_print !jmp # invalid character, print `'\b'`

  !user_buffer @org # memory writeable by user
    # initialization code is here to save memory
    !user_buffer sts # put stack right above user buffer
    !user_buffer # allocate buffer
    !user_buffer # allocate head
    !char.null   # allocate char
    :str_AttoMon :puts.min !call
    :got_line_feed !jmp
    !user_buffer x10 add @org !puts.min.def
    # "\r\n=AttoMon=\r\n\0"
    !user_buffer x20 add @org str_AttoMon: @0D @0A @0D @0A @3D @41 @74 @74 @6F @4D @6F @6E @3D @0D @0A @00

user_buffer! xB0
