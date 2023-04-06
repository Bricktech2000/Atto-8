# clear; cargo run --bin asm asm/tests/counter.asm && mv asm/tests/*.bin emu/tests/ && cargo run --bin emu emu/tests/counter.bin

@../lib/common.asm
@../lib/core_utils.asm

main%
%front_buffer %init_alloc

x00 x00 loop:
x00 ld2 :print_byte %call
x02 ld1 :print_byte %call
x00 x01 ac2 ac2 :loop sti

%hlt

@../lib/print_byte.asm
