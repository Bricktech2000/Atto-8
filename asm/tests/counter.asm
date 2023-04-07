# clear; cargo run --bin asm asm/tests/counter.asm emu/tests/counter.bin && cargo run --bin emu emu/tests/counter.bin

@../../lib/microcomputer/utils.asm
@../../lib/utils/core.asm

main%
%front_buffer %alloc_buffer

x00 x00 loop:
x00 ld2 :print_byte %call
x02 ld1 :print_byte %call
x00 x01 adc2 adc2 :loop sti

%hlt

@../utils/text.asm
@../../lib/print_byte.asm
