@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm

# halts on '0'
main!
  !block_getc
  loop:
    ld0 !putc
    ld0 shr @dyn pop
  !here !bcc :loop !jmp

# # halts on '0'
# main!
#   !block_getc
#   loop:
#     ld0 !putc
#   skip:
#     shr @dyn :skip :loop iff ld1 ad2 @dyn
#   !jmp

# # outputs '\0's on '0'
# main!
#   !block_getc
#   loop:
#     ld0 !putc
#     shr @dyn !'\0' swp iff shl @dyn
#   :loop !jmp
