@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm

main!
  :str_Atto-8 ld0 !str_Atto-8.len dec :sort !call
  :str_Atto-8 !puts # -8Aott
  !hlt

  !sort.def

  str_Atto-8: @41 @74 @74 @6F @2D @38 @00 str_Atto-8.end:

str_Atto-8.len! :str_Atto-8.end :str_Atto-8 sub @const
