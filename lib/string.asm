strcat.def!
  strcat: # strcat(*dst, *str)
    # seek to end of dst
    ld1 !strend st1
    # copy src to dst
    :strcpy !jmp
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
    sw2 su2
  # return* len
  !rt0
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
  strcmp: clc # cmp = strcmp(*str1, *str2)
    ld2 ld2 for_c.
      # break if *str1 != *str2
      ld1 lda ld1 lda sub !z st4 .break !bcc
      # loop if *str1 != '\0'
      ld0 lda !char.is_null
    inc swp inc swp @dyn .for_c !bcc break. pop pop
  # return* *str1 - *str2
  !rt1
strend.def!
  strend: # *ptr = strend(*str)
    # str += strlen(str)
    ld1 !strend st1
  # return* ptr
  !rt0
strend! # *ptr = strend(*str)
  # str += strlen(str)
  for_c.
    ld0 lda !char.is_null
  inc .for_c !bcc dec

memchr.def!
  memchr: clc # *ptr = memchr(*ptr, char, len)
    ld3 dec ad2 # ptr += len - 1
    ld3 for_i. dec
      # break if *ptr == char
      ld2 ld1 sub lda
      ld4 !eq .break !bcs
    # loop if i > 0
    !z .for_i !bcc pop ld1 break. clc
    # compute *ptr
    su2
  # return* ptr
  st1 st1 !rt0
memset.def!
  memset: clc # memset(*ptr, chr len)
    ld3 for_i. dec
      # *ptr = chr
      ld3 ld1 ld4 add sta
    # loop if i > 0
    !z .for_i !bcc pop
  # return* ptr
  !rt3
memcpy.def!
  memcpy: clc # memcpy(*dst, *src, len)
    ld3 for_i. dec
      # *dst = *src
      ld3 ld1 add lda
      ld3 ld2 add sta
    # loop if i > 0
    !z .for_i !bcc memcpy.end: pop
    # prevent unused label warning
    :memcpy.end pop
  # return*
  !rt3
memcmp.def!
  memcmp: clc # cmp = memcmp(*ptr1, *ptr2, len)
    ld3 for_i. dec
      # break if *ptr1 != *ptr2
      ld3 ld1 add lda
      ld3 ld2 add lda
      sub !z st4 .break !bcc
    # loop if i > 0
    !z .for_i !bcc break. pop
  # return* *ptr1 - *ptr2
  !rt2
memswp.def!
  memswp: clc # memswp(*ptr1, *ptr2, len)
    ld3 for_i. dec
      # swap *ptr1 and *ptr2
      ld3 ld1 add
      ld3 ld2 add
      ld1 lda sw2
      ld1 lda swp
      sta sta
    # loop if i > 0
    !z .for_i !bcc pop
  # return*
  !rt3
memxor.def!
  memxor: clc # memxor(*dst, *src, len)
    ld3 for_i. dec
      # *dst ^= *src
      ld3 ld1 add lda
      ld3 ld2 add
      ld0 lda xo2 sta
    # loop if i > 0
    !z .for_i !bcc pop
  # return*
  !rt3
memmove.def!
  memmove: clc # memmove(*dst, *src, len)
    # copy backward if dst > src
    ld2 ld2 !gt :memcpy !bcs
    # copy forward otherwise
    x00 for_i. inc
      # *dst = *src
      ld3 ld1 add lda
      ld3 ld2 add sta
    # loop if i > 0
    ld4 ld1 !eq .for_i !bcc
  # return*
  :memcpy.end !jmp
