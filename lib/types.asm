u8! # u8 n = u8(u8 n)
u16! # u16 n = u16(u16 n)
u32! # u32 n = u16(u32 n)
u64! # u64 n = u16(u64 n)
i8! !u8 # i8 n = i8(i8 n)
i16! !u16 # i16 n = i16(i16 n)
i32! !u32 # i32 n = i16(i32 n)
i64! !u64 # i64 n = i16(u64 n)
u4u4! !u8 # u4u4 n = u4u4(u4u4 n)
u8u8! !u16 # u8u8 n = u8u8(u8u8 n)
i4i4! !i8 # i4i4 n = i4i4(i4i4 n)
i8i8! !i16 # i8i8 n = i8i8(i8i8 n)
u4f4! !u8 # u4f4 n = u4f4(u4 fr, u4 in)
u8f8! !u16 # u8f8 n = u8f8(u8 fr, u8 in)
i4f4! !i8 # i4f4 n = i4f4(u4 fr, i4 in)
i8f8! !i16 # i8f8 n = i8f8(u8 fr, i8 in)
c4f4m4f4! !i16 # c4f4m4f4 n = c4f4m4f4(i4f4 im, i4f8 re)
c8f8m8f8! !i32 # c8f8m8f8 n = c8f8m8f8(i8f8 im, i8f8 re)
char! # char n = char(u8 n)

0u8! x00 !u8 @const
0u16! x00 x00 !u16 @const
0u32! x00 x00 x00 x00 !u32 @const
0u64! x00 x00 x00 x00 x00 x00 x00 x00 !u64 @const
0i8! !0u8 !i8
0i16! !0u16 !i16
0i32! !0u32 !i32
0i64! !0u64 !i64
0u4u4! !0u8 !u4u4
0u8u8! !0u16 !u8u8
0i4i4! !0i8 !i4i4
0i8i8! !0i16 !i8i8
0u4f4! !0u8 !u4f4
0u8f8! !0u16 !u8f8
0i4f4! !0i8 !i4f4
0i8f8! !0i16 !i8f8
0c4f4m4f4! !0i16 !c4f4m4f4
0c8f8m8f8! !0i32 !c8f8m8f8
nul! x00 !char @const '\0'! !nul
soh! x01 !char @const
stx! x02 !char @const
etx! x03 !char @const
eot! x04 !char @const
enq! x05 !char @const
ack! x06 !char @const
bel! x07 !char @const '\a'! !bel
bs! x08 !char @const '\b'! !bs
ht! x09 !char @const '\t'! !ht
lf! x0A !char @const '\n'! !lf
vt! x0B !char @const '\v'! !vt
ff! x0C !char @const '\f'! !ff
cr! x0D !char @const '\r'! !cr
so! x0E !char @const
si! x0F !char @const
dle! x10 !char @const
dc1! x11 !char @const
dc2! x12 !char @const
dc3! x13 !char @const
dc4! x14 !char @const
nak! x15 !char @const
syn! x16 !char @const
etb! x17 !char @const
can! x18 !char @const
em! x19 !char @const
sub! x1A !char @const
esc! x1B !char @const '\e'! !esc
fs! x1C !char @const
gs! x1D !char @const
rs! x1E !char @const
us! x1F !char @const
sp! x20 !char @const '\s'! !sp
'!'! x21 !char @const
'"'! x22 !char @const
'#'! x23 !char @const
'$'! x24 !char @const
'%'! x25 !char @const
'&'! x26 !char @const
'\''! x27 !char @const
'('! x28 !char @const
')'! x29 !char @const
'*'! x2A !char @const
'+'! x2B !char @const
','! x2C !char @const
'-'! x2D !char @const
'.'! x2E !char @const
'/'! x2F !char @const
'0'! x30 !char @const
'1'! x31 !char @const
'2'! x32 !char @const
'3'! x33 !char @const
'4'! x34 !char @const
'5'! x35 !char @const
'6'! x36 !char @const
'7'! x37 !char @const
'8'! x38 !char @const
'9'! x39 !char @const
':'! x3A !char @const
';'! x3B !char @const
'<'! x3C !char @const
'='! x3D !char @const
'>'! x3E !char @const
'?'! x3F !char @const
'@'! x40 !char @const
'A'! x41 !char @const
'B'! x42 !char @const
'C'! x43 !char @const
'D'! x44 !char @const
'E'! x45 !char @const
'F'! x46 !char @const
'G'! x47 !char @const
'H'! x48 !char @const
'I'! x49 !char @const
'J'! x4A !char @const
'K'! x4B !char @const
'L'! x4C !char @const
'M'! x4D !char @const
'N'! x4E !char @const
'O'! x4F !char @const
'P'! x50 !char @const
'Q'! x51 !char @const
'R'! x52 !char @const
'S'! x53 !char @const
'T'! x54 !char @const
'U'! x55 !char @const
'V'! x56 !char @const
'W'! x57 !char @const
'X'! x58 !char @const
'Y'! x59 !char @const
'Z'! x5A !char @const
'['! x5B !char @const
'\\'! x5C !char @const
']'! x5D !char @const
'^'! x5E !char @const
'_'! x5F !char @const
'`'! x60 !char @const
'a'! x61 !char @const
'b'! x62 !char @const
'c'! x63 !char @const
'd'! x64 !char @const
'e'! x65 !char @const
'f'! x66 !char @const
'g'! x67 !char @const
'h'! x68 !char @const
'i'! x69 !char @const
'j'! x6A !char @const
'k'! x6B !char @const
'l'! x6C !char @const
'm'! x6D !char @const
'n'! x6E !char @const
'o'! x6F !char @const
'p'! x70 !char @const
'q'! x71 !char @const
'r'! x72 !char @const
's'! x73 !char @const
't'! x74 !char @const
'u'! x75 !char @const
'v'! x76 !char @const
'w'! x77 !char @const
'x'! x78 !char @const
'y'! x79 !char @const
'z'! x7A !char @const
'{'! x7B !char @const
'|'! x7C !char @const
'}'! x7D !char @const
'~'! x7E !char @const
del! x7F !char @const
char.digit_count! !'9' !'0' sub inc @const
char.small_letter_count! !'z' !'a' sub inc @const
char.capital_letter_count! !'Z' !'A' sub inc @const

u8.add! add @dyn
u16.add! ad2 @dyn ad2 @dyn
u32.add! ad4 @dyn ad4 @dyn ad4 @dyn ad4 @dyn
u64.add! ad8 @dyn ad8 @dyn ad8 @dyn ad8 @dyn ad8 @dyn ad8 @dyn ad8 @dyn ad8 @dyn
i8.add! !u8.add
i16.add! !u16.add
i32.add! !u32.add
i64.add! !u64.add
u4u4.add! ld1 ld1 ld1 clc add x0F and st2 xF0 and clc add xF0 and orr
u8u8.add! ad2 clc ad2
i4i4.add! !u4u4.add
i8i8.add! !u8u8.add
u4f4.add! !u8.add
u8f8.add! !u16.add
i4f4.add! !i8.add
i8f8.add! !i16.add
c4f4m4f4.add! ad4 @dyn ad4 @dyn clc ad4 @dyn ad4 @dyn
c8f8m8f8.add! ad4 @dyn ad4 @dyn clc ad4 @dyn ad4 @dyn
char.add! !u8.add

u8.sub! sub @dyn
u16.sub! su2 @dyn su2 @dyn
u32.sub! su4 @dyn su4 @dyn su4 @dyn su4 @dyn
u64.sub! su8 @dyn su8 @dyn su8 @dyn su8 @dyn su8 @dyn su8 @dyn su8 @dyn su8 @dyn
i8.sub! !u8.sub
i16.sub! !u16.sub
i32.sub! !u32.sub
i64.sub! !u64.sub
u4u4.sub! ld1 ld1 ld1 clc sub x0F and st2 xF0 and clc sub xF0 and orr
u8u8.sub! su2 clc su2
i4i4.sub! !u4u4.sub
i8i8.sub! !u8u8.sub
u4f4.sub! !u8.sub
u8f8.sub! !u16.sub
i4f4.sub! !i8.sub
i8f8.sub! !i16.sub
c4f4m4f4.sub! su4 @dyn su4 @dyn clc su4 @dyn su4 @dyn
c8f8m8f8.sub! su4 @dyn su4 @dyn clc su4 @dyn su4 @dyn
char.sub! !u8.sub

u8.iff! iff
u16.iff! if2 if2
u32.iff! if4 if4 if4 if4
u64.iff! if8 if8 if8 if8 if8 if8 if8 if8
i8.iff! !u8.iff
i16.iff! !u16.iff
i32.iff! !u32.iff
i64.iff! !u64.iff
u4u4.iff! !u8.iff
u8u8.iff! !u16.iff
i4i4.iff! !i8.iff
i8i8.iff! !i16.iff
u4f4.iff! !u8.iff
u8f8.iff! !u16.iff
i4f4.iff! !i8.iff
i8f8.iff! !i16.iff
c4f4m4f4.iff! !i16.iff
c8f8m8f8.iff! !i32.iff
char.iff! !u8.iff

u8.pop! pop
u16.pop! pop pop
u32.pop! pop pop pop pop
u64.pop! pop pop pop pop pop pop pop pop
i8.pop! !u8.pop
i16.pop! !u16.pop
i32.pop! !u32.pop
i64.pop! !u64.pop
u4u4.pop! !u8.pop
u8u8.pop! !u16.pop
i4i4.pop! !i8.pop
i8i8.pop! !i16.pop
u4f4.pop! !u8.pop
u8f8.pop! !u16.pop
i4f4.pop! !i8.pop
i8f8.pop! !i16.pop
c4f4m4f4.pop! !i16.pop
c8f8m8f8.pop! !i32.pop
char.pop! !u8.pop

u8.shl! shl
u16.shl! shl ld1 shl st1
u32.shl! shl ld1 shl st1 ld2 shl st2 ld3 shl st3 ld4 shl st4
u64.shl! shl ld1 shl st1 ld2 shl st2 ld3 shl st3 ld4 shl st4 ld5 shl st5 ld6 shl st6 ld7 shl st7
u4u4.shl! !u8.shl xEF and
u8u8.shl! !u16.shl xFE an2
u4f4.shl! !u8.shl
u8f8.shl! !u16.shl

