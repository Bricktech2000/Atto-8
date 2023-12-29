@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm

main!
  :str_hello_world !puts !hlt

  str_hello_world: @48 @65 @6C @6C @6F @2C @20 @77 @6F @72 @6C @64 @21 @00 # "Hello, world!"
