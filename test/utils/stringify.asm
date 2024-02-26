@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm

# converts a string from `getline()` to its Assembly representation

main! !nop
  loop:
    :line_buffer :line_buffer :getline !call
    :str_newline_label :puts !call
    :line_buffer !char.commercial_at !hex_puts
    :str_literal_pre :puts !call # not escaped
    :str_literal_post :puts !call
  :loop !jmp

  !getline.def
  !puts.def

  str_literal_post: @22 @0A @0A @00 # "\"\n\n"
  str_newline_label: @0A @73 @74 @72 @3A @00 # "\nstr:"
  str_literal_pre: @20 @23 @20 @22 # " # \"" + line_buffer
  line_buffer: x20 !pad
