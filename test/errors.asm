nop
.label

main!
main!
  @ foo.asm
  x
  dZZ
  xca
  xF00
  #comment
  :label
  !macro
  foobar
  dup: dup:
  lda @const
  lda @org
  x00 @org
  :future @org future:
  dyn: :dyn @dyn
  !self
  @err

  !row !row !row !row !row !row !row !row !row !row !row !row !row !row !row !row
row! d00 d00 d00 d00 d00 d00 d00 d00 d00 d00 d00 d00 d00 d00 d00 d00

self! !main
