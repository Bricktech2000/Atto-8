@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm

main!
  x00 # non-empty stack required for `printf` with specifier `%`
  x21 x32 sub x32 x21 :str_0x%x_-_%u_=_%d :printf !call
  !char.latin_capital_letter_a :str_'%c'_uses_%%c :printf !call
  :str_fmt_=_(char*)%p :str_fmt_=_(char*)%p :printf !call
  :str_fmt_=_"%s" :str_fmt_=_"%s" :printf !call
  !hlt

  str_0x%x_-_%u_=_%d: @30 @78 @25 @78 @20 @2D @20 @25 @75 @20 @3D @20 @25 @64 @0D @0A @00
  str_'%c'_uses_%%c: @27 @25 @63 @27 @20 @75 @73 @65 @73 @20 @25 @25 @63 @0D @0A @00
  str_fmt_=_(char*)%p: @66 @6D @74 @20 @3D @20 @28 @63 @68 @61 @72 @2A @29 @25 @70 @0D @0A @00
  str_fmt_=_"%s": @66 @6D @74 @20 @3D @20 @22 @25 @73 @22 @00

  !printf.def
