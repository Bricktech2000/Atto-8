@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm

# converts a string from `getline()` to its Assembly representation

main! !nop
  loop:
    :line_buffer :line_buffer :getline !call
    :str_newline_label :puts !call
    :line_buffer for_c:
      !char.commercial_at !putc
      ld0 lda !u8.to_hex !putc !putc
      !char.space !putc
    ld0 lda !zr inc :for_c !bcc pop
    :str_literal_pre :puts !call # not escaped
    :str_literal_post :puts !call
  :loop !jmp

  !getline.def
  !puts.def

  str_literal_post: @22 @0D @0A @0A @00 # "\"\r\n\n"
  str_newline_label: @0D @0A @73 @74 @72 @3A @20 @00 # "\r\nstr:"
  str_literal_pre: @23 @20 @22 # "# \"" + line_buffer
  line_buffer: x20 !pad
