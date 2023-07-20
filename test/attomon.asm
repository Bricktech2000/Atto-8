@ lib/microprocessor/core.asm
@ lib/microprocessor/stdlib.asm
@ lib/microcomputer/stdio.asm

# AttoMon, the Atto-8 hex monitor
#
# `head` is a pointer. it can be thought of as a "read/write head"
# commands are as follows:
# - typing `/[0-9A-F]{2}/` saves the byte to `buffer`, a one-byte buffer
# - typing `':'` copies the byte from `buffer` to `head`
# - typing `'.'` writes `buffer` to `*head` and increments `head`
# - typing `'?'` prints the byte at `head` and increments `head`
# - typing `'!'` jumps program execution to `buffer`
# - typing `'\n'` prints `"\r\n"` then prints `head`
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
# - `E0:4E.EE.E4.4A.A4.4E.` renders the text _ATTO_ on the display
# - `E0:18.B6.4F.B6.3F.B6.E3.20.41.74.74.6F.2D.38.00.E0!` prints _Atto-8_ and returns to AttoMon

main!
  pop pop !user_buffer !jmp # begin execution in `user_buffer` to save memory

  got_colon:
    ld1 st2 # copy `buffer` to `head`
  !space :stall_print !jmp

  got_full_stop:
    ld2 x00 ad4 ld2 sta # write `buffer` to `*head` and increment `head`
  !space :stall_print !jmp

  got_semi_colon:
    !null x00 su4 pop # decrement `head`
  got_question_mark:
    ld2 x00 ad4 lda st1 # copy `*head` to `buffer` for `buffer_print` and increment `head`
  :buffer_print !jmp

  got_hex:
    x0F and # maps 0xFA..0xFF to 0x0A..0x0F
    x04 rot x0F an2 orr # copy into most significant nibble of `buffer`
    !null # push dummy character
  got_backspace:
    x04 ro2 # swap `buffer` nibbles
  :loop !jmp

  got_line_feed:
    !carriage_return !putchar # `'\n'` was just printed, print `'\r'`
    !dollar_sign !putchar
  got_dollar_sign:
    ld2 st1 # copy `head` to `buffer` for `buffer_print`
  # fall through to `buffer_print`

  # print `buffer` followed by an optional pipe then by a space and fall through
  buffer_print:
    x00 for_n:
      x0F x04 ro4 ld3 and clc !u4.to_char !putchar_dyn
    not :for_n !bcc pop
    !space

  # print the character at the top of the stack and fall through
  stall_print:
    xFF !stall # small delay for visual feedback to user
    !putchar_dyn

  loop:
    # pop previous character and poll `stdin`
    pop !getchar
    # ignore empty `stdin`
    buf :loop !bcs
    # print `stdin` to `stdout`
    ld0 !putchar_dyn

    !colon xor :got_colon !bcs !colon xor
    !full_stop xor :got_full_stop !bcs !full_stop xor
    !semi_colon xor :got_semi_colon !bcs !semi_colon xor
    !question_mark xor :got_question_mark !bcs !question_mark xor
    !backspace xor :got_backspace !bcs !backspace xor
    !line_feed xor :got_line_feed !bcs !line_feed xor
    !dollar_sign xor :got_dollar_sign !bcs !dollar_sign xor
    !exclamation_mark xor ld1 !bcs_dyn !exclamation_mark xor
    xC6 add clc # map '0'..'9' to 0xF6..0xFF
    x0A add :got_hex !bcs # branch if adding 0x0A wrapped around
    x11 sub clc # map 'A'..'F' to 0x00..0x05
    x06 sub :got_hex !bcs # branch if subtracting 0x06 wrapped around
    !backspace :stall_print !jmp # invalid character, print `'\b'`

  !user_buffer @org # memory writeable by user
    # initialization code is here to save memory
    !user_buffer sts # put stack right above user buffer
    !user_buffer # allocate head
    !user_buffer # allocate buffer
    x00          # allocate char
    :str_AttoMon :puts !call
    :got_line_feed !jmp
    !user_buffer x10 add @org !puts_def
    # "\r\n=AttoMon=\r\n\0"
    !user_buffer x20 add @org str_AttoMon: d0D d0A d0D d0A d3D d41 d74 d74 d6F d4D d6F d6E d3D d0D d0A d00

# ASCII character codes
space! x20
colon! x3A
line_feed! x0A
full_stop! x2E
backspace! x08
question_mark! x3F
carriage_return! x0D
exclamation_mark! x21
vertical_line! x7C
dollar_sign! x24
semi_colon! x3B

user_buffer! xB0
