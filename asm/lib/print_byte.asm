# @common.asm
# @core_utils.asm

@text_utils.asm
hex_chars: @hex_chars.asm
print_byte: # print_byte(byte, pos)
# LSB
ld2 inc :hex_chars ld3 x0F and :print_char %call
# MSB
ld2 :hex_chars ld3 x04 %shr :print_char %call
# return*
%rt2
