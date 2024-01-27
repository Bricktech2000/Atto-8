@ lib/core.asm
@ lib/types.asm
@ lib/stdio.asm

main! !nop
  loop:
    :str_prompt :puts !call
    :line_buffer :line_buffer :getline !call
    :str_greeting :puts !call
  :loop !jmp

  !getline.def
  !puts.def

  str_prompt: @0D @0A @45 @6E @74 @65 @72 @20 @79 @6F @75 @72 @20 @6E @61 @6D @65 @3A @20 @00 # "\r\nEnter your name: "
  str_greeting: @0D @0A @47 @72 @65 @65 @74 @69 @6E @67 @73 @2C @20 # "\r\nGreetings, " + name
  line_buffer: x20 !pad