u8.shr! shr
u16.shr! ld1 shr st1 shr
u32.shr! ld3 shr st3 ld2 shr st2 ld1 shr st1 shr
u64.shr! ld7 shr st7 ld6 shr st6 ld5 shr st5 ld4 shr st4 ld3 shr st3 ld2 shr st2 ld1 shr st1 shr
u4u4.shr! !u8.shr xF7 and
u8u8.shr! !u16.shr x7F and
u4f4.shr! !u8.shr
u8f8.shr! !u16.shr

u8.neg! neg
u16.neg! neg ld1 neg st1
u32.neg! neg ld1 neg st1 ld2 neg st2 ld3 neg st3 ld4 neg st4
u64.neg! neg ld1 neg st1 ld2 neg st2 ld3 neg st3 ld4 neg st4 ld5 neg st5 ld6 neg st6 ld7 neg st7
i8.neg! !u8.neg
i16.neg! !u16.neg
i32.neg! !u32.neg
i64.neg! !u64.neg
u4u4.neg! !u8.neg
u8u8.neg! !u16.neg
i4i4.neg! !i8.neg
i8i8.neg! !i16.neg
u4f4.neg! !u8.neg
u8f8.neg! !u16.neg
i4f4.neg! !i8.neg
i8f8.neg! !i16.neg

u8.lda! lda
u16.lda! ld0 lda swp inc lda
u32.lda! ld0 lda swp inc ld0 lda swp inc ld0 lda swp inc lda
u64.lda! ld0 lda swp inc ld0 lda swp inc ld0 lda swp inc ld0 lda swp inc ld0 lda swp inc ld0 lda swp inc ld0 lda swp inc lda
i8.lda! !u8.lda
i16.lda! !u16.lda
i32.lda! !u32.lda
i64.lda! !u64.lda
u4u4.lda! !u8.lda
u8u8.lda! !u16.lda
i4i4.lda! !i8.lda
i8i8.lda! !i16.lda
u4f4.lda! !u8.lda
u8f8.lda! !u16.lda
i4f4.lda! !i8.lda
i8f8.lda! !i16.lda
char.lda! !u8.lda

u8.sta! sta
u16.sta! swp ld1 sta inc sta
u32.sta! swp ld1 sta inc swp ld1 sta inc swp ld1 sta inc sta
u64.sta! swp ld1 sta inc swp ld1 sta inc swp ld1 sta inc swp ld1 sta inc swp ld1 sta inc swp ld1 sta inc swp ld1 sta inc sta
i8.sta! !u8.sta
i16.sta! !u16.sta
i32.sta! !u32.sta
i64.sta! !u64.sta
u4u4.sta! !u8.sta
u8u8.sta! !u16.sta
i4i4.sta! !i8.sta
i8i8.sta! !i16.sta
u4f4.sta! !u8.sta
u8f8.sta! !u16.sta
i4f4.sta! !i8.sta
i8f8.sta! !i16.sta
char.sta! !u8.sta

