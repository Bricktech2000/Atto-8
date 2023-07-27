stdin! x00 @const
stdout! x00 @const

getchar! !stdin lda

gets_def!
  gets: # gets(*str)
    @err # to be implemented

putchar! !stdout sta

puts_def!
  puts: # puts(*str)
    swp for_char.
      ld0 lda !char.null xor !putchar inc
    .for_char !bcc pop
    !ret

wait_char! .skip swp @const !getchar !char.null xor pop iff !jmp skip.
wait_null! @const .skip !getchar !char.null xor pop iff !jmp skip.
