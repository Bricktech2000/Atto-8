display_buffer! xE0 @const
display_buffer_len! x20 @const

bit_addr! # (rot, addr) = bit_addr(buffer, index)
  # addr = index // 8 + buffer
  ld1 x07 not and clc x05 rot add
  # rot = ~index % 8
  swp not x07 and
  # return* (rot, addr)
load_bit! # bit = load_bit(rot, addr)
  # bit = (*addr >> rot) & 0x01
  !rneg ld1 lda st1 rot x01 and
  # return* bit
store_bit! # store_bit(rot, addr, bit)
  # bit <<= rot
  x01 ld1 ro4
  # rest = ~(0x01 << rot) & *addr
  swp rot not ld1 lda and
  # *addr = rest | bit
  or2 sta
  # return*
set_bit! # set_bit(rot, addr)
  # mask = 0x01 << rot
  x01 swp rot swp
  # *addr |= mask
  ld0 lda or2 sta
  # return*
clear_bit! # clear_bit(rot, addr)
  # mask = ~0x01 << rot
  x01 not swp rot swp
  # *addr &= mask
  ld0 lda an2 sta
  # return*
flip_bit! # flip_bit(rot, addr)
  # mask = 0x01 << rot
  x01 swp rot swp
  # *addr ^= mask
  ld0 lda xo2 sta
  # return*

nibble_addr! # (rot, addr) = nibble_addr(buffer, index)
  # addr = index // 2 + buffer
  clc ld1 shr clc add
  # rot = 4 * (~index % 2)
  swp not x01 and x02 rot
  # return*
load_nibble! # nibble = load_nibble(rot, addr)
  # nibble = (*addr >> rot) & 0x0F
  !rneg ld1 lda st1 rot x0F and
  # return* nibble
store_nibble! # store_nibble(rot, addr, nibble)
  # nibble <<= rot
  x0F ld1 ro4
  # rest = ~(0x0F << rot) & *addr
  swp rot not ld1 lda and
  # *addr = rest | nibble
  or2 sta
  # return*
set_nibble! # set_nibble(rot, addr)
  # mask = 0x0F << rot
  x0F swp rot swp
  # *addr |= mask
  ld0 lda or2 sta
  # return*
clear_nibble! # clear_nibble(rot, addr)
  # mask = ~0x0F << rot
  x0F not swp rot swp
  # *addr &= mask
  ld0 lda an2 sta
  # return*
flip_nibble! # flip_nibble(rot, addr)
  # mask = 0x0F << rot
  x0F swp rot swp
  # *addr ^= mask
  ld0 lda xo2 sta
  # return*

print_char.def!
  print_char: # print_char(index, buffer, pos)
    x03 for_i. dec
      # src = i + index * 3
      ld0 ld3 ld0 x01 rot clc add clc add
      # nibble = load_nibble(nibble_addr(buffer, src))
      ld4 !nibble_addr !load_nibble
      # dst = i * 4 + pos // 4 * 16 + pos % 4
      ld1 x02 rot ld6 x02 rot xF0 and clc add ld6 x03 and clc add
      # store_nibble(nibble_addr(&DISPLAY_BUFFER, dst), nibble)
      !display_buffer !nibble_addr !store_nibble
    buf .for_i !bcc pop
  # return*
  !rt3

print_byte.def!
  hex_chars.
  # t m b t m b
    @EA @EC @4E # 0 1
    @C4 @6E @6E # 2 3
    @AE @26 @4C # 4 5
    @8E @EE @22 # 6 7
    @6E @EE @E2 # 8 9
    @4E @AC @EE # A B
    @E8 @EC @AC # C D
    @EC @EE @C8 # E F
  print_byte: # print_byte(byte, pos)
    ld2 inc .hex_chars ld3 !u4u4.snd :print_char !call
    ld2 .hex_chars ld3 !u4u4.fst :print_char !call
  # return*
  !rt2

print_byte.min.def!
  hex_chars.
  # 0 1 2 3 4 5 6 7 8 9 A B C D E F
    @EC @CE @A6 @8E @6E @4C @EC @EE # top row
    @A4 @46 @E4 @E2 @EE @EE @8A @CC # middle row
    @EE @6E @2C @E2 @E2 @AE @EC @E8 # bottom row
  print_byte.min: clc # print_byte.min(byte, addr)
    # loop through rows
    x03 for_row. dec
      # nth_row = hex_chars + row * 8
      ld0 x03 rot .hex_chars add
      # LSN = load_nibble(nibble_addr(nth_row, u4u4.snd(byte))
      ld3 !u4u4.snd ld1 !nibble_addr !load_nibble
      # MSN = load_nibble(nibble_addr(nth_row, u4u4.fst(byte))
      ld4 !u4u4.fst ld2 !nibble_addr !load_nibble
      # row = (MSN << 4) | LSN
      x04 rot orr st0
      # dst = &DISPLAY_BUFFER + addr + row * 2
      !display_buffer ld5 add ld2 x01 rot add
      # *dst = row
      sta
    buf .for_row !bcc pop
  # return*
  !rt2
