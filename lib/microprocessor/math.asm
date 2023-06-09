prng_bits! x1D

# outputs every number in 0x00..=0xFF then repeats
prng! clc # seed = prng(seed)
  shl x00 !prng_bits iff swp
  buf x00 !prng_bits iff
  xor xor

# outputs every number in 0x01..=0xFF then repeats
# will never output 0x00
# seed must never be 0x00
prng_minimal! clc # seed = prng_minimal(seed)
  shl x00 !prng_bits iff xor

u8! # u8 n = u8(u8 n)
u16! # u16 n = u16(u16 n)
u32! # u32 n = u16(u32 n)
u64! # u64 n = u16(u64 n)
i8! !u8 # i8 n = i8(i8 n)
i16! !u16 # i16 n = i16(i16 n)
i32! !u32 # i32 n = i16(i32 n)
i64! !u64 # i64 n = i16(u64 n)
u4f4! !u8 # u4f4 n = u4f4(u4 fr, u4 in)
u8f8! !u16 # u8f8 n = u8f8(u8 fr, u8 in)
i4f4! !i8 # i4f4 n = i4f4(u4 fr, i4 in)
i8f8! !i16 # i8f8 n = i8f8(u8 fr, i8 in)
c4f4! !i16 # c4f4 n = c4f4(i4f4 im, i4f8 re)
c8f8! !i32 # c8f8 n = c8f8(i8f8 im, i8f8 re)

u8.0! x00
u16.0! x00 x00
u32.0! x00 x00 x00 x00
u64.0! x00 x00 x00 x00 x00 x00 x00 x00
i8.0! !u8.0
i16.0! !u16.0
i32.0! !u32.0
i64.0! !u64.0
u4f4.0! !u8.0
u8f8.0! !u16.0
i4f4.0! !i8.0
i8f8.0! !i16.0
c4f4.0! !i16.0
c8f8.0! !i32.0

u8.add! add
u16.add! ad2 ad2
u32.add! ad4 ad4 ad4 ad4
u64.add! ad8 ad8 ad8 ad8 ad8 ad8 ad8 ad8
i8.add! !u8.add
i16.add! !u16.add
i32.add! !u32.add
i64.add! !u64.add
u4f4.add! !u8.add
u8f8.add! !u16.add
i4f4.add! !i8.add
i8f8.add! !i16.add
c4f4.add! ad4 ad4 clc ad4 ad4
c8f8.add! ad4 ad4 clc ad4 ad4

u8.sub! sub
u16.sub! su2 su2
u32.sub! su4 su4 su4 su4
u64.sub! su8 su8 su8 su8 su8 su8 su8 su8
i8.sub! !u8.sub
i16.sub! !u16.sub
i32.sub! !u32.sub
i64.sub! !u64.sub
u4f4.sub! !u8.sub
u8f8.sub! !u16.sub
i4f4.sub! !i8.sub
i8f8.sub! !i16.sub
c4f4.sub! su4 su4 clc su4 su4
c8f8.sub! su4 su4 clc su4 su4

u8.iff! iff
u16.iff! if2 if2
u32.iff! if4 if4 if4 if4
u64.iff! if8 if8 if8 if8 if8 if8 if8 if8
i8.iff! !u8.iff
i16.iff! !u16.iff
i32.iff! !u32.iff
i64.iff! !u64.iff
u4f4.iff! !u8.iff
u8f8.iff! !u16.iff
i4f4.iff! !i8.iff
i8f8.iff! !i16.iff
c4f4.iff! !i16.iff
c8f8.iff! !i32.iff

u8.pop! pop
u16.pop! pop pop
u32.pop! pop pop pop pop
u64.pop! pop pop pop pop pop pop pop pop
i8.pop! !u8.pop
i16.pop! !u16.pop
i32.pop! !u32.pop
i64.pop! !u64.pop
u4f4.pop! !u8.pop
u8f8.pop! !u16.pop
i4f4.pop! !i8.pop
i8f8.pop! !i16.pop
c4f4.pop! !i16.pop
c8f8.pop! !i32.pop

