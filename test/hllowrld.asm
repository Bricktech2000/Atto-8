@ lib/microprocessor/core.asm
@ lib/microprocessor/types.asm
@ lib/microprocessor/string.asm
@ lib/microcomputer/stdio.asm
@ lib/microcomputer/display.asm

main!
  pop pop !display_buffer sts

  loop:
    !display_data_len x00 !display_buffer :memset !call
    !here !wait_char
    !display_data_len :display_data !display_buffer :memcpy !call
    !here !wait_null
  :loop !jmp

  !memset_def
  !memcpy_def

  display_data:
    # dEA dE6 d4E d44 d4A dEC d00 d00 # THIS
    # dE6 d04 d44 d0E dEC d0A d00 d00 # IS A
    # dEE d6E d4C d44 d4E dC4 d00 d00 # TEST

    # d4E dEE dE4 d4A dA4 d4E d00 d00 # ATTO
    # d00 d06 d00 dEE d00 d0E d00 d00 #   -8

    dA8 d8E dE8 d8A dAE dEE d00 d00 # HLLO
    dAE d8C dEC d8A dEA dEC d00 d00 # WRLD
  display_data_end:
display_data_len! :display_data_end :display_data sub @const