u8.ld0!   ld0
u8.ld0+1! ld1
u8.ld0+2! ld2
u8.ld0+3! ld3
u8.ld1!   ld1
u8.ld1+1! ld2
u8.ld1+2! ld3
u8.ld1+3! ld4
u8.ld2!   ld2
u8.ld2+1! ld3
u8.ld2+2! ld4
u8.ld2+3! ld5
u8.ld3!   ld3
u8.ld3+1! ld4
u8.ld3+2! ld5
u8.ld3+3! ld6
u8.ld4!   ld4
u8.ld4+1! ld5
u8.ld4+2! ld6
u8.ld4+3! ld7
u8.ld5!   ld5
u8.ld5+1! ld6
u8.ld5+2! ld7
u8.ld5+3! ld8
u8.ld6!   ld6
u8.ld6+1! ld7
u8.ld6+2! ld8
u8.ld6+3! ld9
u8.ld7!   ld7
u8.ld7+1! ld8
u8.ld7+2! ld9
u8.ld7+3! ldA
u8.ld8!   ld8
u8.ld8+1! ld9
u8.ld8+2! ldA
u8.ld8+3! ldB
u8.ld9!   ld9
u8.ld9+1! ldA
u8.ld9+2! ldB
u8.ld9+3! ldC
u8.ldA!   ldA
u8.ldA+1! ldB
u8.ldA+2! ldC
u8.ldA+3! ldD
u8.ldB!   ldB
u8.ldB+1! ldC
u8.ldB+2! ldD
u8.ldB+3! ldE
u8.ldC!   ldC
u8.ldC+1! ldD
u8.ldC+2! ldE
u8.ldC+3! ldF
u8.ldD!   ldD
u8.ldD+1! ldE
u8.ldD+2! ldF
u8.ldE!   ldE
u8.ldE+1! ldF
u8.ldF!   ldF
u16.ld0!   ld1 ld1
u16.ld0+1! ld2 ld2
u16.ld0+2! ld3 ld3
u16.ld0+3! ld4 ld4
u16.ld1!   ld3 ld3
u16.ld1+1! ld4 ld4
u16.ld1+2! ld5 ld5
u16.ld1+3! ld6 ld6
u16.ld2!   ld5 ld5
u16.ld2+1! ld6 ld6
u16.ld2+2! ld7 ld7
u16.ld2+3! ld8 ld8
u16.ld3!   ld7 ld7
u16.ld3+1! ld8 ld8
u16.ld3+2! ld9 ld9
u16.ld3+3! ldA ldA
u16.ld4!   ld9 ld9
u16.ld4+1! ldA ldA
u16.ld4+2! ldB ldB
u16.ld4+3! ldC ldC
u16.ld5!   ldB ldB
u16.ld5+1! ldC ldC
u16.ld5+2! ldD ldD
u16.ld5+3! ldE ldE
u16.ld6!   ldD ldD
u16.ld6+1! ldE ldE
u16.ld6+2! ldF ldF
u16.ld7!   ldF ldF
u32.ld0!   ld3 ld3 ld3 ld3
u32.ld0+1! ld4 ld4 ld4 ld4
u32.ld0+2! ld5 ld5 ld5 ld5
u32.ld0+3! ld6 ld6 ld6 ld6
u32.ld1!   ld7 ld7 ld7 ld7
u32.ld1+1! ld8 ld8 ld8 ld8
u32.ld1+2! ld9 ld9 ld9 ld9
u32.ld1+3! ldA ldA ldA ldA
u32.ld2!   ldB ldB ldB ldB
u32.ld2+1! ldC ldC ldC ldC
u32.ld2+2! ldD ldD ldD ldD
u32.ld2+3! ldE ldE ldE ldE
u32.ld3!   ldF ldF ldF ldF
u64.ld0!   ld7 ld7 ld7 ld7 ld7 ld7 ld7 ld7
u64.ld0+1! ld8 ld8 ld8 ld8 ld8 ld8 ld8 ld8
u64.ld0+2! ld9 ld9 ld9 ld9 ld9 ld9 ld9 ld9
u64.ld0+3! ldA ldA ldA ldA ldA ldA ldA ldA
u64.ld1!   ldF ldF ldF ldF ldF ldF ldF ldF
i8.ld0!   ld0
i8.ld0+1! ld1
i8.ld0+2! ld2
i8.ld0+3! ld3
i8.ld1!   ld1
i8.ld1+1! ld2
i8.ld1+2! ld3
i8.ld1+3! ld4
i8.ld2!   ld2
i8.ld2+1! ld3
i8.ld2+2! ld4
i8.ld2+3! ld5
i8.ld3!   ld3
i8.ld3+1! ld4
i8.ld3+2! ld5
i8.ld3+3! ld6
i8.ld4!   ld4
i8.ld4+1! ld5
i8.ld4+2! ld6
i8.ld4+3! ld7
i8.ld5!   ld5
i8.ld5+1! ld6
i8.ld5+2! ld7
i8.ld5+3! ld8
i8.ld6!   ld6
i8.ld6+1! ld7
i8.ld6+2! ld8
i8.ld6+3! ld9
i8.ld7!   ld7
i8.ld7+1! ld8
i8.ld7+2! ld9
i8.ld7+3! ldA
i8.ld8!   ld8
i8.ld8+1! ld9
i8.ld8+2! ldA
i8.ld8+3! ldB
i8.ld9!   ld9
i8.ld9+1! ldA
i8.ld9+2! ldB
i8.ld9+3! ldC
i8.ldA!   ldA
i8.ldA+1! ldB
i8.ldA+2! ldC
i8.ldA+3! ldD
i8.ldB!   ldB
i8.ldB+1! ldC
i8.ldB+2! ldD
i8.ldB+3! ldE
i8.ldC!   ldC
i8.ldC+1! ldD
i8.ldC+2! ldE
i8.ldC+3! ldF
i8.ldD!   ldD
i8.ldD+1! ldE
i8.ldD+2! ldF
i8.ldE!   ldE
i8.ldE+1! ldF
i8.ldF!   ldF
i16.ld0!   ld1 ld1
i16.ld0+1! ld2 ld2
i16.ld0+2! ld3 ld3
i16.ld0+3! ld4 ld4
i16.ld1!   ld3 ld3
i16.ld1+1! ld4 ld4
i16.ld1+2! ld5 ld5
i16.ld1+3! ld6 ld6
i16.ld2!   ld5 ld5
i16.ld2+1! ld6 ld6
i16.ld2+2! ld7 ld7
i16.ld2+3! ld8 ld8
i16.ld3!   ld7 ld7
i16.ld3+1! ld8 ld8
i16.ld3+2! ld9 ld9
i16.ld3+3! ldA ldA
i16.ld4!   ld9 ld9
i16.ld4+1! ldA ldA
i16.ld4+2! ldB ldB
i16.ld4+3! ldC ldC
i16.ld5!   ldB ldB
i16.ld5+1! ldC ldC
i16.ld5+2! ldD ldD
i16.ld5+3! ldE ldE
i16.ld6!   ldD ldD
i16.ld6+1! ldE ldE
i16.ld6+2! ldF ldF
i16.ld7!   ldF ldF
i32.ld0!   ld3 ld3 ld3 ld3
i32.ld0+1! ld4 ld4 ld4 ld4
i32.ld0+2! ld5 ld5 ld5 ld5
i32.ld0+3! ld6 ld6 ld6 ld6
i32.ld1!   ld7 ld7 ld7 ld7
i32.ld1+1! ld8 ld8 ld8 ld8
i32.ld1+2! ld9 ld9 ld9 ld9
i32.ld1+3! ldA ldA ldA ldA
i32.ld2!   ldB ldB ldB ldB
i32.ld2+1! ldC ldC ldC ldC
i32.ld2+2! ldD ldD ldD ldD
i32.ld2+3! ldE ldE ldE ldE
i32.ld3!   ldF ldF ldF ldF
i64.ld0!   ld7 ld7 ld7 ld7 ld7 ld7 ld7 ld7
i64.ld0+1! ld8 ld8 ld8 ld8 ld8 ld8 ld8 ld8
i64.ld0+2! ld9 ld9 ld9 ld9 ld9 ld9 ld9 ld9
i64.ld0+3! ldA ldA ldA ldA ldA ldA ldA ldA
i64.ld1!   ldF ldF ldF ldF ldF ldF ldF ldF
u4u4.ld0!   !u8.ld0
u4u4.ld0+1! !u8.ld0+1
u4u4.ld0+2! !u8.ld0+2
u4u4.ld0+3! !u8.ld0+3
u4u4.ld1!   !u8.ld1
u4u4.ld1+1! !u8.ld1+1
u4u4.ld1+2! !u8.ld1+2
u4u4.ld1+3! !u8.ld1+3
u4u4.ld2!   !u8.ld2
u4u4.ld2+1! !u8.ld2+1
u4u4.ld2+2! !u8.ld2+2
u4u4.ld2+3! !u8.ld2+3
u4u4.ld3!   !u8.ld3
u4u4.ld3+1! !u8.ld3+1
u4u4.ld3+2! !u8.ld3+2
u4u4.ld3+3! !u8.ld3+3
u4u4.ld4!   !u8.ld4
u4u4.ld4+1! !u8.ld4+1
u4u4.ld4+2! !u8.ld4+2
u4u4.ld4+3! !u8.ld4+3
u4u4.ld5!   !u8.ld5
u4u4.ld5+1! !u8.ld5+1
u4u4.ld5+2! !u8.ld5+2
u4u4.ld5+3! !u8.ld5+3
u4u4.ld6!   !u8.ld6
u4u4.ld6+1! !u8.ld6+1
u4u4.ld6+2! !u8.ld6+2
u4u4.ld6+3! !u8.ld6+3
u4u4.ld7!   !u8.ld7
u4u4.ld7+1! !u8.ld7+1
u4u4.ld7+2! !u8.ld7+2
u4u4.ld7+3! !u8.ld7+3
u4u4.ld8!   !u8.ld8
u4u4.ld8+1! !u8.ld8+1
u4u4.ld8+2! !u8.ld8+2
u4u4.ld8+3! !u8.ld8+3
u4u4.ld9!   !u8.ld9
u4u4.ld9+1! !u8.ld9+1
u4u4.ld9+2! !u8.ld9+2
u4u4.ld9+3! !u8.ld9+3
u4u4.ldA!   !u8.ldA
u4u4.ldA+1! !u8.ldA+1
u4u4.ldA+2! !u8.ldA+2
u4u4.ldA+3! !u8.ldA+3
u4u4.ldB!   !u8.ldB
u4u4.ldB+1! !u8.ldB+1
u4u4.ldB+2! !u8.ldB+2
u4u4.ldB+3! !u8.ldB+3
u4u4.ldC!   !u8.ldC
u4u4.ldC+1! !u8.ldC+1
u4u4.ldC+2! !u8.ldC+2
u4u4.ldC+3! !u8.ldC+3
u4u4.ldD!   !u8.ldD
u4u4.ldD+1! !u8.ldD+1
u4u4.ldD+2! !u8.ldD+2
u4u4.ldD+3! !u8.ldD+3
u4u4.ldE!   !u8.ldE
u4u4.ldE+1! !u8.ldE+1
u4u4.ldE+2! !u8.ldE+2
u4u4.ldE+3! !u8.ldE+3
u4u4.ldF!   !u8.ldF
u4u4.ldF+1! !u8.ldF+1
u4u4.ldF+2! !u8.ldF+2
u4u4.ldF+3! !u8.ldF+3
u8u8.ld0!   !u16.ld0
u8u8.ld0+1! !u16.ld0+1
u8u8.ld0+2! !u16.ld0+2
u8u8.ld0+3! !u16.ld0+3
u8u8.ld1!   !u16.ld1
u8u8.ld1+1! !u16.ld1+1
u8u8.ld1+2! !u16.ld1+2
u8u8.ld1+3! !u16.ld1+3
u8u8.ld2!   !u16.ld2
u8u8.ld2+1! !u16.ld2+1
u8u8.ld2+2! !u16.ld2+2
u8u8.ld2+3! !u16.ld2+3
u8u8.ld3!   !u16.ld3
u8u8.ld3+1! !u16.ld3+1
u8u8.ld3+2! !u16.ld3+2
u8u8.ld3+3! !u16.ld3+3
u8u8.ld4!   !u16.ld4
u8u8.ld4+1! !u16.ld4+1
u8u8.ld4+2! !u16.ld4+2
u8u8.ld4+3! !u16.ld4+3
u8u8.ld5!   !u16.ld5
u8u8.ld5+1! !u16.ld5+1
u8u8.ld5+2! !u16.ld5+2
u8u8.ld5+3! !u16.ld5+3
u8u8.ld6!   !u16.ld6
u8u8.ld6+1! !u16.ld6+1
u8u8.ld6+2! !u16.ld6+2
u8u8.ld7!   !u16.ld7
i4i4.ld0!   !i8.ld0
i4i4.ld0+1! !i8.ld0+1
i4i4.ld0+2! !i8.ld0+2
i4i4.ld0+3! !i8.ld0+3
i4i4.ld1!   !i8.ld1
i4i4.ld1+1! !i8.ld1+1
i4i4.ld1+2! !i8.ld1+2
i4i4.ld1+3! !i8.ld1+3
i4i4.ld2!   !i8.ld2
i4i4.ld2+1! !i8.ld2+1
i4i4.ld2+2! !i8.ld2+2
i4i4.ld2+3! !i8.ld2+3
i4i4.ld3!   !i8.ld3
i4i4.ld3+1! !i8.ld3+1
i4i4.ld3+2! !i8.ld3+2
i4i4.ld3+3! !i8.ld3+3
i4i4.ld4!   !i8.ld4
i4i4.ld4+1! !i8.ld4+1
i4i4.ld4+2! !i8.ld4+2
i4i4.ld4+3! !i8.ld4+3
i4i4.ld5!   !i8.ld5
i4i4.ld5+1! !i8.ld5+1
i4i4.ld5+2! !i8.ld5+2
i4i4.ld5+3! !i8.ld5+3
i4i4.ld6!   !i8.ld6
i4i4.ld6+1! !i8.ld6+1
i4i4.ld6+2! !i8.ld6+2
i4i4.ld6+3! !i8.ld6+3
i4i4.ld7!   !i8.ld7
i4i4.ld7+1! !i8.ld7+1
i4i4.ld7+2! !i8.ld7+2
i4i4.ld7+3! !i8.ld7+3
i4i4.ld8!   !i8.ld8
i4i4.ld8+1! !i8.ld8+1
i4i4.ld8+2! !i8.ld8+2
i4i4.ld8+3! !i8.ld8+3
i4i4.ld9!   !i8.ld9
i4i4.ld9+1! !i8.ld9+1
i4i4.ld9+2! !i8.ld9+2
i4i4.ld9+3! !i8.ld9+3
i4i4.ldA!   !i8.ldA
i4i4.ldA+1! !i8.ldA+1
i4i4.ldA+2! !i8.ldA+2
i4i4.ldA+3! !i8.ldA+3
i4i4.ldB!   !i8.ldB
i4i4.ldB+1! !i8.ldB+1
i4i4.ldB+2! !i8.ldB+2
i4i4.ldB+3! !i8.ldB+3
i4i4.ldC!   !i8.ldC
i4i4.ldC+1! !i8.ldC+1
i4i4.ldC+2! !i8.ldC+2
i4i4.ldC+3! !i8.ldC+3
i4i4.ldD!   !i8.ldD
i4i4.ldD+1! !i8.ldD+1
i4i4.ldD+2! !i8.ldD+2
i4i4.ldD+3! !i8.ldD+3
i4i4.ldE!   !i8.ldE
i4i4.ldE+1! !i8.ldE+1
i4i4.ldE+2! !i8.ldE+2
i4i4.ldE+3! !i8.ldE+3
i4i4.ldF!   !i8.ldF
i4i4.ldF+1! !i8.ldF+1
i4i4.ldF+2! !i8.ldF+2
i4i4.ldF+3! !i8.ldF+3
i8i8.ld0!   !i16.ld0
i8i8.ld0+1! !i16.ld0+1
i8i8.ld0+2! !i16.ld0+2
i8i8.ld0+3! !i16.ld0+3
i8i8.ld1!   !i16.ld1
i8i8.ld1+1! !i16.ld1+1
i8i8.ld1+2! !i16.ld1+2
i8i8.ld1+3! !i16.ld1+3
i8i8.ld2!   !i16.ld2
i8i8.ld2+1! !i16.ld2+1
i8i8.ld2+2! !i16.ld2+2
i8i8.ld2+3! !i16.ld2+3
i8i8.ld3!   !i16.ld3
i8i8.ld3+1! !i16.ld3+1
i8i8.ld3+2! !i16.ld3+2
i8i8.ld3+3! !i16.ld3+3
i8i8.ld4!   !i16.ld4
i8i8.ld4+1! !i16.ld4+1
i8i8.ld4+2! !i16.ld4+2
i8i8.ld4+3! !i16.ld4+3
i8i8.ld5!   !i16.ld5
i8i8.ld5+1! !i16.ld5+1
i8i8.ld5+2! !i16.ld5+2
i8i8.ld5+3! !i16.ld5+3
i8i8.ld6!   !i16.ld6
i8i8.ld6+1! !i16.ld6+1
i8i8.ld6+2! !i16.ld6+2
i8i8.ld7!   !i16.ld7
u4f4.ld0!   !u8.ld0
u4f4.ld0+1! !u8.ld0+1
u4f4.ld0+2! !u8.ld0+2
u4f4.ld0+3! !u8.ld0+3
u4f4.ld1!   !u8.ld1
u4f4.ld1+1! !u8.ld1+1
u4f4.ld1+2! !u8.ld1+2
u4f4.ld1+3! !u8.ld1+3
u4f4.ld2!   !u8.ld2
u4f4.ld2+1! !u8.ld2+1
u4f4.ld2+2! !u8.ld2+2
u4f4.ld2+3! !u8.ld2+3
u4f4.ld3!   !u8.ld3
u4f4.ld3+1! !u8.ld3+1
u4f4.ld3+2! !u8.ld3+2
u4f4.ld3+3! !u8.ld3+3
u4f4.ld4!   !u8.ld4
u4f4.ld4+1! !u8.ld4+1
u4f4.ld4+2! !u8.ld4+2
u4f4.ld4+3! !u8.ld4+3
u4f4.ld5!   !u8.ld5
u4f4.ld5+1! !u8.ld5+1
u4f4.ld5+2! !u8.ld5+2
u4f4.ld5+3! !u8.ld5+3
u4f4.ld6!   !u8.ld6
u4f4.ld6+1! !u8.ld6+1
u4f4.ld6+2! !u8.ld6+2
u4f4.ld6+3! !u8.ld6+3
u4f4.ld7!   !u8.ld7
u4f4.ld7+1! !u8.ld7+1
u4f4.ld7+2! !u8.ld7+2
u4f4.ld7+3! !u8.ld7+3
u4f4.ld8!   !u8.ld8
u4f4.ld8+1! !u8.ld8+1
u4f4.ld8+2! !u8.ld8+2
u4f4.ld8+3! !u8.ld8+3
u4f4.ld9!   !u8.ld9
u4f4.ld9+1! !u8.ld9+1
u4f4.ld9+2! !u8.ld9+2
u4f4.ld9+3! !u8.ld9+3
u4f4.ldA!   !u8.ldA
u4f4.ldA+1! !u8.ldA+1
u4f4.ldA+2! !u8.ldA+2
u4f4.ldA+3! !u8.ldA+3
u4f4.ldB!   !u8.ldB
u4f4.ldB+1! !u8.ldB+1
u4f4.ldB+2! !u8.ldB+2
u4f4.ldB+3! !u8.ldB+3
u4f4.ldC!   !u8.ldC
u4f4.ldC+1! !u8.ldC+1
u4f4.ldC+2! !u8.ldC+2
u4f4.ldC+3! !u8.ldC+3
u4f4.ldD!   !u8.ldD
u4f4.ldD+1! !u8.ldD+1
u4f4.ldD+2! !u8.ldD+2
u4f4.ldD+3! !u8.ldD+3
u4f4.ldE!   !u8.ldE
u4f4.ldE+1! !u8.ldE+1
u4f4.ldE+2! !u8.ldE+2
u4f4.ldE+3! !u8.ldE+3
u4f4.ldF!   !u8.ldF
u4f4.ldF+1! !u8.ldF+1
u4f4.ldF+2! !u8.ldF+2
u4f4.ldF+3! !u8.ldF+3
u8f8.ld0!   !u16.ld0
u8f8.ld0+1! !u16.ld0+1
u8f8.ld0+2! !u16.ld0+2
u8f8.ld0+3! !u16.ld0+3
u8f8.ld1!   !u16.ld1
u8f8.ld1+1! !u16.ld1+1
u8f8.ld1+2! !u16.ld1+2
u8f8.ld1+3! !u16.ld1+3
u8f8.ld2!   !u16.ld2
u8f8.ld2+1! !u16.ld2+1
u8f8.ld2+2! !u16.ld2+2
u8f8.ld2+3! !u16.ld2+3
u8f8.ld3!   !u16.ld3
u8f8.ld3+1! !u16.ld3+1
u8f8.ld3+2! !u16.ld3+2
u8f8.ld3+3! !u16.ld3+3
u8f8.ld4!   !u16.ld4
u8f8.ld4+1! !u16.ld4+1
u8f8.ld4+2! !u16.ld4+2
u8f8.ld4+3! !u16.ld4+3
u8f8.ld5!   !u16.ld5
u8f8.ld5+1! !u16.ld5+1
u8f8.ld5+2! !u16.ld5+2
u8f8.ld5+3! !u16.ld5+3
u8f8.ld6!   !u16.ld6
u8f8.ld6+1! !u16.ld6+1
u8f8.ld6+2! !u16.ld6+2
u8f8.ld7!   !u16.ld7
i4f4.ld0!   !i8.ld0
i4f4.ld0+1! !i8.ld0+1
i4f4.ld0+2! !i8.ld0+2
i4f4.ld0+3! !i8.ld0+3
i4f4.ld1!   !i8.ld1
i4f4.ld1+1! !i8.ld1+1
i4f4.ld1+2! !i8.ld1+2
i4f4.ld1+3! !i8.ld1+3
i4f4.ld2!   !i8.ld2
i4f4.ld2+1! !i8.ld2+1
i4f4.ld2+2! !i8.ld2+2
i4f4.ld2+3! !i8.ld2+3
i4f4.ld3!   !i8.ld3
i4f4.ld3+1! !i8.ld3+1
i4f4.ld3+2! !i8.ld3+2
i4f4.ld3+3! !i8.ld3+3
i4f4.ld4!   !i8.ld4
i4f4.ld4+1! !i8.ld4+1
i4f4.ld4+2! !i8.ld4+2
i4f4.ld4+3! !i8.ld4+3
i4f4.ld5!   !i8.ld5
i4f4.ld5+1! !i8.ld5+1
i4f4.ld5+2! !i8.ld5+2
i4f4.ld5+3! !i8.ld5+3
i4f4.ld6!   !i8.ld6
i4f4.ld6+1! !i8.ld6+1
i4f4.ld6+2! !i8.ld6+2
i4f4.ld6+3! !i8.ld6+3
i4f4.ld7!   !i8.ld7
i4f4.ld7+1! !i8.ld7+1
i4f4.ld7+2! !i8.ld7+2
i4f4.ld7+3! !i8.ld7+3
i4f4.ld8!   !i8.ld8
i4f4.ld8+1! !i8.ld8+1
i4f4.ld8+2! !i8.ld8+2
i4f4.ld8+3! !i8.ld8+3
i4f4.ld9!   !i8.ld9
i4f4.ld9+1! !i8.ld9+1
i4f4.ld9+2! !i8.ld9+2
i4f4.ld9+3! !i8.ld9+3
i4f4.ldA!   !i8.ldA
i4f4.ldA+1! !i8.ldA+1
i4f4.ldA+2! !i8.ldA+2
i4f4.ldA+3! !i8.ldA+3
i4f4.ldB!   !i8.ldB
i4f4.ldB+1! !i8.ldB+1
i4f4.ldB+2! !i8.ldB+2
i4f4.ldB+3! !i8.ldB+3
i4f4.ldC!   !i8.ldC
i4f4.ldC+1! !i8.ldC+1
i4f4.ldC+2! !i8.ldC+2
i4f4.ldC+3! !i8.ldC+3
i4f4.ldD!   !i8.ldD
i4f4.ldD+1! !i8.ldD+1
i4f4.ldD+2! !i8.ldD+2
i4f4.ldD+3! !i8.ldD+3
i4f4.ldE!   !i8.ldE
i4f4.ldE+1! !i8.ldE+1
i4f4.ldE+2! !i8.ldE+2
i4f4.ldE+3! !i8.ldE+3
i4f4.ldF!   !i8.ldF
i4f4.ldF+1! !i8.ldF+1
i4f4.ldF+2! !i8.ldF+2
i4f4.ldF+3! !i8.ldF+3
i8f8.ld0!   !i16.ld0
i8f8.ld0+1! !i16.ld0+1
i8f8.ld0+2! !i16.ld0+2
i8f8.ld0+3! !i16.ld0+3
i8f8.ld1!   !i16.ld1
i8f8.ld1+1! !i16.ld1+1
i8f8.ld1+2! !i16.ld1+2
i8f8.ld1+3! !i16.ld1+3
i8f8.ld2!   !i16.ld2
i8f8.ld2+1! !i16.ld2+1
i8f8.ld2+2! !i16.ld2+2
i8f8.ld2+3! !i16.ld2+3
i8f8.ld3!   !i16.ld3
i8f8.ld3+1! !i16.ld3+1
i8f8.ld3+2! !i16.ld3+2
i8f8.ld3+3! !i16.ld3+3
i8f8.ld4!   !i16.ld4
i8f8.ld4+1! !i16.ld4+1
i8f8.ld4+2! !i16.ld4+2
i8f8.ld4+3! !i16.ld4+3
i8f8.ld5!   !i16.ld5
i8f8.ld5+1! !i16.ld5+1
i8f8.ld5+2! !i16.ld5+2
i8f8.ld5+3! !i16.ld5+3
i8f8.ld6!   !i16.ld6
i8f8.ld6+1! !i16.ld6+1
i8f8.ld6+2! !i16.ld6+2
i8f8.ld7!   !i16.ld7
c4f4m4f4.ld0!   !i16.ld0
c4f4m4f4.ld0+1! !i16.ld0+1
c4f4m4f4.ld0+2! !i16.ld0+2
c4f4m4f4.ld0+3! !i16.ld0+3
c4f4m4f4.ld1!   !i16.ld1
c4f4m4f4.ld1+1! !i16.ld1+1
c4f4m4f4.ld1+2! !i16.ld1+2
c4f4m4f4.ld1+3! !i16.ld1+3
c4f4m4f4.ld2!   !i16.ld2
c4f4m4f4.ld2+1! !i16.ld2+1
c4f4m4f4.ld2+2! !i16.ld2+2
c4f4m4f4.ld2+3! !i16.ld2+3
c4f4m4f4.ld3!   !i16.ld3
c4f4m4f4.ld3+1! !i16.ld3+1
c4f4m4f4.ld3+2! !i16.ld3+2
c4f4m4f4.ld3+3! !i16.ld3+3
c4f4m4f4.ld4!   !i16.ld4
c4f4m4f4.ld4+1! !i16.ld4+1
c4f4m4f4.ld4+2! !i16.ld4+2
c4f4m4f4.ld4+3! !i16.ld4+3
c4f4m4f4.ld5!   !i16.ld5
c4f4m4f4.ld5+1! !i16.ld5+1
c4f4m4f4.ld5+2! !i16.ld5+2
c4f4m4f4.ld5+3! !i16.ld5+3
c4f4m4f4.ld6!   !i16.ld6
c4f4m4f4.ld6+1! !i16.ld6+1
c4f4m4f4.ld6+2! !i16.ld6+2
c4f4m4f4.ld7!   !i16.ld7
c8f8m8f8.ld0!   !i32.ld0
c8f8m8f8.ld0+1! !i32.ld0+1
c8f8m8f8.ld0+2! !i32.ld0+2
c8f8m8f8.ld0+3! !i32.ld0+3
c8f8m8f8.ld1!   !i32.ld1
c8f8m8f8.ld1+1! !i32.ld1+1
c8f8m8f8.ld1+2! !i32.ld1+2
c8f8m8f8.ld1+3! !i32.ld1+3
c8f8m8f8.ld2!   !i32.ld2
c8f8m8f8.ld2+1! !i32.ld2+1
c8f8m8f8.ld2+2! !i32.ld2+2
c8f8m8f8.ld2+3! !i32.ld2+3
c8f8m8f8.ld3!   !i32.ld3
char.ld0!   !u8.ld0
char.ld0+1! !u8.ld0+1
char.ld0+2! !u8.ld0+2
char.ld0+3! !u8.ld0+3
char.ld1!   !u8.ld1
char.ld1+1! !u8.ld1+1
char.ld1+2! !u8.ld1+2
char.ld1+3! !u8.ld1+3
char.ld2!   !u8.ld2
char.ld2+1! !u8.ld2+1
char.ld2+2! !u8.ld2+2
char.ld2+3! !u8.ld2+3
char.ld3!   !u8.ld3
char.ld3+1! !u8.ld3+1
char.ld3+2! !u8.ld3+2
char.ld3+3! !u8.ld3+3
char.ld4!   !u8.ld4
char.ld4+1! !u8.ld4+1
char.ld4+2! !u8.ld4+2
char.ld4+3! !u8.ld4+3
char.ld5!   !u8.ld5
char.ld5+1! !u8.ld5+1
char.ld5+2! !u8.ld5+2
char.ld5+3! !u8.ld5+3
char.ld6!   !u8.ld6
char.ld6+1! !u8.ld6+1
char.ld6+2! !u8.ld6+2
char.ld6+3! !u8.ld6+3
char.ld7!   !u8.ld7
char.ld7+1! !u8.ld7+1
char.ld7+2! !u8.ld7+2
char.ld7+3! !u8.ld7+3
char.ld8!   !u8.ld8
char.ld8+1! !u8.ld8+1
char.ld8+2! !u8.ld8+2
char.ld8+3! !u8.ld8+3
char.ld9!   !u8.ld9
char.ld9+1! !u8.ld9+1
char.ld9+2! !u8.ld9+2
char.ld9+3! !u8.ld9+3
char.ldA!   !u8.ldA
char.ldA+1! !u8.ldA+1
char.ldA+2! !u8.ldA+2
char.ldA+3! !u8.ldA+3
char.ldB!   !u8.ldB
char.ldB+1! !u8.ldB+1
char.ldB+2! !u8.ldB+2
char.ldB+3! !u8.ldB+3
char.ldC!   !u8.ldC
char.ldC+1! !u8.ldC+1
char.ldC+2! !u8.ldC+2
char.ldC+3! !u8.ldC+3
char.ldD!   !u8.ldD
char.ldD+1! !u8.ldD+1
char.ldD+2! !u8.ldD+2
char.ldD+3! !u8.ldD+3
char.ldE!   !u8.ldE
char.ldE+1! !u8.ldE+1
char.ldE+2! !u8.ldE+2
char.ldE+3! !u8.ldE+3
char.ldF!   !u8.ldF
char.ldF+1! !u8.ldF+1
char.ldF+2! !u8.ldF+2
char.ldF+3! !u8.ldF+3

