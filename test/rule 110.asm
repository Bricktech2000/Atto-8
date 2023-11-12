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
      !char.commercial_at !char.space !char.iff !char.ld0 !putc !putc
      !back_buffer !bit_addr !store_bit
    # loop until `x == BUFFER_LEN << 3`, which is the number of bits aka
    # pixels in a buffer of length `BUFFER_LEN`
    ld0 !buffer_len x03 rot @const !eq
    :for_x !bcc !u8.pop

    # copy back buffer to display buffer and print newline
    !buffer_len !back_buffer !display_buffer :memcpy !call clc
    !char.carriage_return !putc !char.line_feed !putc
  :loop !jmp

  !memcpy.def

  !display_buffer @org
    # !buffer_len shr !pad @80 # for rule 30
    # !buffer_len shr !pad @80 # for rule 90
    !buffer_len dec !pad @01 # for rule 110
    # x00 !pad @80 # for rule 124

rule!
  # x1E # rule 30
  # x5A # rule 90
  x6E # rule 110
  # x7C # rule 124

buffer_len! !display_buffer.len shr shr @const # short
# buffer_len! !display_buffer.len shr @const # medium
# buffer_len! !display_buffer.len dec @const # long
