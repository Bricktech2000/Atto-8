# clear; cargo run --bin asm test/count.asm && cargo run --bin emu test/count.bin

@lib/core.asm
@lib/common.asm
@lib/print_char.asm

main%
%front_buffer %init_alloc

x00 x00 loop:
x03 :characters ld2 x0F and :print_char %call
x02 :characters ld2 x04 %shr :print_char %call
x01 :characters ld3 x0F and :print_char %call
x00 :characters ld3 x04 %shr :print_char %call
# xFF delay:
# dec buf :delay :delay_end iff sti
# delay_end: pop
x00 x01 ac2 ac2 :loop sti

hlt

%print_char

characters:
dEA dEC d4E # 0 1
dC4 d6E d6E # 2 3
dAE d26 d4C # 4 5
d8E dEE d22 # 6 7
d6E dEE dE2 # 8 9
d4E dAC dEE # A B
dE8 dEC dAC # C D
dEC dEE dC8 # E F
