bit_addr: # (rot, addr) = bit_addr(index, buffer)
# addr = index // 8 + buffer
ld2 ld2 x03 %shr add st2
# rot = ~index % 8
ld1 not x07 and st1
# return* (rot, addr)
%rt0

load_bit: # bit = get_bit(index, buffer)
# (addr, rot) = swp(bit_addr(index, buffer))
ld2 ld2 :bit_addr %call swp
# bin = (*addr >> rot) & 0x01
lda swp %ror x01 and
# return* bin
st2
%rt1

store_bit: # store_bit(index, buffer, bit)
# (rot, addr) = bit_addr(index, buffer)
ld2 ld2 :bit_addr %call
# rest = *addr & ~(0x01 << rot)
ld1 lda x01 ld2 shf not and
# new = rest | (bit << rot)
swp ld6 swp shf orr
# *addr = new
sta
%rt3
