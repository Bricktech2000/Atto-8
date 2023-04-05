# clear; cargo run --bin asm src/test/print\ character.asm && cargo run --bin emu src/test/this\ is\ a\ test.bin

@core.asm
@common.asm

main%
%front_buffer %init_alloc
# x00 loop:
# x00 for_a:
# ld0 ld0 ld3 add x0F and :print_char %call pop pop
# # :clear_display %call
# inc ld0 x10 xor pop :for_a :for_a_end iff sti
# for_a_end: pop
# inc :loop sti

# x00 loop:
# x00 ld1 :print_byte %call pop pop
# inc :loop sti

x00 x00 :print_char %call pop pop
hlt


print_char: # print_char(char, position)
x04 for_i: dec
# dst = i * 2 + (position // 2 % 2) + position // 4 * 8 + &FRONT_BUFFER
ld0 x01 shf ld4 x01 %neg shf x01 and add ld4 xFC and x01 shf add %front_buffer add
# curr = *dst & (position % 2 ? 0x0F : 0xF0)
ld0 lda x0F ld6 x01 and x02 shf rot and
# src = char * 2 + (i // 2) + &display_data
ld4 x01 shf ld3 x01 %neg shf add :display_data add lda
# swap nibbles if i % 2 == 0
ld3 not x01 and x02 shf rot
# only keep lower nibble
x0F and
# swap nibbles if position % 2 == 0
ld6 not x01 and x02 shf rot
# *dst = curr | ...
orr sta
buf :for_i :for_i_end iff sti
for_i_end: pop
%ret


print_byte: # print_byte(byte, position)
ld2 ld2 xF0 and x04 %neg shf :print_char %call pop pop
ld2 inc ld2 x0F and :print_char %call pop pop
%ret


display_data:
dEA dE0 # 0
dC4 dE0 # 1
dC4 d60 # 2
dE6 dE0 # 3
dAE d20 # 4
d64 dC0 # 5
d8E dE0 # 6
dE2 d20 # 7
d6E dE0 # 8
dEE d20 # 9
d4E dA0 # A
dCE dE0 # B
dE8 dE0 # C
dCA dC0 # D
dEC dE0 # E
dEC d80 # F


unused%
clear_display: # clear_display()
x20 for_j: dec
ld0 %front_buffer add x00 sta
buf :for_j :for_j_end iff sti
for_j_end: pop
%ret
