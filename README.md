# Atto-8

A minimalist 8-bit microcomputer with stack-based microprocessor

## Work in Progress

This is a work in progress. See [/spec](./spec) for more information.

## TODOs

potential improvements:

- [x] create "carryful" and "carryless" instruction variants (adc, sbc, shl, shr...)
- [x] add new useful instructions (sta, lda, sto, ldo...)
- [x] merge rol with ror and shl with shr? (replace with `xXX rot` and `xXX shf`)
- [x] `dup` VS `ld0`, `adc` VS `ad0`, `rot` VS `sh0`
- [ ] use carry with `inc` and `dec`?
- [x] rotate without carry?
- [x] add macros and labels to IS?
- [ ] come up with microcode
- [ ] handle invalid instructions?
- [x] fix `adc` in asm
- [x] add `neg` to asm optimization
- [x] fix `x80` push instruction and opt with `xFX`
- [ ] add input capabilities to computer

constituents:

- assembler
- emulator
- encoder
- microcode
- schematics
- circuits
