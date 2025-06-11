@ lib/core.asm
@ lib/types.asm
@ lib/string.asm
@ lib/stdlib.asm
@ lib/stdio.asm
@ lib/display.asm
@ lib/controller.asm
@ misc/common/common.asm

# it's more likely than not this game already exists. this program is a clone of an
# unnamed game I wrote back on 2019-12-15, but I can't remember what inspired me to
# write that game in the first place. for now, I'm calling it--- scratch that. it's
# called "Color Maze". GitHub Copilot figured it out

# the back buffer holds the tiles that have been visited / colored. every frame
# we move the player one tile and XOR the back buffer into the front buffer. this
# creates the flickering checkerboard pattern that we use as a "third color". the
# worse your display, the better the effect. when stationary after encountering a
# wall, we keep track of frame parity and ensure we only begin moving again after
# an even number of frames has elapsed. not doing so would create artifacts in the
# checkerboard pattern

main!
  pop pop !back_buffer dec @const sts

  nop !u4u4 # player_pos

  x00 !u8 # player_vel
  x00 # frame parity

  loop:
    # default: `player_vel`
    !u8.ld1 !getc !primary_to_delta
    # ignore user input if velocity isn't `0x00`
    ld2 !zr if2

    # if `parity` is `0x00` then update `player_pos` with `player_vel`
    !z !u4u4.ld1 x00 dec !u4u4.iff !u8.ld3 add @dyn
    # load pixel located at new `player_pos`
    !u4u4.ld0 !display_buffer !bit_addr !load_bit
    # if the pixel is set then we ran into a wall. if that is the
    # case then ignore the new `player_pos`, set `player_vel` to `0x00`
    # and flip `parity` to from `0x00` to `0x01` or vice versa
    !z swp if4 xor x00 if2 clc

    # XOR the back buffer into the front buffer
    !display_buffer.len !back_buffer !display_buffer :memxor !call

    # set the pixel at the new `player_pos` in both buffers
    !u4u4.ld2 !display_buffer !bit_addr
    !u8u8.ld0 !set_bit
    !display_buffer.len su2 !set_bit

    # delay of `x01` seems to look best on 60Hz displays
    x01 !delay
  :loop !jmp

  !memxor.def

  # we use `dec` because levels begin with `player_pos` which
  # we want to appear as the first item on the stack
  !back_buffer dec @org
    # !level_1
    # !level_2
    # !level_3
    # !level_4
    !level_5

level_1! @72 # player_pos
  @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00
  @FF @FF @F0 @05 @F7 @F5 @F7 @C1 @F7 @CD @F7 @89 @F7 @83 @B7 @A1 @B7 @A1 @B0 @11 @A0 @0D @8E @8D @FE @81 @FE @9D @FE @01 @FF @FF
  # solution: LDRURDRULDRDLURDRULULDLDLULURDRDLDLULURURU; reversible: no; chaos: 0.2
level_2! @27 # player_pos
  @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00
  @FF @FF @C7 @E1 @C0 @01 @80 @01 @80 @41 @8F @5F @A0 @0F @AF @0F @87 @DF @B7 @DF @B7 @D3 @B0 @03 @BF @E1 @80 @35 @80 @31 @FF @FF
  # solution: DLURULDLURULDLDRULDRULULULURDLULURDLURULDLDLULDRULURULDRULDLULDLDRULURULDLULDRDLURULDRULDLULURDRDRULDLURDL; reversible: no; chaos: 0.2
level_3! @7C # player_pos
  @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00
  @FF @FF @FE @01 @86 @BD @80 @3D @E2 @FD @C0 @FD @C0 @FD @A0 @41 @80 @5F @F8 @51 @F8 @55 @F8 @01 @80 @07 @BC @01 @80 @41 @FF @FF
  # solution: LDRULULDRDRULULDRULDLURDRULDLURDLURULDLURURDLULURDLU; reversible: no; chaos: 0.2
level_4! @66 # player_pos
  @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00
  @FF @FF @90 @01 @95 @E9 @C0 @09 @81 @E9 @85 @EB @85 @C9 @81 @8D @E0 @09 @E7 @19 @E0 @19 @EB @01 @EB @1B @E8 @09 @E3 @21 @FF @FF
  # solution: DRURDLDRDLURULDRDRDRULURDLDRURDLDRURDLURDLURDLDLULULDRDRULDLULDLURURDLULDRURDLDRURDRDRULURDLULDRULURURDLULDLURUL; reversible: no; chaos: 0.2
level_5! @75 # player_pos
  @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00 @00
  @FF @FF @91 @01 @80 @1D @82 @45 @B2 @71 @B2 @0F @BA @EF @82 @EF @90 @2F @86 @2F @8E @03 @C1 @01 @ED @69 @E4 @09 @F0 @79 @FF @FF
  # solution: DRDLDLULULDRULDRULURDRDLURURDRULDLULURDRURDRDRDRULDRDLULULURDLURDRDLDLDRURULURDLURDLURDLURDLDRURURURDRULDLDRDRULULURURDLDRURURULDRU; reversible: no; chaos: 0.2
