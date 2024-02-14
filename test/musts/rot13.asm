@ lib/core.asm
@ lib/types.asm
@ lib/stdio.asm

main! !nop
  loop:
    # read user input
    !getc ld0
      # make lowercase. if outside of alphabet, will become
      # corrupted but will stay outside of alphabet
      !char.space orr
      # map `'a'..='z'` to `0..=25`
      !char.latin_small_letter_a sub
      x00 # default offset: `0x00`
        # if character in `'a'..='m'` then offset is `13`
        !offset su2 !offset iff
        # if character in `'n'..='z'` then offset is `-13`
        !offset su2 !offset neg dec @const iff
      st0
    # add offset to character and print it
    add @dyn !putc
  :loop !jmp

offset! !char.small_letter_count shr @const # half the alphabet
