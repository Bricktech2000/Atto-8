strlen_def!
  strlen: # len = strlen(*str)
    ld1 for_c:
      ld0 lda buf pop
    inc :for_c !bcc
    # compute and store length
    ld2 sub
    # return*
    st1 !rt0

strcmp_def!
  strcmp: clc # strcmp(*str1, *str2)
    ld2 ld2 while:
      # break if *str1 != *str2
      ld1 lda ld1 lda xor pop :break !bcc
      # break if *str1 == 0
      ld1 lda buf pop :break !bcs
    swp inc swp inc :while !jmp break:
    # compute and store *str1 - *str2
    lda swp lda clc sub
    # return*
    st2 !rt1
