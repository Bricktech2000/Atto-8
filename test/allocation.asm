@ lib/microprocessor/core.asm
@ lib/microprocessor/types.asm
@ lib/microprocessor/stdlib.asm
@ lib/microcomputer/stdio.asm
@ lib/microcomputer/display.asm

main!
  pop pop !display_buffer sts
  !wc x04 :malloc !call # allocate 0x04 bytes
  !wc x05 :malloc !call # allocate 0x05 bytes
  !wc :free !call       # free the 0x05 bytes
  !wc x07 :malloc !call # allocate 0x07 bytes
  !wc swp :free !call   # free the 0x04 bytes
  !wc x0F :malloc !call # allocate 0x0F bytes
  !wc :free !call       # free the 0x0F bytes
  !wc :free !call       # free the 0x07 bytes
  !wc x1E :malloc !call # allocate 0x1E bytes
  !wc :free !call       # free the 0x1E bytes
  !hlt

  !malloc.def
  !free.def

  # use display buffer as heap for demo
  !display_buffer @org

  heap_start: !heap_unlimited
heap_start! :heap_start @const

wc! !here !wait_char
