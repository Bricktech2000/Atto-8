@ lib/core.asm
@ lib/types.asm
@ lib/stdio.asm

main!
  nop @dyn loop:
    !getc x00 # default offset: `0x00`
      # do not touch the sacred code below. arranging it this way allows
      # the assembler to generate highly space-efficient code. most of it
      # disappears during the optimization phase.
      !char.latin_capital_letter_a su2 clc !offset su2 !offset     iff clc !char.latin_capital_letter_a ad2 !offset ad2
      !char.latin_capital_letter_n su2 clc !offset su2 !offset neg iff clc !char.latin_capital_letter_n ad2 !offset ad2
      !char.latin_small_letter_a   su2 clc !offset su2 !offset     iff clc !char.latin_small_letter_a   ad2 !offset ad2
      !char.latin_small_letter_n   su2 clc !offset su2 !offset neg iff clc !char.latin_small_letter_n   ad2 !offset ad2
    # add offset to character and print it
    clc add clc !putc
  :loop !jmp

offset! !char.capital_letter_count shr @const # half the alphabet
