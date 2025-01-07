@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm
@ misc/common/common.asm

# a best-effort animation of the Matrix code that uses no terminal escapes

main!
  pop pop :cols sts

  !rand_seed # rand_seed

  x00 loop:
    # `!rand` has cycle length 256 and is called once per column on every row, and so the
    # pattern reappears shifted `256 % COL_COUNT` columns to the right every `256 / COL_COUNT`
    # rows (typically 2 or 3). to avoid this, we decrement `rand_seed` after every row to jump
    # to a random spot in the `!rand` cycle
    sub @dyn

    !frame_delay !delay
    !'\n' !putc

    # refer to C implementation for this sub-loop
    :cols for_col:
      ld0 lda # load `cols[c]`
        ld2 !rand st2 ld2 # load `rand()`
          # cols[c] = rand() < DENSITY ? TRAIL_LEN : cols[c]
          !density sub @dyn !trail_len if2
        # rand_char = (rand() & 0x3f) + ';'
        x3F and !';' add :rand_char sta
        # cols[c] -= cols[c] != 0
        !z xFF add @dyn # sets carry
        # putchar(*min(&TAIL[cols[c]], &rand_char))
        :tail dec @const ld1 add @dyn :rand_char !min.dyn lda !putc
      ld1 sta # store `cols[c]'
    inc !z :for_col !bcc # bleed `0x00` and set carry
  :loop !jmp

  tail: @20 @27 @27 @3A @3B @21 @2B @24 rand_char: @00 # " '':;!+$"

  # put the last column right before address `0x00` as it's easily recognizable through `!z`
  !col_count neg @org cols:

# density! x01 # light
density! x03 # medium
# density! x05 # heavy
# density! xFF # maximum

col_count! x50 # 80 columns
# col_count! x78 # 120 columns
# col_count! xAD # maximum

# trail_len! x08 # tail only
trail_len! x10 # length 16
# trail_len! x18 # length 24
# trail_len! xBC # maximum

# frame_delay! x00 # fast
frame_delay! x02 # normal
# frame_delay! x04 # slow