u8.st0!   st0
u8.st0+1! st1
u8.st0+2! st2
u8.st0+3! st3
u8.st1!   st1
u8.st1+1! st2
u8.st1+2! st3
u8.st1+3! st4
u8.st2!   st2
u8.st2+1! st3
u8.st2+2! st4
u8.st2+3! st5
u8.st3!   st3
u8.st3+1! st4
u8.st3+2! st5
u8.st3+3! st6
u8.st4!   st4
u8.st4+1! st5
u8.st4+2! st6
u8.st4+3! st7
u8.st5!   st5
u8.st5+1! st6
u8.st5+2! st7
u8.st5+3! st8
u8.st6!   st6
u8.st6+1! st7
u8.st6+2! st8
u8.st6+3! st9
u8.st7!   st7
u8.st7+1! st8
u8.st7+2! st9
u8.st7+3! stA
u8.st8!   st8
u8.st8+1! st9
u8.st8+2! stA
u8.st8+3! stB
u8.st9!   st9
u8.st9+1! stA
u8.st9+2! stB
u8.st9+3! stC
u8.stA!   stA
u8.stA+1! stB
u8.stA+2! stC
u8.stA+3! stD
u8.stB!   stB
u8.stB+1! stC
u8.stB+2! stD
u8.stB+3! stE
u8.stC!   stC
u8.stC+1! stD
u8.stC+2! stE
u8.stC+3! stF
u8.stD!   stD
u8.stD+1! stE
u8.stD+2! stF
u8.stE!   stE
u8.stE+1! stF
u8.stF!   stF
u16.st0!   st1 st1
u16.st0+1! st2 st2
u16.st0+2! st3 st3
u16.st0+3! st4 st4
u16.st1!   st3 st3
u16.st1+1! st4 st4
u16.st1+2! st5 st5
u16.st1+3! st6 st6
u16.st2!   st5 st5
u16.st2+1! st6 st6
u16.st2+2! st7 st7
u16.st2+3! st8 st8
u16.st3!   st7 st7
u16.st3+1! st8 st8
u16.st3+2! st9 st9
u16.st3+3! stA stA
u16.st4!   st9 st9
u16.st4+1! stA stA
u16.st4+2! stB stB
u16.st4+3! stC stC
u16.st5!   stB stB
u16.st5+1! stC stC
u16.st5+2! stD stD
u16.st5+3! stE stE
u16.st6!   stD stD
u16.st6+1! stE stE
u16.st6+2! stF stF
u16.st7!   stF stF
u32.st0!   st3 st3 st3 st3
u32.st0+1! st4 st4 st4 st4
u32.st0+2! st5 st5 st5 st5
u32.st0+3! st6 st6 st6 st6
u32.st1!   st7 st7 st7 st7
u32.st1+1! st8 st8 st8 st8
u32.st1+2! st9 st9 st9 st9
u32.st1+3! stA stA stA stA
u32.st2!   stB stB stB stB
u32.st2+1! stC stC stC stC
u32.st2+2! stD stD stD stD
u32.st2+3! stE stE stE stE
u32.st3!   stF stF stF stF
u64.st0!   st7 st7 st7 st7 st7 st7 st7 st7
u64.st0+1! st8 st8 st8 st8 st8 st8 st8 st8
u64.st0+2! st9 st9 st9 st9 st9 st9 st9 st9
u64.st0+3! stA stA stA stA stA stA stA stA
u64.st1!   stF stF stF stF stF stF stF stF
i8.st0!   st0
i8.st0+1! st1
i8.st0+2! st2
i8.st0+3! st3
i8.st1!   st1
i8.st1+1! st2
i8.st1+2! st3
i8.st1+3! st4
i8.st2!   st2
i8.st2+1! st3
i8.st2+2! st4
i8.st2+3! st5
i8.st3!   st3
i8.st3+1! st4
i8.st3+2! st5
i8.st3+3! st6
i8.st4!   st4
i8.st4+1! st5
i8.st4+2! st6
i8.st4+3! st7
i8.st5!   st5
i8.st5+1! st6
i8.st5+2! st7
i8.st5+3! st8
i8.st6!   st6
i8.st6+1! st7
i8.st6+2! st8
i8.st6+3! st9
i8.st7!   st7
i8.st7+1! st8
i8.st7+2! st9
i8.st7+3! stA
i8.st8!   st8
i8.st8+1! st9
i8.st8+2! stA
i8.st8+3! stB
i8.st9!   st9
i8.st9+1! stA
i8.st9+2! stB
i8.st9+3! stC
i8.stA!   stA
i8.stA+1! stB
i8.stA+2! stC
i8.stA+3! stD
i8.stB!   stB
i8.stB+1! stC
i8.stB+2! stD
i8.stB+3! stE
i8.stC!   stC
i8.stC+1! stD
i8.stC+2! stE
i8.stC+3! stF
i8.stD!   stD
i8.stD+1! stE
i8.stD+2! stF
i8.stE!   stE
i8.stE+1! stF
i8.stF!   stF
i16.st0!   st1 st1
i16.st0+1! st2 st2
i16.st0+2! st3 st3
i16.st0+3! st4 st4
i16.st1!   st3 st3
i16.st1+1! st4 st4
i16.st1+2! st5 st5
i16.st1+3! st6 st6
i16.st2!   st5 st5
i16.st2+1! st6 st6
i16.st2+2! st7 st7
i16.st2+3! st8 st8
i16.st3!   st7 st7
i16.st3+1! st8 st8
i16.st3+2! st9 st9
i16.st3+3! stA stA
i16.st4!   st9 st9
i16.st4+1! stA stA
i16.st4+2! stB stB
i16.st4+3! stC stC
i16.st5!   stB stB
i16.st5+1! stC stC
i16.st5+2! stD stD
i16.st5+3! stE stE
i16.st6!   stD stD
i16.st6+1! stE stE
i16.st6+2! stF stF
i16.st7!   stF stF
i32.st0!   st3 st3 st3 st3
i32.st0+1! st4 st4 st4 st4
i32.st0+2! st5 st5 st5 st5
i32.st0+3! st6 st6 st6 st6
i32.st1!   st7 st7 st7 st7
i32.st1+1! st8 st8 st8 st8
i32.st1+2! st9 st9 st9 st9
i32.st1+3! stA stA stA stA
i32.st2!   stB stB stB stB
i32.st2+1! stC stC stC stC
i32.st2+2! stD stD stD stD
i32.st2+3! stE stE stE stE
i32.st3!   stF stF stF stF
i64.st0!   st7 st7 st7 st7 st7 st7 st7 st7
i64.st0+1! st8 st8 st8 st8 st8 st8 st8 st8
i64.st0+2! st9 st9 st9 st9 st9 st9 st9 st9
i64.st0+3! stA stA stA stA stA stA stA stA
i64.st1!   stF stF stF stF stF stF stF stF
u4u4.st0!   !u8.st0
u4u4.st0+1! !u8.st0+1
u4u4.st0+2! !u8.st0+2
u4u4.st0+3! !u8.st0+3
u4u4.st1!   !u8.st1
u4u4.st1+1! !u8.st1+1
u4u4.st1+2! !u8.st1+2
u4u4.st1+3! !u8.st1+3
u4u4.st2!   !u8.st2
u4u4.st2+1! !u8.st2+1
u4u4.st2+2! !u8.st2+2
u4u4.st2+3! !u8.st2+3
u4u4.st3!   !u8.st3
u4u4.st3+1! !u8.st3+1
u4u4.st3+2! !u8.st3+2
u4u4.st3+3! !u8.st3+3
u4u4.st4!   !u8.st4
u4u4.st4+1! !u8.st4+1
u4u4.st4+2! !u8.st4+2
u4u4.st4+3! !u8.st4+3
u4u4.st5!   !u8.st5
u4u4.st5+1! !u8.st5+1
u4u4.st5+2! !u8.st5+2
u4u4.st5+3! !u8.st5+3
u4u4.st6!   !u8.st6
u4u4.st6+1! !u8.st6+1
u4u4.st6+2! !u8.st6+2
u4u4.st6+3! !u8.st6+3
u4u4.st7!   !u8.st7
u4u4.st7+1! !u8.st7+1
u4u4.st7+2! !u8.st7+2
u4u4.st7+3! !u8.st7+3
u4u4.st8!   !u8.st8
u4u4.st8+1! !u8.st8+1
u4u4.st8+2! !u8.st8+2
u4u4.st8+3! !u8.st8+3
u4u4.st9!   !u8.st9
u4u4.st9+1! !u8.st9+1
u4u4.st9+2! !u8.st9+2
u4u4.st9+3! !u8.st9+3
u4u4.stA!   !u8.stA
u4u4.stA+1! !u8.stA+1
u4u4.stA+2! !u8.stA+2
u4u4.stA+3! !u8.stA+3
u4u4.stB!   !u8.stB
u4u4.stB+1! !u8.stB+1
u4u4.stB+2! !u8.stB+2
u4u4.stB+3! !u8.stB+3
u4u4.stC!   !u8.stC
u4u4.stC+1! !u8.stC+1
u4u4.stC+2! !u8.stC+2
u4u4.stC+3! !u8.stC+3
u4u4.stD!   !u8.stD
u4u4.stD+1! !u8.stD+1
u4u4.stD+2! !u8.stD+2
u4u4.stD+3! !u8.stD+3
u4u4.stE!   !u8.stE
u4u4.stE+1! !u8.stE+1
u4u4.stE+2! !u8.stE+2
u4u4.stE+3! !u8.stE+3
u4u4.stF!   !u8.stF
u4u4.stF+1! !u8.stF+1
u4u4.stF+2! !u8.stF+2
u4u4.stF+3! !u8.stF+3
u8u8.st0!   !u16.st0
u8u8.st0+1! !u16.st0+1
u8u8.st0+2! !u16.st0+2
u8u8.st0+3! !u16.st0+3
u8u8.st1!   !u16.st1
u8u8.st1+1! !u16.st1+1
u8u8.st1+2! !u16.st1+2
u8u8.st1+3! !u16.st1+3
u8u8.st2!   !u16.st2
u8u8.st2+1! !u16.st2+1
u8u8.st2+2! !u16.st2+2
u8u8.st2+3! !u16.st2+3
u8u8.st3!   !u16.st3
u8u8.st3+1! !u16.st3+1
u8u8.st3+2! !u16.st3+2
u8u8.st3+3! !u16.st3+3
u8u8.st4!   !u16.st4
u8u8.st4+1! !u16.st4+1
u8u8.st4+2! !u16.st4+2
u8u8.st4+3! !u16.st4+3
u8u8.st5!   !u16.st5
u8u8.st5+1! !u16.st5+1
u8u8.st5+2! !u16.st5+2
u8u8.st5+3! !u16.st5+3
u8u8.st6!   !u16.st6
u8u8.st6+1! !u16.st6+1
u8u8.st6+2! !u16.st6+2
u8u8.st7!   !u16.st7
i4i4.st0!   !i8.st0
i4i4.st0+1! !i8.st0+1
i4i4.st0+2! !i8.st0+2
i4i4.st0+3! !i8.st0+3
i4i4.st1!   !i8.st1
i4i4.st1+1! !i8.st1+1
i4i4.st1+2! !i8.st1+2
i4i4.st1+3! !i8.st1+3
i4i4.st2!   !i8.st2
i4i4.st2+1! !i8.st2+1
i4i4.st2+2! !i8.st2+2
i4i4.st2+3! !i8.st2+3
i4i4.st3!   !i8.st3
i4i4.st3+1! !i8.st3+1
i4i4.st3+2! !i8.st3+2
i4i4.st3+3! !i8.st3+3
i4i4.st4!   !i8.st4
i4i4.st4+1! !i8.st4+1
i4i4.st4+2! !i8.st4+2
i4i4.st4+3! !i8.st4+3
i4i4.st5!   !i8.st5
i4i4.st5+1! !i8.st5+1
i4i4.st5+2! !i8.st5+2
i4i4.st5+3! !i8.st5+3
i4i4.st6!   !i8.st6
i4i4.st6+1! !i8.st6+1
i4i4.st6+2! !i8.st6+2
i4i4.st6+3! !i8.st6+3
i4i4.st7!   !i8.st7
i4i4.st7+1! !i8.st7+1
i4i4.st7+2! !i8.st7+2
i4i4.st7+3! !i8.st7+3
i4i4.st8!   !i8.st8
i4i4.st8+1! !i8.st8+1
i4i4.st8+2! !i8.st8+2
i4i4.st8+3! !i8.st8+3
i4i4.st9!   !i8.st9
i4i4.st9+1! !i8.st9+1
i4i4.st9+2! !i8.st9+2
i4i4.st9+3! !i8.st9+3
i4i4.stA!   !i8.stA
i4i4.stA+1! !i8.stA+1
i4i4.stA+2! !i8.stA+2
i4i4.stA+3! !i8.stA+3
i4i4.stB!   !i8.stB
i4i4.stB+1! !i8.stB+1
i4i4.stB+2! !i8.stB+2
i4i4.stB+3! !i8.stB+3
i4i4.stC!   !i8.stC
i4i4.stC+1! !i8.stC+1
i4i4.stC+2! !i8.stC+2
i4i4.stC+3! !i8.stC+3
i4i4.stD!   !i8.stD
i4i4.stD+1! !i8.stD+1
i4i4.stD+2! !i8.stD+2
i4i4.stD+3! !i8.stD+3
i4i4.stE!   !i8.stE
i4i4.stE+1! !i8.stE+1
i4i4.stE+2! !i8.stE+2
i4i4.stE+3! !i8.stE+3
i4i4.stF!   !i8.stF
i4i4.stF+1! !i8.stF+1
i4i4.stF+2! !i8.stF+2
i4i4.stF+3! !i8.stF+3
i8i8.st0!   !i16.st0
i8i8.st0+1! !i16.st0+1
i8i8.st0+2! !i16.st0+2
i8i8.st0+3! !i16.st0+3
i8i8.st1!   !i16.st1
i8i8.st1+1! !i16.st1+1
i8i8.st1+2! !i16.st1+2
i8i8.st1+3! !i16.st1+3
i8i8.st2!   !i16.st2
i8i8.st2+1! !i16.st2+1
i8i8.st2+2! !i16.st2+2
i8i8.st2+3! !i16.st2+3
i8i8.st3!   !i16.st3
i8i8.st3+1! !i16.st3+1
i8i8.st3+2! !i16.st3+2
i8i8.st3+3! !i16.st3+3
i8i8.st4!   !i16.st4
i8i8.st4+1! !i16.st4+1
i8i8.st4+2! !i16.st4+2
i8i8.st4+3! !i16.st4+3
i8i8.st5!   !i16.st5
i8i8.st5+1! !i16.st5+1
i8i8.st5+2! !i16.st5+2
i8i8.st5+3! !i16.st5+3
i8i8.st6!   !i16.st6
i8i8.st6+1! !i16.st6+1
i8i8.st6+2! !i16.st6+2
i8i8.st7!   !i16.st7
u4f4.st0!   !u8.st0
u4f4.st0+1! !u8.st0+1
u4f4.st0+2! !u8.st0+2
u4f4.st0+3! !u8.st0+3
u4f4.st1!   !u8.st1
u4f4.st1+1! !u8.st1+1
u4f4.st1+2! !u8.st1+2
u4f4.st1+3! !u8.st1+3
u4f4.st2!   !u8.st2
u4f4.st2+1! !u8.st2+1
u4f4.st2+2! !u8.st2+2
u4f4.st2+3! !u8.st2+3
u4f4.st3!   !u8.st3
u4f4.st3+1! !u8.st3+1
u4f4.st3+2! !u8.st3+2
u4f4.st3+3! !u8.st3+3
u4f4.st4!   !u8.st4
u4f4.st4+1! !u8.st4+1
u4f4.st4+2! !u8.st4+2
u4f4.st4+3! !u8.st4+3
u4f4.st5!   !u8.st5
u4f4.st5+1! !u8.st5+1
u4f4.st5+2! !u8.st5+2
u4f4.st5+3! !u8.st5+3
u4f4.st6!   !u8.st6
u4f4.st6+1! !u8.st6+1
u4f4.st6+2! !u8.st6+2
u4f4.st6+3! !u8.st6+3
u4f4.st7!   !u8.st7
u4f4.st7+1! !u8.st7+1
u4f4.st7+2! !u8.st7+2
u4f4.st7+3! !u8.st7+3
u4f4.st8!   !u8.st8
u4f4.st8+1! !u8.st8+1
u4f4.st8+2! !u8.st8+2
u4f4.st8+3! !u8.st8+3
u4f4.st9!   !u8.st9
u4f4.st9+1! !u8.st9+1
u4f4.st9+2! !u8.st9+2
u4f4.st9+3! !u8.st9+3
u4f4.stA!   !u8.stA
u4f4.stA+1! !u8.stA+1
u4f4.stA+2! !u8.stA+2
u4f4.stA+3! !u8.stA+3
u4f4.stB!   !u8.stB
u4f4.stB+1! !u8.stB+1
u4f4.stB+2! !u8.stB+2
u4f4.stB+3! !u8.stB+3
u4f4.stC!   !u8.stC
u4f4.stC+1! !u8.stC+1
u4f4.stC+2! !u8.stC+2
u4f4.stC+3! !u8.stC+3
u4f4.stD!   !u8.stD
u4f4.stD+1! !u8.stD+1
u4f4.stD+2! !u8.stD+2
u4f4.stD+3! !u8.stD+3
u4f4.stE!   !u8.stE
u4f4.stE+1! !u8.stE+1
u4f4.stE+2! !u8.stE+2
u4f4.stE+3! !u8.stE+3
u4f4.stF!   !u8.stF
u4f4.stF+1! !u8.stF+1
u4f4.stF+2! !u8.stF+2
u4f4.stF+3! !u8.stF+3
u8f8.st0!   !u16.st0
u8f8.st0+1! !u16.st0+1
u8f8.st0+2! !u16.st0+2
u8f8.st0+3! !u16.st0+3
u8f8.st1!   !u16.st1
u8f8.st1+1! !u16.st1+1
u8f8.st1+2! !u16.st1+2
u8f8.st1+3! !u16.st1+3
u8f8.st2!   !u16.st2
u8f8.st2+1! !u16.st2+1
u8f8.st2+2! !u16.st2+2
u8f8.st2+3! !u16.st2+3
u8f8.st3!   !u16.st3
u8f8.st3+1! !u16.st3+1
u8f8.st3+2! !u16.st3+2
u8f8.st3+3! !u16.st3+3
u8f8.st4!   !u16.st4
u8f8.st4+1! !u16.st4+1
u8f8.st4+2! !u16.st4+2
u8f8.st4+3! !u16.st4+3
u8f8.st5!   !u16.st5
u8f8.st5+1! !u16.st5+1
u8f8.st5+2! !u16.st5+2
u8f8.st5+3! !u16.st5+3
u8f8.st6!   !u16.st6
u8f8.st6+1! !u16.st6+1
u8f8.st6+2! !u16.st6+2
u8f8.st7!   !u16.st7
i4f4.st0!   !i8.st0
i4f4.st0+1! !i8.st0+1
i4f4.st0+2! !i8.st0+2
i4f4.st0+3! !i8.st0+3
i4f4.st1!   !i8.st1
i4f4.st1+1! !i8.st1+1
i4f4.st1+2! !i8.st1+2
i4f4.st1+3! !i8.st1+3
i4f4.st2!   !i8.st2
i4f4.st2+1! !i8.st2+1
i4f4.st2+2! !i8.st2+2
i4f4.st2+3! !i8.st2+3
i4f4.st3!   !i8.st3
i4f4.st3+1! !i8.st3+1
i4f4.st3+2! !i8.st3+2
i4f4.st3+3! !i8.st3+3
i4f4.st4!   !i8.st4
i4f4.st4+1! !i8.st4+1
i4f4.st4+2! !i8.st4+2
i4f4.st4+3! !i8.st4+3
i4f4.st5!   !i8.st5
i4f4.st5+1! !i8.st5+1
i4f4.st5+2! !i8.st5+2
i4f4.st5+3! !i8.st5+3
i4f4.st6!   !i8.st6
i4f4.st6+1! !i8.st6+1
i4f4.st6+2! !i8.st6+2
i4f4.st6+3! !i8.st6+3
i4f4.st7!   !i8.st7
i4f4.st7+1! !i8.st7+1
i4f4.st7+2! !i8.st7+2
i4f4.st7+3! !i8.st7+3
i4f4.st8!   !i8.st8
i4f4.st8+1! !i8.st8+1
i4f4.st8+2! !i8.st8+2
i4f4.st8+3! !i8.st8+3
i4f4.st9!   !i8.st9
i4f4.st9+1! !i8.st9+1
i4f4.st9+2! !i8.st9+2
i4f4.st9+3! !i8.st9+3
i4f4.stA!   !i8.stA
i4f4.stA+1! !i8.stA+1
i4f4.stA+2! !i8.stA+2
i4f4.stA+3! !i8.stA+3
i4f4.stB!   !i8.stB
i4f4.stB+1! !i8.stB+1
i4f4.stB+2! !i8.stB+2
i4f4.stB+3! !i8.stB+3
i4f4.stC!   !i8.stC
i4f4.stC+1! !i8.stC+1
i4f4.stC+2! !i8.stC+2
i4f4.stC+3! !i8.stC+3
i4f4.stD!   !i8.stD
i4f4.stD+1! !i8.stD+1
i4f4.stD+2! !i8.stD+2
i4f4.stD+3! !i8.stD+3
i4f4.stE!   !i8.stE
i4f4.stE+1! !i8.stE+1
i4f4.stE+2! !i8.stE+2
i4f4.stE+3! !i8.stE+3
i4f4.stF!   !i8.stF
i4f4.stF+1! !i8.stF+1
i4f4.stF+2! !i8.stF+2
i4f4.stF+3! !i8.stF+3
i8f8.st0!   !i16.st0
i8f8.st0+1! !i16.st0+1
i8f8.st0+2! !i16.st0+2
i8f8.st0+3! !i16.st0+3
i8f8.st1!   !i16.st1
i8f8.st1+1! !i16.st1+1
i8f8.st1+2! !i16.st1+2
i8f8.st1+3! !i16.st1+3
i8f8.st2!   !i16.st2
i8f8.st2+1! !i16.st2+1
i8f8.st2+2! !i16.st2+2
i8f8.st2+3! !i16.st2+3
i8f8.st3!   !i16.st3
i8f8.st3+1! !i16.st3+1
i8f8.st3+2! !i16.st3+2
i8f8.st3+3! !i16.st3+3
i8f8.st4!   !i16.st4
i8f8.st4+1! !i16.st4+1
i8f8.st4+2! !i16.st4+2
i8f8.st4+3! !i16.st4+3
i8f8.st5!   !i16.st5
i8f8.st5+1! !i16.st5+1
i8f8.st5+2! !i16.st5+2
i8f8.st5+3! !i16.st5+3
i8f8.st6!   !i16.st6
i8f8.st6+1! !i16.st6+1
i8f8.st6+2! !i16.st6+2
i8f8.st7!   !i16.st7
c4f4m4f4.st0!   !i16.st0
c4f4m4f4.st0+1! !i16.st0+1
c4f4m4f4.st0+2! !i16.st0+2
c4f4m4f4.st0+3! !i16.st0+3
c4f4m4f4.st1!   !i16.st1
c4f4m4f4.st1+1! !i16.st1+1
c4f4m4f4.st1+2! !i16.st1+2
c4f4m4f4.st1+3! !i16.st1+3
c4f4m4f4.st2!   !i16.st2
c4f4m4f4.st2+1! !i16.st2+1
c4f4m4f4.st2+2! !i16.st2+2
c4f4m4f4.st2+3! !i16.st2+3
c4f4m4f4.st3!   !i16.st3
c4f4m4f4.st3+1! !i16.st3+1
c4f4m4f4.st3+2! !i16.st3+2
c4f4m4f4.st3+3! !i16.st3+3
c4f4m4f4.st4!   !i16.st4
c4f4m4f4.st4+1! !i16.st4+1
c4f4m4f4.st4+2! !i16.st4+2
c4f4m4f4.st4+3! !i16.st4+3
c4f4m4f4.st5!   !i16.st5
c4f4m4f4.st5+1! !i16.st5+1
c4f4m4f4.st5+2! !i16.st5+2
c4f4m4f4.st5+3! !i16.st5+3
c4f4m4f4.st6!   !i16.st6
c4f4m4f4.st6+1! !i16.st6+1
c4f4m4f4.st6+2! !i16.st6+2
c4f4m4f4.st7!   !i16.st7
c8f8m8f8.st0!   !i32.st0
c8f8m8f8.st0+1! !i32.st0+1
c8f8m8f8.st0+2! !i32.st0+2
c8f8m8f8.st0+3! !i32.st0+3
c8f8m8f8.st1!   !i32.st1
c8f8m8f8.st1+1! !i32.st1+1
c8f8m8f8.st1+2! !i32.st1+2
c8f8m8f8.st1+3! !i32.st1+3
c8f8m8f8.st2!   !i32.st2
c8f8m8f8.st2+1! !i32.st2+1
c8f8m8f8.st2+2! !i32.st2+2
c8f8m8f8.st2+3! !i32.st2+3
c8f8m8f8.st3!   !i32.st3
char.st0!   !u8.st0
char.st0+1! !u8.st0+1
char.st0+2! !u8.st0+2
char.st0+3! !u8.st0+3
char.st1!   !u8.st1
char.st1+1! !u8.st1+1
char.st1+2! !u8.st1+2
char.st1+3! !u8.st1+3
char.st2!   !u8.st2
char.st2+1! !u8.st2+1
char.st2+2! !u8.st2+2
char.st2+3! !u8.st2+3
char.st3!   !u8.st3
char.st3+1! !u8.st3+1
char.st3+2! !u8.st3+2
char.st3+3! !u8.st3+3
char.st4!   !u8.st4
char.st4+1! !u8.st4+1
char.st4+2! !u8.st4+2
char.st4+3! !u8.st4+3
char.st5!   !u8.st5
char.st5+1! !u8.st5+1
char.st5+2! !u8.st5+2
char.st5+3! !u8.st5+3
char.st6!   !u8.st6
char.st6+1! !u8.st6+1
char.st6+2! !u8.st6+2
char.st6+3! !u8.st6+3
char.st7!   !u8.st7
char.st7+1! !u8.st7+1
char.st7+2! !u8.st7+2
char.st7+3! !u8.st7+3
char.st8!   !u8.st8
char.st8+1! !u8.st8+1
char.st8+2! !u8.st8+2
char.st8+3! !u8.st8+3
char.st9!   !u8.st9
char.st9+1! !u8.st9+1
char.st9+2! !u8.st9+2
char.st9+3! !u8.st9+3
char.stA!   !u8.stA
char.stA+1! !u8.stA+1
char.stA+2! !u8.stA+2
char.stA+3! !u8.stA+3
char.stB!   !u8.stB
char.stB+1! !u8.stB+1
char.stB+2! !u8.stB+2
char.stB+3! !u8.stB+3
char.stC!   !u8.stC
char.stC+1! !u8.stC+1
char.stC+2! !u8.stC+2
char.stC+3! !u8.stC+3
char.stD!   !u8.stD
char.stD+1! !u8.stD+1
char.stD+2! !u8.stD+2
char.stD+3! !u8.stD+3
char.stE!   !u8.stE
char.stE+1! !u8.stE+1
char.stE+2! !u8.stE+2
char.stE+3! !u8.stE+3
char.stF!   !u8.stF
char.stF+1! !u8.stF+1
char.stF+2! !u8.stF+2
char.stF+3! !u8.stF+3

