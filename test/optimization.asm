@ lib/microprocessor/core.asm

main!
  x4F x06 add @const
  x50 x05 orr @const
  xF5 x5F and @const
  xAB neg @const
  ld0 !cabs @const
  xAB !cabs @const
  xAA x01 rot @const
  x01 x55 swp pop @const

  !hlt
