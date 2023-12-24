@ lib/core.asm
@ lib/types.asm
@ lib/string.asm
@ lib/stdlib.asm

main!
  x63 :str_abcdef :strchr !call :strlen !call # 0x04
  xCC :str_abcdef :strchr !call # 0x00
  # :str_abc :strlen !call # 0x03
  # :str_abcdef :strlen !call # 0x06
  :str_abc :str_buf :strcpy !call :str_buf :strlen !call # 0x03
  # :str_ac :str_abcdef :strcat !call :str_abcdef :strlen !call # 0x08
  :str_ac :str_ab :strcmp !call # 0x01
  :str_ab :str_abc :strcmp !call # 0x9D
  :str_abc :str_abc :strcmp !call # 0x00
  x06 x63 :str_abcdef :memchr !call :strlen !call # 0x04
  # x06 xCC :str_abcdef :memchr !call # 0x00
  x04 :str_abc :str_buf :memcmp !call # 0x00
  x03 :str_ac :str_ab :memcmp !call # 0x01
  !hlt

  str_ab: @61 @62 @00
  str_ac: @61 @63 @00
  str_abc: @61 @62 @63 @00
  str_abcdef: @61 @62 @63 @64 @65 @66 @00
  str_buf: @CC @CC @CC @CC

  !strchr.def
  !strlen.def
  !strcpy.def
  # !strcat.def
  !strcmp.def
  !memchr.def
  # !memset.def
  # !memcpy.def
  !memcmp.def
