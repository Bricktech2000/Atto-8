@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm
@ lib/display.asm

# controls:
# - Secondary Left and Right -- rotate left and right
# - Primary Left and Right -- move left and right
# - Primary Down -- soft drop
#
# worth noting:
# - no hold, no hard drop, no kicks, no delayed autoshift
# - all timings and tetromino spawn probabilities are made up
# - rotations conform to the Super Rotation System modulo kicks
#
# some implementation details:
# - `frame` is a frame counter. we use it to move tetrominoes down every few game ticks
# - `type` means "tetromino type", from `0x00` to `0x07` for tetrominoes "OIJLISTZ"
# - `rot` means "tetromino rotation", from `0x00` to `0x03` for `0/4`, `1/4`, `2/4`, `3/4`
# - `index = (rot << 3) | type` such that `tetrominoes[index]` is the tetromino pixel data
# - `pos` as a `u4u4` means "tetromino position"

main!
  pop pop :tetrominoes dec dec dec @const sts # (pos, index, frame)

  spawn_tetromino:
    # if we're here we probably failed to move a tetromino down; move it back up
    x10 sub
    # if moving the tetromino upwards overflowed, game over and halt. otherwise,
    # display back the tetromino at its previous (current) position.
    # below is equivalent to `!flip !here !bcs :display_tetromino !call`
    !flip .ret !here !bcs :display_tetromino !jmp ret.

    # new_type = (pos + frame + type) & TYPE_MASK; new_pos = SPAWN_POS
    # `pos, frame, type` happen to be easily accessible on the stack, so we use
    # them as a random seed
    add ld1 add !type_mask and !spawn_pos

    # move lines down if some lines below them are full. we actually check whole
    # bytes in the display buffer for `0xFF`, meaning pixel art is not possible.
    # we start at the end of the display buffer and loop backward using two
    # pointers, `src` and `dst`. if both `src` and the other byte on the same
    # line are `0xFF` then we don't advance `dst`. therefore, if a line is full,
    # `dst` will lag behind `src` by 2 bytes, moving subsequent lines downward
    xFD xFD ld0 lda for_line: # (src, dst)
      # *src = *dst
      ld0 ld3 sta
      # load value at `src ^ 0x01`, the other byte in the same line
      ld1 x01 xor lda
      # if `*src == *(src ^ 0x01) == 0xFF` then don't advance `dst`
      !nand @dyn pop xFF ad2
      # loop while `*(--src) != 0x00`. the byte at `display_buffer - 1`
      # is `ofst[0x04]`, which happens to be `0x00`
      dec ld0 lda !z
    :for_line !bcc sta pop # `sta` used as `pop pop`

  loop:
    # briefly display at `pos` the tetromino at index `index`
    !flip :display_tetromino !call
    # below is equivalent to `!tick_delay !delay !flip :display_tetromino !call`
    !tick_delay :ret
      delay: x1F !stall x01 su2 @dyn :delay !bcc
    :display_tetromino !jmp ret:

    # compute `(pos_delta, index_delta)`
    x00 x00 !getc ld0
      shl !rot_parity if4 # secondary_right
      shl !rot_mask if4 # secondary_left
    pop shr
      shr x10 if2 # primary_down
      shr xFF if2 # primary_left
      shr @dyn # primary_right
    pop
    # (candidate_pos, candidate_index) = (pos, index) + (pos_delta, index_delta)
    # `candidate_index &= ROT_MASK | TYPE_MASK` so rotations wrap around
    !u8u8.ld1 !u8u8.add !rot_mask !type_mask orr @const an2
    # if `index == 0x00` then `candidate_index = 0x00`. this ensures "O"
    # tetrominoes never rotate, as required by the `tetrominoes` encoding
    ld3 !z if2
    # if `(candidate_pos, candidate_index)` is a valid tetromino position
    # then `(pos, index) = (candidate_pos, candidate_index)`
    !check :display_tetromino !call !u8u8.iff

    # `frame += fall_speed`. if the operation overflowed then `pos += 0x10`,
    # moving the tetromino down
    x00 !fall_speed ad4 x10 dec @const iff add @dyn
    # if the new tetromino position is invalid then spawn a new tetromino.
    # otherwise, loop
    !check :display_tetromino !call
    :spawn_tetromino !bcc
  :loop !jmp

  # if `do_flip` is `0x00` then return whether a tetromino's position is valid in `CF`.
  # if `do_flip` is `0xFF` then flip the tetromino's pixels in the display buffer to
  # display it. behavior is undefined for other values of `do_flip`
  display_tetromino: clc # (u4u4 pos index) = display_tetromino(do_flip, u4u4 pos, index)
    # `is_invalid = do_flip`. because of this, the carry flag won't ever get set if we're
    # flipping a tetromino's pixels but may get set if we're checking a tetromino's position
    ld1
    x33 !u4u4 # pos_delta
    # `pixels = tetrominoes[index]`. load the tetromino's pixels
    ld5 :tetrominoes !ofst lda
    for_dxdy:
      # pos_delta &= (pos & ROT_PARITY) ? 0x31 : 0x13
      # this selects the correct orientation of the 2x4 block of possible pixels
      ld6 !rot_parity !cl x31 x13 iff an2
      # get `pixel`, the least significant bit from `pixels`
      shr @dyn x00 shl @dyn
        # `index_ = pos + pos_delta`. by `index_` I mean "pixel index", not "tetromino index"
        !u4u4.ld6 !u4u4.ld3 add
        # `index_ += ofst[index & ROT_MASK >> 3] + 1`. `+1` beecause `ofst` sets the carry flag
        ld8 !rot_mask and clc :ofst !ofst !u4u4.lda add
        # `(rot_, addr) = bit_addr(display_buffer, index_)`. by `rot_` I mean "bit rotation",
        # not "tetromino rotation"
        !display_buffer !bit_addr
      # `pixel <<= rot_`
      ro2
      # load `is_set = pixel & *addr`, which represents whether the pixel is
      # already set in the display buffer
      ld0 lda ld0 ld3 and
        # `pixel &= do_flip`, so we ignore pixels if `do_flip` is `0x00`
        ld6 an4
      # `is_invalid |= is_set`
      or8
      # `*addr ^= pixel`. this flips the pixel in the display buffer
      xo2 sta
    # loop while `pos_delta--` is not `0x00`
    x01 su2 :for_dxdy !bcc pop # bleed `0xFF`
    # return with `CF = !is_invalid`
    orr an2 @dyn st0 !ret

  x0C !pad # just enough for stack

  # `(initial_pos, initial_index, initial_frame)`. `initial_pos = SPAWN_POS + 0x10`
  # because falling through to `spawn_tetromino` at startup will decrement it by `0x10`.
  # `initial_index = 0x08` because `tetrominoes[0x08]` is `0x00`, an invisible tetromino,
  # which we want because falling through to `spawn_tetromino` at startup will display
  # it. `initial_frame = 0x00` is arbitrary
  !spawn_pos x40 add @data @08 @00

  # every byte is one tetromino at one rotation. bits within a byte, from the most to
  # the least significant, represents one `@` in the diagram below, to be read left to
  # right, top to bottom. `.` characters are unset pixels. it is worth noting that:
  # - `rot & 0x01` aka `index & ROT_PARITY`, tetromino rotation parity, dictates the
  #   orientation of the 2x4 block of possible pixels for rotation `rot`
  # - `ofst[rot]` as a `u4u4` is the position of the top left corner of the 2x4 block
  #   of possible pixels for rotation `rot`
  #
  # .---0/4---.---1/4---.---2/4---.---3/4---.
  # | @ @ @ @ | . @ @ . | . . . . | @ @ . . |
  # | @ @ @ @ | . @ @ . | @ @ @ @ | @ @ . . |
  # | . . . . | . @ @ . | @ @ @ @ | @ @ . . |
  # | . . . . | . @ @ . | . . . . | @ @ . . |
  # '---------'---------'---------'---------'

  # all this allows us to encode the Super Rotation System within minimal memory. note
  # that the "O" tetromino must alawys be in rotation `0/4` because its other ones, which
  # are all identical, fail to fit within the 2x4 block of possible pixels defined above.
  # the "I" tetromino is duplicated so tetromino count is a power of two.
  #
  #   .----O----. .--JLSTZ--. .----I----.
  #   | . @ @ . | | @ @ @ . | | . @ @ . |
  #   | . @ @ . | | @ @ @ . | | @ @ @ @ |
  #   | . . . . | | @ @ @ . | | @ @ @ @ |
  #   | . . . . | | . . . . | | . @ @ . |
  #   '---------' '---------' '---------'

  tetrominoes:
  # =O= =I= =J= =L= =I= =S= =T= =Z=
    @66 @0F @8E @2E @0F @6C @4E @C6 ofst: @00 # 0/4 turn clockwise
        @55 @E8 @AC @55 @B4 @B8 @78       @01 # 1/4 turn clockwise
        @0F @E2 @E8 @0F @6C @E4 @C6       @10 # 2/4 turn clockwise
        @55 @5C @D4 @55 @B4 @74 @78       @00 # 3/4 turn clockwise

  !display_buffer @org
    !classic
    # !12_wide
    # !14_wide
    # !checker
    # !garbage

