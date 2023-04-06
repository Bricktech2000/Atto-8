# clear; cargo run --bin asm asm/tests/optimization.asm emu/tests/optimization.bin && cargo run --bin emu emu/tests/optimization.bin

@../../lib/utils/core.asm

main% %dbg

x4F x06 add

x50 x05 orr

xF5 x5F and

xAB neg

ld0 %abs

xAB %abs

xAA x01 rot

x54 :function %call

%hlt

function:
swp inc swp %rt0
