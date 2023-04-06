# @common.asm
# @core_utils.asm

get_addr: # (rot, addr) = get_addr(index, buffer)
# addr = index // 2 + buffer
ld2 ld2 x01 %shr add st2
# rot = 4 * (~index % 2)
ld1 not x01 and x02 shf st1
# return* (rot, addr)
%ret

get_nibble: # nibble = get_nibble(index, buffer)
# (addr, rot) = swp(get_addr(index, buffer))
ld2 ld2 :get_addr %call swp
# nibble = (*addr << rot) & 0x0F
lda swp rot x0F and
# return* nibble
st2 swp pop
%ret

set_nibble: # set_nibble(index, buffer, nibble)
# (rot, addr) = get_addr(index, buffer)
ld2 ld2 :get_addr %call
# other = get_nibble(index ^ 1, buffer)
ld4 ld4 x01 xor :get_nibble %call
# new = ((other << 4) | nibble) << rot
x04 shf ld6 orr swp rot sta
# return*
st2 pop pop
%ret

print_char: # print_char(index, buffer, pos)
x03 print_char_for_i: dec
# src = i + index * 3
ld0 ld3 ld0 x01 shf add add
# nibble = get_nibble(src, buffer)
ld4 swp :get_nibble %call
# dst = i * 4 + pos // 4 * 8 + pos % 4
ld1 x02 shf ld6 x02 %shr x04 shf add ld6 x03 and add
# set_nibble(dst, &FRONT_BUFFER, nibble)
%front_buffer swp :set_nibble %call
buf :print_char_for_i :print_char_for_i_end iff sti
print_char_for_i_end: pop
# return*
st2 pop pop
%ret
