nop
.label

main!
  @ foo.asm
  dZZ
  xF00
  x
  ad3
  ad12
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

  !row !row !row !row !row !row !row !row !row !row !row !row !row !row !row !row
row! d00 d00 d00 d00 d00 d00 d00 d00 d00 d00 d00 d00 d00 d00 d00 d00

self! !main
