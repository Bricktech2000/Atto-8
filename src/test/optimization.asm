# clear; cargo run --bin asm src/test/optimization.asm && cargo run --bin emu src/test/optimization.bin

@core.asm

main% dbg

x4F x06 add

x50 x05 orr

xF5 x5F and

xAB %neg

ld0 %abs

xAB %abs

xaa x01 rot

x54 :function %call

hlt

function:
swp inc swp %ret
