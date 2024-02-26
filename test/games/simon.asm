@ lib/core.asm
@ lib/types.asm
@ lib/string.asm
@ lib/stdlib.asm
@ lib/stdio.asm
@ lib/display.asm
@ lib/controller.asm

main!
  pop pop !display_buffer dec dec dec @const sts # (previous_char, unused padding, rand_seed)

  # replay `:sequence_buffer` sequence and generate new random move.
  # use `(0x01, SEQUENCE_BUFFER << 1)` as defaults for `(0x00, ptr)`, which are
  # normally bled onto the stack from `:loop` below. since `0x01` ends up being
  # shifted left by one, the first move of the sequence is always `!primary_down`
  :sequence_buffer shl @const x01 replay:
    # note that `0x00` was bled onto the stack from `:loop` below. if we're coming
    # from there then the carry flag is currently set, which means that `shl` turns
    # the `0x00` into a `0x01`. then, `random_move = 0x01 << (rand_seed & 0x03)`.
    # `x01 ad8` is unrelated and increments the score
    shl ld4 x01 ad8 x03 and rot
    # `ptr` was bled onto the stack from `:loop` below. it currently points to the
    # last byte of `:sequence_buffer`. `*ptr = random_move`. see below for `sec shr`
    # rationale
    swp sec shr sta
    # replay `:sequence_buffer` sequence. `!primary_down` is always the first move
    :sequence_buffer shl @const !primary_down replay_loop:
      x20 !delay :toggle_button !call
      # increment `*ptr` and loop while `*ptr`; otherwise fall through to `:loop`.
      # we omit `sec` because `:toggle_button` always sets carry. see below for
      # `sec shr` rationale
      inc ld0 shr lda
    !z :replay_loop !bcc # bleed `(0x00, ??)`

  # loop through `:sequence_buffer` and ensure the player repeats the sequence
  :sequence_buffer shl @const st1 loop:
    # note that `0x00` was bled onto the stack from `:replay_loop` above. loop
    # until `c = getc() != previous_char` and return `delta = c ^ previous_char`.
    # also update `rand_seed` while waiting
    block: ad4 !getc ld2 xor @dyn :block !bcs
    # `previous_char ^= delta`, which stores `c` into `previous_char` as
    # XOR is an involution
    ld0 ld0 xo4
    # toggle the button corresponding to `delta` on the representation
    :toggle_button !call
    # game over if `*ptr != delta`, where `ptr` is the pointer to the current
    # byte in `:sequence_buffer`. see below for `sec shr` rationale
    ld1 sec shr lda !eq !here !bcc
    # increment `ptr` and loop while `*ptr`; otherwise go to `replay`. we
    # omit `sec` because the only way `!bcc` exits is with carry set. see
    # below for `sec shr` rationale
    inc ld0 shr lda !z :replay !bcs # bleed `(0x00, ptr)`
  :loop !jmp

  # format is `DST dir_xor: DATA`; `DATA` must be `DIR_XOR_LEN` bytes long. `DATA`
  # represents which bits to XOR with the display buffer to toggle a button on the
  # representation. `DST` is a pointer to the target region in the display buffer
  @E0 up_xor: @07 @C0 @04 @00 @04 @20 @04 @00 @04 @20 @01 @40
  @F4 down_xor: @03 @C0 @04 @00 @04 @20 @04 @00 @04 @20 @01 @40
  @EA left_xor: @F8 @00 @80 @00 @84 @00 @80 @00 @84 @00 @28 @00
  @EA right_xor: @00 @1E @00 @20 @00 @21 @00 @20 @00 @21 @00 @0A

  toggle_button: # toggle_button(state)
    swp :right_xor swp # default to right
      shr :up_xor if2
      shr :down_xor if2
      shr :left_xor if2
    # src = dir_xor
    pop
    # dst = *(dir_xor - 1)
    ld0 dec lda
    # load return address
    ld2
    # len = DIR_XOR_LEN
    !dir_xor.len st3
    # stack currently `ret_addr, dst, src, len`. XOR `src` into `dst`
    !memxor.def
    # prevent unused label warning
    :memxor pop

  # both `:loop` and `:replay_loop` above toggle buttons on the representation,
  # which must happen twice for every byte in `:sequence_buffer` -- once for
  # pressing down and once for releasing. to achieve that, we use the for loop
  # `for (i = (SEQUENCE_BUFFER shl); *(i sec shr); i++)` and access the current
  # byte through `*(i sec shr)`. essentially, we loop over a pointer that was
  # shifted left by one (the least significant bit becomes a parity flag) and
  # shift it back to the right by one to access the current byte. as a result,
  # the most significant bit of the pointer is thrown away. to prevent this
  # from causing issues, we assume the most significant bit of the pointer is
  # always set. this is why we use `sec shr` above and assert `x80 orr` below.
  # to prevent regressions, we also assert `shl x80 not and shr`, equivalent to
  # `x40 not and`, to ensure that `:sequence_buffer shl @const` can be pushed
  # through a single `psh` instruction
  !here shl x80 not and shr x80 orr @org
  sequence_buffer: # sequence of controller states. grows as game progresses

  !display_buffer @org
    @07 @C0 @07 @E0 @07 @C0 @07 @E0
    @07 @C0 @FA @BE @FC @3F @F8 @3E
    @FC @3F @F8 @3E @57 @D5 @07 @E0
    @07 @C0 @07 @E0 @07 @C0 @02 @A0

dir_xor.len! x0C
