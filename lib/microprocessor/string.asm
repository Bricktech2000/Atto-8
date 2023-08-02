strcat.def! strcat: @err # to be implemented
strchr.def! strchr: @err # to be implemented
strlen.def!
  strlen: # len = strlen(*str)
    ld1 for_c.
      ld0 lda !char.null xor pop
    inc .for_c !bcc
    # compute and store length
    ld2 sub
  # return* len
  st1 !rt0
strcpy.def!
  strcpy: # strcpy(*dst, *src)
    ld2 ld2 for_c.
      # loop if *dst != '\0'
      ld1 lda !char.null xor
      ld1 sta
    swp inc swp inc .for_c !bcc pop pop
  # return*
  !rt2
strcmp.def!
  strcmp: # cpm = strcmp(*str1, *str2)
    ld2 ld2 for_c.
      # break if *str1 != *str2
      ld1 lda ld1 lda sub buf st4 .break !bcc
      # loop if *str1 != '\0'
      ld0 lda !char.null xor pop
    swp inc swp inc .for_c !bcc break. pop pop
  # return* *str1 - *str2
  !rt1

memchr.def! memchr: @err # to be implemented
memset.def!
  memset: # memset(*ptr, val, len)
    ld3 for_i. dec
      ld3 ld1 ld4 add sta
    buf .for_i !bcc pop
  !rt3
memcpy.def!
  memcpy: clc # memcpy(*dst, *src, len)
    ld3 for_i. dec
      ld0 ld4 add lda
      ld1 ld4 add sta
    buf .for_i !bcc pop
  !rt3
memcmp.def!
  memcmp: clc # cpm = memcmp(*ptr1, *ptr2, len)
    ld3 for_i. dec
      # break if *ptr1 != *ptr2
      ld0 ld4 add lda
      ld1 ld4 add lda
      sub buf st4 .break !bcc
      # loop if i < len
    buf .for_i !bcc break. pop
  # return* *ptr1 - *ptr2
  !rt2
