display_buffer! xE0 @const

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

hex_chars_def!
  hex_chars:
  # t m b t m b
    dEA dEC d4E # 0 1
    dC4 d6E d6E # 2 3
    dAE d26 d4C # 4 5
    d8E dEE d22 # 6 7
    d6E dEE dE2 # 8 9
    d4E dAC dEE # A B
    dE8 dEC dAC # C D
    dEC dEE dC8 # E F

hex_chars_minimal_def!
  hex_chars_minimal:
  # 0 1 2 3 4 5 6 7 8 9 A B C D E F
    dEC dCE dA6 d8E d6E d4C dEC dEE # top row
    dA4 d46 dE4 dE2 dEE dEE d8A dCC # middle row
    dEE d6E d2C dE2 dE2 dAE dEC dE8 # bottom row


print_char_def!
  print_char: # print_char(index, buffer, pos)
    x03 for_i. dec
      # src = i + index * 3
      ld0 ld3 ld0 x01 rot clc add clc add
      # nibble = load_nibble(nibble_addr(src, buffer))
      ld4 swp !nibble_addr !load_nibble
      # dst = i * 4 + pos // 4 * 16 + pos % 4
      ld1 x02 rot ld6 x02 rot xF0 and clc add ld6 x03 and clc add
      # store_nibble(nibble_addr(dst, &DISPLAY_BUFFER), nibble)
      !display_buffer swp !nibble_addr !store_nibble
    buf .for_i !bcc pop
  # return*
  !rt3

print_byte_def!
  print_byte: # print_byte(byte, pos)
    ld2 inc :hex_chars ld3 x0F and :print_char !call
    ld2 :hex_chars ld3 x04 !ror x0F and :print_char !call
  # return*
  !rt2

print_byte_minimal_def!
  print_byte: clc # print_byte(byte, addr)
    # loop through rows
    x03 for_row. dec
      # nth_row = hex_chars_minimal + row * 8
      ld0 x03 rot :hex_chars_minimal add
      # MSN = load_nibble(nibble_addr(byte & 0x0F, nth_row))
      ld0 ld4 x0F and !nibble_addr !load_nibble
      # LSN = load_nibble(nibble_addr((byte >> 4) & 0x0F, nth_row))
      swp ld4 x04 !ror x0F and !nibble_addr !load_nibble
      # row = (MSN << 4) | LSN
      x04 rot orr
      # dst = &DISPLAY_BUFFER + addr + row * 2
      ld4 !display_buffer add ld2 x01 rot add
      # *dst = row
      swp sta
    buf .for_row !bcc pop
  # return*
  !rt2
