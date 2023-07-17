prng_bits! x1D

# outputs every number in 0x00..=0xFF then repeats
prng! clc # seed = prng(seed)
  shl x00 !prng_bits iff swp
  buf x00 !prng_bits iff
  xor xor

# outputs every number in 0x01..=0xFF then repeats
# will never output 0x00 (if and only if seed is not 0x00)
# seed must never be 0x00 (otherwise will only output 0x00)
prng_minimal! clc # seed = prng_minimal(seed)
  shl x00 !prng_bits iff xor

stall! # stall(iterations)
  loop. x01 sub @dyn .loop !bcc pop

stall_long! # stall_long(iterations)
  x00 loop. x00 x01 su2 su2 .loop !bcc pop pop

popcnt! # count = popcnt(a)
  # count = a == 0 ? -1 : 0
  buf @dyn x00 xFF iff
  # do { count++ } while (a != 0)
  while. inc
    # a &= a - 1 (unsets lowest set bit)
    ld1 dec an2
  .while !bcc
  # return* count
  st0
