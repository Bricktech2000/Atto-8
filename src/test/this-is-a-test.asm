start: x00 x00
lds xDD xor pop :start :memcpy iif sti
memcpy: x00 loop: dup dup %display_buffer :display_data @01 add add
stw dup @00 swp stw str @00
inc
dup %len xor pop :loop :stop iif sti
stop: pop x00 hlt


display_buffer% dDF
len% d18
display_data:
dEA dE6 d4E d44 d4A dEC d00 d00
dE6 d0E d44 d0E dEC d0A d00 d00
dEE d6E d4C d44 d4E dC4 d00 d00
