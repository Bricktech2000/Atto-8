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
