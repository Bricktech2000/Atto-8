@ lib/core.asm
@ lib/stdlib.asm
@ lib/display.asm
@ misc/common/common.asm

# this program simulates up to 6-bit grayscale by quickly alternating between several buffers,
# each of which contains one bit of a grayscale value per pixel. buffers are layed out in memory
# from "least significant" to "most significant", and each succeeding buffer is on display twice
# as frequently as the preceeding buffer. to help with flickering we want to avoid displaying a
# buffer for more than a frame at a time, and we want to space buffer appearances evenly. the
# buffer sequence chosen was the one given by `b(d) = [...b(d-1), d, ...b(d-1)]` with base case
# `b(0) = []`, where `d` is the grayscale bit depth. it goes `0,1,0,2,0,1,0,3,0,1,0,2,0,1,0...`.
# this is sequence A007814 in the OESIS, aka the "ruler sequence", aka the 2-adic valuation of N,
# aka the number of trailing zero bits of N, aka disk numbers for the Towers of Hanoi. here, the
# "most significant" buffer corresponds to sequence value 0, because it appears most frequently.

main!
  pop pop !stack sts

  # could start with `n = 0` too, doesn't really matter
  x01 for_n:
    !display_buffer !display_buffer # (src, dst)
    x00 ld3
      # the "most significant" buffer, the buffer corresponding to sequence value 0, coincides
      # with the display buffer. if we want the display buffer to display our sequence
      # `0,1,0,2,0,1,0,3,0,1,0,2,0,1,0...`, we need to swap it in sequence with buffers
      #  `1,1,2,2,1,1,3,3,1,1,2,2,1,1...`. we can get this new sequence by clearing out the
      # least significant bit of `n` (so each swap repeats twice), pretending `n = 1` when its
      # `DEPTH` least significant bits are zero (so we don't read buffers beyond `DEPTH`), and
      # counting trailing zeroes using `!ctz` (2-adic metric, as with the original sequence).
      !depth_mask dec @const and @dyn x00 add @dyn
    # src -= buffer_number * DISPLAY_BUFFER_LEN
    !ctz x05 rot sub
    for_byte:
      # swap `*src++` and `*dst++`
      ld1 lda ld1 lda
      ld3 sta ld1 sta
      x01 ad2 @dyn inc
    :for_byte !bcc pop # bleed `0x00` and set carry

    # n++
    add @dyn

    x01 !delay # normal speed
    # x80 !delay # for debugging
  :for_n !jmp

  # we put the "least significant" buffer at address `0x100 - DEPTH * DISPLAY_BUFFER_LEN so
  # that the "most significant" buffer ends up coinciding with the display buffer
  !depth x05 rot neg @org
    !image

depth_mask! x01 !depth rot dec @const # `!depth` least significant bits are set
stack! x40 # least possible address, rounded to a multiple of `!display_buffer_len`

# depth! x01 image! !avatar
# depth! x01 image! !mushroom
# depth! x01 image! !yin_yang
# depth! x01 image! !music_notes
# depth! x02 image! !planet_earth
# depth! x02 image! !hello_world
# depth! x02 image! !nes_controller
# depth! x02 image! !snes_controller
# depth! x02 image! !quartz_bricks
# depth! x02 image! !stone_bricks
# depth! x02 image! !mountains
# depth! x02 image! !gemstone
# depth! x01 image! !human_face-1
# depth! x02 image! !human_face-2
# depth! x03 image! !human_face-3
# depth! x04 image! !human_face-4
# depth! x05 image! !human_face-5
depth! x06 image! !human_face-6
