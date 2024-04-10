@ lib/core.asm
@ lib/types.asm
@ lib/string.asm
@ lib/stdio.asm
@ lib/display.asm
@ misc/common/common.asm

main!
  pop pop !back_buffer sts

  loop:
    # loop through cells
    xFF !u8 for_x:
      !rule # wolfram code
      x20 # bit_index

      for_dx:
        # load `x` then increment the original `x`
        ld2 x01 ad4
        # load pixel `x` and shift into `bit_index`. `!load_bit` ends in
        # `x01 and`, which means that the bits in `bit_index` are inverted
        !display_buffer !bit_addr !load_bit @dyn pop shl
      :for_dx !bcc

      # we use a `rot` to extract the required bit from `!rule`. however,
      # `rot` rotates left, which makes `n rot x01 and` give us the `n`th
      # bit from the left. since `bit_index` is `not`ed from above, we can
      # leverage `~n + 1 == -n` to get the `n`th bit from the right, hence
      # the `inc` instruction below. also note that we use `x02 su2` so
      # `x` increments by `1` on every iteration instead of by `3`
      inc rot x02 su2 x01 and ld1
      # print `@@` or `  ` depending on carry and store bit in back buffer
      !'@' !'\s' !char.iff !char.ld0 !putc !putc
      !back_buffer !bit_addr !store_bit
    # loop until `x == BUFFER_LEN << 3`, which is the number of bits aka
    # pixels in a buffer of length `BUFFER_LEN`
    ld0 !buffer.len x03 rot @const !eq
    :for_x !bcc !u8.pop

    # copy back buffer to display buffer and print newline
    !display_buffer.len !back_buffer !display_buffer :memcpy !call clc
    !'\n' !putc
  :loop !jmp

  !memcpy.def

  !display_buffer @org !configuration

# identical to `rule 110 fast.asm`
random! !random_noise
center! !buffer.len shr !pad @80
right! !buffer.len dec !pad @01
left! @80

# identical to `rule 110 fast.asm`
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

buffer.len! !display_buffer.len shr shr @const # short
# buffer.len! !display_buffer.len shr @const # medium
# buffer.len! !display_buffer.len dec @const # long
