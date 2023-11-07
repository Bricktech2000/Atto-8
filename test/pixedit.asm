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
# - type or paste a bitmap into `stdin` to load it
# - type `'\n'` to output the current bitmap to `stdout`
#
# most designs from `misc/common/common.asm` can be loaded into PixEdit by directly
# pasting them in. below is a sample design spelling out "PIXEDIT" in the Atto-8 font:
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
    !display_buffer for_load: clc
      # assume input always well formed
      !getc !char.to_u4 x04 rot
      !getc !char.to_u4 orr !u8
      # write byte to display buffer
      ld1 !u8.sta
    inc !z :break !bcs
      # block until `'@'` is sent through `stdin`
      block: !getc !char.commercial_at !eq :block
    :for_load iff !jmp break: pop
    # fall through

  store:
    # output display buffer as hex string
    !display_buffer for_store:
      # ld0 x07 !cl # every 8 bytes
      # !char.space !char.line_feed iff !putc
      # !char.null !char.carriage_return iff !putc
      !char.commercial_at !putc
      ld0 !u8.lda !u8.to_chars !putc !putc
      !char.space !putc
    inc !z :for_store !bcc pop
    !char.carriage_return !putc !char.line_feed !putc
    # fall through

  loop:
    !char.pop !getc

    # xy_pos += xy_vel
    x00 !primary_to_delta clc ad2

    :default
      !char.space xo2 :ignore iff !char.space xo2
      !char.line_feed xo2 :store iff !char.line_feed xo2
      !char.commercial_at xo2 :load iff !char.commercial_at xo2
    !jmp default:
      # first flip pixel at xy_pos
      !u4u4.ld0+1 !display_buffer !bit_addr !flip_bit
      x05 !delay
    ignore:
      # second flip pixel at xy_pos
      !u4u4.ld0+1 !display_buffer !bit_addr !flip_bit
      x20 !delay
  :loop !jmp