u4u4.fst! xF0 and x04 rot # u8 first = u4u4.fst(u4u4 n)
u4u4.snd! x0F and # u8 second = u4u4.snd(u4u4 n)
u8u8.fst! !u8.pop # u8 first = u8u8.fst(u8u8 n)
u8u8.snd! !u8.st0 # u8 second = u8u8.snd(u8u8 n)
i4i4.fst! !u4u4.fst # i8 first = i4i4.fst(i4i4 n)
i4i4.snd! !u4u4.snd # i8 second = i4i4.snd(i4i4 n)
i8i8.fst! !u8u8.fst # i8 first = i8i8.fst(i8i8 n)
i8i8.snd! !u8u8.snd # i8 second = i8i8.snd(i8i8 n)
u4f4.in! !u4u4.fst # u8 integer_part = u4f4.in(u4f4 n)
u4f4.fr! !u4u4.snd # u8 fraction_part = u4f4.fr(u4f4 n)
u8f8.in! !u8u8.fst # u8 integer_part = u8f8.in(u8f8 n)
u8f8.fr! !u8u8.snd # u8 fraction_part = u8f8.fr(u8f8 n)
i4f4.in! !u4f4.in # i8 integer_part = i4f4.in(i4f4 n)
i4f4.fr! !u4f4.fr # u8 fraction_part = i4f4.fr(i4f4 n)
i8f8.in! !u8f8.in # i8 integer_part = i8f8.in(i8f8 n)
i8f8.fr! !u8f8.fr # u8 fraction_part = i8f8.fr(i8f8 n)
c4f4m4f4.re! !u8.pop # i4f4 real_part = c4f4m4f4.re(c4f4m4f4 c)
c4f4m4f4.im! !u8.st0 # i4f4 imaginary_part = c4f4m4f4.im(c4f4m4f4 c)
c8f8m8f8.re! !u16.pop # i8f8 real_part = c8f8m8f8.re(c8f8m8f8 c)
c8f8m8f8.im! !u16.st0 # i8f8 imaginary_part = c8f8m8f8.im(c8f8m8f8 c)

