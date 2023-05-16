memset_def!
  memset: # memset(ptr, val, len)
    ld3 for_i. dec
      ld0 ld3 add ld4 sta
    buf .for_i !bcc pop
  !rt3

memcpy_def!
  memcpy: clc # memcpy(dst, src, len)
    ld3 for_i. dec
      ld0 ld3 add
      ld1 ld5 add
      lda sta
    buf .for_i !bcc pop
  !rt3


bit_addr! clc # (rot, addr) = bit_addr(index, buffer)
  # addr = index // 8 + buffer
  swp ld1 x03 !ror add
  # rot = ~index % 8
  swp not x07 and
  # return* (rot, addr)

load_bit! # bit = load_bit(rot, addr)
  # bit = (*addr >> rot) & 0x01
  swp lda swp !ror x01 and
  # return* bin

store_bit! # store_bit(rot, addr, bit)
  # rest = *addr & ~(0x01 << rot)
  ld1 lda x01 ld2 rot not and
  # new = rest | (bit << rot)
  swp ld3 swp rot orr
  # *addr = new
  sta
  # return*
  pop

set_bit! # set_bit(rot, addr)
  # new = *addr | (0x01 << rot)
  x01 swp rot ld1 lda orr
  # *addr = new
  sta
  # return*

clear_bit! # clear_bit(rot, addr)
  # new = *addr & ~(0x01 << rot)
  x01 swp rot not ld1 lda and
  # *addr = new
  sta
  # return*

flip_bit! # flip_bit(rot, addr)
  # new = *addr ^ (0x01 << rot)
  x01 swp rot ld1 lda xor
  # *addr = new
  sta
  # return*


nibble_addr! clc # (rot, addr) = nibble_addr(index, buffer)
  # addr = index // 2 + buffer
  swp ld1 x01 !ror add
  # rot = 4 * (~index % 2)
  swp not x01 and x02 rot
  # return*

load_nibble! # nibble = load_nibble(rot, addr)
  # nibble = (*addr >> rot) & 0x0F
  swp lda swp !ror x0F and
  # return* nibble

store_nibble! # store_nibble(rot, addr, nibble)
  # rest = *addr & ~(0x0F << rot)
  ld1 lda x0F ld2 rot not and
  # new = rest | (nibble << rot)
  swp ld3 swp rot orr
  # *addr = new
  sta
  # return*
  pop

set_nibble! # set_nibble(rot, addr)
  # new = *addr | (0x0F << rot)
  x0F swp rot ld1 lda orr
  # *addr = new
  sta
  # return*

clear_nibble! # clear_nibble(rot, addr)
  # new = *addr & ~(0x0F << rot)
  x0F swp rot not ld1 lda and
  # *addr = new
  sta
  # return*

flip_nibble! # flip_nibble(rot, addr)
  # new = *addr ^ (0x0F << rot)
  x0F swp rot ld1 lda xor
  # *addr = new
  sta
  # return*
