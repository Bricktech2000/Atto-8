load_nibble!
  load_nibble: # nibble = load_nibble(index, buffer)
    # addr = index // 2 + buffer
    ld2 ld2 x01 !shr add
    # rot = 4 * (~index % 2)
    ld2 not x01 and x02 shf
    # nibble = (*addr >> rot) & 0x0F
    swp lda swp !shr x0F and
  # return* nibble
  st2
  !rt1

store_nibble!
  store_nibble: # store_nibble(index, buffer, nibble)
    # addr = index // 2 + buffer
    ld2 ld2 x01 !shr add
    # rot = 4 * (~index % 2)
    ld2 not x01 and x02 shf
    # rest = *addr & ~(0x0F << rot)
    ld1 lda x0F ld2 shf not and
    # new = rest | (nibble << rot)
    swp ld6 swp shf orr
    # *addr = new
    sta
  # return*
  !rt3
