# outputs every number in 0x00..=0xFF then repeats
prng!
  prng: # rand = prng(&seed)
    ld1 lda
    buf .do_xor !bcs
    x01 shf .no_xor !bcc
    buf .no_xor !bcs
    do_xor. x1D xor
    no_xor. ld2 ld1 sta
  st1 !rt0

# outputs every number in 0x01..=0xFF then repeats
# will never output 0x00
prng_minimal!
  prng: # rand = prng(&seed)
    ld1 lda
    x01 shf .no_xor !bcc x1D xor
    no_xor. ld2 ld1 sta
  st1 !rt0
