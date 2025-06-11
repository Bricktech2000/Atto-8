@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm
@ lib/display.asm
@ misc/common/common.asm

main! begin:
  pop pop !slides_buffer dec sts # (dst,)

  # we do a `memswp` using two pointers: a "slides" pointer `src` and a "display buffer"
  # pointer `dst`. `src` gets reset to `SLIDES_BUFFER` after the last slide is displayed,
  # and `dst` gets reset to `DISPLAY_BUFFER` after each slide is displayed.

  # reset `src = SLIDES_BUFFER`
  !slides_buffer loop:
    # reset `dst = DISPLAY_BUFFER` while clearing the carry flag
    !display_buffer not @const not @dyn st1
    for_byte:
      # swap `*src++` and `*dst++`
      ld1 lda ld1 lda
      ld3 sta ld1 sta
      x01 add @dyn
      # loop back to `:for_byte` until `dst` wraps around, at which point one screenful
      # will have been `memswp`ed over. when it does, if we've hit the last slide then
      # jump to `:begin` to reset `src = SLIDES_BUFFER`, and otherwise jump to `:block`
      # to wait for user input before `memswp`ing the next slide into the display buffer
      :for_byte
        :block :begin iff
        x01 ad4 @dyn
      iff !jmp
    block:
      !block_null !block_any
  :loop !jmp

  !slides_buffer @org
    !lyrics_first
    # !lyrics_second
    # !pixel_art
    # !numbered_slides

slides_buffer! x40

lyrics_first!
  !music_notes
  !nver !gon' !give !you !up !---
  !nver !gon' !let !you !down !---
  !nver !gon' !run !arnd !and !dsrt !you !---

lyrics_second!
  !music_notes
  !nver !gon' !make !you !cry !---
  !nver !gon' !say !good !bye !---
  !nver !gon' !tell !alie !and !hurt !you !---

pixel_art!
  !atto_-8
  !avatar
  !mushroom
  !yin_yang
  !music_notes
  !checkerboard

numbered_slides!
  !empty !slide !-1- !empty
  !empty !slide !-2- !empty
  !empty !slide !-3- !empty
  !empty !slide !-4- !empty
  !empty !slide !-5- !empty
  !empty !slide !-6- !empty

# identical to 'scroll.asm'
empty! @00 @00 @00 @00 @00 @00 @00 @00 #

slide! @69 @67 @49 @56 @CD @67 @00 @00 # SLIDE
-1-!  @03 @00 @39 @38 @03 @80 @00 @00 # -1-
-2-!  @03 @00 @39 @38 @01 @80 @00 @00 # -2-
-3-!  @03 @80 @39 @B8 @03 @80 @00 @00 # -3-
-4-!  @02 @80 @3B @B8 @00 @80 @00 @00 # -4-
-5-!  @01 @80 @39 @38 @03 @00 @00 @00 # -5-
-6-!  @02 @00 @3B @B8 @03 @80 @00 @00 # -6-

# identical to 'scroll.asm'
nver! @CA @EC @AA @CE @A4 @EA @00 @00 # NVER
gon'! @CE @C8 @AA @A8 @EE @A0 @00 @00 # GON'
give! @CE @AE @A4 @AC @EE @4E @00 @00 # GIVE
you!  @AE @A0 @4A @A0 @4E @E0 @00 @00 # YOU
up!   @AE @00 @AE @00 @E8 @00 @00 @00 # UP
---!  @00 @00 @11 @10 @00 @00 @00 @00 # •••
let!  @8E @E0 @8C @40 @EE @40 @00 @00 # LET
down! @CE @AC @AA @EA @CE @EA @00 @00 # DOWN
run!  @CA @C0 @EA @A0 @AE @A0 @00 @00 # RUN
arnd! @4C @CC @EE @AA @AA @AC @00 @00 # ARND
and!  @4C @C0 @EA @A0 @AA @C0 @00 @00 # AND
dsrt! @C6 @CE @A4 @E4 @CC @A4 @00 @00 # DSRT
make! @E4 @AE @EE @CC @AA @AE @00 @00 # MAKE
cry!  @EC @A0 @8E @40 @EA @40 @00 @00 # CRY
say!  @64 @A0 @4E @40 @CA @40 @00 @00 # SAY
good! @CE @EC @AA @AA @EE @EC @00 @00 # GOOD
bye!  @CA @E0 @EE @C0 @E4 @E0 @00 @00 # BYE
tell! @EE @88 @4C @88 @4E @EE @00 @00 # TELL
alie! @42 @2E @E2 @2C @A3 @AE @00 @00 # A LIE
hurt! @AA @CE @EA @E4 @AE @A4 @00 @00 # HURT
