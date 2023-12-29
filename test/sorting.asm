@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm

main!
  :str_atto-8 !str_atto-8.len dec :sort !call
  :str_atto-8 !puts # -8Aott
  !hlt

  !sort.def

  str_atto-8: @41 @74 @74 @6F @2D @38 @00 str_atto-8.end: # "Atto-8"

str_atto-8.len! :str_atto-8.end :str_atto-8 sub @const
