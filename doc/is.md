# Instruction Set

## Flags

- `CF` &mdash; Carry Flag
- `DF` &mdash; Debug Flag

## Registers

- `SP` &mdash; Stack Pointer
- `IP` &mdash; Instruction Pointer

## Memory

- `RAM` &mdash; Random Access Memory

## Defaults

| Item  | State |
| ----- | ----- |
| `CF`  | `0`   |
| `DF`  | `0`   |
| `SP`  | `0`   |
| `IP`  | `0`   |
| `RAM` | `0`   |

## Instructions

- The form `AAA` indicates that `AAA` is a `Mnemonic` and that `AAA` is an `Instruction`.
- The form `AAA, BBB` indicates that both `AAA` and `BBB` are `Mnemonic`s and that `AAA` is an `Instruction`.
- The form `AAA (BBB)` indicates that `AAA` is an `Instruction` and that `BBB` is a `Mnemonic`.
- The form `[AAA]` indicates that `AAA` is a `Mnemonic`.

| Mnemonic      | Description               | Operation                                                      | Flags | Opcode              |
| ------------- | ------------------------- | -------------------------------------------------------------- | ----- | ------------------- |
| [`LABEL:`]    | Label Definition          |                                                                |       |                     |
| [`:LABEL`]    | Label Reference           |                                                                |       |                     |
| [`MACRO%`]    | Macro Definition          |                                                                |       |                     |
| [`%MACRO`]    | Macro Reference           |                                                                |       |                     |
| `nop`         | No Operation              | `;`                                                            |       | `0b10100000` `0xA0` |
| `hlt`         | Halt                      | `while(true);`                                                 |       | `0b10101111` `0xAF` |
| `dbg`         | Debug                     | `1 -> DF;`                                                     |       | `0b10101010` `0xAA` |
| `clc`         | Clear Carry               | `0 -> CF;`                                                     |       | `0b10100001` `0xA1` |
| `sec`         | Set Carry                 | `1 -> CF;`                                                     |       | `0b10100010` `0xA2` |
| `flc`         | Flip Carry                | `!CF -> CF;`                                                   |       | `0b10100011` `0xA3` |
| `inc`         | Increment                 | `*SP + 1 -> *SP;`                                              | #todo | `0b01??0000` `0x?0` |
| `dec`         | Decrement                 | `*SP - 1 -> *SP;`                                              | #todo | `0b01??0001` `0x?1` |
| `add`, `adS`  | Add                       | `*(SP++) + *(SP + 2 ** S) -> *(SP + 2 ** S);`                  |       | `0b01SS0010` `0x?2` |
| `adc`, `acS`  | Add with Carry            | `*(SP++) + *(SP + 2 ** S) -> *(SP + 2 ** S);`                  | #todo | `0b01SS0011` `0x?3` |
| `sub`, `suS`  | Subtract                  | `- *(SP++) + *(SP + 2 ** S) -> *(SP + 2 ** S);`                |       | `0b01SS0100` `0x?4` |
| `sbc`, `scS`  | Subtract with Carry       | `- *(SP++) + *(SP + 2 ** S) -> *(SP + 2 ** S);`                | #todo | `0b01SS0101` `0x?5` |
| `shf`, `shS`  | Shift                     | #todo                                                          |       | `0b01SS0110` `0x?6` |
| `rot`, `roS`  | Rotate                    | #todo                                                          | #todo | `0b01SS0111` `0x?7` |
| `orr`, `orS`  | Bitwise OR                | `*(SP++) \| *(SP + 2 ** S) -> *(SP + 2 ** S); *SP == 0 -> CF;` |       | `0b01SS1000` `0x?8` |
| `and`, `anS`  | Bitwise AND               | `*(SP++) & *(SP + 2 ** S) -> *(SP + 2 ** S); *SP == 0 -> CF;`  |       | `0b01SS1001` `0x?9` |
| `xor`, `xoS`  | Bitwise XOR               | `*(SP++) ^ *(SP + 2 ** S) -> *(SP + 2 ** S); *SP == 0 -> CF;`  |       | `0b01SS1010` `0x?A` |
| `xnd`, `xnS`  | Bitwise XAND              | `SP++; 0 -> *(SP + 2 ** S); *SP == 0 -> CF;`                   |       | `0b01SS1011` `0x?B` |
| `not`         | Bitwise NOT               | `~*SP -> *SP; *SP == 0 -> CF;`                                 |       | `0b01??1100` `0x?C` |
| `buf`         | Buffer                    | `*SP -> *SP; *SP == 0 -> CF;`                                  |       | `0b01??1101` `0x?D` |
| `iff`, `ifS`  | If-Then-Else              | `CF ? *((SP++)++) : *((++SP)++ + 2 ** S) -> *(--SP); 0 -> CF;` |       | `0b01SS1110` `0x?E` |
| `swp`         | Swap                      | `*(SP++) -> *SP -> *(--SP);`                                   |       | `0b10110000` `0xB0` |
| `pop`         | Pop                       | `SP++;`                                                        |       | `0b10110001` `0xB1` |
| `phb` (`xXX`) | Push Byte                 | `*(++IP) -> *(--SP);`                                          |       | `0b10110010` `0xB2` |
| `php` (`xXX`) | Push Positive             | `XX & 0b00111111 -> *(--SP);`                                  |       | `0b00XXXXXX` `0x?X` |
| `phn` (`xXX`) | Push Negative             | `XX \| 0b11000000 -> *(--SP);`                                 |       | `0b11XXXXXX` `0x?X` |
| `lda`         | Load from Address         | `*(*(SP++)) -> *(--SP);`                                       |       | `0b10111000` `0xB8` |
| `sta`         | Store to Address          | `*(SP++) -> *(*(SP++));`                                       |       | `0b10111001` `0xB9` |
| `ldi`         | Load Instruction Pointer  | `IP -> *(--SP);`                                               |       | `0b10111010` `0xBA` |
| `sti`         | Store Instruction Pointer | `*(SP++) -> IP;`                                               |       | `0b10111011` `0xBB` |
| `lds`         | Load Stack Pointer        | `SP -> *(--SP);`                                               |       | `0b10111100` `0xBC` |
| `sts`         | Store Stack Pointer       | `*(SP++) -> SP;`                                               |       | `0b10111101` `0xBD` |
| `ldo` (`ldO`) | Load from Offset          | `*(SP + O) -> *(--SP);`                                        |       | `0b1000OOOO` `0x8O` |
| `sto` (`stO`) | Store to Offset           | `*(SP + O) -> *(--SP);`                                        |       | `0b1001OOOO` `0x9O` |
| `raw` (`dDD`) | Raw Data                  |                                                                |       | `0bDDDDDDDD` `0xDD` |

## TODOs

potential instructions:

| neg | Negate | `-*SP -> *SP;` | Carry to carry in, overflow to carry. | |

potential improvements:

- [x] create "carryful" and "carryless" instruction variants (adc, sbc, shl, shr...)
- [x] add new useful instructions (sta, lda, sto, ldo...)
- [x] merge rol with ror and shl with shr? (replace with `xXX rot` and `xXX shf`)
- [x] `dup` VS `ld0`, `adc` VS `ad0`, `rot` VS `sh0`
- [ ] use carry with `inc` and `dec`?
- [ ] rotate without carry?
- [ ] add macros and labels to IS?
