delay! # delay(iterations)
  delay. dec
  buf .delay !bcc pop

delay_long! # delay_long(iterations)
  delay. dec
  x7F !delay
  buf .delay !bcc pop
