stdin! x00 @const
stdout! x00 @const

fgetc! lda # char = fgetc(stream)
getc! !stdin !fgetc # char = getc()
fgets! @err # to be implemented
gets! @err # to be implemented
fgets_def! fgets: @err # to be implemented
gets_def! gets: @err # to be implemented

gets_min! @err # to be implemented
gets_min_def! @err # to be implemented

fputc! sta # fputc(stream, char)
putc! !stdout !fputc # putc(char)
fputs! # fputs(stream, *str)
  swp for_c.
    ld0 lda
    !char.null xor
    ld2 !fputc inc
  .for_c !bcc pop pop
puts! !stdout !fputs # puts(*str)
fputs_def! fputs: swp ld2 swp !fputs !rt1 # fputs(stream, *str)
puts_def! puts: swp !puts !ret # puts(*str)

puts_min! # puts_min(*str)
  for_c.
    ld0 lda
    !char.null xor
    !putc inc
  .for_c !bcc pop
puts_min_def! puts_min: swp !puts_min !ret # puts_min(*str)

wait_char! .skip swp @const !getc !char.null xor pop iff !jmp skip.
wait_null! @const .skip !getc !char.null xor pop iff !jmp skip.
