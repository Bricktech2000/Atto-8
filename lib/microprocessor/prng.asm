prng_bits! x1D

# outputs every number in 0x00..=0xFF then repeats
prng! # seed = prng(seed)
  x01 shf x00 !prng_bits iff
  swp buf x00 !prng_bits iff
  xor xor

# outputs every number in 0x01..=0xFF then repeats
# will never output 0x00
# seed must never be 0x00
prng_minimal! # seed = prng_minimal(seed)
  x01 shf x00 !prng_bits iff xor
