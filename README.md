# Atto-8

A minimalist 8-bit microcomputer with stack-based microprocessor

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

### doc

#### processor

- stack grows to lower addresses
- starts execution at 0x00
- uses 2's complement for negatives

conventions:

- calling conventions (clear parameters)
- little endian (and LSB on top of stack)

#### computer

- display buffer
- clock frequency
- ram chip?

conventions:

- use 0xE0.. as general purpose inputs/outputs

---

cargo folder structure

software/documentation

processor/computer

assembler/emulator/encoder/microcode/schematics/circuits
