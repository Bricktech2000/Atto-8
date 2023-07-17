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
- [x] get rid of `adn` instruction
- [x] create `u4` and `i4` pair types
- [x] update programs with new types
- [ ] come up with microcode
- [ ] build computer

- potentially remove: neg, not, sub, xnd, clc, sec, flc
- only flappy and prng use `shf` with carry
- instructions removed: adc, sbc, shf, sfc

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

---

terminal IO:

reads to `0x00`:

```
if joystick engaged:
  return joystick state
elif stdin not empty:
  return then clear stdin
else:
  return memory at 0x00
```

writes to `0x00`:

```
write to memory at 0x00
write to stdout
```

todos:

- [x] show SP and IP in MEM
- [x] make `emu.rs` not overwrite on pop
- [x] rename `input_buffer` in `emu.rs`
- [ ] implement `gets`
- [x] figure out `emu.rs` printout
- [ ] implement reads to `0x00` as described above and fix the following programs:
  - [ ] hllwrld
  - [ ] random
  - [ ] pong
  - [ ] flappy

---

microcode:

```
OP = SP + OL
mem always reads from addr
zero and nonzero always read from data
add, sub, xor, not, etc. always read from x and y

ldo:
[load OL] OP_ADDR MEM_DATA DECSP_SP
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
ZERO_DATA NONZERO_CARRY DONE

sec:
ZERO_DATA ZERO_CARRY DONE

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
SP_ADDR OP_DATA DATA_MEM DONE

---

fetch:
IP_ADDR MEM_DATA DATA_IR ZERO_OL

nop:
IP_ADDR MEM_DATA DATA_IR ZERO_OL
ZERO_SC

inc:
IP_ADDR MEM_DATA DATA_IR ZERO_OL
SP_ADDR MEM_DATA DATA_AL
...
SP_ADDR DATA_MEM ..._DATA
```

- A latch: `AL`
- B latch: `BL`
- offset latch: `OL`
- step counter: `SC`
- instruction register: `IR`
- instruction pointer: `IP`
- stack pointer: `SP`

`OP_DATA` will put `OL + SP` on the bus

- `add`: `AL + BL`
- `sub`: `~AL + BL + 1`
- `rot`: `AL << BL`
- `orr`:
- `and`:
- `xor`:
- `xnd`: `0`, is_zero to carry
- `inc`: `AL + 1`
- `dec`: `AL + ~0`
- `neg`: `~AL + 1`
- `shl`: `AL + AL`
- `shr`: `(AL >> 1 + AL >> 1) >> 1`
- `not`: `~AL`, is_zero to carry
- `buf`: `AL`, is_zero to carry

things needed:

- carry in
- ~~half carry enable~~
- sum to data
- zero to bl
- rot to data
- [ ] not
- [ ] latch carry
