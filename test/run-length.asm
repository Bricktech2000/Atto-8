@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm

# the image to be displayed is 24 characters wide by 12 characters tall. that's 288 bytes. the Atto-8
# only has 256 bytes of RAM, and so compression is necessary. this demo uses run-length encoding

main!
  :rle_profile_picture !rle_puts

  !hlt

  rle_profile_picture:
    !06 @20 @2C @2B @2A !02 @25 !02 @40 !02 @25 @2A @2B @2C @0D @0A
    !03 @20 @3A @2A !0E @40 @2A @3A @0D @0A
    !02 @20 @2A !06 @40 @23 @2A !02 @2B @2A @23 !06 @40 @2A @0D @0A
    @20 @23 !05 @40 @25 !03 @2B !02 @3A !03 @2B @25 !05 @40 @23 @0D @0A
    @2B !06 @40 @2A !03 @2B !02 @2A !03 @2B @23 !06 @40 @2B @0D @0A
    @25 !06 @40 @25 @2B !06 @3A @2B @25 !06 @40 @25 @0D @0A
    @25 !08 @40 @23 @2A !02 @2B @2A @23 !08 @40 @25 @0D @0A
    @2B !04 @40 @23 @2A @2B @3D @2B !04 @40 @2B @3D @2B @2A @23 !04 @40 @2B @0D @0A
    @20 @23 !02 @40 @23 !05 @3A @25 !02 @40 @25 !05 @3A @23 !02 @40 @23 @0D @0A
    !02 @20 @2A @40 @23 !05 @3A @2B !02 @40 @2B !05 @3A @23 @40 @2A @0D @0A
    !03 @20 @3A @2A @3A !05 @3A !02 @2A @3A !05 @3A @2A @3A @0D @0A
    !05 @20 !02 @27 @22 !02 @2D !04 @3A !02 @2D @22 !02 @27 @0D @0A
    @00

rle_puts!
  for_c.
    # count = *ptr & 0b10000000 ? *ptr : 0x01
    x01 shl @const ld1 lda shl iff
    # ptr += *ptr & 0b10000000 ? 0x01 : 0x00
    # count &= 0b01111111
    x00 ad2 @dyn shr
    # char = *ptr
    ld1 lda swp
    # while (count--) putc(char)
    for_r. dec
      ld1 !putc
    !check_zero .for_r !bcc # bleed `0x00`
    # loop if *ptr != '\0'
    !is_equal
  inc .for_c !bcc pop

00! @80 01! @81 02! @82 03! @83 04! @84 05! @85 06! @86 07! @87
08! @88 09! @89 0A! @8A 0B! @8B 0C! @8C 0D! @8D 0E! @8E 0F! @8F
