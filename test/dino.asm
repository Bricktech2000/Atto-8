@ lib/microprocessor/core.asm
@ lib/microprocessor/math.asm
@ lib/microprocessor/memory.asm
@ lib/microcomputer/time.asm
@ lib/microcomputer/input.asm
@ lib/microcomputer/display.asm

main!
  pop !front_buffer sts
  !reset_input

  xF0 # prng_seed

  x00 # cactus_bot
  x00 # cactus_top

  x00 # x_pos
  x10 # x_vel
  xB0 # y_pos
  x00 # y_vel

  loop:
    # set y_vel to jump_vel if any button is pressed and y_pos > GROUND_POS
    !input_buffer lda buf pop !jump_vel ld1 iff ld2 !ground_pos sub pop ld1 iff st0 !reset_input
    # compute bit_addr of (DINO_POS, y_pos)
    !front_buffer ld2 xF0 and x05 rot orr x07 !dino_pos sub @const
    # clear pixel at (x_pos, y_pos - 2)
    # ld1 dec dec ld1 !clear_bit
    # clear pixel at (x_pos, y_pos)
    !clear_bit clc

    # x_pos += x_vel
    ld3 ld3 add st3
    # y_vel += !y_accel
    !y_accel add
    # y_pos += y_vel
    ld1 ld1 add st1
    # if y_pos > GROUND_POS, (y_pos, y_vel) = (GROUND_POS, 0x00)
    ld1 !ground_pos sub pop !ground_pos x00 !iff_u16

    # shift bottom halh of screen left by 1 pixel,
    # regardless of x_vel because we're out of memory
    !front_buffer x10 add @const for_addr:
      ld0 inc lda shl pop # load carry
      ld0 ld0 lda shl sta inc
      ld0 ld0 lda shl sta inc
    buf :for_addr !bcc pop

    # shift in cactus from cactus_top and cactus_bot
    !front_buffer x15 add @const
    ld0 ld0 lda shr clc
    ld7 shl st7 shl sta
    inc inc ld0 lda shr clc
    ld7 shl st7 shl sta

    # compute bit_addr of (DINO_POS, y_pos)
    !front_buffer ld2 xF0 and x05 rot orr x07 !dino_pos sub @const
    # if pixel at (x_pos, y_pos) is set, game over
    !ld0_u16 !load_bit buf pop :game_over !bcc
    # set pixel at (x_pos, y_pos - 2)
    # ld1 dec dec ld1 !set_bit
    # set pixel at (x_pos, y_pos)
    !set_bit

    # if x_pos % 0x100 == 0, generate a new cactus
    ld3 xFF and pop :ignore_cactus !bcc
      # generate a pointer to a random cactus
      # the x06 (0b00000110) below requires 4 cacti
      # x0E (0x00001110) could be used for 8 cacti
      ld6 !prng_minimal st6 ld6
      x06 and clc :cacti add
      # copy cactus data to cactus_top and cactus_bot
      ld0 lda st5
      inc lda st5
    ignore_cactus:

    x60 !delay
  :loop !jmp

  !nop !nop !nop

  game_over:
    !hlt

  cacti:
  # top bot
    d04 d05 # ______:.
    d10 d50 # __.:____
    d80 d80 # :_______
    d00 d00 # ________

  !front_buffer @org
  # !void
  !planet_and_stars
  # !clouds_and_sun
  # !stars_and_moon
  # !light_ground
  !dark_ground

dino_pos! x02 # from left edge of the screen
y_accel! x04 # gravity
jump_vel! xE8 # upward velocity when jumping
ground_pos! xB0 # y_pos of the ground

void!
  d00 d00 d00 d00 d00 d00 d00 d00
  d00 d00 d00 d00 d00 d00 d00 d00

planet_and_stars!
  d01 d08 d60 d08 d60 d36 d04 d08
  d00 d08 d00 d00 d00 d00 d00 d00

clouds_and_sun!
  d70 d0C dFC d1E d00 d1E d03 d0C
  d07 d80 d00 d00 d00 d00 d00 d00

stars_and_moon!
  d04 d18 d00 d30 d40 d32 d01 d3E
  d00 d1C d00 d00 d00 d00 d00 d00

dark_ground!
  d00 d00 d00 d00 d00 d00 d00 d00
  dFF dFF d02 d04 d90 d80 d24 d4A

light_ground!
  d00 d00 d00 d00 d00 d00 d00 d00
  dFF dFF dFD dFB d6F d7F dDB dB5
