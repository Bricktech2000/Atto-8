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
  hex_chars:
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
      # store_nibble(nibble_addr(dst, &FRONT_BUFFER), nibble)
      !front_buffer swp !nibble_addr !store_nibble
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
      # nth_row = hex_chars + row * 8
      ld0 x03 rot :hex_chars add
      # MSN = load_nibble(nibble_addr(byte & 0x0F, nth_row))
      ld0 ld4 x0F and !nibble_addr !load_nibble
      # LSN = load_nibble(nibble_addr((byte >> 4) & 0x0F, nth_row))
      swp ld4 x04 !ror x0F and !nibble_addr !load_nibble
      # row = (MSN << 4) | LSN
      x04 rot orr
      # dst = &FRONT_BUFFER + addr + row * 2
      ld4 !front_buffer add ld2 x01 rot add
      # *dst = row
      swp sta
    buf .for_row !bcc pop
  # return*
  !rt2
