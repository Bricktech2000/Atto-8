@ lib/core.asm
@ lib/types.asm
@ lib/stdio.asm
@ misc/common/common.asm

# bytewise in-place simulation of elementary cellular automata. the current state
# of an automaton is updated progressively, byte by byte, within a single buffer

main!
  pop pop @dyn !buffer sts

  !buffer.end dec lda # left

  # refer to C implementation for this main loop
  loop:
    # loop through bytes of `buffer`
    !buffer for_i:
      # compute `above`
      ld1 x01 and # loads `left & 0x01` as MSB of `above`
      ld1 inc !z !buffer iff lda shl @dyn pop # loads `buffer[(i + 1) % BUFFER_LEN]` into `CF`
      ld1 lda st2 ld2 shl @dyn ld1 ad2 @dyn # `above <<= 1, above |= CF` and `left = buffer[i]`

      x00 # [~curr]
      # loop through bit windows of `above`
      sec while:
        x01 # bit mask to extract `[~bit]` from `RULE`
          # above <<= 1
          ld3 ld3 ad4 @dyn ad4 @dyn
        # compute `[~bit]` from `RULE` and bit window
        ld3 x07 and rot !rule and @dyn pop
        # print either '@@' or '  ' depending on `[~bit]`
        !'@' !'\s' !char.iff !char.ld0 !putc !putc
        # [~curr] <<= 1, [~curr] |= [~bit]
        shl @dyn
      # loop while `sec` bit not shifted out
      ld1 shl !zr :while !bcc
      not # curr = ~[~curr]

      # buffer[i] = curr
      st1 pop ld1 sta
    # loop until `BUFFER_END` reached
    inc !z :for_i !bcc pop

    !'\n' !putc
  :loop !jmp

  !buffer @org !configuration

buffer! !buffer.end !buffer.len sub @const
buffer.end! x00 # `0x00` as it's easily recognizable through `!z`


# identical to 'rule 110.asm'
random! !random_noise
center! !buffer.len shr !pad @80
right! !buffer.len dec !pad @01
left! @80

# identical to 'rule 110.asm'
# rule! x1A configuration! !random # rule 26
# rule! x1E configuration! !center # rule 30
# rule! x39 configuration! !center # rule 57
# rule! x49 configuration! !random # rule 73
# rule! x5A configuration! !center # rule 90
# rule! x69 configuration! !center # rule 105
# rule! x69 configuration! !random # rule 105
# rule! x6D configuration! !random # rule 109
rule! x6E configuration! !right # rule 110
# rule! x78 configuration! !random # rule 120
# rule! x7C configuration! !left # rule 124
# rule! x81 configuration! !random # rule 129
# rule! x92 configuration! !random # rule 146
# rule! x96 configuration! !random # rule 150
# rule! xA6 configuration! !random # rule 166
# rule! xE1 configuration! !random # rule 225

buffer.len! x08 # short
# buffer.len! x10 # medium
# buffer.len! x20 # long
# buffer.len! x40 # longer
# buffer.len! x80 # longerer
