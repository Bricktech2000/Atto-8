@ lib/core.asm
@ lib/types.asm
@ lib/string.asm
@ lib/stdlib.asm
@ lib/display.asm
@ misc/common/common.asm

# to count neighbors, display buffer is read from and back buffer is written to.
# back buffer is copied to display buffer at the end of each iteration

main!
  pop pop !back_buffer sts

  loop:
    # loop through cells
    x00 !u4u4 for_xy: dec
      x00 # rule_byte
      x00 # neighbor_count

      # count cells in neighborhood
      !neighborhood.len for_dxdy: dec
        :neighborhood ld1 add !i4i4.lda !u4u4.ld4 !i4i4.add
        !display_buffer !bit_addr !load_bit @dyn
        # we use `x07 ro2` on `!rule` because `neighbor_count` includes
        # the current cell, whereas the layout of `!rule` assumes it does
        # not. the line below not only counts neighbors but also repeatedly
        # overwrites `rule_byte` with either the first or second byte of
        # `!rule` depending on whether the current cell is alive or dead.
        # the last cell to be checked is that with offset `0x00`, and so
        # `rule_byte` ends up containing the correct half of the ruleset
        !rule x07 ro2 @const iff st3 clc ad2
      !z :for_dxdy !bcc pop

      # apply `rule_byte` ruleset and store result
      rot x01 and
      !u4u4.ld1 !back_buffer !bit_addr !store_bit
    !z :for_xy !bcc !u4u4.pop

    # copy back buffer to display buffer
    !display_buffer.len !back_buffer !display_buffer :memcpy !call
  :loop !jmp

  # implicitly include the current cell into its own neighborhood
  # for the last iteration of `for_dxdy` above
  neighborhood: @00 !i4i4 !neighborhood neighborhood.end:

  !memcpy.def

  !display_buffer @org
    # !blinker
    # !glider
    # !diehard
    # !r-pentomino
    # !lightweight_spaceship
    !heavyweight_spaceship
    # !compact_pulsar
    # !copperhead
    # !figure_eight

neighborhood.len! :neighborhood.end :neighborhood sub @const

# the first byte of `!rule` is the ruleset for an alive "survive" cell,
# and the second byte is the ruleset for a dead "born" cell. we assume
# a neighborhood excludes the current cell. the `n`th bit of both bytes
# below represents the new state of a cell with `n + 1` neighbors. that
# is, bits are arranged as `0b12345678`. since we only have 8 bits to
# work with, we can't differentiate between a cell with 8 neighbors and
# a cell with 0 neighbors, meaning rulesets that include both of these
# cases are unrepresentable with this encoding
rule! x60 x20 neighborhood! !moore_neighborhood # Conway's Life
# rule! xF7 xF3 neighborhood! !moore_neighborhood # AntiLife
# rule! x00 x40 neighborhood! !moore_neighborhood # Seeds
# rule! xF8 x20 neighborhood! !moore_neighborhood # Maze


moore_neighborhood!
  @FF !i4i4
  @F0 !i4i4
  @F1 !i4i4
  @0F !i4i4
  @01 !i4i4
  @1F !i4i4
  @10 !i4i4
  @11 !i4i4

von_neumann_neighborhood!
  @F0 !i4i4
  @0F !i4i4
  @01 !i4i4
  @10 !i4i4


blinker! x0C !pad
  @07 @00

glider! x0C !pad
  @07 @00
  @01 @00
  @02 @00

diehard! x0C !pad
  # already advanced 2 generations
  @30 @80
  @31 @C0

r-pentomino! x0C !pad
  @06 @00
  @0C @00
  @04 @00

lightweight_spaceship! x0A !pad
  @00 @09
  @00 @10
  @00 @11
  @00 @1E

heavyweight_spaceship! x0A !pad
  @00 @0C
  @00 @21
  @00 @40
  @00 @41
  @00 @7E

compact_pulsar! x0C !pad
  # pattern that turns into a pulsar
  @07 @C0
  @08 @40
  @07 @C0

copperhead! x08 !pad
  @06 @60
  @01 @80
  @01 @80
  @0A @50
  @08 @10
  @00 @00
  @08 @10
  @06 @60
  @03 @C0
  @00 @00
  @01 @80
  @01 @80

figure_eight! x0A !pad
  @00 @E0
  @00 @E0
  @00 @E0
  @07 @00
  @07 @00
  @07 @00
