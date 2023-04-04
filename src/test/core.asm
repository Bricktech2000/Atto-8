neg% not inc # might set carry
norr% orr not
nand% and not
xnorr% xor not
xnand% xnd not
abs% ld0 %neg ld1 x01 rot pop iff
call% ldi x04 add swp sti
ret% sti
