rneg! x07 not @const orr neg # prepares for rotate right
norr! orr not
nand! and not
xnorr! xor not
xnand! xnd not
abs! ld0 x01 rot x01 and neg swp ld1 add xor @const
abs.dyn! ld0 neg ld1 !ng iff
min.dyn! ld1 ld1 !lt iff
max.dyn! ld1 ld1 !gt iff
asr.dyn! ld0 !ng shr @dyn

z! buf @dyn               # check zero
o! not @dyn               # check ones
e! xor @dyn               # check equal
gt! sub @dyn pop          # greater than
ngt! sub @dyn pop flc     # non-greater than
lt! swp sub @dyn pop      # less than
nlt! swp sub @dyn pop flc # non-less than
zr! buf @dyn pop          # zero
nzr! buf @dyn pop flc     # non-zero
on! not @dyn pop          # ones
non! not @dyn pop flc     # non-ones
eq! xor @dyn pop          # equal to
neq! xor @dyn pop flc     # non-equal to
ng! shl @dyn pop          # negative
nng! shl @dyn pop flc     # non-negative
ps! neg shl @dyn pop      # positive
nps! neg shl @dyn pop flc # non-positive
cl! and @dyn pop          # cleared in
ncl! and @dyn pop flc     # non-cleared in

jmp! sti
bcc! @const .skip iff !jmp skip.
bcs! @const .skip swp iff !jmp skip.
call! @const .ret swp !jmp ret.
ret! !jmp
rt0! !ret
rt1! st0 !ret
rt2! st1 pop !ret
rt3! st2 pop pop !ret

dbg! @BB # unofficial opcode `0xBB` is treated as a debug request
here! here. .here
nop! nop @dyn
hlt! !here !jmp
pad! .here add here. @org
stall! shl shl shl @const rot @dyn # argument at most 0x1F
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
