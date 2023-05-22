ror! neg rot
norr! orr not
nand! and not
xnorr! xor not
xnand! xnd not
abs! ld0 neg ld1 shl @dyn pop iff
cabs! ld0 x01 rot x01 and neg swp ld1 add xor @const
min! ld1 ld1 swp sub @dyn pop iff
max! ld1 ld1 sub @dyn pop iff

jmp! sti
bcc! @const .skip iff !jmp skip.
bcs! .skip swp @const iff !jmp skip.
call! .ret swp @const !jmp ret.
ret! !jmp
rt0! !ret
rt1! st0 !ret
rt2! st1 pop !ret
rt3! st2 pop pop !ret
rt4! st3 pop pop pop !ret

dbg! dBB # emulator treats unofficial `BB` as debug request
nop! nop @dyn
here! lbl. .lbl
hlt! !here !jmp
