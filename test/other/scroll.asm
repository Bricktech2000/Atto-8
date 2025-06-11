@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm
@ misc/common/common.asm

main!
  pop pop !image_buffer sts

  loop:
    x04 !delay
    !block_any

    # rotate `!img_buf` inclusive to `x00` exclusive two bytes left to move the
    # contents up one line
    !image_buffer @const lda
    !image_buffer inc @const lda
    !image_buffer inc inc @const
      for_byte:
        ld0 lda
        ld1 dec dec
        ld0 lda
        ld3 sta
        sta
      inc !z :for_byte !bcc # bleed `0x00`
    dec sta
    xFE sta
  :loop !jmp

  !image_buffer @org
    !lyrics_first
    # !lyrics_second
    # !pixel_art

image_buffer! x40 x08 sub @const

lyrics_first!
  !empty
  !nver !gon' !give !you !up !---
  !nver !gon' !let !you !down !---
  !nver !gon' !run !arnd !and !dsrt !you !---
  !music_notes

lyrics_second!
  !empty
  !nver !gon' !make !you !cry !---
  !nver !gon' !say !good !bye !---
  !nver !gon' !tell !alie !and !hurt !you !---
  !music_notes

pixel_art!
  !empty !avatar
  !empty !mushroom
  !empty !yin_yang
  !empty !music_notes
  !empty !checkerboard

# identical to 'slideshow.asm'
empty! @00 @00 @00 @00 @00 @00 @00 @00 #

# identical to 'slideshow.asm'
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
