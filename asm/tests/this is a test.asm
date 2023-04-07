# clear; cargo run --bin asm asm/tests/this\ is\ a\ test.asm emu/tests/this\ is\ a\ test.bin && cargo run --bin emu emu/tests/this\ is\ a\ test.bin

@../../lib/microcomputer/utils.asm
@../../lib/utils/core.asm

main%
%front_buffer %alloc_buffer
%display_data_len for_i: dec
ld0 %front_buffer add
ld1 :display_data add
lda sta
buf :for_i :for_i_end iff sti
for_i_end: pop
%hlt

display_data:
dEA dE6 d4E d44 d4A dEC d00 d00
dE6 d0E d44 d0E dEC d0A d00 d00
dEE d6E d4C d44 d4E dC4 d00 d00
display_data_end:

display_data_len% :display_data_end :display_data sub
