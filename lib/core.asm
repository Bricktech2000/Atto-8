rneg! x07 not @const orr neg # prepares for rotate right
norr! orr not
nand! and not
xnorr! xor not
xnand! xnd not
abs! ld0 x01 rot x01 and neg swp ld1 add xor @const
abs.dyn! ld0 neg ld1 !ng iff
min.dyn! ld1 ld1 !le iff
max.dyn! ld1 ld1 !gt iff
asr.dyn! ld0 !ng shr @dyn

z! buf @dyn               # check zero
e! xor @dyn               # check equal
gt! sub @dyn pop          # greater than
ngt! sub @dyn pop flc     # non-greater than
lt! swp sub @dyn pop      # less than
nlt! swp sub @dyn pop flc # non-less than
zr! buf @dyn pop          # zero
nzr! buf @dyn pop flc     # non-zero
eq! xor @dyn pop          # equal to
neq! xor @dyn pop flc     # non-equal to
ng! shl @dyn pop          # negative
nng! shl @dyn pop flc     # non-negative
ps! neg shl @dyn pop      # positive
nps! neg shl @dyn pop flc # non-positive
cl! and @dyn pop          # cleared in
ncl! and @dyn pop flc     # non-cleared in

jmp! sti
bcc! @const !bcc.dyn
bcc.dyn! .skip iff !jmp skip.
bcs! @const !bcs.dyn
bcs.dyn! .skip swp iff !jmp skip.
call! @const !call.dyn
call.dyn! .ret swp !jmp ret.
ret! !jmp
rt0! !ret
rt1! st0 !ret
rt2! st1 pop !ret
rt3! st2 pop pop !ret
rt4! st3 pop pop pop !ret

dbg! @BB # unofficial `0xBB` treated as debug request
here! lbl. .lbl
nop! nop @dyn
hlt! !here !jmp
pad! .lbl add lbl. @org
stall! @const !stall.dyn # argument at most 0x1F
stall.dyn! shl shl shl rot @dyn # argument at most 0x1F
ofst! neg @const sub # add large constant by subtraction

mul! clc # product = mul(a, b)
  x00 inc @const loop.
    ld1 add
    .loop x01 su4 @dyn
    .break iff !jmp
  break. st1 sub

div! clc # quotient = div(a, b)
  x00 dec @const loop.
    x01 add
    .loop ld2 su4 @dyn
    .break iff !jmp
  break. st1 pop

mod! clc # remainder = mod(a, b)
  loop.
    ld0 su2 @dyn
  .loop !bcc clc add

divmod! clc # (quotient, remainder) = divmod(a, b)
  x00 dec @const loop.
    x01 add
    .loop ld2 su4 @dyn
    .break iff !jmp
  break. swp clc ad2
