@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm

main!
  x00 # non-empty stack required for `printf` with conversion specifier '%'
  x21 x32 sub x32 x21 :str_format_0 :printf !call
  !'A' :str_format_1 :printf !call
  :str_format_2 :str_format_2 :printf !call
  :str_format_3 :str_format_3 :printf !call !'\n' !putc
  !hlt

  !printf.def

  str_format_0: @30 @78 @25 @78 @20 @2D @20 @25 @75 @20 @3D @20 @25 @64 @0A @00 # "0x%x - %u = %d\n"
  str_format_1: @27 @25 @63 @27 @20 @75 @73 @65 @73 @20 @25 @25 @63 @0A @00 # "'%c' uses %%c\n"
  str_format_2: @66 @6D @74 @20 @3D @20 @28 @63 @68 @61 @72 @2A @29 @25 @70 @0A @00 # "fmt = (char*)%p\n"
  str_format_3: @66 @6D @74 @20 @3D @20 @22 @25 @73 @22 @00 # "fmt = \"%s\""

