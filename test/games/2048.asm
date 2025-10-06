@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm
@ lib/display.asm
@ lib/controller.asm

# this implementation differs from the original game in a few ways:
#   - numbers on the tiles are the base-2 logarithms of the numbers on the original -- we only have one character per tile
#   - implementation has '0' tiles (would be equivalent to '1' tiles in the original) -- no one but mathematicians counts from 1
#   - implementation has greedy merge, meaning tiles are merged multiple times within a single move -- simplest way to implement
#   - only '0' tiles (equivalent to '2' tiles in the original) are generated -- currently not enough memory for generating '1' tiles
#
# when no room is left to spawn a new tile, the program stalls, indicating the game is over

main!
  pop pop !display_buffer dec dec @const sts # (rand_seed, moved)

  !primary_up # direction

  while:
    # iteration count must be no less than x03.
    # a higher iteration count serves as a stall
    x04 for_iteration: dec
      # refer to C implementation for this main loop
      x10 for_n: dec
        ld2 !primary_up !primary_down orr @const !cl
        x04 x01 iff # soon-to-be offset
        ld1 # soon-to-be curr
        x03 # soon-to-be equality
        x06 x00 iff # orientation
        ld3 neg # -offset
        x0F xo4 ld5 # curr ^ 0x0F
        ld8 !primary_up !primary_left orr @const !cl
        # set `curr` to either `curr` or `curr ^ 0x0F`
        if4
        # set `offset` to either `offset` or `-offset`
        if4
        # set `equality` to either `0x00` or `0x03`
        x00 if2
        # if ((curr >> orientation & 0x03) == equality) continue;
        ld2 swp rot x03 and !eq :continue !bcs
        # curr = &board + curr
        :board add
        # `offset += curr` to produce `prev`
        ld0 ad2

        ld1 lda # board[prev]
        ld1 lda # board[curr]

        !z :zero !bcs
        !eq :equal !bcs
        :continue !jmp

        zero: # `board[prev]` and `board[curr]` on the stack
          # board[curr] = board[prev] - 1
          pop dec ld1 sta
        equal: # `board[prev]` and `board[curr]` popped off the stack
          # board[curr] = board[curr] + 1
          ld0 lda inc
            # if (board[curr] != 0) moved = true;
            !z x00 flc shl @dyn or8
          ld1 sta
          # board[prev] = 0
          x00 ld2 sta
          # `prev` and `curr` are bled onto the stack for `continue` below,
          # which happens to pop two bytes off the stack

        continue: pop pop
      !z :for_n !bcc # bleed `0x00`

      # if we're at on last iteration and `moved` is `true`, generate a
      # `0x01` tile. otherwise, generate a `0x00` tile, which is a no-op.
      # note that making an invalid move when no room is left for a new
      # tile will cause an infinite stall even though the game shouldn't
      # be over yet
      !e x00 shl @dyn ld4 and

      # keep adding `&board` to `rand_seed`, modulo `0x10` to prevent
      # out-of-bounds access, until we find a zero tile. the cycle length
      # of this operation is `0x10` if and only if `&board & 0x0F` is
      # coprime with `0x10`. this is guaranteed at assembly time. see below
      ld3 generate:
        x0F and clc :board !ofst
      ld0 lda !zr :generate !bcc sta

      # instead of looping through the board and displaying tiles, we loop
      # through nibbles of the display buffer and figure out what to display
      # as we go along. we figure out which tile occupies that nibble, then
      # figure out which row of the tile's sprite data we need for the nibble,
      # and finally display that row. this is done to save memory
      !display_buffer.len for_byte: dec
        x00 # result
        x00 for_nibble:
          # tile = board[((byte & ~0x07) >> 1) | ((byte & 0x01) << 1) | nibble]
          # first term maps every byte to the first element of its board row. second term moves right by
          # two if the byte is on the right half of the display. third term moves right by one if the current
          # nibble is the second nibble of the byte. this gives a pointer to the tile we're looking for
          :board ld3 x06 not @const and clc shr @dyn x00 shl @dyn xFF xo4 @dyn shl @dyn orr clc add lda
          # 2048_char = 2048_chars[tile]
          :2048_chars !ofst lda
          # nibble = (2048_char << (byte & 0x06)) & 0x60
          # we use `0x60` as a mask to center the character horizontally.
          # this is also why `shr @data` is used in `2048_chars` below
          ld3 x06 and rot x60 and
          # result |= nibble
          # result <<= 4
          or2 x04 ro2
        !z :for_nibble !bcc pop
        # display_buffer[byte] = result
        !display_buffer dec @const ld2 add sta
      !z :for_byte !bcc # bleed `0x00`

    !e :for_iteration !bcc # bleed `0x00`

    # moved = false
    st2

    # the controller being in any state other than `0x01 | 0x02 | 0x04 | 0x08`
    # unfortunately breaks the game logic. not much we can do about that.
    # below is similar to `!block_getc`, but updates `rand_seed` while waiting
    block: !char.add !getc !char.check_null :block !bcs
  :while !jmp

  # characters below are 2x4 pixels in size. the bits of their encoding from MSB to LSB correspond
  # to the pixels of the font from left to right, top to bottom
  2048_chars:
    x00 shr @data #
    x3C shr @data # 0
    x54 shr @data # 1
    x78 shr @data # 2
    xDC shr @data # 3
    xB4 shr @data # 4
    xD8 shr @data # 5
    x6C shr @data # 6
    xD4 shr @data # 7
    xFC shr @data # 8
    xF4 shr @data # 9
    x9C shr @data # A
    xBC shr @data # B
    x64 shr @data # C
    x7C shr @data # D
    xE4 shr @data # E
    x68 shr @data # F

  # `&board & 0x0F` must be coprime with `0x10` for random number generation.
  # 'x' is coprime with `0x10` if and only if 'x' is odd. therefore we ensure
  # that `&board & 0x0F` is odd by setting its least significant bit to '1'
  !here x01 orr @org

  # `moved` is initialized to `false` and the initial controller input is set
  # to `!primary_up`, and so the board must contain at least one `0x01` tile,
  # ideally on the top row
  board:
    @01 @00 @00 @00
    @00 @00 @00 @00
    @00 @00 @00 @00
    @00 @00 @00 @00
