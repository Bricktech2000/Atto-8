# clear; cargo run --bin asm tests/game\ of\ life.asm && cargo run --bin emu tests/game\ of\ life.bin

main%
# init: x00 x00 lds xBD xor pop :init :main iif sti
xC0 sts x00 str @00
            xFF stw xFF str @00
%display_buffer stw xC0 str @00

main:
x00 for_y:
x00 for_x:
x00

xFF for_dy:
xFF for_dx:

x00 x00 @05 dup dup x00 dup dup @02 add clc add clc pop @00
:get_encoded %call pop pop
%display_buffer add stw dup @00 and x00 xnd rol @02 add @00

inc dup x01 xor pop :for_dx :for_dx_end iif sti
for_dx_end: pop
inc dup x01 xor pop :for_dy :for_dy_end iif sti
for_dy_end: pop

# x00 x00 @05 dup dup @00
# :get_encoded %call pop pop
# %display_buffer add stw dup @00 :set :else iif sti else: :clear :no_change iif sti
# set: clear: no_change:

# 3 => 1
# 4 => ;
# _ => 0

# potential improvements:
# - create "carryful" and "carryless" instruction variants (adc, sbc, shl, shr...)
# - add new useful instructions (sta, lda, sto, ldo...)
# - merge rol with ror and shl with shr? (replace with `xXX rot` and `xXX shf`)

pop
inc dup x10 xor pop :for_x :for_x_end iif sti
for_x_end: pop
inc dup x10 xor pop :for_y :for_y_end iif sti
for_y_end: pop
hlt

get_encoded: # (address, bit) = get_encoded(x, y)
# x = x % 16 = x & 0xFF
# y = y % 16 = y & 0xFF
# dummy `xFF` and `pop` are to prevent overriting return address
@02 dup dup xFF x0F x0F and and pop str str
# bit = 0x01 << (0x07 - (x & 0x07))
x01 @02 dup @00
x07 and clc :shifts add sti
shifts: rol rol rol rol rol rol rol
@04 str @00
# address = (x >> 3) | (y << 1) = ((y << 4) | x) >> 3
@02 dup dup @00
swp rol rol rol rol oor clc ror clc ror clc ror clc
@03 str @00
%ret


back_buffer% xC0
display_buffer% xE0
call% ldi swp sti
ret% clc x02 add sti
