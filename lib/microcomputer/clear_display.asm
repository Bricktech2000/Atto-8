clear_display!
  clear_display: # clear_display()
    x20 for_i. dec
      ld0 !front_buffer add x00 sta
    buf .for_i !bcc pop
  !rt0
