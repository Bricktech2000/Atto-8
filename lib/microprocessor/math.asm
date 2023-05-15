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

u8! # x = u8(x)
u16! # x = u16(x)
u32! # x = u16(x)
u64! # x = u16(x)

u8.0! x00
u16.0! x00 x00
u32.0! x00 x00 x00 x00
u64.0! x00 x00 x00 x00 x00 x00 x00 x00

u8.add! add
u16.add! ad2 ad2
u32.add! ad4 ad4 ad4 ad4
u64.add! ad8 ad8 ad8 ad8 ad8 ad8 ad8 ad8

u8.sub! sub
u16.sub! su2 su2
u32.sub! su4 su4 su4 su4
u64.sub! su8 su8 su8 su8 su8 su8 su8 su8

u8.iff! iff
u16.iff! if2 if2
u32.iff! if4 if4 if4 if4
u64.iff! if8 if8 if8 if8 if8 if8 if8 if8

u8.ld0! ld0
u8.ld1! ld1
u8.ld2! ld2
u8.ld3! ld3
u8.ld4! ld4
u8.ld5! ld5
u8.ld6! ld6
u8.ld7! ld7
u16.ld0! ld1 ld1
u16.ld0.8! ld2 ld2
u16.ld1! ld3 ld3
u16.ld1.8! ld4 ld4
u16.ld2! ld5 ld5
u16.ld2.8! ld6 ld6
u16.ld3! ld7 ld7
u16.ld3.8! ld8 ld8
u16.ld4! ld9 ld9
u16.ld4.8! ldA ldA
u16.ld5! ldB ldB
u16.ld5.8! ldC ldC
u16.ld6! ldD ldD
u16.ld.8! ldE ldE
u16.ld7! ldF ldF
u32.ld0! ld3 ld3 ld3 ld3
u32.ld0.4! ld4 ld4 ld4 ld4
u32.ld0.8! ld5 ld5 ld5 ld5
u32.ld0.C! ld6 ld6 ld6 ld6
u32.ld1! ld7 ld7 ld7 ld7
u32.ld1.4! ld8 ld8 ld8 ld8
u32.ld1.8! ld9 ld9 ld9 ld9
u32.ld1.C! ldA ldA ldA ldA
u32.ld2! ldB ldB ldB ldB
u32.ld2.4! ldC ldC ldC ldC
u32.ld2.8! ldD ldD ldD ldD
u32.ld2.C! ldE ldE ldE ldE
u32.ld3! ldF ldF ldF ldF
u64.ld0! ld7 ld7 ld7 ld7 ld7 ld7 ld7 ld7
u64.ld0.2! ld8 ld8 ld8 ld8 ld8 ld8 ld8 ld8
u64.ld0.4! ld9 ld9 ld9 ld9 ld9 ld9 ld9 ld9
u64.ld0.6! ldA ldA ldA ldA ldA ldA ldA ldA
u64.ld0.8! ldB ldB ldB ldB ldB ldB ldB ldB
u64.ld0.A! ldC ldC ldC ldC ldC ldC ldC ldC
u64.ld0.C! ldD ldD ldD ldD ldD ldD ldD ldD
u64.ld0.E! ldE ldE ldE ldE ldE ldE ldE ldE
u64.ld1! ldF ldF ldF ldF ldF ldF ldF ldF

u8.st0! st0
u8.st1! st1
u8.st2! st2
u8.st3! st3
u8.st4! st4
u8.st5! st5
u8.st6! st6
u8.st7! st7
u16.st0! st1 st1
u16.st0.8! st2 st2
u16.st1! st3 st3
u16.st1.8! st4 st4
u16.st2! st5 st5
u16.st2.8! st6 st6
u16.st3! st7 st7
u16.st3.8! st8 st8
u16.st4! st9 st9
u16.st4.8! stA stA
u16.st5! stB stB
u16.st5.8! stC stC
u16.st6! stD stD
u16.st.8! stE stE
u16.st7! stF stF
u32.st0! st3 st3 st3 st3
u32.st0.4! st4 st4 st4 st4
u32.st0.8! st5 st5 st5 st5
u32.st0.C! st6 st6 st6 st6
u32.st1! st7 st7 st7 st7
u32.st1.4! st8 st8 st8 st8
u32.st1.8! st9 st9 st9 st9
u32.st1.C! stA stA stA stA
u32.st2! stB stB stB stB
u32.st2.4! stC stC stC stC
u32.st2.8! stD stD stD stD
u32.st2.C! stE stE stE stE
u32.st3! stF stF stF stF
u64.st0! st7 st7 st7 st7 st7 st7 st7 st7
u64.st0.2! st8 st8 st8 st8 st8 st8 st8 st8
u64.st0.4! st9 st9 st9 st9 st9 st9 st9 st9
u64.st0.6! stA stA stA stA stA stA stA stA
u64.st0.8! stB stB stB stB stB stB stB stB
u64.st0.A! stC stC stC stC stC stC stC stC
u64.st0.C! stD stD stD stD stD stD stD stD
u64.st0.E! stE stE stE stE stE stE stE stE
u64.st1! stF stF stF stF stF stF stF stF

u8.shr! shr
u16.shr! ld1 shr st1 shr
u32.shr! ld3 shr st3 ld2 shr st2 ld1 shr st1 shr
u64.shr! ld7 shr st7 ld6 shr st6 ld5 shr st5 ld4 shr st4 ld3 shr st3 ld2 shr st2 ld1 shr st1 shr

u8.mul!
  u8.mul: # u16 r = u8.mul(u8 a, u8 b)
    !u16.0 # result
    x08 for_bit. dec
      # b >>= 1
      !u8.ld5 !u8.shr !u8.st5
      # r += CF ? a : 0
      !u8.ld2 !u8.0 !u8.ld6 !u8.iff clc !u8.add
      # r >>= 1 (stairstep shift)
      !u8.ld2 !u16.shr !u16.st0.8
    buf .for_bit !bcc pop
  # return* r
  !u16.st0.8 !ret
u16.mul!
  u16.mul: # u32 r = u16.mul(u16 a2 u16 b)
    !u32.0 # result
    x10 for_bit. dec
      # b >>= 1
      !u16.ld4 !u16.shr !u16.st4
      # r += CF ? a : 0
      !u16.ld1.8 !u16.0 !u16.ld5 !u16.iff clc !u16.add
      # r >>= 1 (stairstep shift)
      !u16.ld1.8 !u32.shr !u32.st0.4
    buf .for_bit !bcc pop
  !u32.st0.4 !ret
