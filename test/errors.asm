nop
.label

main!
main!
  @ foo.asm
  x
  x2
  @ZZ
  xca
  xF00
  ad9
  ld10
  #comment
  :label
  unused:
  unused.
  !macro
  foobar
  dup: dup:
  dup. dup.
  lda @const
  lda @org
  x00 @org
  lda @data
  :future @org future:
  dyn: :dyn @dyn
  !self
  @error

  !row !row !row !row !row !row !row !row !row !row !row !row !row !row !row !row
row! @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00

self! !main
