# Atto-8

A minimalist 8-bit microcomputer with stack-based microprocessor

## Hardware

| Item  | Description          | Default         |
| ----- | -------------------- | --------------- |
| `IP`  | Instruction Pointer  | `0x00`          |
| `SP`  | Stack Pointer        | `0x00`          |
| `CF`  | Carry Flag           | `0x00`          |
| `RAM` | Random Access Memory | `[0x00; 0x100]` |

## Instruction Set

`Instruction` mnemonics directly translate to CPU instructions. `Token` mnemonics are used in Assembly as an abstraction over CPU instructions.

The `*` operator dereferences values from `RAM`.

| `Instruction` | `Token`       | Description               | Operation                                                      | Flags | Opcode              |
| ------------- | ------------- | ------------------------- | -------------------------------------------------------------- | ----- | ------------------- |
|               | `LBL:` `LBL.` | Label Definition          |                                                                |       |                     |
|               | `:LBL` `.LBL` | Label Reference           |                                                                |       |                     |
|               | `MACRO%`      | Macro Definition          |                                                                |       |                     |
|               | `%MACRO`      | Macro Reference           |                                                                |       |                     |
|               | `xXX`         | Push Byte                 |                                                                |       |                     |
| `psh`         |               | Push                      | `XX & 0b01111111 -> *(--SP);`                                  |       | `0b0XXXXXXX`        |
| `phn`         |               | Push Negative             | `XX \| 0b11110000 -> *(--SP);`                                 |       | `0b1111XXXX` `0xFX` |
| `ldo`         | `ldO`         | Load from Offset          | `*(SP + O) -> *(--SP);`                                        |       | `0b1100OOOO` `0xCO` |
| `sto`         | `stO`         | Store to Offset           | `*SP++ -> *(SP + O);`                                          |       | `0b1101OOOO` `0xDO` |
| `add`         | `add` `adS`   | Add                       | `*(SP++) + *(SP + 2 ** S) -> *(SP + 2 ** S);`                  |       | `0b100000SS`        |
| `adc`         | `adc` `acS`   | Add with Carry            | `*(SP++) + *(SP + 2 ** S) -> *(SP + 2 ** S);`                  | #todo | `0b100001SS`        |
| `sub`         | `sub` `suS`   | Subtract                  | `- *(SP++) + *(SP + 2 ** S) -> *(SP + 2 ** S);`                |       | `0b100010SS`        |
| `sbc`         | `sbc` `scS`   | Subtract with Carry       | `- *(SP++) + *(SP + 2 ** S) -> *(SP + 2 ** S);`                | #todo | `0b100011SS`        |
| `shf`         | `shf` `shS`   | Shift                     | #todo                                                          |       | `0b100100SS`        |
| `sfc`         | `sfc` `scS`   | Shift with Carry          | #todo                                                          |       | `0b100101SS`#todo   |
| `rot`         | `rot` `roS`   | Rotate                    | #todo                                                          | #todo | `0b100110SS`        |
| `iff`         | `iff` `ifS`   | If-Then-Else              | `CF ? *((SP++)++) : *((++SP)++ + 2 ** S) -> *(--SP); 0 -> CF;` |       | `0b100111SS`        |
| `orr`         | `orr` `orS`   | Bitwise OR                | `*(SP++) \| *(SP + 2 ** S) -> *(SP + 2 ** S); *SP == 0 -> CF;` |       | `0b101000SS`        |
| `and`         | `and` `anS`   | Bitwise AND               | `*(SP++) & *(SP + 2 ** S) -> *(SP + 2 ** S); *SP == 0 -> CF;`  |       | `0b101001SS`        |
| `xor`         | `xor` `xoS`   | Bitwise XOR               | `*(SP++) ^ *(SP + 2 ** S) -> *(SP + 2 ** S); *SP == 0 -> CF;`  |       | `0b101010SS`        |
| `xnd`         | `xnd` `xnS`   | Bitwise XAND              | `SP++; 0 -> *(SP + 2 ** S); *SP == 0 -> CF;`                   |       | `0b101011SS`        |
| `inc`         | `inc`         | Increment                 | `*SP + 1 -> *SP;`                                              | #todo | `0b10110000`        |
| `dec`         | `dec`         | Decrement                 | `*SP - 1 -> *SP;`                                              | #todo | `0b10110001`        |
| `neg`         | `neg`         | Negate                    | `-*SP -> *SP`                                                  |       | `0b10110010`        |
| `not`         | `not`         | Bitwise NOT               | `!*SP -> *SP; *SP == 0 -> CF;`                                 |       | `0b10110100`        |
| `buf`         | `buf`         | Bitwise Buffer            | `*SP -> *SP; *SP == 0 -> CF;`                                  |       | `0b10110101`        |
| `nop`         | `nop`         | No Operation              | `;`                                                            |       | `0b11100000` `0xE0` |
| `clc`         | `clc`         | Clear Carry               | `0 -> CF;`                                                     |       | `0b11100001` `0xE1` |
| `sec`         | `sec`         | Set Carry                 | `1 -> CF;`                                                     |       | `0b11100010` `0xE2` |
| `flc`         | `flc`         | Flip Carry                | `!CF -> CF;`                                                   |       | `0b11100011` `0xE3` |
| `swp`         | `swp`         | Swap                      | `*(SP++) -> *SP -> *(--SP);`                                   |       | `0b11100100` `0xE4` |
| `pop`         | `pop`         | Pop                       | `SP++;`                                                        |       | `0b11100101` `0xE5` |
| `lda`         | `lda`         | Load from Address         | `*(*(SP++)) -> *(--SP);`                                       |       | `0b11101000` `0xE8` |
| `sta`         | `sta`         | Store to Address          | `*(SP++) -> *(*(SP++));`                                       |       | `0b11101001` `0xE9` |
| `ldi`         | `ldi`         | Load Instruction Pointer  | `IP -> *(--SP);`                                               |       | `0b11101010` `0xEA` |
| `sti`         | `sti`         | Store Instruction Pointer | `*(SP++) -> IP;`                                               |       | `0b11101011` `0xEB` |
| `lds`         | `lds`         | Load Stack Pointer        | `SP -> *(--SP);`                                               |       | `0b11101100` `0xEC` |
| `sts`         | `sts`         | Store Stack Pointer       | `*(SP++) -> SP;`                                               |       | `0b11101101` `0xED` |
| `raw`         | `dDD`         | Raw Data                  |                                                                |       | `0bDDDDDDDD` `0xDD` |
