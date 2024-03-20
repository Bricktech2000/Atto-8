@ lib/core.asm
@ lib/types.asm
@ lib/string.asm
@ lib/stdio.asm

main!
  x02 :str_helo_wooorld x02 add @const :str_helo_wooorld x03 add @const :memmove !call
  x06 :str_helo_wooorld x09 add @const :str_helo_wooorld x07 add @const :memmove !call
  :str_helo_wooorld !puts # hello world
  !hlt

  str_helo_wooorld: @68 @65 @6C @6F @21 @20 @77 @6F @6F @6F @72 @6C @64 @00 # "helo! wooorld"

  !memcpy.def
  !memmove.def
