@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm
@ lib/display.asm
@ lib/controller.asm

# PixEdit, the Atto-8 pixel editor
#
# controls are as follows:
# - use the primary controller to move the cursor around
# - type `' '` to toggle the pixel under the cursor
# - type `','` then paste a bitmap into `stdin` to load it
# - type `'.'` to output the current bitmap to `stdout`
#
# most designs from `/misc/common/common.asm` can be loaded into PixEdit by directly
# pasting them in after sending a `','` character. below is a sample design spelling
# out "PIXEDIT" in the Atto-8 font:
#
# ```
# @00 @00 @00 @00
# @FF @FE @00 @00 @1D @50 @1D @20
# @11 @50 @00 @00 @76 @5C @65 @48
# @76 @48 @00 @00 @FF @FE @00 @00
# @00 @00 @00 @00
# ```

main!
  pop pop !display_buffer sts

  x00 !u4u4 # xy_pos

  !char.null # dummy character
  :store !jmp # do not `load` yet

  load:
    !display_buffer.len !display_buffer !char.commercial_at !hex_getn
    !block_null

  store:
    !display_buffer.len !display_buffer !char.commercial_at !hex_putn
    !char.line_feed !putc

  loop:
    !char.pop !getc

    # xy_pos += xy_vel
    x00 ld1
      # ignore if any bits of the most significant nibble are set.
      # this helps prevent cursor movement on junk input from `stdin`
      ld0 xF0 an2 @dyn iff
    !primary_to_delta clc ad2

    :default
      !char.full_stop xo2 :store iff !char.full_stop xo2
      !char.comma xo2 :load iff !char.comma xo2
      !char.space xo2 # set carry if space was pressed
    !jmp default:
    # if carry clear, blink pixel under cursor
    # if carry set, toggle pixel under cursor
    :default :loop iff x08 x18 iff
      !u4u4.ld0+3 !display_buffer !bit_addr !flip_bit
    !delay !jmp