u8.shl! shl
u16.shl! shl ld1 shl st1
u32.shl! shl ld1 shl st1 ld2 shl st2 ld3 shl st3 ld4 shl st4
u64.shl! shl ld1 shl st1 ld2 shl st2 ld3 shl st3 ld4 shl st4 ld5 shl st5 ld6 shl st6 ld7 shl st7
u4f4.shl! !u8.shl
u8f8.shl! !u16.shl

u8.shr! shr
u16.shr! ld1 shr st1 shr
u32.shr! ld3 shr st3 ld2 shr st2 ld1 shr st1 shr
u64.shr! ld7 shr st7 ld6 shr st6 ld5 shr st5 ld4 shr st4 ld3 shr st3 ld2 shr st2 ld1 shr st1 shr
u4f4.shr! !u8.shr
u8f8.shr! !u16.shr

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
c4f4.ld0!   !i16.ld0
c4f4.ld0+1! !i16.ld0+1
c4f4.ld0+2! !i16.ld0+2
c4f4.ld0+3! !i16.ld0+3
c4f4.ld1!   !i16.ld1
c4f4.ld1+1! !i16.ld1+1
c4f4.ld1+2! !i16.ld1+2
c4f4.ld1+3! !i16.ld1+3
c4f4.ld2!   !i16.ld2
c4f4.ld2+1! !i16.ld2+1
c4f4.ld2+2! !i16.ld2+2
c4f4.ld2+3! !i16.ld2+3
c4f4.ld3!   !i16.ld3
c4f4.ld3+1! !i16.ld3+1
c4f4.ld3+2! !i16.ld3+2
c4f4.ld3+3! !i16.ld3+3
c4f4.ld4!   !i16.ld4
c4f4.ld4+1! !i16.ld4+1
c4f4.ld4+2! !i16.ld4+2
c4f4.ld4+3! !i16.ld4+3
c4f4.ld5!   !i16.ld5
c4f4.ld5+1! !i16.ld5+1
c4f4.ld5+2! !i16.ld5+2
c4f4.ld5+3! !i16.ld5+3
c4f4.ld6!   !i16.ld6
c4f4.ld6+1! !i16.ld6+1
c4f4.ld6+2! !i16.ld6+2
c4f4.ld7!   !i16.ld7
c8f8.ld0!   !i32.ld0
c8f8.ld0+1! !i32.ld0+1
c8f8.ld0+2! !i32.ld0+2
c8f8.ld0+3! !i32.ld0+3
c8f8.ld1!   !i32.ld1
c8f8.ld1+1! !i32.ld1+1
c8f8.ld1+2! !i32.ld1+2
c8f8.ld1+3! !i32.ld1+3
c8f8.ld2!   !i32.ld2
c8f8.ld2+1! !i32.ld2+1
c8f8.ld2+2! !i32.ld2+2
c8f8.ld2+3! !i32.ld2+3
c8f8.ld3!   !i32.ld3

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
c4f4.st0!   !i16.st0
c4f4.st0+1! !i16.st0+1
c4f4.st0+2! !i16.st0+2
c4f4.st0+3! !i16.st0+3
c4f4.st1!   !i16.st1
c4f4.st1+1! !i16.st1+1
c4f4.st1+2! !i16.st1+2
c4f4.st1+3! !i16.st1+3
c4f4.st2!   !i16.st2
c4f4.st2+1! !i16.st2+1
c4f4.st2+2! !i16.st2+2
c4f4.st2+3! !i16.st2+3
c4f4.st3!   !i16.st3
c4f4.st3+1! !i16.st3+1
c4f4.st3+2! !i16.st3+2
c4f4.st3+3! !i16.st3+3
c4f4.st4!   !i16.st4
c4f4.st4+1! !i16.st4+1
c4f4.st4+2! !i16.st4+2
c4f4.st4+3! !i16.st4+3
c4f4.st5!   !i16.st5
c4f4.st5+1! !i16.st5+1
c4f4.st5+2! !i16.st5+2
c4f4.st5+3! !i16.st5+3
c4f4.st6!   !i16.st6
c4f4.st6+1! !i16.st6+1
c4f4.st6+2! !i16.st6+2
c4f4.st7!   !i16.st7
c8f8.st0!   !i32.st0
c8f8.st0+1! !i32.st0+1
c8f8.st0+2! !i32.st0+2
c8f8.st0+3! !i32.st0+3
c8f8.st1!   !i32.st1
c8f8.st1+1! !i32.st1+1
c8f8.st1+2! !i32.st1+2
c8f8.st1+3! !i32.st1+3
c8f8.st2!   !i32.st2
c8f8.st2+1! !i32.st2+1
c8f8.st2+2! !i32.st2+2
c8f8.st2+3! !i32.st2+3
c8f8.st3!   !i32.st3

