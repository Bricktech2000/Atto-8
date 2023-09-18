@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm
@ lib/display.asm
@ lib/controller.asm

# PixEdit, a pixel editor
#
# controls are as follows:
# - use the primary controller to move the cursor around
# - type `' '` to toggle the pixel under the cursor
# - type or paste a bitmap into `stdin` to load it
# - type `'\n'` to output the current bitmap to `stdout`
#
# designs to try:
# `X00000000FFFE00001D501D2011500000765C654876480000FFFE000000000000` loads "PIXEDIT"
# `XCCCCCCCC33333333CCCCCCCC33333333CCCCCCCC33333333CCCCCCCC33333333` loads a checkerboard pattern
# `XA88EE88AAEEE0000AE8CEC8AEAEC000000000000000000000000000000000000` loads "HLLO WRLD" from `hllowrld.asm`
# `X4EEEE44AA44E0000000600EE000E000000000000000000000000000000000000` loads "ATTO-8" from `hllowrld.asm`
# `X07E01878387C703E679E8FC18FCDCFDFE79FE00DCFF1724E2244200410080FF0` loads the mushroom from `mushroom.asm`

main!
  pop pop !display_buffer sts

  x00 !u4u4 # xy_pos

  !char.null # dummy character
  :store !jmp # do not `load` yet

  load:
    !display_buffer for_load:
      # assume input always well formed
      !block_getc !char.to_u4 x04 rot
      !block_getc !char.to_u4 orr !u8
      # write byte to display buffer
      ld1 !u8.sta
    inc !check_zero :for_load !bcc pop
    # fall through

  store:
    !char.latin_capital_letter_x !putc
    # output display buffer as hex string
    !display_buffer for_store:
      ld0 !u8.lda !u8.to_chars !putc !putc
    inc !check_zero :for_store !bcc pop
    !char.carriage_return !putc !char.line_feed !putc
    # fall through

  loop:
    !char.pop !getc

    # xy_pos += xy_vel
    x00 !primary_to_delta clc ad2

    :default
      !char.space xo2 :ignore iff !char.space xo2
      !char.line_feed xo2 :store iff !char.line_feed xo2
      !char.latin_capital_letter_x xo2 :load iff !char.latin_capital_letter_x xo2
    !jmp default:
      # first flip pixel at xy_pos
      !u4u4.ld0+1 !display_buffer !bit_addr !flip_bit
      x05 !delay
    ignore:
      # second flip pixel at xy_pos
      !u4u4.ld0+1 !display_buffer !bit_addr !flip_bit
      x20 !delay
  :loop !jmp
