@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm
@ lib/display.asm

main!
  pop pop !display_buffer sts
  !block_any x04 :malloc !call # allocate 0x04 bytes
  !block_any x05 :malloc !call # allocate 0x05 bytes
  !block_any :free !call       # free the 0x05 bytes
  !block_any x07 :malloc !call # allocate 0x07 bytes
  !block_any swp :free !call   # free the 0x04 bytes
  !block_any x0F :malloc !call # allocate 0x0F bytes
  !block_any :free !call       # free the 0x0F bytes
  !block_any :free !call       # free the 0x07 bytes
  !block_any x1E :malloc !call # allocate 0x1E bytes
  !block_any :free !call       # free the 0x1E bytes
  !hlt

  !malloc.def
  !free.def

  # use display buffer as heap for demo
  !display_buffer @org

  heap_start: !heap_unlimited
heap_start! :heap_start @const
