@ lib/core.asm
@ lib/types.asm
@ lib/stdio.asm

# brainfuck-to-C99 transpiler. most programs from `/bf/test/` can be pasted into this
# transpiler directly. note the following:
# - `,` is non-blocking; if no input is currently available, `'\0'` is returned
# - cells are 8-bit unsigned integers, wrapping on overflow and underflow
# - writing beyond the start of the tape will result in undefined behavior
# - unbalanced brackets in the source code will result in undefined behavior

main!
  # :source_buffer :source_buffer :getline !call
  :source_buffer :getline.min !call

  :source_buffer dec @const :str_prologue loop: !puts.min inc
    ld0 !char.lda
    :_ neg @const # default: an empty string
      !char.greater_than_sign xo2 :> neg @const iff !char.greater_than_sign xo2
      !char.less_than_sign xo2 :< neg @const iff !char.less_than_sign xo2
      !char.plus_sign xo2 :+ neg @const iff !char.plus_sign xo2
      !char.hyphen_minus xo2 :- neg @const iff !char.hyphen_minus xo2
      !char.full_stop xo2 :. neg @const iff !char.full_stop xo2
      !char.comma xo2 :, neg @const iff !char.comma xo2
      !char.left_square_bracket xo2 :[ neg @const iff !char.left_square_bracket xo2
      !char.right_square_bracket xo2 :] neg @const iff !char.right_square_bracket xo2
    st0 neg
  :loop !bcc

  !char.right_curly_bracket !putc
  !hlt

  # !getline.def
  !getline.min.def


  str_prologue: @0A @23 @69 @6E @63 @6C @75 @64 @65 @3C @73 @74 @64 @69 @6F @2E @68 @3E @0A @63 @68 @61 @72 @20 @74 @5B @39 @39 @5D @3D @7B @30 @7D @3B @69 @6E @74 @20 @6D @61 @69 @6E @28 @76 @6F @69 @64 @29 @7B @63 @68 @61 @72 @2A @70 @3D @74 @3B @00 # "\n#include<stdio.h>\nchar t[99]={0};int main(void){char*p=t;" (C99)
  # str_prologue: @0A @63 @68 @61 @72 @20 @74 @5B @39 @39 @5D @3D @7B @30 @7D @3B @6D @61 @69 @6E @28 @29 @7B @63 @68 @61 @72 @2A @70 @3D @74 @3B @00 # "\nchar t[99]={0};main(){char*p=t;" (K&R)
  >: @2B @2B @70 @3B @00 # "++p;"
  <: @2D @2D @70 @3B @00 # "--p;"
  +: @2B @2B @2A @70 @3B @00 # "++*p;"
  -: @2D @2D @2A @70 @3B @00 # "--*p;"
  .: @70 @75 @74 @63 @68 @61 @72 @28 @2A @70 @29 @3B @00 # "putchar(*p);"
  ,: @2A @70 @3D @67 @65 @74 @63 @68 @61 @72 @28 @29 @3B @00 # "*p=getchar();"
  [: @77 @68 @69 @6C @65 @28 @2A @70 @29 @7B @00 # "while(*p){"
  ]: @7D _: @00 # "}"

  source_buffer:
