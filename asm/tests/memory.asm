@../../lib/core.asm
@../../lib/memcpy.asm
@../../lib/memset.asm
@../../lib/microcomputer/pause.asm
@../../lib/microcomputer/core.asm

main!
  !front_buffer !alloc_buffer
  !reset_input

  loop:
    !display_data_len :display_data !front_buffer :memcpy !call
    !pause
    !display_data_len x00 !front_buffer :memset !call
    !pause
  :loop sti
  !hlt

  display_data:
    dEA dE6 d4E d44 d4A dEC d00 d00
    dE6 d0E d44 d0E dEC d0A d00 d00
    dEE d6E d4C d44 d4E dC4 d00 d00
  display_data_end:

  !memset
  !memcpy

display_data_len! :display_data_end :display_data sub
