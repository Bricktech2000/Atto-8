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
    ld1 ld3 sta x00 ad2 # write `buffer` to `*head` and increment `head`
  !char.space :stall_print !jmp

  got_semicolon:
    x00 su2 # decrement `head`, which clears the carry flag
  got_question_mark:
    ld1 lda st2 x00 ad2 # copy `*head` to `buffer` for `buffer_print` and increment `head`
  :buffer_print !jmp

  got_hex:
    x0F and # maps 0xFA..0xFF to 0x0A..0x0F
    x04 x0F an4 rot or2 # copy into most significant nibble of `buffer`
    !char.null # previous character was consumed, push dummy character
  got_backspace:
    pop # pop previous character
    x04 ro2 # swap `buffer` nibbles
  :getchar_loop !jmp # do not pop previous character again

  # fall through to `buffer_print`
  got_line_feed:
    !char.carriage_return !putchar # `'\n'` was just printed, print `'\r'`
    !char.dollar_sign !putchar
  got_dollar_sign:
    ld1 st2 # copy `head` to `buffer` for `buffer_print`

  # print `buffer` followed by a space and fall through
  buffer_print:
    x00 for_n:
      x04 ro4 ld3 x0F and clc !u4.to_char !putchar_dyn
    not :for_n !bcc pop
    !char.space

  # print the character at the top of the stack and fall through
  stall_print:
    xFF !stall # small delay for visual feedback to user
    !putchar_dyn

  # pop previous character and fall through
  pop_loop:
    pop

  getchar_loop:
    !getchar
    # ignore empty `stdin`
    !char.null xor :pop_loop !bcs
    # print `stdin` to `stdout`
    ld0 !putchar_dyn

    !char.colon xor :got_colon !bcs !char.colon xor
    !char.full_stop xor :got_full_stop !bcs !char.full_stop xor
    !char.semicolon xor :got_semicolon !bcs !char.semicolon xor
    !char.question_mark xor :got_question_mark !bcs !char.question_mark xor
    !char.backspace xor :got_backspace !bcs !char.backspace xor
    !char.line_feed xor :got_line_feed !bcs !char.line_feed xor
    !char.dollar_sign xor :got_dollar_sign !bcs !char.dollar_sign xor
    !char.exclamation_mark xor ld2 !bcs_dyn !char.exclamation_mark xor
    x3A sub clc # map '0'..='9' to 0xF6..=0xFF
    x0A add :got_hex !bcs # branch if adding 0x0A wrapped around
    x11 sub clc # map 'A'..='F' to 0x00..=0x05
    x06 sub :got_hex !bcs # branch if subtracting 0x06 wrapped around
    !char.backspace :stall_print !jmp # invalid character, print `'\b'`

  !user_buffer @org # memory writeable by user
    # initialization code is here to save memory
    !user_buffer sts # put stack right above user buffer
    !user_buffer # allocate buffer
    !user_buffer # allocate head
    x00          # allocate char
    :str_AttoMon :puts !call
    :got_line_feed !jmp
    !user_buffer x10 add @org !puts_def
    # "\r\n=AttoMon=\r\n\0"
    !user_buffer x20 add @org str_AttoMon: d0D d0A d0D d0A d3D d41 d74 d74 d6F d4D d6F d6E d3D d0D d0A d00

user_buffer! xB0
