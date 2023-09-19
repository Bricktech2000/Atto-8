# Atto-8 Microarchitecture

## Overview

The Atto-8 microarchitecture is a minimalist stack-based architecture with 8-bit data and address buses. It is designed to be simple enough to be realistically built from discrete logic ICs, yet powerful enough to run useful programs. It is intended to be used as a learning tool for students and hobbyists, and as a basis for more complex architectures.

## Features

- 8-bit data bus
- 8-bit address bus
- 8-bit instruction size
- 8-bit special-purpose registers
- 0 general-purpose registers
- 0 internal oscillators
- 0 hardware timers
- 0 memory banks
- 0 interrupts
- 0 I/O ports

## Components

| Component | Name                | Size   |
| --------- | ------------------- | ------ |
| `IP`      | Instruction Pointer | 8 bits |
| `SP`      | Stack Pointer       | 8 bits |
| `CF`      | Carry Flag          | 1 bit  |

`IP` is a pointer to the instruction being executed in memory. Writing to `IP` through `sti` will cause a jump to the specified address. `IP` is incremented after the execution of each instruction.

`SP` is a pointer to the item at top of the stack, which grows downward. Writing to `SP` through `sts` while the stack is empty will move the location of the stack in memory. Instructions increment or decrement `SP` as needed.

`CF` is a one-bit status flag shared by various instructions. As a rule of thumb:

- `CF` is set by `sec`, cleared by `clc` and flipped by `flc`. `CF` is cleared by `rot`.
- `CF` is set when a logical operation (`orr`, `and`, `xor`, `xnd`, `not`, `buf`) results in `0x00` and cleared otherwise.
- `CF` is set when an arithmetic operation (`add`, `sub`, `shl`, `shr`) overflows or underflows and cleared otherwise.
- `CF` is used as carry-in for arithmetic operations (`add`, `sub`, `shl`, `shr`).
- `CF` is used as the condition operand for the conditional instruction `iff`.

## Instruction Set

All instructions are 8 bits in length and most operands are sourced from the stack. The architecture assumes it can address memory of the shape `[u8; 0x100]`. The `*` operator dereferences values from said memory.

