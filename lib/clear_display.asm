# @../defs/display.asm
# @../utils/core.asm

clear_display: # clear_display()
x20 clear_display_for_j: dec
ld0 %front_buffer add x00 sta
buf :clear_display_for_j :clear_display_for_j_end iff sti
clear_display_for_j_end: pop
%rt0
