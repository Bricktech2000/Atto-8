nibble_addr!
  nibble_addr: # (rot, addr) = nibble_addr(index, buffer)
    # addr = index // 2 + buffer
    ld2 ld2 x01 !shr add st2
    # rot = 4 * (~index % 2)
    ld1 not x01 and x02 shf st1
  # return* (rot, addr)
  !rt0

load_nibble!
  load_nibble: # nibble = load_nibble(index, buffer)
    # (addr, rot) = swp(nibble_addr(index, buffer))
    ld2 ld2 :nibble_addr !call swp
    # nibble = (*addr >> rot) & 0x0F
    lda swp !shr x0F and
  # return* nibble
  st2
  !rt1

store_nibble!
  store_nibble: # store_nibble(index, buffer, nibble)
    # (rot, addr) = nibble_addr(index, buffer)
    ld2 ld2 :nibble_addr !call
    # rest = *addr & ~(0x0F << rot)
    ld1 lda x0F ld2 shf not and
    # new = rest | (nibble << rot)
    swp ld6 swp shf orr
    # *addr = new
    sta
  # return*
  !rt3
