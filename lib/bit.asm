bit_addr!
  # addr = index // 8 + buffer
  ld2 ld2 x03 !shr add
  # rot = ~index % 8
  ld2 not x07 and

load_bit!
  load_bit: # bit = get_bit(index, buffer)
    !bit_addr
    # bit = (*addr >> rot) & 0x01
    swp lda swp !ror x01 and
  # return* bin
  st2
  !rt1

store_bit!
  store_bit: # store_bit(index, buffer, bit)
    !bit_addr
    # rest = *addr & ~(0x01 << rot)
    ld1 lda x01 ld2 shf not and
    # new = rest | (bit << rot)
    swp ld6 swp shf orr
    # *addr = new
    sta
  # return*
  !rt3
