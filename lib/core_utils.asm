neg% not inc # might set carry
shr% x0F xor inc shf
norr% orr not
nand% and not
xnorr% xor not
xnand% xnd not
abs% ld0 x01 rot x01 and %neg swp ld1 add xor
call% .ret swp sti ret.
ret% sti
dbg% xCC dbg pop
