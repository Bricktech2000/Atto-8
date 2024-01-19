@ lib/core.asm
@ lib/types.asm
@ lib/string.asm
@ lib/stdlib.asm
@ lib/stdio.asm
@ lib/display.asm

main!
  pop pop !display_buffer sts

  # wait for input then create screen tear effect
  !block_any
  xC0 x01 :delay_shift_rows !call
  xE0 x03 :delay_shift_rows !call
  xF8 x03 :delay_shift_rows !call
  xFC x67 :delay_shift_rows !call
  xFC xFF :delay_shift_rows !call
  x10 for_i: dec
    xFF xFF :delay_shift_rows !call # `xFF xFF` shifts all rows
  !z :for_i !bcc pop

  # `memcpy` mock playfield into display buffer
  !display_buffer.len :playfield !display_buffer :memcpy !call
  # wait for input then `memcpy` "if only" text into display buffer
  x7F !delay !block_any
  x12 :if_only !display_buffer x04 add @const :memcpy !call !hlt

  !memcpy.def
  !delay_shift_rows.def

  if_only: !if_only
  # playfield: !playfield_black
  playfield: !playfield_checkered

  # # poke into display buffer to set initial state of loading indicator
  # x01 xEC sta x80 xED sta x02 xEE sta
  # # iterate through `:indicator_pixels` and flip pixels in display buffer.
  # # this renders a loading indicator that loads forever
  # x00 loop: dec
  #   !indicator_pixels.len dec and clc
  #   :indicator_pixels ld1 add lda !display_buffer !bit_addr !flip_bit
  #   x08 !delay
  # :loop !jmp
  #
  # !delay_shift_rows.def
  #
  # # sequence of pixel positions for a loading indicator. right column
  # # is identical to left column but offset by 3 steps
  # indicator_pixels:
  #   @86 !u4u4 @68 !u4u4
  #   @97 !u4u4 @67 !u4u4
  #   @98 !u4u4 @76 !u4u4
  #   @89 !u4u4 @86 !u4u4
  #   @79 !u4u4 @97 !u4u4
  #   @68 !u4u4 @98 !u4u4
  #   @67 !u4u4 @89 !u4u4
  #   @76 !u4u4 @79 !u4u4
  # indicator_pixels.end:

  !display_buffer @org
    @00 @00 @EE @EE @AA @AE @AA @AE
    @AE @EA @C0 @02 @80 @02 @0C @00
    @1F @00 @3F @81 @3F @C6 @3E @D8
    @1F @60 @0F @83 @1F @83 @39 @80

indicator_pixels.len! :indicator_pixels.end :indicator_pixels sub @const

if_only! @00 @00
  @0E @E0 @04 @C0 @0E @80 @00 @01 #  if
  @EC @95 @AA @88 @EA @E9 @00 @00 # only!

playfield_black!
  @00 @00 @00 @00 @80 @01 @E0 @07
  @F8 @FF @F8 @FF @F8 @FF @F8 @FF
  @E0 @07 @80 @01 @00 @00 @00 @00
  @00 @00 @01 @80 @03 @C0 @03 @C0

playfield_checkered!
  @55 @55 @2A @AA @95 @51 @E0 @07
  @F8 @FF @F8 @FF @F8 @FF @F8 @FF
  @E0 @07 @95 @51 @2A @AA @55 @55
  @AA @2A @54 @95 @A9 @CA @55 @D5

delay_shift_rows.def!
  delay_shift_rows:
    # stall for some time then fall through
    x0C !delay !shift_rows.def
    # prevent unused label warning
    :shift_rows pop

shift_rows.def!
  shift_rows: clc # shift_rows(u16 mask)
    # loop through display buffer rows
    !display_buffer for_row:
      # load `(*row, *row << 1)`
      ld0 !u16.lda !u16.ld0 !u16.shl
        # check if `(mask << i) & 0x8000`
        !u16.ld3 ad8 ad8 # `!u16.ad4`
      # if so, store `*row << 1`; else, store `*row`
      !u16.iff swp ld2 !u16.sta
    inc inc !z :for_row !bcc pop
  # return*
  !rt2
