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
# - typing `'!'` jumps program execution to `head`
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
# - `E0:4E.EE.E4.4A.A4.4E.` renders the text `ATTO` on the display

main!
  !user_buffer !jmp # start execution at `user_buffer` to save memory

  line_feed:
    ld2 st1 # copy head to buffer for `init`
    !carriage_return !putchar # `'\n'` was just printed, print `'\r'`
  :init !jmp # carry will be set for `init`

  colon:
    ld1 st2
  !space :stall_print !jmp

  full_stop:
    ld2 x00 ad4 ld2 sta
  !space :stall_print !jmp

  question_mark:
    ld2 x00 ad4 lda st1 # copy *head to buffer for `init`
  clc :init !jmp # carry will be cleared for `init`

  hex:
    x0F and # maps 0xFA..0xFF to 0x0A..0x0F
    x0F an2 x04 ro2 orr x00 # rotate 4 bits into `buffer`
  :loop !jmp


  # print `buffer` followed by an optional pipe then by a space and fall through
  init:
    !null !vertical_line iff # if `CF` print `null` else print `vertical_line`
    x00 for_n:
      x04 ro4 ld3 x0F and clc !u4.to_char !putchar_dyn
    not :for_n !bcc pop
    !putchar_dyn
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

    ld0 !backspace xor pop :loop !bcs
    ld0 !line_feed xor pop :line_feed !bcs
    ld0 !colon xor pop :colon !bcs
    ld0 !full_stop xor pop :full_stop !bcs
    ld0 !question_mark xor pop :question_mark !bcs
    ld0 !exclamation_mark xor pop ld1 !bcs_dyn
    xC6 add clc # map '0'..'9' to 0xF6..0xFF
    x0A add :hex !bcs # branch if adding 0x0A wrapped around
    x11 sub clc # map 'A'..'F' to 0x00..0x05
    x06 sub :hex !bcs # branch if subtracting 0x06 wrapped around
    !backspace :stall_print !jmp # invalid character, print `'\b'`

    # x10 for_c: dec
    #   ld0 !to_char
    #   ld2 xor pop :hex !bcs
    # buf :for_c !bcc pop
    # !backspace :stall_print !jmp

  !user_buffer @org # memory writeable by user
    # initialization code is here to save memory
    !user_buffer sts # put stack right above user buffer
    !user_buffer # allocate head
    !user_buffer # allocate buffer
    x00          # allocate char
    :str_AttoMon :puts !call
    :init !jmp # carry will be set for `init`
    !puts_def
    # "\r\n=AttoMon=\r\n\0"
    xD0 @org str_AttoMon: d0D d0A d0D d0A d3D d41 d74 d74 d6F d4D d6F d6E d3D d0D d0A d00

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

user_buffer! xB0
