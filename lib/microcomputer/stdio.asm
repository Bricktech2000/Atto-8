stdin! x00 @const
stdout! x00 @const

getchar! !stdin lda

gets_def!
  gets: # gets(*str)
    @err # to be implemented

putchar! @const !putchar_dyn
putchar_dyn! !stdout swp sta

puts_def!
  puts: # puts(*str)
    swp for_char.
      !stdout ld1 lda !char.null xor sta inc
    .for_char !bcc pop
    !ret

wait_char! .skip swp @const !getchar !char.null xor pop iff !jmp skip.
wait_null! @const .skip !getchar !char.null xor pop iff !jmp skip.
