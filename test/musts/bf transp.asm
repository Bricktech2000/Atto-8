@ lib/core.asm
@ lib/types.asm
@ lib/stdio.asm

# brainfuck-to-C99 transpiler; pattern-matches against source characters then outputs
# equivalent C99 code
#
# most programs from /bf/test/ can be pasted in directly. note the following:
# - `,` is non-blocking; if no input is currently available, '\0' is returned
# - cells are 8-bit unsigned integers, wrapping on overflow and underflow
# - writing beyond the start of the tape will result in undefined behavior
# - unbalanced brackets in the source code will result in undefined behavior

main!
  # :source_buffer :source_buffer :getline !call
  :source_buffer :getline.min !call

  :source_buffer dec @const :str_prologue loop: !puts.min inc
    ld0 !char.lda
    :_ neg @const # default: an empty string
      !'>' xo2 :'>' neg @const iff !'>' xo2
      !'<' xo2 :'<' neg @const iff !'<' xo2
      !'+' xo2 :'+' neg @const iff !'+' xo2
      !'-' xo2 :'-' neg @const iff !'-' xo2
      !'.' xo2 :'.' neg @const iff !'.' xo2
      !',' xo2 :',' neg @const iff !',' xo2
      !'[' xo2 :'[' neg @const iff !'[' xo2
      !']' xo2 :']' neg @const iff !']' xo2
    st0 neg
  :loop !bcc

  !'}' !putc
  !'\n' !putc
  !hlt

  # !getline.def
  !getline.min.def


  str_prologue: @0A @23 @69 @6E @63 @6C @75 @64 @65 @3C @73 @74 @64 @69 @6F @2E @68 @3E @0A @63 @68 @61 @72 @20 @74 @5B @39 @39 @5D @2C @2A @70 @3D @74 @3B @69 @6E @74 @20 @6D @61 @69 @6E @28 @76 @6F @69 @64 @29 @7B @00 # "\n#include<stdio.h>\nchar t[99],*p=t;int main(void){" (C99)
  # str_prologue: @0A @63 @68 @61 @72 @20 @74 @5B @39 @39 @5D @2C @2A @70 @3D @74 @3B @6D @61 @69 @6E @28 @29 @7B @00 # "char t[99],*p=t;main(){" (K&R)
  '>': @2B @2B @70 @3B @00 # "++p;"
  '<': @2D @2D @70 @3B @00 # "--p;"
  '+': @2B @2B @2A @70 @3B @00 # "++*p;"
  '-': @2D @2D @2A @70 @3B @00 # "--*p;"
  '.': @70 @75 @74 @63 @68 @61 @72 @28 @2A @70 @29 @3B @00 # "putchar(*p);"
  ',': @2A @70 @3D @67 @65 @74 @63 @68 @61 @72 @28 @29 @3B @00 # "*p=getchar();"
  '[': @77 @68 @69 @6C @65 @28 @2A @70 @29 @7B @00 # "while(*p){"
  ']': @7D _: @00 # "}"

  source_buffer:
