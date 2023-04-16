# outputs every number in 0x01..=0xFF then repeats
# will never output 0x00
prng_minimal!
  prng:
    !seed @const lda
    x01 shf .no_xor !bcc x1D xor
    no_xor. !seed @const ld1 sta
  swp !ret

# outputs every number in 0x00..=0xFF then repeats
prng!
  prng:
    !seed @const lda
    buf .do_xor !bcs
    x01 shf .no_xor !bcc
    buf .no_xor !bcs
    do_xor. x1D xor
    no_xor. !seed @const ld1 sta
  swp !ret
