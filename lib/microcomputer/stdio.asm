stdin! x00 @const
stdout! x00 @const

getchar! !stdin lda
gets_def!
  gets: # *str = gets()
    @err # to be implemented
putchar_dyn! !stdout swp sta
putchar! @const !putchar_dyn
puts_def!
  puts: # puts(*str)
    swp for_char.
      !stdout ld1 lda buf sta inc
    .for_char !bcc pop
    !ret

wait_char! .skip swp @const !getchar buf pop iff !jmp skip.
wait_null! @const .skip !getchar buf pop iff !jmp skip.
