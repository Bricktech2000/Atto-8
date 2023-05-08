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
