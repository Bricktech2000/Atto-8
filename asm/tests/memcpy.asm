@../../lib/utils/core.asm
@../../lib/utils/memcpy.asm
@../../lib/microcomputer/utils.asm

main!
  !front_buffer !alloc_buffer
  !display_data_len :display_data !front_buffer :memcpy !call
  !hlt

  display_data:
    dEA dE6 d4E d44 d4A dEC d00 d00
    dE6 d0E d44 d0E dEC d0A d00 d00
    dEE d6E d4C d44 d4E dC4 d00 d00
  display_data_end:

  !memcpy

display_data_len! :display_data_end :display_data sub