fall_speed! x56 # speed at which tetrominoes fall
tick_delay! x0E # delay per game tick
spawn_pos! x05 !u4u4 # position at which tetrominoes spawn

flip! xFF # constant for `display_tetromino`
check! x00 # constant for `display_tetromino`
rot_mask! x18 # `index & ROT_MASK >> 3` gives `rot`
type_mask! x07 # `index & TYPE_MASK` gives `type`
rot_parity! x08 # `index & ROT_PARITY` gives rotation parity

classic!
  @E0 @07 @E0 @07 @E0 @07 @E0 @07
  @E0 @07 @E0 @07 @E0 @07 @E0 @07
  @E0 @07 @E0 @07 @E0 @07 @E0 @07
  @E0 @07 @E0 @07 @E0 @07 @FF @FF

checker!
  @E0 @07 @E0 @07 @E0 @07 @E0 @07
  @E0 @07 @E0 @07 @E0 @07 @E0 @07
  @E0 @07 @E0 @07 @E0 @07 @E0 @07
  @EA @AF @F5 @57 @EA @AF @FF @FF

garbage!
  @E0 @07 @E0 @07 @E0 @07 @E0 @07
  @E0 @07 @E0 @07 @E0 @07 @E0 @07
  @E0 @07 @E0 @07 @E0 @07 @E0 @07
  @FD @77 @E4 @8F @F1 @B7 @FF @FF

12_wide!
  @C0 @03 @C0 @03 @C0 @03 @C0 @03
  @C0 @03 @C0 @03 @C0 @03 @C0 @03
  @C0 @03 @C0 @03 @C0 @03 @C0 @03
  @C0 @03 @C0 @03 @C0 @03 @FF @FF


14_wide!
  @80 @01 @80 @01 @80 @01 @80 @01
  @80 @01 @80 @01 @80 @01 @80 @01
  @80 @01 @80 @01 @80 @01 @80 @01
  @80 @01 @80 @01 @80 @01 @FF @FF

