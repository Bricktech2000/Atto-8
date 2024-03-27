@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm

main!
  :str_tail :puts !call
  x00 loop: x01 xor
    :str_left :puts !call
    !print_propeller
    :str_body :puts !call
    !print_propeller
    :str_right :puts !call
    x10 !delay
  :loop !jmp

  !puts.def

  #   "     __|__\n"
  # "\r'---x-(_)-x---' "
  str_tail: @20 @20 @20 @20 @20 @5F @5F @7C @5F @5F @0A @00 # "     __|__\n"
  str_left: @0D @27 @2D @2D @2D @00 # "\r'---"
  str_body: @2D @28 @5F @29 @2D @00 # "-(_)-"
  str_right: @2D @2D @2D @27 @20 @00 # "---' "

  propeller: !'x' @data !'+' @data

# does not consume its argument. assumes `sec`
print_propeller! :propeller dec @const ld1 add lda !putc
