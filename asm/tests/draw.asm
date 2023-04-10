@../../lib/utils/core.asm
@../../lib/utils/pixel.asm
@../../lib/microcomputer/utils.asm

main!
!front_buffer !alloc_buffer
!reset_input

x70 # x_pos
x70 # y_pos
x00 # x_vel
x00 # y_vel

loop:
# x_pos, y_pos += x_vel, y_vel
ld3 ld2 add st3
ld2 ld1 add st2
# draw pixel at (x_pos, y_pos)
x01 !front_buffer ld5 x04 !shr ld5 xF0 and orr :store_bit !call
# input = *INPUT_BUFFER
!input_buffer lda x0F and :ignore !bcs
# reset x_vel and y_vel to 0x00
x00 x00 st2 st2
# velocity = (input & 0b1010) ? 0xFF : 0x01
ld0 x0A and pop x01 xFF iff
# address = (input & 0b0011) ? 0x04 : 0x05
ld1 x03 and pop x04 x05 iff lds add
# store velocity
swp sta
# reset input
!reset_input
# pop input
ignore: pop
:loop sti

!bit_addr
!store_bit
