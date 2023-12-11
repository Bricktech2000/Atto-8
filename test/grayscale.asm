@ lib/core.asm
@ lib/string.asm
@ lib/stdlib.asm
@ lib/display.asm
@ misc/common/common.asm

# this program simulates 2-bit grayscale by quickly alternating between two buffers,
# the first containing the pixel LSBs and the second containing the pixel MSBs. that
# is, given the colors white (`0b11`), light (`0b10`), dark (`0b01`), black (`0b00`),
# - the first buffer contains light (`0b10`) and white (`0b11`) pixels
# - the second buffer contains dark (`0b01`) and white (`0b11`) pixels

main!
  pop pop !back_buffer sts

  !lsb_delay loop:
    # alternate between `delay(LSB_DELAY)` and `delay(MSB_DELAY)`
    !lsb_delay xor !msb_delay xor ld0 !delay
    # swap buffers back and forth
    !display_buffer.len !back_buffer !display_buffer :memswp !call
  :loop !jmp

  !memswp.def

  !back_buffer @org
    !planet_earth
    # !hello_world
    # !nes_controller
    # !snes_controller
    # !quartz_bricks
    # !stone_bricks
    # !mountains
    # !gemstone

lsb_delay! x02
msb_delay! x04
