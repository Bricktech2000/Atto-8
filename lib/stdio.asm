stdin! x00 @const
stdout! x00 @const

fgetc! lda # char = fgetc(stream)
getc! !stdin !fgetc # char = getc()
fgetc.def! fgetc: @err # to be implemented
getc.def! getc: !getc swp !ret # char = getc()
fgets! # fgets(stream, *str)
  swp for_c.
    ld1 !fgetc
    !char.check_null
    ld1 sta
  inc .for_c !bcc pop pop
gets! !stdin !fgets # gets(*str)
fgets.def! fgets: sw2 swp !fgets !rt0 # fgets(stream, *str)
gets.def! gets: swp !gets !ret # gets(*str)

gets.min! # gets.min(*str)
  for_c.
    !getc
    !char.check_null
    ld1 sta
  inc .for_c !bcc pop
gets.min.def! gets.min: swp !gets.min !ret # gets.min(*str)


fputc! sta # fputc(stream, char)
putc! !stdout !fputc # putc(char)
fputc.def! fputc: @err # to be implemented
putc.def! putc: swp !putc !ret # putc(char)
fputs! # fputs(stream, *str)
  swp for_c.
    ld0 lda
    !char.check_null
    ld2 !fputc
  inc .for_c !bcc pop pop
puts! !stdout !fputs # puts(*str)
fputs.def! fputs: sw2 swp !fputs !rt0 # fputs(stream, *str)
puts.def! puts: swp !puts !ret # puts(*str)

puts.min! # puts.min(*str)
  for_c.
    ld0 lda
    !char.check_null
    !putc
  inc .for_c !bcc pop
puts.min.def! puts.min: swp !puts.min !ret # puts.min(*str)


# prints and consumes a null-terminated string from the stack
stack_puts! # stack_puts(str[])
  for_c. !char.check_null !putc .for_c !bcc

# inputs and pushes in reverse order a null-terminated string onto the stack
stack_gets! # str[] = stack_gets()
  !char.null for_c. !getc !char.check_null .for_c !bcc !char.pop


# a `printf` immitation that supports a few conversion specifiers. in `format`,
# - `%d` prints a signed integer as decimal with precision `1`
# - `%u` prints an unsigned integer as decimal with precision `1`
# - `%x` prints an unsigned integer as uppercase hex with precision `2` (nonstandard)
# - `%c` prints a character
# - `%s` prints a null-terminated string from its address
# - `%p` prints a pointer-to-void as `"0x"` followed by upperacase hex with precision `2`
# - `%%` prints a literal `'%'` character
# note that:
# - integers are assumed to be 8 bits wide (nonstandard)
# - the common conversion specifiers `i`, `o`, `X`, `n` are unsupported (nonstandard)
# - in `format`, a `%` followed by an unknown conversion specifier will print a `'%'`
# - if the last character of `format` is `'%'`, the behavior is undefined
# - passing insufficient arguments for the format results in undefined behavior
# - passing excess arguments for the format results in undefined behavior (nonstandard)
printf.def!
    got_latin_small_letter_p:
      # print `"0x"` then fall through to specifier `x`
      !char.digit_zero str_: !putc !char.latin_small_letter_x !putc
    got_latin_small_letter_x:
      # print first character then fall through to specifier `c` with second character
      !u8.to_hex !putc
    got_latin_small_letter_c:
    got_other:
      # print char on stack and fall through to specifier `s` with empty string
      !putc :str_
    got_latin_small_letter_s:
      # print as string and fall through to `:printf`
      !puts.min
  printf: # printf(*format, ...)
    # load `char` from `format`
    ld1 lda
    :got_other
      !char.percent_sign xo2 :got_percent_sign iff !char.percent_sign xo2
      !char.null xo2 :got_null iff !char.null xo2
    x01 ad4 # increment `format` here because carry is cleared
    !jmp # keep `char` on stack for `:got_other`
    got_percent_sign:
      pop # pops `char` from stack
      swp sw2 # loads one argument from `va_list`
      ld2 lda # loads `specifier` from `format`
      x01 ad4 # increments `format`
    :got_unknown_specifier
      !char.latin_small_letter_p xo2 :got_latin_small_letter_p iff !char.latin_small_letter_p xo2
      !char.latin_small_letter_x xo2 :got_latin_small_letter_x iff !char.latin_small_letter_x xo2
      !char.latin_small_letter_d xo2 :got_latin_small_letter_d iff !char.latin_small_letter_d xo2
      !char.latin_small_letter_u xo2 :got_latin_small_letter_u iff !char.latin_small_letter_u xo2
      !char.latin_small_letter_c xo2 :got_latin_small_letter_c iff !char.latin_small_letter_c xo2
      !char.latin_small_letter_s xo2 :got_latin_small_letter_s iff # !char.latin_small_letter_s xo2
    st0 !jmp # pops `specifier` off stack
    got_latin_small_letter_d:
      # compute absolute value, print `-` if was negative and fall through to specifier `u`
      !abs.dyn !char.null !char.hyphen_minus iff !putc clc
    got_latin_small_letter_u:
      # print as decimal and jump back to `:printf`
      !char.null swp !u8.to_dec !stack_puts :printf !jmp
    got_unknown_specifier: # includes specifier `%`
      # store back argument from `va_list` and jump to specifier `c` with `'%'`
      sw2 swp !char.percent_sign :got_latin_small_letter_c !jmp
    got_null: # null terminator
  # pop `char` from stack then return*
  pop !rt1


