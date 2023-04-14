@ ../../lib/core.asm
@ ../../lib/prng.asm
@ ../../lib/nibble.asm
@ ../../lib/memcpy.asm
@ ../../lib/microcomputer/core.asm
@ ../../lib/microcomputer/pause.asm
@ ../../lib/microcomputer/hex_chars.asm
@ ../../lib/microcomputer/print_byte.asm

main!
  !front_buffer !alloc_buffer
  !reset_input

  !display_data_len :display_data !front_buffer :memcpy !call

  loop:
    x13 :prng !call :print_byte !call
    x12 :prng !call :print_byte !call
    !pause
  :loop sti

  !prng

  !memcpy
  !load_nibble_minimal
  !hex_chars_minimal
  !print_byte_minimal

  display_data:
    # dEE dEC dEC dAA d8A dAE # PRNG
    dE4 dEC dCE dAA dAA dAC # RAND
    d00 d00 dFF dFF         # ----
    # d00 d00 d00 d00 d00 d00 d00 d00 # (empty lines)
    # dEA d00 dA4 d00 dEA d00 # 0X
  display_data_end:

display_data_len! :display_data_end :display_data sub @const
