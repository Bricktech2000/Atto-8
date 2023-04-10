memcpy!
  memcpy: # memcpy(dst, src, len)
    ld3 for_i. dec
      ld0 ld3 add
      ld1 ld5 add
      lda sta
    buf .for_i !bcc pop
  !rt3