# reads into `dst` and echoes to `stdout` characters from `stdin` until `'\n'` is
# encountered. supports `'\b'`. supports placeholder text through `end` parameter:
# - `:buf :buf :getline !call` (where `dst == end`) does not use placeholder text
# - `:buf !puts :buf :buf !strend :getline !call` uses `:buf` as placeholder text
getline.def!
    got_other.
      # increment by `2` because `.got_backspace` will decrement by `1`
      # *end = char; end += 2
      ld2 x02 ad4
      !char.ld1 swp !char.sta # bleed `char`
    got_backspace.
      # `char` is either `'\b'` or `other` from above
      # putc(dst == end ? 0 : char)
      # end -= dst == end ? o : 1
      ld3 ld3 !e iff !putc xFF ad2 @dyn
      # putc(' ')
      !char.space !putc !char.backspace # bleed `'\b'`
    got_null.
      # `char` is either `'\0'` or `'\b'` from above
      # putc(char)
      !putc
  getline: # getline(*dst, *end)
      !getc
    .got_other
      !char.line_feed xo2 .got_line_feed iff !char.line_feed xo2
      !char.backspace xo2 .got_backspace iff !char.backspace xo2
      !char.null xo2 .got_null iff !char.null xo2
    !jmp
    got_line_feed.
      # pop `char`, which is a `'\n'`
      !char.pop
      # *end = '\0'
      st1 !char.null swp sta
  # return*
  !ret

# identical to `getline`, but does not echo to `stdout`
getpass.def!
    got_other.
      # increment by `2` because `.got_backspace` will decrement by `1`
      # *end = char; end += 2
      ld2 !char.sta
      x02 ad2 !char.null # bleed `'\0'`
    got_backspace.
      # end -= dst == end ? 0 : 1
      ld3 ld3 !e xFF ad4 @dyn pop # bleed `char`
    got_null.
      # pop `char`
      !char.pop
  getpass: # getpass(*dst, *end)
      !getc
    .got_other
      !char.line_feed xo2 .got_line_feed iff !char.line_feed xo2
      !char.backspace xo2 .got_backspace iff !char.backspace xo2
      !char.null xo2 .got_null iff !char.null xo2
    !jmp
    got_line_feed.
      # pop `char`, which is a `'\n'`
      !char.pop
      # *end = '\0'
      st1 !char.null swp sta
  # return*
  !ret
