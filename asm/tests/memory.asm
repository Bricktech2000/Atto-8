@ ../../lib/core.asm
@ ../../lib/memcpy.asm
@ ../../lib/memset.asm
@ ../../lib/microcomputer/input.asm
@ ../../lib/microcomputer/display.asm

main!
  pop !front_buffer sts
  !reset_input

  loop:
    !display_data_len :display_data !front_buffer :memcpy !call
    !wait_input
    !display_data_len x00 !front_buffer :memset !call
    !wait_input
  :loop sti
  !hlt

  display_data:
    dEA dE6 d4E d44 d4A dEC d00 d00 # THIS
    dE6 d04 d44 d0E dEC d0A d00 d00 # IS A
    dEE d6E d4C d44 d4E dC4 d00 d00 # TEST

    # d4E dEE dE4 d4A dA4 d4E d00 d00 # ATTO
    # d00 d06 d00 dEE d00 d0E d00 d00 #   -8
  display_data_end:

  !memset
  !memcpy

display_data_len! :display_data_end :display_data sub @const
