@ ../../lib/core.asm
@ ../../lib/microcomputer/display.asm

main!
  pop !hlt

  !front_buffer @org
  d07 dE0
  d18 d78
  d38 d7C
  d70 d3E
  d67 d9E
  d8F dC1
  d8F dCD
  dCF dDF
  dE7 d9F
  dE0 d0D
  dCF dF1
  d72 d4E
  d22 d44
  d20 d04
  d10 d08
  d0F dF0
