shr! neg shf
ror! neg rot
norr! orr not
nand! and not
xnorr! xor not
xnand! xnd not
abs! ld0 x01 rot x01 and neg swp ld1 add xor @const

bcc! @const .skip iff sti skip.
bcs! .skip swp @const iff sti skip.
call! .ret swp @const sti ret.
ret! sti
rt0! !ret
rt1! st0 !ret
rt2! st1 pop !ret
rt3! st2 pop pop !ret
rt4! st3 pop pop pop !ret

dbg! dBB # emulator treats unofficial `BB` as debug request
here! lbl. .lbl
hlt! !here sti
