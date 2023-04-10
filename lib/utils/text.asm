nibble_addr!
nibble_addr: # (rot, addr) = nibble_addr(index, buffer)
# addr = index // 2 + buffer
ld2 ld2 x01 !shr add st2
# rot = 4 * (~index ! 2)
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
!rt3

print_char!
print_char: # print_char(index, buffer, pos)
x03 for_i. dec
# src = i + index * 3
ld0 ld3 ld0 x01 shf add add
# nibble = load_nibble(src, buffer)
ld4 swp :load_nibble !call
# dst = i * 4 + pos // 4 * 8 + pos ! 4
ld1 x02 shf ld6 x02 !shr x04 shf add ld6 x03 and add
# store_nibble(dst, &FRONT_BUFFER, nibble)
!front_buffer swp :store_nibble !call
buf .for_i !bcc pop
# return*
!rt3
