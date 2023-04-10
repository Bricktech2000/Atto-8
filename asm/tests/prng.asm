@../../lib/utils/core.asm
@../../lib/utils/math.asm
@../../lib/utils/text.asm
@../../lib/microcomputer/utils.asm
@../../lib/microcomputer/print_byte.asm

main!
!front_buffer !alloc_buffer
!reset_input

xF0 x0F sta
xF1 xE0 sta

loop:
x05 :prng !call :print_byte !call
wait: !input_buffer lda buf pop :wait !bcs
!reset_input
:loop sti

!prng_minimal

!nibble_addr
!load_nibble
!store_nibble
!print_char
!print_byte
