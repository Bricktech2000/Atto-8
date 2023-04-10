memset!
  memset: # memset(ptr, val, len)
    ld3 for_i. dec
      ld0 ld3 add ld4 sta
    buf .for_i !bcc pop
  !rt3
