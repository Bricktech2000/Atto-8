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

  !memset.def
  !memcpy.def

  display_data:
    # @EA @E6 @4E @44 @4A @EC @00 @00 # THIS
    # @E6 @04 @44 @0E @EC @0A @00 @00 # IS A
    # @EE @6E @4C @44 @4E @C4 @00 @00 # TEST

    # @4E @EE @E4 @4A @A4 @4E @00 @00 # ATTO
    # @00 @06 @00 @EE @00 @0E @00 @00 #   -8

    @A8 @8E @E8 @8A @AE @EE @00 @00 # HLLO
    @AE @8C @EC @8A @EA @EC @00 @00 # WRLD
  display_data_end:
display_data_len! :display_data_end :display_data sub @const
