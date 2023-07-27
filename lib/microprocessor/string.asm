strlen_def!
  strlen: # len = strlen(*str)
    ld1 for_c.
      ld0 lda !char.null xor pop
    inc .for_c !bcc
    # compute and store length
    ld2 sub
  # return*
  st1 !rt0

strcmp_def!
  strcmp: clc # cpm = strcmp(*str1, *str2)
    ld2 ld2 loop.
      # break if *str1 == '\0'
      ld1 lda !char.null xor .break !bcs
      # break if *str1 != *str2
      ld1 lda xor .break !bcc pop
    swp inc swp inc .loop !jmp break.
    # compute and store *str1 - *str2
    pop lda swp lda clc sub
  # return*
  st2 !rt1

memset_def!
  memset: # memset(ptr, val, len)
    ld3 for_i. dec
      ld3 ld1 ld4 add sta
    buf .for_i !bcc pop
  !rt3

memcpy_def!
  memcpy: clc # memcpy(dst, src, len)
    ld3 for_i. dec
      ld0 ld4 add lda
      ld1 ld4 add sta
    buf .for_i !bcc pop
  !rt3