u8.mul! :u8.mul !call # u16 product = u8.mul(u8 a, u8 b)
u16.mul! :u16.mul !call # u32 product = u16.mul(u16 a, u16 b)
i8.mul! :i8.mul !call # i16 product = i8.mul(i8 a, i8 b)
i16.mul! :i16.mul !call # i32 product = i16.mul(i16 a, i16 b)
u4f4.mul! :u4f4.mul !call # u4f4 product = u4f4.mul(u4f4 a, u4f4 b)
u8f8.mul! :u8f8.mul !call # u8f8 product = u8f8.mul(u8f8 a, u8f8 b)
i4f4.mul! :i4f4.mul !call # i4f4 product = i4f4.mul(i4f4 a, i4f4 b)
i8f8.mul! :i8f8.mul !call # i8f8 product = i8f8.mul(i8f8 a, i8f8 b)
c4f4m4f4.mul! # c4f4m4f4 product = c4f4m4f4.mul(c4f4m4f4 a, c4f4m4f4 b)
  !i4f4.ld3 !i4f4.ld2 !i4f4.mul !i4f4.ld3 !i4f4.ld2 !i4f4.mul !i4f4.sub # real part
  !i4f4.ld4 !i4f4.ld2 !i4f4.mul !i4f4.ld4 !i4f4.ld4 !i4f4.mul !i4f4.add # imaginary part
  !i16.st1 !i16.pop
