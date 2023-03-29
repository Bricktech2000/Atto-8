main%
init: x00 x00
lds xDD xor pop :init :main iif sti
main: x00 loop: dup dup %display_buffer :display_data @01 add add
stw dup @00 swp inc stw str @00
inc
dup %display_data_len xor pop :loop :exit iif sti
exit: pop x00 hlt

display_data:
dEA dE6 d4E d44 d4A dEC d00 d00
dE6 d0E d44 d0E dEC d0A d00 d00
dEE d6E d4C d44 d4E dC4 d00 d00
display_data_end:

display_buffer% xE0
display_data_len% :display_data_end :display_data sub
