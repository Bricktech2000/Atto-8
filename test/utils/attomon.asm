@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm
@ lib/display.asm

# AttoMon, the Atto-8 hex monitor
#
# `head` is a read/write head pointing into memory. `buffer` is a one-byte data buffer
#
# commands are as follows:
# - typing '$' prints `head`
# - typing /[0-9A-F]/ rotates the nibble into `buffer`
# - typing ':' copies the byte from `buffer` into `head`
# - typing '.' writes `buffer` to `*head` then increments `head`
# - typing ',' prints the byte `*head` then increments `head`
# - typing ';' decrements `head` then prints the byte `*head`
# - typing '\b' prints '\b' then swaps the nibbles of `buffer`
# - typing '\n' prints a line feed then prints '$' and `head`
# - typing '!' jumps program execution to `buffer`
# - typing any other character prints it then prints '\b'
#
# memory layout is as follows:
# - `0x00..0x99` is reserved for AttoMon
# - `0x99..0xC0` is initialization code only used during warm restarts
# - `0xC0..0x100` is unused and available for user programs
#
# compound commands to try:
# - '00!' warm restarts AttoMon
# - '00:45.' prints a single 'E' to `stdout`
# - '99:00.00!' moves the stack to the display buffer
# - 'B4:4F.00!' warm restarts AttoMon but prints _OttoMon_ instead
# - 'F0:F0.E3.F0:' then '!' halts the processor at `0xF0`
# - '00:,,,,,,,,,,,,,,,,' prints the first 16 bytes of memory
# - 'F0:00.E5.00.C0.D1.B0.F3.E3.F0:' then '!' prints the ASCII character set in a loop
# - 'F0:00.E5.40.D0.B7.00.F2.90.E3.F0:' then '!' prints '@' in a loop until a key is pressed
# - 'E0:CC....33....CC....33....CC....33....CC....33....' displays a checkerboard pattern
# - 'C0:3A.B2.27.5D.B2.E3.0A.41.74.74.6F.2D.38.0A.00.C0:' then '!' prints _Atto-8_ and returns to AttoMon
# - 'B1:1B.5B.32.4A.1B.5B.48.00.' makes warm restarts clear the terminal screen

main!
  pop pop !attomon_init !jmp # begin execution in `attomon_init` to save memory

  ':':
    ld2 st1 # copy `buffer` to `head`
  :space_print !jmp

  '.':
    ld2 ld2 sta x01 ad2 # write `buffer` to `*head` and increment `head`
  :space_print !jmp

  ';':
    xFF ad2 # decrement `head`, and set carry so it is not incremented below
  ',':
    ld1 lda st2 xFF su2 # copy `*head` to `buffer` for `buffer_print` and increment `head`
  :buffer_print !jmp

  hex:
    !char.pop # pop extraneous '\b' character
    x0F and # maps 0xFA..0xFF to 0x0A..0x0F
    x04 x0F an4 rot or2 # copy into most significant nibble of `buffer`
    !'\0' # previous character was consumed, push dummy character
  '\b':
    !char.pop # pop previous character
    x04 ro2 # swap `buffer` nibbles
  :loop !jmp # do not pop previous character again

  # prepare stack and fall through to `buffer_print`
  '\n':
    !'$' !putc
  '$':
    ld1 st2 # copy `head` to `buffer` for `buffer_print`

  # print `buffer` followed by a space and fall through
  buffer_print:
    ld2 !hex_putc.min

  # print a space and fall through
  space_print:
    !'\s'

  # print the character at the top of the stack and fall through
  stall_print:
    x10 !delay # small delay for visual feedback to user
    !putc

  # pop previous character and fall through
  pop_loop:
    !char.pop

  loop:
    # wait for input
    !getc
    # print `stdin` to `stdout`
    !char.ld0 !putc

    :default
      !':' xo2 :':' iff !':' xo2
      !'.' xo2 :'.' iff !'.' xo2
      !';' xo2 :';' iff !';' xo2
      !',' xo2 :',' iff !',' xo2
      !'\b' xo2 :'\b' iff !'\b' xo2
      !'\n' xo2 :'\n' iff !'\n' xo2
      !'$' xo2 :'$' iff !'$' xo2
      !'!' xo2 ld3 iff !'!' xo2
      !'\0' xo2 :pop_loop iff !'\0' xo2
    !jmp default:
    !'\b'
      xC6 ad2 # map '0'..='9' to 0xF6..=0xFF
      x0A ad2 @dyn :hex !bcs # branch if adding 0x0A wrapped around
      x12 su2 # map 'A'..='F' to 0x00..=0x05
      x06 su2 @dyn :hex !bcs # branch if subtracting 0x06 wrapped around
    :stall_print !jmp # invalid character, print '\b'

  !attomon_init @org # initialization code
    !attomon_init sts # put stack right above initialization code
    !user_buffer # allocate `buffer`
    !user_buffer # allocate `head`
    !'\0' # push dummy character
    :str_attomon_init :puts.min !call :'\n' !jmp
    !puts.min.def
    str_attomon_init: @0A @0A @3D @41 @74 @74 @6F @4D @6F @6E @3D @0A @00 # "\n\n=AttoMon=\n"
    # str_attomon_init: @1B @5B @32 @4A @1B @5B @48 @00 # "\x1B[2J\x1B[H"
  !user_buffer @org # memory writable by user
    # :str_atto-8 :puts.min !call :'\n' !jmp
    # str_atto-8: @0A @41 @74 @74 @6F @2D @38 @0A @00 # "\nAtto-8\n"
    @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00
    @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00
  !display_buffer @org # memory writable by user
    @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00
    @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00

attomon_init! x99
user_buffer! xC0
