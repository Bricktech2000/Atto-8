@ lib/core.asm
@ lib/types.asm
@ lib/stdlib.asm
@ lib/stdio.asm

# Min-Asm, the Atto-8 minimal native assembler
#
# input a stream of assembly mnemonics and Min-Asm will output a stream of bytes as hexadecimal.
# mnemonics must be separated by exactly one whitespace character. input stream is assumed to be
# well-formed; if it is not, the behavior is undefined. unsupported features include directives,
# labels, macros and comments
#
# as an example, `ld0 x61 sub clc x1A sub x7B x5A iff add clc st0 x00 sti` outputs a program that
# capitalizes its input

main! !nop
  loop:
    # load characters from `stdin` into `tok_buffer` onto the stack until
    # until whitespace is hit. characters are stored in reverse order
    !tok_buffer sts for_c:
      !block_getc
    ld0 xD0 !cl :for_c !bcc

    # refer to C implementation for parser logic below
    !'\0' # chr
    :parsers # parser
    x00 # hex
    next: st1
      # while (!IS_SET(*parser++)); parser--
      walk: ld0 lda shl @dyn pop inc :walk !bcc dec
      # chr = tok; chr++ (reverse order)
      !tok_buffer dec @const inc
    sw2
    consume: clc
      # parser++, chr-- (reverse order)
      x01 x01 su4 clc ad2
      ld2 lda # *chr
      :next # jump
      ld3 lda # *parser
          # compute `[ctz] = ctz(*chr > `@` ? 0x01 : *chr)`
          ld2 !'@' !gt x00 x01 ld4 iff clc !ctz
        # if (*parser == SIZE) hex = [ctz], jump = CONSUME
        !size xo2 if4 :consume if2
          # compute `[hex] = (b << 4) | hex.to_u4(*chr)`
          ld3 x04 rot ld3 !hex.to_u4 orr
        !size xo2
        # if (*parser == HEX) hex = [hex], jump = CONSUME
        !hex xo2 if4 :consume if2 !hex xo2
        # if (*parser == *chr) jump = CONSUME
        xo2 :consume iff st0
        # if (IS_SET(*parser)) hex |= *parser == DONE ? 0x00 : *parser, jump = RETURN
        x00 ld3 lda shl @dyn iff
          :return if2
        shr
          ld0 !o iff
        or2
      # goto jump
      !jmp
    return:

    # output assembled byte as hexadecimal
    !u8.to_hex !putc !putc
    ld2 !putc # outputs same whitespace as input
    # !'\s' !putc # outputs a space as whitespace
    # !'.' !putc # outputs `.` for AttoMon
  :loop !jmp

  parsers:
    !done @data
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
    !'x' !lit @data !'n' !lit @data !size @data xAC !set @data # xnd, xnS
    !'l' !lit @data !'d' !lit @data !hex @data xC0 !set @data # ldO
    !'s' !lit @data !'t' !lit @data !hex @data xD0 !set @data # stO
    !'c' !lit @data xE8 !set @data # clc
    !'s' !lit @data xE9 !set @data # sec
    !'f' !lit @data xEA !set @data # flc
    !'n' !lit @data xEE !set @data # nop
    !'p' !lit @data xEF !set @data # pop
    !'x' !lit @data !hex @data !hex @data !done @data # xXX
    xBB !set @data # `!dbg` as fallback

lit! x7F and # match literal char then advance
set! x80 orr # OR in bits then done parsing
done! xFF    # done parsing
hex! x00     # rotate in hex digit then advance
size! x01    # set to size or 0 then advance

tok_buffer! x00
