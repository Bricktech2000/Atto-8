print_char!
  print_char: # print_char(index, buffer, pos)
    x03 for_i. dec
      # src = i + index * 3
      ld0 ld3 ld0 x01 shf add add
      # nibble = load_nibble(nibble_addr(src, buffer))
      ld4 swp !nibble_addr !load_nibble
      # dst = i * 4 + pos // 4 * 8 + pos % 4
      ld1 x02 shf ld6 x02 !shr x04 shf add ld6 x03 and add
      # store_nibble(nibble_addr(dst, &FRONT_BUFFER), nibble)
      !front_buffer swp !nibble_addr !store_nibble
    buf .for_i !bcc pop
  # return*
  !rt3

print_byte!
  print_byte: # print_byte(byte, pos)
    ld2 inc :hex_chars ld3 x0F and :print_char !call
    ld2 :hex_chars ld3 x04 !shr :print_char !call
  # return*
  !rt2

print_byte_minimal!
  print_byte: # print_byte(byte, addr)
    # loop through rows
    x03 for_row. dec
      # nth_row = hex_chars + row * 8
      ld0 x03 shf :hex_chars add
      # MSN = load_nibble(nibble_addr(byte & 0x0F, nth_row))
      ld0 ld4 x0F and !nibble_addr !load_nibble
      # LSN = load_nibble(nibble_addr(byte >> 4, nth_row))
      swp ld4 x04 !shr !nibble_addr !load_nibble
      # row = (MSN << 4) | LSN
      x04 shf orr
      # dst = &FRONT_BUFFER + addr + row * 2
      ld4 !front_buffer add ld2 x01 shf add
      # *dst = row
      swp sta
    buf .for_row !bcc pop
  # return*
  !rt2