c8f8m8f8.mul! # c8f8m8f8 product = c8f8m8f8.mul(c8f8m8f8 a, c8f8m8f8 b)
  !i8f8.ld3 !i8f8.ld2 !i8f8.mul !i8f8.ld3 !i8f8.ld2 !i8f8.mul !i8f8.sub # real part
  !i8f8.ld4 !i8f8.ld2 !i8f8.mul !i8f8.ld4 !i8f8.ld4 !i8f8.mul !i8f8.add # imaginary part
  !i32.st1 !i32.pop

c4f4m4f4.norm! !i4f4.ld1 !i4f4.ld0 !i4f4.mul !i4f4.st1 !i4f4.ld0 !i4f4.mul !i4f4.add # i4f4 norm = c4f4m4f4.norm(c4f4m4f4 c)
c8f8m8f8.norm! !i8f8.ld1 !i8f8.ld0 !i8f8.mul !i8f8.st1 !i8f8.ld0 !i8f8.mul !i4f4.add # i8f8 norm = c8f8m8f8.norm(c8f8m8f8 c)

u8.check_null! !z
u8.is_null! !zr

# converts `0x0..=0xF` to `'0'..='9', 'A'..='F'`. undefined for other values
u4.to_hex!
  !char.digit_count sub @dyn
    !'A' @const
    !char.digit_count !'0' add dec @const
  iff add @dyn
