# copies back buffer to front buffer
back_to_front%
back_to_front: # back_to_front()
%front_buffer %back_buffer sub for_i. dec
ld0 %front_buffer add
ld1 %back_buffer add
lda sta
buf .for_i %bcc pop
%ret
