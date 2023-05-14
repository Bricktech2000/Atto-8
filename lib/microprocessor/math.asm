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

# order of operands is swapped

add_u8! clc add # clc ld1 add st0
add_u16! clc ld2 add st1 ld2 add st1
add_u32! clc ld4 add st3 ld4 add st3 ld4 add st3 ld4 add st3
add_u64! clc ld8 add st7 ld8 add st7 ld8 add st7 ld8 add st7 ld8 add st7 ld8 add st7 ld8 add st7 ld8 add st7

sub_u8! clc swp sub # clc ld1 sub st0
sub_u16! clc ld2 sub st1 ld2 sub st1
sub_u32! clc ld4 sub st3 ld4 sub st3 ld4 sub st3 ld4 sub st3
sub_u64! clc ld8 sub st7 ld8 sub st7 ld8 sub st7 ld8 sub st7 ld8 sub st7 ld8 sub st7 ld8 sub st7 ld8 sub st7

iff_u8! swp iff # ld1 iff st0
iff_u16! ld2 iff st1 ld2 iff st1
iff_u32! ld4 iff st3 ld4 iff st3 ld4 iff st3 ld4 iff st3
iff_u64! ld8 iff st7 ld8 iff st7 ld8 iff st7 ld8 iff st7 ld8 iff st7 ld8 iff st7 ld8 iff st7 ld8 iff st7

ld0_u8! ld0 ld1_u8! ld1 ld2_u8! ld2 ld3_u8! ld3 ld4_u8! ld4 ld5_u8! ld5 ld6_u8! ld6 ld7_u8! ld7
ld0_u16! ld1 ld1 ld1_u16! ld3 ld3 ld2_u16! ld5 ld5 ld3_u16! ld7 ld7 ld4_u16! ld9 ld9 ld5_u16! ldB ldB ld6_u16! ldD ldD ld7_u16! ldF ldF
ld0_u32! ld3 ld3 ld3 ld3 ld1_u32! ld7 ld7 ld7 ld7 ld2_u32! ldB ldB ldB ldB ld3_u32! ldF ldF ldF ldF
ld0_u64! ld7 ld7 ld7 ld7 ld7 ld7 ld7 ld7 ld1_u64! ldF ldF ldF ldF ldF ldF ldF ldF