# converts `'0'..='9', 'A'..='F'` to `0x0..=0xF`. undefined for other values
hex.to_u4!
  !'A' sub @dyn
    !char.digit_count @const
    !'A' !'0' sub dec @const
  iff add @dyn
# converts `0x00..=0xFF` to `'00'..='FF'`. see also `!hex_putc.min`
u8.to_hex!
  .loop .break sw2 ld0 x04 rot
  # stack is now `n >> 4, n, &loop, &break`
  loop. x0F and clc !u4.to_hex sw2 !jmp break.
# converts `'00'..='FF'` to `0x00..=0xFF`
hex.to_u8!
  @error # to be implemented
# converts `0x00..=0xFF` to a sequence of digits `'0'..='9'`. converts `0x00` to `'0'`
u8.to_dec!
  while_value.
    # (div_10, mod_10) = (value / 10, value % 10)
    !divmod_10
    # char = '0' + mod_10
    !'0' ad2 # bleeds `char`
  # loop while `div_10 != 0`
  !z .while_value !bcc !u8.pop
# converts an unspecified number of digits `'0'..='9'` to `0x00..=0xFF`
dec.to_u8!
  @error # to be implemented

char.check_null! !z
char.is_null! !zr

# converts `'A'..='Z'` to `'a'..='z'`. leaves other values unchanged
char.to_lower!
  !'A' sub clc
  !char.capital_letter_count sub @dyn
    !char.capital_letter_count !'A' add @const
    !char.capital_letter_count !'a' add dec @const
  iff add
# converts `'a'..='z'` to `'A'..='Z'`. leaves other values unchanged
char.to_upper!
  !'a' sub clc
  !char.small_letter_count sub @dyn
    !char.small_letter_count !'a' add @const
    !char.small_letter_count !'A' add dec @const
  iff add

u8.mul.def!
  u8.mul: clc # u16 product = u8.mul(u8 a, u8 b)
    !0u16 # product
    x08 for_bit. dec
      # b >>= 1
      !u8.ld3+2 !u8.shr !u8.st3+2
      # product += CF ? a : 0
      !u8.ld1+1 !0u8 !u8.ld4+2 !u8.iff clc !u8.add
      # product >>= 1 (stairstep shift)
      !u8.ld1+1 !u16.shr !u16.st0+1
    !z .for_bit !bcc pop
  # return* product
  !u16.st0+1 !rt0
i8.mul.def!
  i8.mul: clc # i16 product = i8.mul(i8 a, i8 b)
    # u16 product = (u8)a * (u8)b
    !u16.ld0+1 !u8.mul
    # product -= (a & 0x80 ? b << 0x08 : 0x00)
    ld3 !ng !0u16 !u8.ld5+1 !0u8 !u16.iff clc !u16.sub
    # product -= (b & 0x80 ? a << 0x08 : 0x00)
    ld4 !ng !0u16 !u8.ld4+1 !0u8 !u16.iff clc !u16.sub
  # return* product
  !u16.st0+1 !rt0
u16.mul.def!
  u16.mul: clc # u32 product = u16.mul(u16 a, u16 b)
    !0u32 # product
    x10 for_bit. dec
      # b >>= 1
      !u16.ld3+2 !u16.shr !u16.st3+2
      # product += CF ? a : 0
      !u16.ld1+1 !0u16 !u16.ld4+2 !u16.iff clc !u16.add
      # product >>= 1 (stairstep shift)
      !u16.ld1+1 !u32.shr !u32.st0+1
    !z .for_bit !bcc pop
  # return* product
  !u32.st0+1 !rt0
i16.mul.def!
  i16.mul: clc # i32 product = i16.mul(i16 a, i16 b)
    # u32 product = (u16)a * (u16)b
    !u32.ld0+1 !u16.mul
    # product -= (a & 0x8000 ? b << 0x10 : 0x00)
    ld6 !ng !0u32 !u16.ld5+1 !0u16 !u32.iff clc !u32.sub
    # product -= (b & 0x8000 ? a << 0x10 : 0x00)
    ld8 !ng !0u32 !u16.ld4+1 !0u16 !u32.iff clc !u32.sub
  # return* product
  !u32.st0+1 !rt0
u4f4.mul.def!
  u4f4.mul: clc # u4f4 product = u4f4.mul(u4f4 a, u4f4 b)
    # product = (u8)a * (u8)b
    !u16.ld0+1 !u8.mul
    # product >>= 0x04
    x0F xF0 an2 an2 orr x04 rot !u8.st1+1
  # return* product
  !rt1
u8f8.mul.def!
  u8f8.mul: clc # u8f8 product = u8f8.mul(u8f8 a, u8f8 b)
    # product = (u16)a * (u16)b
    !u32.ld0+1 !u16.mul
    # product >>= 0x08
    pop !u8.st4+1 !u8.st4+1 pop
  # return* product
  !rt2
i4f4.mul.def!
  i4f4.mul: clc # i4f4 product = i4f4.mul(i4f4 a, i4f4 b)
    # product = (i8)a * (i8)b
    !i16.ld0+1 !i8.mul
    # product >>= 0x04
    x0F xF0 an2 an2 orr x04 rot !i8.st1+1
  # return* product
  !rt1
i8f8.mul.def!
  i8f8.mul: clc # i8f8 product = i8f8.mul(i8f8 a, i8f8 b)
    # product = (i16)a * (i16)b
    !i32.ld0+1 !i16.mul
    # product >>= 0x08
    pop !i8.st4+1 !i8.st4+1 pop
  # return* product
  !rt2
