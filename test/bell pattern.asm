@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm

main!
  !pattern.len for_byte: dec
    # emit sound by sending `'\a'` to `stdout`
    !char.bell !putc
    # decrement the delay argument by `1` to account for the clocks required to execute
    # the rest of the loop. this was determined experimentally using `!tuning_pattern`
    :pattern ld1 add lda dec !delay
    # if we're at the begining of the pattern then loop back to the end of the pattern
    !z !pattern.len iff clc
  :for_byte !jmp

  pattern:
    !rumba_clave !son_clave !son_clave !son_clave
    # !tresillo
    # !cinquillo
    # !son_clave
    # !rumba_clave
    # !bossa_nova_clave
    # !standard_pattern
    # !tuning_pattern
  pattern.end:

pattern.len! :pattern.end :pattern sub @const

# note durations, in reverse order
tresillo! @40 @60 @60
cinquillo! @40 @20 @40 @20 @40
son_clave! @80 @40 @80 @60 @60
rumba_clave! @80 @40 @60 @80 @60
bossa_nova_clave! @60 @60 @80 @60 @60
standard_pattern! @20 @60 @40 @60 @20 @60 @60
tuning_pattern! @20 @40
