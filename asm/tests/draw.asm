# clear; cargo run --bin asm asm/tests/draw.asm emu/tests/draw.bin && cargo run --bin emu emu/tests/draw.bin

@../../lib/microcomputer/utils.asm
@../../lib/utils/core.asm

main%
%front_buffer %alloc_buffer
%reset_input

x70 # x_pos
x70 # y_pos
x00 # x_vel
x00 # y_vel

loop:
# draw pixel at (x_pos, y_pos)
ld3 ld2 add st3
ld2 ld1 add st2
x01 %front_buffer ld5 x04 %shr ld5 xF0 and orr :store_bit %call
# input = *INPUT_BUFFER
%input_buffer lda x0F and :process :ignore iff sti
process:
# reset x_vel and y_vel to 0x00
x00 x00 st2 st2
# velocity = (input & 0b0110) ? 0xFF : 0x01
ld0 x06 and pop x01 xFF iff
# address = (input & 0b0101) ? 0x04 : 0x05
ld1 x05 and pop x04 x05 iff lds add
# store velocity
swp sta
# reset input
%reset_input
# pop input
ignore: pop
:loop sti

@../../lib/utils/pixel.asm
