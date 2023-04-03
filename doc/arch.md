# Architecture

## Hardware

| Item  | Description          | Default         |
| ----- | -------------------- | --------------- |
| `IP`  | Instruction Pointer  | `0x00`          |
| `SP`  | Stack Pointer        | `0x00`          |
| `CF`  | Carry Flag           | `0x00`          |
| `DF`  | Debug Flag           | `0x00`          |
| `RAM` | Random Access Memory | `[0x00; 0x100]` |

## Instruction Set

`Instruction` mnemonics directly translate to CPU instructions. `Token` mnemonics are used in Assembly as an abstraction over CPU instructions.

The `*` operator dereferences values from `RAM`.

| `Instruction` | `Token`     | Description               | Operation                                                      | Flags | Opcode              |
| ------------- | ----------- | ------------------------- | -------------------------------------------------------------- | ----- | ------------------- |
|               | `LABEL:`    | Label Definition          |                                                                |       |                     |
|               | `:LABEL`    | Label Reference           |                                                                |       |                     |
|               | `MACRO%`    | Macro Definition          |                                                                |       |                     |
|               | `%MACRO`    | Macro Reference           |                                                                |       |                     |
|               | `xXX`       | Push Byte                 |                                                                |       |                     |
| `nop`         | `nop`       | No Operation              | `;`                                                            |       | `0b10100000` `0xA0` |
| `hlt`         | `hlt`       | Halt                      | `loop {}`                                                      |       | `0b10101111` `0xAF` |
| `dbg`         | `dbg`       | Debug                     | `1 -> DF;`                                                     |       | `0b10101010` `0xAA` |
| `clc`         | `clc`       | Clear Carry               | `0 -> CF;`                                                     |       | `0b10100001` `0xA1` |
| `sec`         | `sec`       | Set Carry                 | `1 -> CF;`                                                     |       | `0b10100010` `0xA2` |
| `flc`         | `flc`       | Flip Carry                | `!CF -> CF;`                                                   |       | `0b10100011` `0xA3` |
| `inc`         | `inc`       | Increment                 | `*SP + 1 -> *SP;`                                              | #todo | `0b11??0000` `0x?0` |
| `dec`         | `dec`       | Decrement                 | `*SP - 1 -> *SP;`                                              | #todo | `0b11??0001` `0x?1` |
| `add`         | `add` `adS` | Add                       | `*(SP++) + *(SP + 2 ** S) -> *(SP + 2 ** S);`                  |       | `0b11SS0010` `0x?2` |
| `adc`         | `adc` `acS` | Add with Carry            | `*(SP++) + *(SP + 2 ** S) -> *(SP + 2 ** S);`                  | #todo | `0b11SS0011` `0x?3` |
| `sub`         | `sub` `suS` | Subtract                  | `- *(SP++) + *(SP + 2 ** S) -> *(SP + 2 ** S);`                |       | `0b11SS0100` `0x?4` |
| `sbc`         | `sbc` `scS` | Subtract with Carry       | `- *(SP++) + *(SP + 2 ** S) -> *(SP + 2 ** S);`                | #todo | `0b11SS0101` `0x?5` |
| `shf`         | `shf` `shS` | Shift                     | #todo                                                          |       | `0b11SS0110` `0x?6` |
| `rot`         | `rot` `roS` | Rotate                    | #todo                                                          | #todo | `0b11SS0111` `0x?7` |
| `orr`         | `orr` `orS` | Bitwise OR                | `*(SP++) \| *(SP + 2 ** S) -> *(SP + 2 ** S); *SP == 0 -> CF;` |       | `0b11SS1000` `0x?8` |
| `and`         | `and` `anS` | Bitwise AND               | `*(SP++) & *(SP + 2 ** S) -> *(SP + 2 ** S); *SP == 0 -> CF;`  |       | `0b11SS1001` `0x?9` |
| `xor`         | `xor` `xoS` | Bitwise XOR               | `*(SP++) ^ *(SP + 2 ** S) -> *(SP + 2 ** S); *SP == 0 -> CF;`  |       | `0b11SS1010` `0x?A` |
| `xnd`         | `xnd` `xnS` | Bitwise XAND              | `SP++; 0 -> *(SP + 2 ** S); *SP == 0 -> CF;`                   |       | `0b11SS1011` `0x?B` |
| `not`         | `not`       | Bitwise NOT               | `!*SP -> *SP; *SP == 0 -> CF;`                                 |       | `0b11??1100` `0x?C` |
| `buf`         | `buf`       | Buffer                    | `*SP -> *SP; *SP == 0 -> CF;`                                  |       | `0b11??1101` `0x?D` |
| `iff`         | `iff` `ifS` | If-Then-Else              | `CF ? *((SP++)++) : *((++SP)++ + 2 ** S) -> *(--SP); 0 -> CF;` |       | `0b11SS1110` `0x?E` |
| `swp`         | `swp`       | Swap                      | `*(SP++) -> *SP -> *(--SP);`                                   |       | `0b10110000` `0xB0` |
| `pop`         | `pop`       | Pop                       | `SP++;`                                                        |       | `0b10110001` `0xB1` |
| `phs`         |             | Push Small                | `XX & 0b00111111 \| 0b00000000 -> *(--SP);`                    |       | `0b00XXXXXX` `0x?X` |
| `phl`         |             | Push Large                | `XX & 0b00111111 \| 0b01000000 -> *(--SP);`                    |       | `0b01XXXXXX` `0x?X` |
| `lda`         | `lda`       | Load from Address         | `*(*(SP++)) -> *(--SP);`                                       |       | `0b10111000` `0xB8` |
| `sta`         | `sta`       | Store to Address          | `*(SP++) -> *(*(SP++));`                                       |       | `0b10111001` `0xB9` |
| `ldi`         | `ldi`       | Load Instruction Pointer  | `IP -> *(--SP);`                                               |       | `0b10111010` `0xBA` |
| `sti`         | `sti`       | Store Instruction Pointer | `*(SP++) -> IP;`                                               |       | `0b10111011` `0xBB` |
| `lds`         | `lds`       | Load Stack Pointer        | `SP -> *(--SP);`                                               |       | `0b10111100` `0xBC` |
| `sts`         | `sts`       | Store Stack Pointer       | `*(SP++) -> SP;`                                               |       | `0b10111101` `0xBD` |
| `ldo`         | `ldO`       | Load from Offset          | `*(SP + O) -> *(--SP);`                                        |       | `0b1000OOOO` `0x8O` |
| `sto`         | `stO`       | Store to Offset           | `*(SP + O) -> *(--SP);`                                        |       | `0b1001OOOO` `0x9O` |
| `raw`         | `dDD`       | Raw Data                  |                                                                |       | `0bDDDDDDDD` `0xDD` |

## TODOs

potential improvements:

- [x] create "carryful" and "carryless" instruction variants (adc, sbc, shl, shr...)
- [x] add new useful instructions (sta, lda, sto, ldo...)
- [x] merge rol with ror and shl with shr? (replace with `xXX rot` and `xXX shf`)
- [x] `dup` VS `ld0`, `adc` VS `ad0`, `rot` VS `sh0`
- [ ] use carry with `inc` and `dec`?
- [ ] rotate without carry?
- [x] add macros and labels to IS?