| Instruction | Name                      | Operation                                                                   | Opcode                |
| ----------- | ------------------------- | --------------------------------------------------------------------------- | --------------------- |
| `psh IMM`   | Push                      | `IMM & 0b01111111 -> *(--SP);`                                              | `0b0IIIIIII` (`0xII`) |
| `add SIZE`  | Add with Carry            | `*(SP++) + *(SP + SIZE) + CF -> *(SP + SIZE); *SP > 0xFF -> CF;`            | `0b100000SS`          |
| `sub SIZE`  | Subtract with Carry       | `-*(SP++) + *(SP + SIZE) - CF -> *(SP + SIZE); *SP < 0x00 -> CF;`           | `0b100001SS`          |
| `iff SIZE`  | Conditional with Carry    | `CF ? *((SP++)++) : *((++SP)++ + SIZE) -> *(--SP);`                         | `0b100100SS`          |
| `rot SIZE`  | Rotate                    | `(*(SP + SIZE) << *SP) \| ((*(SP + SIZE) << *SP) >> 8) -> *(++SP); 0 -> CF` | `0b100101SS`          |
| `orr SIZE`  | Bitwise OR                | `*(SP++) \| *(SP + SIZE) -> *(SP + SIZE); *SP == 0 -> CF;`                  | `0b101000SS`          |
| `and SIZE`  | Bitwise AND               | `*(SP++) & *(SP + SIZE) -> *(SP + SIZE); *SP == 0 -> CF;`                   | `0b101001SS`          |
| `xor SIZE`  | Bitwise XOR               | `*(SP++) ^ *(SP + SIZE) -> *(SP + SIZE); *SP == 0 -> CF;`                   | `0b101010SS`          |
| `xnd SIZE`  | Bitwise XAND              | `SP++; 0 -> *(SP + SIZE); *SP == 0 -> CF;`                                  | `0b101011SS`          |
| `inc`       | Increment                 | `*SP + 1 -> *SP;`                                                           | `0b10110000`          |
| `dec`       | Decrement                 | `*SP - 1 -> *SP;`                                                           | `0b10110001`          |
| `neg`       | Negate                    | `-*SP -> *SP`                                                               | `0b10110010`          |
| `shl`       | Shift Left with Carry     | `(*SP & 0x80) -> CF; (*SP << 1) -> *SP;`                                    | `0b10110100`          |
| `shr`       | Shift Right with Carry    | `(*SP & 0x01) -> CF; (*SP >> 1) -> *SP;`                                    | `0b10110101`          |
| `not`       | Bitwise NOT               | `!*SP -> *SP; *SP == 0 -> CF;`                                              | `0b10110110`          |
| `buf`       | Bitwise Buffer            | `*SP -> *SP; *SP == 0 -> CF;`                                               | `0b10110111`          |
| `ldo OFST`  | Load from Offset          | `*(SP + OFST) -> *(--SP);`                                                  | `0b1100OOOO` (`0xCO`) |
| `sto OFST`  | Store to Offset           | `*SP++ -> *(SP + OFST);`                                                    | `0b1101OOOO` (`0xDO`) |
| `lda`       | Load from Address         | `*(*(SP++)) -> *(--SP);`                                                    | `0b11101000` (`0xE0`) |
| `sta`       | Store to Address          | `*(SP++ + 1) -> *(*(SP++ - 1));`                                            | `0b11101001` (`0xE1`) |
| `ldi`       | Load Instruction Pointer  | `IP -> *(--SP);`                                                            | `0b11101010` (`0xE2`) |
| `sti`       | Store Instruction Pointer | `*(SP++) -> IP;`                                                            | `0b11101011` (`0xE3`) |
| `lds`       | Load Stack Pointer        | `SP -> *(--SP);`                                                            | `0b11101100` (`0xE4`) |
| `sts`       | Store Stack Pointer       | `*(SP++) -> SP;`                                                            | `0b11101101` (`0xE5`) |
| `nop`       | No Operation              | `;`                                                                         | `0b11100000` (`0xE8`) |
| `clc`       | Clear Carry               | `0 -> CF;`                                                                  | `0b11100001` (`0xE9`) |
| `sec`       | Set Carry                 | `1 -> CF;`                                                                  | `0b11100010` (`0xEA`) |
| `flc`       | Flip Carry                | `!CF -> CF;`                                                                | `0b11100011` (`0xEB`) |
| `swp`       | Swap                      | `*(SP++) -> *SP -> *(--SP);`                                                | `0b11100100` (`0xEC`) |
| `pop`       | Pop                       | `0 -> *(SP++);`                                                             | `0b11100101` (`0xED`) |
| `phn IMM`   | Push Negative             | `IMM \| 0b11110000 -> *(--SP);`                                             | `0b1111IIII` (`0xFI`) |

`SIZE` operands are encoded in such a way that `2 ** SS == SIZE`. Consequently, `SIZE` may only be one of `1`, `2`, `4`, `8`.

Negative values are represented in two's complement.

## Initial State

The Atto-8 microarchitecture is initialized with the following state:

| Commponent | Value  |
| ---------- | ------ |
| `IP`       | `0x00` |
| `SP`       | `0x00` |
| `CF`       | `0b0`  |

This implies that:

- Execution begins at address `0x00`.
- The first item to be pushed onto the stack will located be at address `0xFF`.

## Conventions

The Atto-8 microarchitecture has no inherent endianness. With that said, instructions such as `add SIZE` and `sub SIZE` work best when least significant bytes are at the top of the stack, which grows downward. Consequently, it is recommended that the Atto-8 microarchitecture be assumed to be little-endian.
