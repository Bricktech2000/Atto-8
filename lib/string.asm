strcat.def!
  strcat: # strcat(*dst, *str)
    # src += strlen(src)
    ld1 for_c.
      ld0 lda !char.is_null
    inc .for_c !bcc dec st1
    # copy src to dst
    !strcpy.def
    # use :strcpy label
    :strcpy pop
strchr.def!
  strchr: # *ptr = strchr(*str, char)
    ld1 for_c.
      # break if *str == char
      ld0 lda ld4 xor .break !bcs
      # loop if *str != '\0'
      ld4 xor !char.is_null
    # return 0x00 if match not found
    inc .for_c !bcc x00 swp break. pop
  # return* ptr
  st2 !rt1
strlen.def!
  strlen: # len = strlen(*str)
    # find null byte
    ld1 for_c.
      ld0 lda !char.is_null
    inc .for_c !bcc
    # compute len
    ld2 sub
  # return* len
  st1 !rt0
strcpy.def!
  strcpy: # strcpy(*dst, *src)
    ld2 ld2 for_c.
      # loop if *dst != '\0'
      ld1 lda !char.check_null
      ld1 sta
    inc swp inc swp @dyn .for_c !bcc pop pop
  # return*
  !rt2
strcmp.def!
  strcmp: clc # cpm = strcmp(*str1, *str2)
    ld2 ld2 for_c.
      # break if *str1 != *str2
      ld1 lda ld1 lda sub !check_zero st4 .break !bcc
      # loop if *str1 != '\0'
      ld0 lda !char.is_null
    inc swp inc swp @dyn .for_c !bcc break. pop pop
  # return* *str1 - *str2
  !rt1

memchr.def!
  memchr: clc # *ptr = memchr(*buf, char, len)
    ld3 dec ad2 # buf += len - 1
    ld3 for_i. dec
      # break if *buf == char
      ld2 ld1 sub lda
      ld4 xor pop .break !bcs
    # loop if i > 0
    !check_zero .for_i !bcc pop ld1 break. clc
    # compute *ptr
    su2
  # return* ptr
  st1 st1 !rt0
memset.def!
  memset: clc # memset(*ptr, val, len)
    ld3 for_i. dec
      # *ptr = val
      ld3 ld1 ld4 add sta
    # loop if i > 0
    !check_zero .for_i !bcc pop
  # return*
  !rt3
memcpy.def!
  memcpy: clc # memcpy(*dst, *src, len)
    ld3 for_i. dec
      # *dst = *src
      ld3 ld1 add lda
      ld3 ld2 add sta
    # loop if i > 0
    !check_zero .for_i !bcc pop
  # return*
  !rt3
memcmp.def!
  memcmp: clc # cpm = memcmp(*ptr1, *ptr2, len)
    ld3 for_i. dec
      # break if *ptr1 != *ptr2
      ld3 ld1 add lda
      ld3 ld2 add lda
      sub !check_zero st4 .break !bcc
    # loop if i > 0
    !check_zero .for_i !bcc break. pop
  # return* *ptr1 - *ptr2
  !rt2
