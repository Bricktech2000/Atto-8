delay! # delay(iterations)
  delay. dec
  buf .delay !bcc pop

delay_long! # delay_long(iterations)
  delay. dec
  xFF !delay
  buf .delay !bcc pop
