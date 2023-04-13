# TODO

tasks:

- [x] create "carryful" and "carryless" instruction variants (adc, sbc, shl, shr...)
- [x] add new useful instructions (sta, lda, sto, ldo...)
- [x] merge rol with ror and shl with shr? (replace with `xXX rot` and `xXX shf`)
- [x] `dup` VS `ld0`, `adc` VS `ad0`, `rot` VS `sh0`
- [x] use carry with `inc` and `dec`?
- [x] rotate without carry?
- [x] add macros and labels to IS?
- [x] handle invalid instructions?
- [x] fix `adc` in asm
- [x] add `neg` to asm optimization
- [x] fix `x80` push instruction and opt with `xFX`
- [x] add input capabilities to computer
- [x] come up with clock frequency and add to spec
- [x] buy components for computer
- [ ] build computer
- [ ] come up with microcode

constituents:

- [x] assembler
- [x] emulator
- [x] encoder
- [ ] microcode
- [ ] schematics
- [ ] circuits

microcode:

```
OP = SP + OR

ldo:
[load OR] OP_ADDR ADDR_MEM MEM_DATA DECSP_SP
SP_ADDR ADDR_MEM DATA_MEM DONE

inc:
SP_ADDR ADDR_MEM MEM_DATA DATA_XR
SP_ADDR ADDR_MEM INCXR_DATA DATA_MEM DONE
```
