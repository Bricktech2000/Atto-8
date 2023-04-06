neg% not inc # might set carry
shr% x0F xor inc shf
norr% orr not
nand% and not
xnorr% xor not
xnand% xnd not
abs% ld0 x01 rot x01 and %neg swp ld1 add xor

call% .ret swp sti ret.
ret% sti
rt0% %ret
rt1% st0 %ret
rt2% st1 pop %ret
rt3% st2 pop pop %ret
rt4% st3 pop pop pop %ret

hlt% lbl. .lbl sti
dbg% dAA
