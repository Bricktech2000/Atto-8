# to count neighbours, front buffer is read from and back buffer is written to.
# back buffer is copied to front buffer at the end of each iteration.
#
# rules used:
#
# ```rust
# let next_state = match neighbour_count {
#   3 => State::Alive,
#   4 => current_state,
#   _ => State::Dead,
# }
# ```

@../../lib/utils/core.asm
@../../lib/utils/pixel.asm
@../../lib/microcomputer/utils.asm
@../../lib/microcomputer/back_to_front.asm

main%
%back_buffer %alloc_buffer

# diehard (advanced 2 generations)
# %back_buffer x09 add xC2 sta
# %back_buffer x0B add xC7 sta
# blinker
# %back_buffer x0C add x07 sta
# glider
%back_buffer x08 add x07 sta
%back_buffer x0A add x01 sta
%back_buffer x0C add x02 sta
# r-pentomino
# %back_buffer x0B add x06 sta
# %back_buffer x0D add x0C sta
# %back_buffer x0F add x04 sta

loop:
# copy back buffer to front buffer
:back_to_front %call

# loop through every cell
x00 for_xy: dec

x00 # allocate neighbour count

# count neighbours
x02 for_dx: dec
x20 for_dy: x10 sub
# neighbour = (for_xy + for_dx & 0x0F) | (for_xy + for_dy & 0xF0)
%front_buffer ld4 ld3 add x0F and ld5 ld3 add xF0 and orr :load_bit %call
# neighbour_count += neighbour
ld3 add st2
ld0 xF0 xor pop :for_dy %bcc pop
ld0 xFF xor pop :for_dx %bcc pop

# apply rules outlined above
ld0 x04 xor pop :ignore %bcs
ld0 x03 xor pop x00 x01 iff %back_buffer ld3 :store_bit %call
ignore: pop # pop neighbour count

buf :for_xy %bcc pop

:loop sti

%back_to_front
%bit_addr
%load_bit
%store_bit
