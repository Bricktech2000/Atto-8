# Atto-8 Microprocessor

## Overview

The Atto-8 microprocessor is a minimalist stack-based processor with 8-bit data and address buses. It is designed to be simple enough to be realistically built from discrete logic gates, but powerful enough to run useful programs. It is intended to be used as a learning tool for students and hobbyists, and as a basis for more complex processors.

## Features

- 8-bit data bus
- 8-bit address bus
- 8-bit instruction size
- 8-bit special-purpose registers
- 0 general-purpose registers
- 0 internal oscillators
- 0 hardware timers
- 0 interrupts
- 0 I/O ports

## Hardware

| Item | Description         |
| ---- | ------------------- |
| `IP` | Instruction Pointer |
| `SP` | Stack Pointer       |
| `CF` | Carry Flag          |

## Instruction Set

All instructions are 8 bits in length and most take no operands.

The processor assumes it can address memory of the shape `[u8; 0x100]`. The `*` operator dereferences values from said memory.

Negative values are represented in two's complement.

| Instruction | Description               | Operation                                                      | Opcode              |
| ----------- | ------------------------- | -------------------------------------------------------------- | ------------------- |
| `psh X`     | Push                      | `XX & 0b01111111 -> *(--SP);`                                  | `0b0XXXXXXX`        |
| `phn X`     | Push Negative             | `XX \| 0b11110000 -> *(--SP);`                                 | `0b1111XXXX` `0xFX` |
| `ldo O`     | Load from Offset          | `*(SP + O) -> *(--SP);`                                        | `0b1100OOOO` `0xCO` |
| `sto O`     | Store to Offset           | `*SP++ -> *(SP + O);`                                          | `0b1101OOOO` `0xDO` |
| `add S`     | Add                       | `*(SP++) + *(SP + 2 ** S) -> *(SP + 2 ** S);`                  | `0b100000SS`        |
| `adc S`     | Add with Carry            | `*(SP++) + *(SP + 2 ** S) -> *(SP + 2 ** S);` #todo flags      | `0b100001SS`        |
| `sub S`     | Subtract                  | `- *(SP++) + *(SP + 2 ** S) -> *(SP + 2 ** S);`                | `0b100010SS`        |
| `sbc S`     | Subtract with Carry       | `- *(SP++) + *(SP + 2 ** S) -> *(SP + 2 ** S);` #todo flags    | `0b100011SS`        |
| `shf S`     | Shift                     | #todo                                                          | `0b100100SS`        |
| `sfc S`     | Shift with Carry          | #todo                                                          | `0b100101SS`#todo   |
| `rot S`     | Rotate                    | #todo #todo flags                                              | `0b100110SS`        |
| `iff S`     | If-Then-Else              | `CF ? *((SP++)++) : *((++SP)++ + 2 ** S) -> *(--SP); 0 -> CF;` | `0b100111SS`        |
| `orr S`     | Bitwise OR                | `*(SP++) \| *(SP + 2 ** S) -> *(SP + 2 ** S); *SP == 0 -> CF;` | `0b101000SS`        |
| `and S`     | Bitwise AND               | `*(SP++) & *(SP + 2 ** S) -> *(SP + 2 ** S); *SP == 0 -> CF;`  | `0b101001SS`        |
| `xor S`     | Bitwise XOR               | `*(SP++) ^ *(SP + 2 ** S) -> *(SP + 2 ** S); *SP == 0 -> CF;`  | `0b101010SS`        |
| `xnd S`     | Bitwise XAND              | `SP++; 0 -> *(SP + 2 ** S); *SP == 0 -> CF;`                   | `0b101011SS`        |
| `inc`       | Increment                 | `*SP + 1 -> *SP;` #todo flags                                  | `0b10110000`        |
| `dec`       | Decrement                 | `*SP - 1 -> *SP;` #todo flags                                  | `0b10110001`        |
| `neg`       | Negate                    | `-*SP -> *SP`                                                  | `0b10110010`        |
| `not`       | Bitwise NOT               | `!*SP -> *SP; *SP == 0 -> CF;`                                 | `0b10110100`        |
| `buf`       | Bitwise Buffer            | `*SP -> *SP; *SP == 0 -> CF;`                                  | `0b10110101`        |
| `nop`       | No Operation              | `;`                                                            | `0b11100000` `0xE0` |
| `clc`       | Clear Carry               | `0 -> CF;`                                                     | `0b11100001` `0xE1` |
| `sec`       | Set Carry                 | `1 -> CF;`                                                     | `0b11100010` `0xE2` |
| `flc`       | Flip Carry                | `!CF -> CF;`                                                   | `0b11100011` `0xE3` |
| `swp`       | Swap                      | `*(SP++) -> *SP -> *(--SP);`                                   | `0b11100100` `0xE4` |
| `pop`       | Pop                       | `SP++;`                                                        | `0b11100101` `0xE5` |
| `lda`       | Load from Address         | `*(*(SP++)) -> *(--SP);`                                       | `0b11101000` `0xE8` |
| `sta`       | Store to Address          | `*(SP++) -> *(*(SP++));`                                       | `0b11101001` `0xE9` |
| `ldi`       | Load Instruction Pointer  | `IP -> *(--SP);`                                               | `0b11101010` `0xEA` |
| `sti`       | Store Instruction Pointer | `*(SP++) -> IP;`                                               | `0b11101011` `0xEB` |
| `lds`       | Load Stack Pointer        | `SP -> *(--SP);`                                               | `0b11101100` `0xEC` |
| `sts`       | Store Stack Pointer       | `*(SP++) -> SP;`                                               | `0b11101101` `0xED` |

## Initial State

The Atto-8 is initialized with the following state:

| Item | Value  |
| ---- | ------ |
| `IP` | `0x00` |
| `SP` | `0x00` |
| `CF` | `0b0`  |

This implies the following:

- Execution starts at address `0x00`.
- The first item to be pushed onto the stack will located be at address `0xFF`.

## Execution

Program execution is as follows:

1. Fetch the instruction at `IP++`.
2. Execute the instruction.
3. Repeat.

## Conventions

The Atto-8 has no inherent endianness. With that said, instructions such as `adS` and `sbS` work best when least significant bytes are at the top of the stack, which grows downward. Consequently, it is recommended that the Atto-8 be assumed to be little-endian.
