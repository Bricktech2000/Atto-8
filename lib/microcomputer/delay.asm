delay!
  delay: # delay(iterations)
    swp delay. dec
    buf .delay !bcc pop
  !ret

delay_long!
  delay_long: # delay_long(iterations)
    swp delay. dec
    xFF :delay !call
    buf .delay !bcc pop
  !ret
