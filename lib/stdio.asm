stdin! x00 @const
stdout! x00 @const

fgetc! !char.lda # char = fgetc(stream)
getc! !stdin !fgetc # char = getc()
fgetc.def! fgetc: @error # to be implemented
getc.def! getc: !getc swp !ret # char = getc()
fgets! # fgets(stream, *str)
  swp for_c.
    ld1 !fgetc
    !char.check_null
    ld1 !char.sta
  inc .for_c !bcc pop pop
gets! !stdin !fgets # gets(*str)
fgets.def! fgets: sw2 swp !fgets !rt0 # fgets(stream, *str)
gets.def! gets: swp !gets !ret # gets(*str)

gets.min! # gets.min(*str)
  for_c.
    !getc
    !char.check_null
    ld1 !char.sta
  inc .for_c !bcc pop
gets.min.def! gets.min: swp !gets.min !ret # gets.min(*str)

hex_getc! # u8 = hex_getc(char sep)
  # block until `sep` is sent through `stdin`
  block. !getc !char.ld1 !eq .block !bcc !char.pop clc
  # assume input well formed
  !getc !hex.to_u4 x04 rot
  !getc !hex.to_u4 orr !u8
hex_gets! # hex_gets(char sep, *str)
  swp for_c.
    ld1 !hex_getc # includes `!char.check_null`
    ld1 !u8.sta
  # loop if *str != '\0'
  inc .for_c !bcc pop pop
hex_getn! clc # hex_getn(char sep, *str, len)
  ld2 dec ad2 # str += len - 1
  sw2 for_i. dec
    !char.ld2 !hex_getc clc
    ld2 ld2 sub !u8.sta
  # loop if i > 0
  !z .for_i !bcc pop pop pop


fputc! !char.sta # fputc(stream, char)
putc! !stdout !fputc # putc(char)
fputc.def! fputc: @error # to be implemented
putc.def! putc: swp !putc !ret # putc(char)
fputs! # fputs(stream, *str)
  swp for_c.
    ld0 !char.lda
    !char.check_null
    ld2 !fputc
  inc .for_c !bcc pop pop
puts! !stdout !fputs # puts(*str)
fputs.def! fputs: sw2 swp !fputs !rt0 # fputs(stream, *str)
puts.def! puts: swp !puts !ret # puts(*str)

puts.min! # puts.min(*str)
  for_c.
    ld0 !char.lda
    !char.check_null
    !putc
  inc .for_c !bcc pop
puts.min.def! puts.min: swp !puts.min !ret # puts.min(*str)

hex_putc! # hex_putc(char sep, u8 char)
  !char.space !putc !putc
  !u8.to_hex !putc !putc
hex_puts! # hex_puts(char sep, *str)
  swp for_c.
    ld0 !u8.lda
    !char.ld2 !hex_putc
  # loop if *str != '\0'
  ld0 !u8.lda !u8.is_null inc .for_c !bcc pop pop
hex_putn! clc # hex_putn(char sep, *str, len)
  ld2 dec ad2 # str += len - 1
  sw2 for_i. dec
    ld1 ld1 sub !u8.lda
    !char.ld3 !hex_putc
  # loop if i > 0
  !z .for_i !bcc pop pop pop

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
      # print `"0x"` then fall through to conversion specifier `x`
      !char.digit_zero str_empty: !putc !char.latin_small_letter_x !putc
    got_latin_small_letter_x:
      # print first character then fall through to conversion specifier `c` with second character
      !u8.to_hex !putc
    got_latin_small_letter_c:
    got_other:
      # print char on stack and fall through to conversion specifier `s` with empty string
      !putc :str_empty
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
      ld2 lda # loads `conversion_specifier` from `format`
      x01 ad4 # increments `format`
    :got_unknown_conversion_specifier
      !char.latin_small_letter_p xo2 :got_latin_small_letter_p iff !char.latin_small_letter_p xo2
      !char.latin_small_letter_x xo2 :got_latin_small_letter_x iff !char.latin_small_letter_x xo2
      !char.latin_small_letter_d xo2 :got_latin_small_letter_d iff !char.latin_small_letter_d xo2
      !char.latin_small_letter_u xo2 :got_latin_small_letter_u iff !char.latin_small_letter_u xo2
      !char.latin_small_letter_c xo2 :got_latin_small_letter_c iff !char.latin_small_letter_c xo2
      !char.latin_small_letter_s xo2 :got_latin_small_letter_s iff # !char.latin_small_letter_s xo2
    st0 !jmp # pops `conversion_specifier` off stack
    got_latin_small_letter_d:
      # compute absolute value, print `-` if was negative and fall through to conversion specifier `u`
      !abs.dyn !char.null !char.hyphen_minus iff !putc clc
    got_latin_small_letter_u:
      # print as decimal and jump back to `:printf`
      !char.null swp !u8.to_dec !stack_puts :printf !jmp
    got_unknown_conversion_specifier: # includes conversion specifier `%`
      # store back argument from `va_list` and jump to conversion specifier `c` with `'%'`
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
      # *end = other; end += 2
      ld2 x02 ad4
      !char.ld1 swp !char.sta # bleed `other`
    got_backspace.
      # `char` is either `'\b'` or `other` from above
      # putc(dst == end ? 0 : char)
      ld3 ld3 !e iff # bleed `char`
    got_null.
      !putc
      # end -= dst == end ? 0 : 1
      xFF ad2 @dyn
  getline: # getline(*end, *dst)
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
      # *end = other; end += 2
      ld2 !char.sta
      x02 ad2 !char.null # bleed `'\0'`
    got_backspace.
      ld3 ld3 !e pop # bleed `char`
    got_null.
      # pop `char`
      !char.pop
      # end -= dst == end ? 0 : 1
      xFF ad2 @dyn
  getpass: # getpass(*end, *dst)
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

# reads into `dst` and echoes to `stdout` characters from `stdin` until `'\n'` is
# encountered. does not support `'\b'`. assumes `dst` to be initialized to `{0}`
getline.min.def!
    got_other.
      # *dst = other; end += 1
      ld2 x01 ad4
      !char.ld1 swp !char.sta # bleed `other`
    got_null.
      # `char` is either `'\0'` or `other` from above
      # putc(char)
      !putc
  getline.min: # getline.min(*dst)
      !getc
    .got_other
      !char.line_feed xo2 .got_line_feed iff !char.line_feed xo2
      !char.null xo2 .got_null iff !char.null xo2
    !jmp
    got_line_feed.
      # pop `char`, which is a `'\n'`
      !char.pop
  # return*
  !rt1

# identical to `getline.min`, but does not echo to `stdout`
getpass.min.def!
    got_other.
      ld2 !char.sta
      x01 ad2 !char.null # bleed `'\0'`
    got_null.
      # pop `char`
      !char.pop
  getpass.min: # getpass.min(*dst)
      !getc
    .got_other
      !char.line_feed xo2 .got_line_feed iff !char.line_feed xo2
      !char.null xo2 .got_null iff !char.null xo2
    !jmp
    got_line_feed.
      # pop `char`, which is a `'\n'`
      !char.pop
  # return*
  !rt1
