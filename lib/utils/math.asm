# outputs every number in 0x01..=0xFF then repeats
# will never output 0x00
prng_minimal!
seed. d80
prng:
.seed lda
x01 shf .no_xor !bcc x1D xor
no_xor. .seed ld1 sta
swp !ret

# outputs every number in 0x00..=0xFF then repeats
prng_full!
seed. d00
prng:
.seed lda
buf .do_xor !bcs
x01 shf .no_xor !bcc
buf .no_xor !bcs
do_xor. x1D xor
no_xor. .seed ld1 sta
swp !ret
