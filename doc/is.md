# Instruction Set

## Flags

- `CF` &mdsh; Carry Flag
- `DF` &mdsh; Debug Flag

## Registers

- `SP` &mdash; Stack Pointer
- `WP` &mdash; Work Pointer
- `IP` &mdash; Instruction Pointer

## Defaults

| Item   | State |
| ------ | ----- |
| `CF`   | `0`   |
| `DF`   | `0`   |
| `SP`   | `0`   |
| `WP`   | `0`   |
| `IP`   | `0`   |
| memory | `0`   |

## Instructions

| Mnemonic | Description               | Operation                                             | Flags                                  | Opcode              |
| -------- | ------------------------- | ----------------------------------------------------- | -------------------------------------- | ------------------- |
| nop      | No Operation              | `;`                                                   |                                        | `0b10000000` `0x80` |
| hlt      | Halt                      | `while(true);`                                        |                                        | `0b10000001` `0x81` |
| dbg      | Debug                     | `1 -> DF;`                                            | Set debug.                             | `0b10001000` `0x88` |
| clc      | Clear Carry               | `0 -> CF;`                                            | Clear carry.                           | `0b10001001` `0x82` |
| sec      | Set Carry                 | `1 -> CF;`                                            | Set carry.                             | `0b10001010` `0x83` |
| flc      | Flip Carry                | `!CF -> CF;`                                          | Flip carry.                            | `0b10001011` `0x84` |
| inc      | Increment                 | `*WP + 1 -> *WP;`                                     | Carry to carry in, overflow to carry.  | `0b10100000` `0xA0` |
| dec      | Decrement                 | `*WP - 1 -> *WP;`                                     | Carry to carry in, underflow to carry. | `0b10100001` `0xA1` |
| add      | Add                       | `*(SP--) + *WP -> *WP;`                               | Carry to carry in, overflow to carry.  | `0b10100010` `0xA2` |
| sub      | Subtract                  | `*(SP--) - *WP -> *WP;`                               | Carry to carry in, underflow to carry. | `0b10100011` `0xA3` |
| rol      | Rotate Left               | `*WP << 1 -> *WP;`                                    | Carry to LSB, MSB to carry.            | `0b10100100` `0xA4` |
| ror      | Rotate Right              | `*WP >> 1 -> *WP;`                                    | Carry to MSB, LSB to carry.            | `0b10100101` `0xA5` |
| oor      | Bitwise OR                | `*(SP--) \| *WP -> *WP;`                              | Set carry if zero. Clear otherwise.    | `0b10100110` `0xA6` |
| and      | Bitwise AND               | `*(SP--) & *WP -> *WP;`                               | Set carry if zero. Clear otherwise.    | `0b10100111` `0xA7` |
| xor      | Bitwise XOR               | `*(SP--) ^ *WP -> *WP;`                               | Set carry if zero. Clear otherwise.    | `0b10101000` `0xA8` |
| xnd      | Bitwise XAND              | `*(SP--) & 0 -> *WP;`                                 | Set carry if zero. Clear otherwise.    | `0b10101001` `0xA9` |
| not      | Bitwise NOT               | `~*WP -> *WP;`                                        | Set carry if zero. Clear otherwise.    | `0b10101010` `0xAA` |
| iif      | If-Then-Else              | `CF ? *((SP--)--) : *((--SP)--) -> *(++SP); 0 -> CF;` | Carry as condition. Clear carry.       | `0b10010000` `0x90` |
| swp      | Swap                      | `*(SP--) -> *WP -> *(++SP);`                          |                                        | `0b10010001` `0x91` |
| dup      | Duplicate                 | `*WP -> *(++SP);`                                     |                                        | `0b10010010` `0x92` |
| str      | Store                     | `*(SP--) -> *WP;`                                     |                                        | `0b10010011` `0x93` |
| pop      | Pop                       | `SP--;`                                               |                                        | `0b10010100` `0x94` |
| xXX      | Push Positive             | `XX \| 0x00 -> *(++SP);`                              |                                        | `0b00XXXXXX`        |
| xXX      | Push Negative             | `XX \| 0xFF -> *(++SP);`                              |                                        | `0b11XXXXXX`        |
| xXX      | Push Next                 | `*(++IP) -> *(++SP);`                                 |                                        | `0b10010101` `0x95` |
| @WW      | Relative Work             | `(WW + SP) -> WP;`                                    |                                        | `0b01WWWWWW`        |
| dDD      | Raw Data                  |                                                       |                                        | `0bDDDDDDDD` `0xDD` |
| ldi      | Load Instruction Pointer  | `IP -> *(++SP);`                                      |                                        | `0b10010110` `0x96` |
| sti      | Store Instruction Pointer | `*(SP--) -> IP;`                                      |                                        | `0b10010111` `0x97` |
| ldw      | Load Work Pointer         | `*WP -> *(++SP);`                                     |                                        | `0b10011000` `0x98` |
| stw      | Store Work Pointer        | `*(SP--) -> *WP;`                                     |                                        | `0b10011001` `0x99` |
| lds      | Load Stack Pointer        | `SP -> *(++SP);`                                      |                                        | `0b10011010` `0x9A` |
| sts      | Store Stack Pointer       | `*(SP--) -> SP;`                                      |                                        | `0b10011011` `0x9B` |
