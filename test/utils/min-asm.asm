@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm

# Min-Asm, the Atto-8 minimal native assembler
#
# input a stream of assembly mnemonics and Min-Asm will output a stream of bytes as hexadecimal.
# mnemonics can be separated by any number of whitespace characters, and `#` line comments are
# supported. input stream is assumed to be well-formed; if it is not, the behavior is undefined.
# unsupported features include directives, labels and macros
#
# as an example, here is a position-independent rot13 program that Min-Asm can assemble:
#
# ```
# x00 lda         # read from standard input
# ld0 x20 orr     # clone and convert to lowercase
# x61 sub         # map lowercase alphabet to 0-25
# x00             # default offset is 0
# x0D su2 x0D iff # if between a-m, offset is 13
# x0D su2 xF2 iff # if between n-z, offset is -13
# st0 add         # add offset to original input
# x00 sta         # write to standard output
# ldi x15 sub sti # loop back to start
# ```

main!
  :main !jmp

  comment:
    # consume input until a newline is hit
    !getc !'\n' !eq :comment !bcc
    :main !jmp

  output:
    # output assembled byte as hexadecimal
    !hex_putc.min
    ld2 !putc # outputs same whitespace as input
    # !'\s' !putc # outputs a space as whitespace
    # !'.' !putc # outputs '.' for AttoMon
  ignore:
  main:
    # load characters from `stdin` into `tok` on the stack until whitespace
    # or a comment is hit. characters are stored in reverse order
    !tok sts for_c:
      # similar to `!block_getc`, but intertwined for efficiency
      !'\0' block: !char.pop !getc
    :for_c # default: loop to next iteration
      # if `c` is whitespace then we've parsed the entire token
      ld1 xD0 !cl :break iff
      # except if `c` is '#' then parse a comment
      !'#' xo2 :comment iff
      # except if `c` is '\0' then jump back to `getc()`
      !'#' xo2 :block iff
    !jmp break:

    # refer to C implementation for parser logic below
    x00 # chr
    :parsers # parser
    x00 # res
    next: st1
      # while (!IS_SET(*parser++)); parser--
      walk: ld0 lda shl @dyn pop inc :walk !bcc dec
      # chr = tok; chr++ (reverse order)
      !tok dec @const inc
    sw2
    consume: clc
      # parser++, chr-- (reverse order)
      x01 x01 su4 clc ad2
      ld2 lda # *chr
      :next # jump
      ld3 lda # *parser
          # compute `[ctz] = ctz(*chr > `@` ? 0x01 : *chr)`
          ld2 !'@' !gt x00 x01 ld4 iff clc !ctz
        # if (*parser == SIZE) res = [ctz], jump = CONSUME
        !size xo2 if4 :consume if2
          # compute `[rot] = (b << 4) | hex.to_u4(*chr)`
          ld3 x04 rot ld3 !hex.to_u4 orr
        !size xo2
        # if (*parser == HEX) res = [rot], jump = CONSUME
        !hex xo2 if4 :consume if2 !hex xor
        # if (*parser == NONE) jump = IGNORE
        !none xor :ignore if2 !none xor
        # if (*parser == *chr) jump = CONSUME
        xo2 :consume iff st0
        # if (IS_SET(*parser)) res |= *parser == DONE ? 0x00 : *parser, jump = OUTPUT
        x00 ld3 lda shl @dyn iff
          :output if2
        shr
          ld0 !o iff
        or2
      # goto jump
  parsers: !jmp
    !'i' !lit @data !'f' !lit @data !size x90 !set @data # iff, ifS
    !'i' !lit @data xB0 !set @data # inc
    !'d' !lit @data xB1 !set @data # dec
    !'n' !lit @data !'e' !lit @data xB2 !set @data # neg
    !'s' !lit @data !'h' !lit @data !'l' !lit @data xB4 !set @data # shl
    !'s' !lit @data !'h' !lit @data xB5 !set @data # shr
    !'n' !lit @data !'o' !lit @data !'t' !lit @data xB6 !set @data # not
    !'b' !lit @data xB7 !set @data # buf
    !'l' !lit @data !'d' !lit @data !'a' !lit @data xE0 !set @data # lda
    !'s' !lit @data !'t' !lit @data !'a' !lit @data xE1 !set @data # sta
    !'l' !lit @data !'d' !lit @data !'i' !lit @data xE2 !set @data # ldi
    !'s' !lit @data !'t' !lit @data !'i' !lit @data xE3 !set @data # sti
    !'l' !lit @data !'d' !lit @data !'s' !lit @data xE4 !set @data # lds
    !'s' !lit @data !'t' !lit @data !'s' !lit @data xE5 !set @data # sts
    !'a' !lit @data !'d' !lit @data !size @data x80 !set @data # add, adS
    !'s' !lit @data !'u' !lit @data !size @data x84 !set @data # sub, suS
    !'s' !lit @data !'w' !lit @data !size @data x94 !set @data # swp, swS
    !'r' !lit @data !'o' !lit @data !size @data x98 !set @data # rot, roS
    !'o' !lit @data !'r' !lit @data !size @data xA0 !set @data # orr, orS
    !'a' !lit @data !'n' !lit @data !size @data xA4 !set @data # and, anS
    !'x' !lit @data !'o' !lit @data !size @data xA8 !set @data # xor, xoS
    # !'x' !lit @data !'n' !lit @data !size @data xAC !set @data # xnd, xnS
    !'l' !lit @data !'d' !lit @data !hex @data xC0 !set @data # ldO
    !'s' !lit @data !'t' !lit @data !hex @data xD0 !set @data # stO
    !'c' !lit @data xE8 !set @data # clc
    !'s' !lit @data xE9 !set @data # sec
    !'f' !lit @data xEA !set @data # flc
    !'n' !lit @data xEE !set @data # nop
    !'p' !lit @data xEF !set @data # pop
    !'x' !lit @data !hex @data !hex @data !done @data # xXX
    !none @data # output nothing as fallback

lit! x7F and # match literal char then advance
set! x80 orr # OR in bits then return result
done! xFF    # return result
none! x00    # ignore result and return
size! x01    # set to size or 0 then advance
hex! x02     # rotate in hex digit then advance

tok! x00 # address of token buffer