u4f4.in! x04 rot x0F and # u8 integer_part = u4f4.in(u4f4 n)
u4f4.fr! x0F and # u8 integer_part = u4f4.in(u4f4 n)
u8f8.in! !u8.pop # u8 integer_part = u8f8.in(u8f8 n)
u8f8.fr! !u8.st0 # u8 fraction_part = u8f8.fr(u8f8 n)
i4f4.in! x04 rot x0F and # i8 integer_part = i4f4.in(i4f4 n)
i4f4.fr! x0F and # u8 integer_part = i4f4.in(i4f4 n)
i8f8.in! !u8.pop # i8 integer_part = i8f8.in(i8f8 n)
i8f8.fr! !u8.st0 # u8 fraction_part = i8f8.fr(i8f8 n)
c8f8.re! !u16.pop # i8f8 real_part = c8f8.re(c8f8 c)
c8f8.im! !u16.st0 # i8f8 imaginary_part = c8f8.im(c8f8 c)

u8.mul! :u8.mul !call # u16 product = u8.mul(u8 a, u8 b)
i8.mul! :i8.mul !call # u16 product = u8.mul(u8 a, u8 b)
u16.mul! :u16.mul !call # u32 product = u16.mul(u16 a, u16 b)
i16.mul! :i16.mul !call # i32 product = i16.mul(i16 a, i16 b)
u4f4.mul! :u4f4.mul !call # u4f4 product = u4f4.mul(u4f4 a, u4f4 b)
u8f8.mul! :u8f8.mul !call # u8f8 product = u8f8.mul(u8f8 a, u8f8 b)
i4f4.mul! :i4f4.mul !call # i4f4 product = i4f4.mul(i4f4 a, i4f4 b)
i8f8.mul! :i8f8.mul !call # i8f8 product = i8f8.mul(i8f8 a, i8f8 b)
c4f4.mul! # c4f4 product = c4f4.mul(c4f4 a, c4f4 b)
  !i4f4.ld3 !i4f4.ld2 !i4f4.mul !i4f4.ld3 !i4f4.ld2 !i4f4.mul !i4f4.sub # real part
  !i4f4.ld4 !i4f4.ld2 !i4f4.mul !i4f4.ld4 !i4f4.ld4 !i4f4.mul !i4f4.add # imaginary part
  !i16.st1 !i16.pop
c8f8.mul! # c8f8 product = c8f8.mul(c8f8 a, c8f8 b)
  !i8f8.ld3 !i8f8.ld2 !i8f8.mul !i8f8.ld3 !i8f8.ld2 !i8f8.mul !i8f8.sub # real part
  !i8f8.ld4 !i8f8.ld2 !i8f8.mul !i8f8.ld4 !i8f8.ld4 !i8f8.mul !i8f8.add # imaginary part
  !i32.st1 !i32.pop

c4f4.norm! !i4f4.ld1 !i4f4.ld0 !i4f4.mul !i4f4.st1 !i4f4.ld0 !i4f4.mul !i4f4.add # i4f4 norm = c4f4.norm(c4f4 c)
c8f8.norm! !i8f8.ld1 !i8f8.ld0 !i8f8.mul !i8f8.st1 !i8f8.ld0 !i8f8.mul !i4f4.add # i8f8 norm = c8f8.norm(c8f8 c)

u8.mul_def!
  u8.mul: clc # u16 product = u8.mul(u8 a, u8 b)
    !u16.0 # product
    x08 for_bit. dec
      # b >>= 1
      !u8.ld3+2 !u8.shr !u8.st3+2
      # product += CF ? a : 0
      !u8.ld1+1 !u8.0 !u8.ld4+2 !u8.iff clc !u8.add
      # product >>= 1 (stairstep shift)
      !u8.ld1+1 !u16.shr !u16.st0+1
    buf .for_bit !bcc pop
  # return* product
  !u16.st0+1 !rt0
i8.mul_def!
  i8.mul: clc # i16 product = i8.mul(i8 a, i8 b)
    # u16 product = (u8)a * (u8)b
    !u16.ld0+1 !u8.mul
    # product -= (a & 0x80 ? b << 0x08 : 0x00)
    ld3 shl pop !u16.0 !u8.ld5+1 !u8.0 !u16.iff clc !u16.sub
    # product -= (b & 0x80 ? a << 0x08 : 0x00)
    ld4 shl pop !u16.0 !u8.ld4+1 !u8.0 !u16.iff clc !u16.sub
  # return* product
  !u16.st0+1 !rt0
u16.mul_def!
  u16.mul: clc # u32 product = u16.mul(u16 a, u16 b)
    !u32.0 # product
    x10 for_bit. dec
      # b >>= 1
      !u16.ld3+2 !u16.shr !u16.st3+2
      # product += CF ? a : 0
      !u16.ld1+1 !u16.0 !u16.ld4+2 !u16.iff clc !u16.add
      # product >>= 1 (stairstep shift)
      !u16.ld1+1 !u32.shr !u32.st0+1
    buf .for_bit !bcc pop
  # return* product
  !u32.st0+1 !rt0
i16.mul_def!
  i16.mul: clc # i32 product = i16.mul(i16 a, i16 b)
    # u32 product = (u16)a * (u16)b
    !u32.ld0+1 !u16.mul
    # product -= (a & 0x8000 ? b << 0x10 : 0x00)
    ld6 shl pop !u32.0 !u16.ld5+1 !u16.0 !u32.iff clc !u32.sub
    # product -= (b & 0x8000 ? a << 0x10 : 0x00)
    ld8 shl pop !u32.0 !u16.ld4+1 !u16.0 !u32.iff clc !u32.sub
  # return* product
  !u32.st0+1 !rt0
u4f4.mul_def!
  u4f4.mul: clc # u4f4 product = u4f4.mul(u4f4 a, u4f4 b)
    # product = (u8)a * (u8)b
    !u16.ld0+1 !u8.mul
    # product >>= 0x04
    x0F xF0 an2 an2 orr x04 rot !u8.st1+1
  # return* product
  !rt1
u8f8.mul_def!
  u8f8.mul: clc # u8f8 product = u8f8.mul(u8f8 a, u8f8 b)
    # product = (u16)a * (u16)b
    !u32.ld0+1 !u16.mul
    # product >>= 0x08
    pop !u8.st4+1 !u8.st4+1 pop
  # return* product
  !rt2
i4f4.mul_def!
  i4f4.mul: clc # i4f4 product = i4f4.mul(i4f4 a, i4f4 b)
    # product = (i8)a * (i8)b
    !i16.ld0+1 !i8.mul
    # product >>= 0x04
    x0F xF0 an2 an2 orr x04 rot !i8.st1+1
  # return* product
  !rt1
i8f8.mul_def!
  i8f8.mul: clc # i8f8 product = i8f8.mul(i8f8 a, i8f8 b)
    # product = (i16)a * (i16)b
    !i32.ld0+1 !i16.mul
    # product >>= 0x08
    pop !i8.st4+1 !i8.st4+1 pop
  # return* product
  !rt2
