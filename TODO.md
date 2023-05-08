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
- [x] fix assembler stack overflow with self-referencing macros
- [ ] build computer
- [ ] come up with microcode

- potentially remove: neg, not, sub, xnd, clc, sec, flc
- only flappy and prng use `shf` with carry
- instructions removed: adc, sbc, shf, sfc

todos:

- go through all programs (pong and test are the only ones left) and fix them with new `shl` and `shr` instructions
- fix all TODO items
- test `draw` with PRNG
- test `print_byte` (not `_minimal`)
- update instruction set

moving forward:

- [ ] command line interface
- [ ] text editor
- [ ] file system
- [ ] native assembler

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
mem always reads from addr
zero and nonzero always read from data
add, sub, xor, not, etc. always read from x and y

ldo:
[load OR] OP_ADDR MEM_DATA DECSP_SP
SP_ADDR DATA_MEM DONE

inc:
SP_ADDR MEM_DATA x01_ALU
ADD_DATA
SP_ADDR DATA_MEM DONE

dec:
SP_ADDR MEM_DATA x01_ALU
SUB_DATA
SP_ADDR DATA_MEM DONE

neg:
SP_ADDR MEM_DATA DATA_ALU
x00_DATA
SUB_DATA
SP_ADDR DATA_MEM DONE

not:
SP_ADDR MEM_DATA
NOT_DATA
SP_ADDR DATA_MEM DONE

nop:
DONE

clc:
0_DATA NONZERO_CARRY DONE

sec:
0_DATA ZERO_CARRY DONE

flc:
CARRY_DATA ZERO_CARRY DONE

swp:
SP_ADDR MEM_DATA DATA_XR INCSP_SP
#todo

pop:
INCSP_SP DONE

lda:
SP_ADDR MEM_DATA
DATA_ADDR MEM_DATA
SP_ADDR DATA_MEM DONE

ldi:
SP_ADDR IP_DATA DATA_MEM DONE

lds:
DECSP_SP
SP_ADDR SP_DATA DATA_MEM DONE
```
