@ lib/core.asm
@ lib/types.asm
@ lib/string.asm
@ lib/stdlib.asm
@ lib/stdio.asm
@ lib/display.asm

main!
  pop pop !display_buffer sts

  loop:
    !display_data_len x00 !display_buffer :memset !call
    !block_any
    !display_data_len :display_data !display_buffer :memcpy !call
    !block_null
  :loop !jmp

  !memset.def
  !memcpy.def

  display_data:
    # @4E @EE @E4 @4A @A4 @4E @00 @00 # ATTO
    # @00 @06 @00 @EE @00 @0E @00 @00 #   -8

    @A8 @8E @E8 @8A @AE @EE @00 @00 # HLLO
    @AE @8C @EC @8A @EA @EC @00 @00 # WRLD
  display_data_end:
display_data_len! :display_data_end :display_data sub @const
