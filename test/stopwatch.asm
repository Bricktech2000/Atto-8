@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/display.asm

main!
  pop pop !display_buffer sts

  x00 x00 x00 x00 # (S, M, H, D)
  loop:
    x00 ld4 :display_byte.min !call
    x08 ld3 :display_byte.min !call
    x10 ld2 :display_byte.min !call
    x18 ld1 :display_byte.min !call

    inc x00 x00 x00 x00 # (S, M, H, D)
    x0A x0F !mask_max # S low nibble
    x60 !xFF_mask_max # S high nibble
    pop !carry_into   # S carry into M
    x0A x0F !mask_max # M low nibble
    x60 !xFF_mask_max # M high nibble
    pop !carry_into   # M carry into H
    x0A x0F !mask_max # H low nibble
    x24 !xFF_mask_max # H high nibble
    pop !carry_into   # H carry into D
    x0A x0F !mask_max # D low nibble
    # xA0 !xFF_mask_max # D high nibble
    pop               # discard carry

    # delay determined through `emu` "clocks" readout
    xCB !delay x07 !stall
  :loop !jmp

  !display_byte.min.def

  !display_buffer @org
    @00 @30 @00 @28 @00 @30 @00 @00 #   D
    @00 @28 @00 @38 @00 @28 @00 @00 #   H
    @00 @38 @00 @38 @00 @28 @00 @00 #   M
    @00 @18 @00 @10 @00 @30 @00 @00 #   S

mask_max! # (0x00, ..., out) = mask_max(mask, max, 0x00, ..., in)
  # replace (mask, max) with (mask, in, max, mask ^ max) at assembly time
  swp ld6 ld2 ld2 xo4 @const
  # CF = (in & mask) == max; out = CF ? in + (mask ^ max) + CF : in + 0x00
  and !eq iff ad4 @dyn x00
xFF_mask_max! # (0x00, ..., out) = xFF_mask_max(max, 0x00, ..., in)
  # xFF_mask_max(max, in) = mask_max(0xFF, max, in)
  # CF = in == max; out = CF ? 0x00 : in
  ld5 !eq if4 @dyn x00
carry_into! # (0x00, ..., out) = carry_into(0x00, ..., in)
  # out = in + CF
  ad4 @dyn x00
