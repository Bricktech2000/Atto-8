stdin! x00 @const
stdout! x00 @const

getchar! !stdin lda
gets_def!
  gets: # *str = gets()
    @err # to be implemented
putchar! !stdout swp @const sta
puts_def!
  puts: # puts(*str)
    swp for_char.
      !stdout ld1 lda buf sta inc
    .for_char !bcc
    pop !ret

branch_input! .skip swp @const !getchar buf pop iff !jmp skip.
reset_input! !stdin x00 sta
wait_input! !here !branch_input !reset_input
