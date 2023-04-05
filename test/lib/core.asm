neg% not inc # might set carry
shr% x0F xor inc shf
norr% orr not
nand% and not
xnorr% xor not
xnand% xnd not
abs% ld0 x01 rot x01 and %neg swp ld1 add xor
call% ldi x04 add swp sti
ret% sti
dbg% xCC dbg pop
