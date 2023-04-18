bit_addr! # (rot, addr) = bit_addr(index, buffer)
  # addr = index // 8 + buffer
  swp ld1 x03 !shr add
  # rot = ~index % 8
  swp not x07 and
  # return* (rot, addr)

load_bit! # bit = load_bit(rot, addr)
  # bit = (*addr >> rot) & 0x01
  swp lda swp !ror x01 and
  # return* bin

store_bit! # store_bit(rot, addr, bit)
  # rest = *addr & ~(0x01 << rot)
  ld1 lda x01 ld2 shf not and
  # new = rest | (bit << rot)
  swp ld3 swp shf orr
  # *addr = new
  sta
  # return*
  pop

set_bit! # set_bit(rot, addr)
  # new = *addr | (0x01 << rot)
  x01 swp shf ld1 lda orr
  # *addr = new
  sta
  # return*

clear_bit! # clear_bit(rot, addr)
  # new = *addr & ~(0x01 << rot)
  x01 swp shf not ld1 lda and
  # *addr = new
  sta
  # return*

flip_bit! # flip_bit(rot, addr)
  # new = *addr ^ (0x01 << rot)
  x01 swp shf ld1 lda xor
  # *addr = new
  sta
  # return*
