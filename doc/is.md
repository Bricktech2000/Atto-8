# Instruction Set

## Flags

- `CF` &mdsh; Carry Flag
- `DF` &mdsh; Debug Flag

## Registers

- `SP` &mdash; Stack Pointer
- `IP` &mdash; Instruction Pointer

## Defaults

| Item   | State |
| ------ | ----- |
| `CF`   | `0`   |
| `DF`   | `0`   |
| `SP`   | `0`   |
| `IP`   | `0`   |
| memory | `0`   |

## Instructions

| Mnemonic     | Description               | Operation                                             | Flags                                  | Opcode              |
| ------------ | ------------------------- | ----------------------------------------------------- | -------------------------------------- | ------------------- |
| `nop`        | No Operation              | `;`                                                   |                                        | `0b10100000` `0xA0` |
| `hlt`        | Halt                      | `while(true);`                                        |                                        | `0b10101111` `0xAF` |
| `dbg`        | Debug                     | `1 -> DF;`                                            | Set debug.                             | `0b10101010` `0xAA` |
| `clc`        | Clear Carry               | `0 -> CF;`                                            | Clear carry.                           | `0b10100001` `0xA1` |
| `sec`        | Set Carry                 | `1 -> CF;`                                            | Set carry.                             | `0b10100010` `0xA2` |
| `flc`        | Flip Carry                | `!CF -> CF;`                                          | Flip carry.                            | `0b10100011` `0xA3` |
| `inc`        | Increment                 | `*SP + 1 -> *SP;`                                     | Carry to carry in, overflow to carry.  | `0b01??0000`        |
| `dec`        | Decrement                 | `*SP - 1 -> *SP;`                                     | Carry to carry in, underflow to carry. | `0b01??0001`        |
| `add`, `adS` | Add                       | `*(SP--) + *SP -> *SP;`                               | #todo                                  | `0b01SS0010`        |
| `adc`, `acS` | Add with Carry            | `*(SP--) + *SP -> *SP;`                               | #todo                                  | `0b01SS0011`        |
| `sub`, `suS` | Subtract                  | `*(SP--) - *SP -> *SP;`                               | #todo                                  | `0b01SS0100`        |
| `sbc`, `scS` | Subtract with Carry       | `*(SP--) - *SP -> *SP;`                               | #todo                                  | `0b01SS0101`        |
| `shf`, `shS` | Shift                     | #todo                                                 | #todo                                  | `0b01SS0110`        |
| `rot`, `roS` | Rotate                    | #todo                                                 | #todo                                  | `0b01SS0111`        |
| `orr`, `orS` | Bitwise OR                | `*(SP--) \| *SP -> *SP;`                              | Set carry if zero. Clear otherwise.    | `0b01SS1000`        |
| `and`, `anS` | Bitwise AND               | `*(SP--) & *SP -> *SP;`                               | Set carry if zero. Clear otherwise.    | `0b01SS1001`        |
| `xor`, `xoS` | Bitwise XOR               | `*(SP--) ^ *SP -> *SP;`                               | Set carry if zero. Clear otherwise.    | `0b01SS1010`        |
| `xnd`, `xnS` | Bitwise XAND              | `*(SP--) & 0 -> *SP;`                                 | Set carry if zero. Clear otherwise.    | `0b01SS1011`        |
| `not`        | Bitwise NOT               | `~*SP -> *SP;`                                        | Set carry if zero. Clear otherwise.    | `0b01??1100`        |
| `iff`, `ifS` | If-Then-Else              | `CF ? *((SP--)--) : *((--SP)--) -> *(++SP); 0 -> CF;` | Carry as condition.                    | `0b01SS1101`        |
| `swp`        | Swap                      | `*(SP--) -> *SP -> *(++SP);`                          |                                        | `0b10110000` `0xB0` |
| `pop`        | Pop                       | `SP--;`                                               |                                        | `0b10110001` `0xB1` |
| `xXX`        | Push Next                 | `*(++IP) -> *(++SP);`                                 |                                        | `0b10110010` `0xB2` |
| `xXX`        | Push Positive             | `XX \& 0b00111111 -> *(++SP);`                        |                                        | `0b00XXXXXX`        |
| `xXX`        | Push Negative             | `XX \| 0b11000000 -> *(++SP);`                        |                                        | `0b11XXXXXX`        |
| `dup`, `ldO` | Load from Offset          | `*(SP + O) -> *(++SP);`                               |                                        | `0b1000OOOO` `0x8O` |
| `str`, `stO` | Store to Offset           | `*(SP + O) -> *(++SP);`                               |                                        | `0b1001OOOO` `0x9O` |
| `lda`        | Load from Address         | `*(*(SP--)) -> *(++SP);`                              |                                        | `0b10111000` `0xB8` |
| `sta`        | Store to Address          | `*(SP--) -> *(*(SP--));`                              |                                        | `0b10111001` `0xB9` |
| `ldi`        | Load Instruction Pointer  | `IP -> *(++SP);`                                      |                                        | `0b10111010` `0xBA` |
| `sti`        | Store Instruction Pointer | `*(SP--) -> IP;`                                      |                                        | `0b10111011` `0xBB` |
| `lds`        | Load Stack Pointer        | `SP -> *(++SP);`                                      |                                        | `0b10111100` `0xBC` |
| `sts`        | Store Stack Pointer       | `*(SP--) -> SP;`                                      |                                        | `0b10111101` `0xBD` |
| `dDD`        | Raw Data                  |                                                       |                                        | `0bDDDDDDDD` `0xDD` |

## TODOs

potential instructions:

| neg | Negate | `-*SP -> *SP;` | Carry to carry in, overflow to carry. | |

potential improvements:

- [x] create "carryful" and "carryless" instruction variants (adc, sbc, shl, shr...)
- [x] add new useful instructions (sta, lda, sto, ldo...)
- [x] merge rol with ror and shl with shr? (replace with `xXX rot` and `xXX shf`)
- [x] `dup` VS `ld0`, `adc` VS `ad0`, `rot` VS `sh0`
- [ ] use carry with `inc` and `dec`?

- 0b00XXXXXX push positive
- 0b11XXXXXX push negative
- 0b01SSIIII (arithmetic IIII with size SS)
- 0b1000OOOO ldO
- 0b1001OOOO stO
- 0b1010
- 0b1011

SS:

- 00: 1 byte
- 01: 2 bytes
- 10: 4 bytes
- 11: 8 bytes
