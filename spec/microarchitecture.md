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

`IP` is a pointer to the next instruction to be executed. Writing to `IP` through `sti` will cause a jump to the specified address. `IP` is incremented before the execution of every instruction.

`SP` is a pointer to the item at top of the stack, which grows downward. Writing to `SP` through `sts` while the stack is empty will move the location of the stack in memory. Instructions increment or decrement `SP` as needed.

`CF` is a one-bit status flag shared by various instructions. As a rule of thumb:

- `CF` is set by `sec`, cleared by `clc` and flipped by `flc`. `CF` is cleared by `rot`.
- `CF` is set when a logical operation (`orr`, `and`, `xor`, `xnd`, `not`, `buf`) results in `0x00` and cleared otherwise.
- `CF` is set when an arithmetic operation (`add`, `sub`, `shl`, `shr`) overflows or underflows and cleared otherwise.
- `CF` is used as carry-in for arithmetic operations (`add`, `sub`, `shl`, `shr`).
- `CF` is used as the condition operand for the conditional instruction `iff`.

## Instruction Set

All instructions are 8 bits in length and most operands are sourced from the stack. The architecture assumes memory of the shape `[u8; 0x100]`, from which the `*` operator dereferences values.

| Instruction | Name                      | Operation                                                                               | Opcode                       |
| ----------- | ------------------------- | --------------------------------------------------------------------------------------- | ---------------------------- |
| `psh IMM`   | Push                      | `SP--; IMM -> *SP;`                                                                     | `0b0IIIIIII` (`0x00..=0x7F`) |
| `add SIZE`  | Add with Carry            | `*SP + *(SP + SIZE) + CF -> *(SP + SIZE); *(SP + SIZE) > 0xFF -> CF; SP++;`             | `0b100000SS` (`0x80..=0x83`) |
| `sub SIZE`  | Subtract with Carry       | `-*SP + *(SP + SIZE) - CF -> *(SP + SIZE); *(SP + SIZE) < 0x00 -> CF; SP++;`            | `0b100001SS` (`0x84..=0x87`) |
| `iff SIZE`  | Conditional with Carry    | `CF ? *SP : *(SP + SIZE) -> *(SP + SIZE); SP++;`                                        | `0b100100SS` (`0x90..=0x93`) |
| `swp SIZE`  | Swap                      | `*(SP + SIZE) <-> *SP;`                                                                 | `0b100101SS` (`0x94..=0x97`) |
| `rot SIZE`  | Rotate                    | `(*(SP + SIZE) << *SP) \| (*(SP + SIZE) << *SP >> 8) -> *(SP + SIZE); 0b0 -> CF; SP++;` | `0b100110SS` (`0x98..=0x9B`) |
| `orr SIZE`  | Bitwise OR                | `*SP \| *(SP + SIZE) -> *(SP + SIZE); *(SP + SIZE) == 0b0 -> CF; SP++;`                 | `0b101000SS` (`0xA0..=0xA3`) |
| `and SIZE`  | Bitwise AND               | `*SP & *(SP + SIZE) -> *(SP + SIZE); *(SP + SIZE) == 0b0 -> CF; SP++;`                  | `0b101001SS` (`0xA4..=0xA7`) |
| `xor SIZE`  | Bitwise XOR               | `*SP ^ *(SP + SIZE) -> *(SP + SIZE); *(SP + SIZE) == 0b0 -> CF; SP++;`                  | `0b101010SS` (`0xA8..=0xAB`) |
| `xnd SIZE`  | Bitwise XAND              | `0x00 -> *(SP + SIZE); *(SP + SIZE) == 0x00 -> CF; SP++`                                | `0b101011SS` (`0xAC..=0xAF`) |
| `inc`       | Increment                 | `*SP + 1 -> *SP;`                                                                       | `0b10110000` (`0xB0`)        |
| `dec`       | Decrement                 | `*SP - 1 -> *SP;`                                                                       | `0b10110001` (`0xB1`)        |
| `neg`       | Negate                    | `-*SP -> *SP`                                                                           | `0b10110010` (`0xB2`)        |
| `shl`       | Shift Left with Carry     | `*SP & 0b10000000 -> CF; *SP << 1 -> *SP;`                                              | `0b10110100` (`0xB4`)        |
| `shr`       | Shift Right with Carry    | `*SP & 0b00000001 -> CF; *SP >> 1 -> *SP;`                                              | `0b10110101` (`0xB5`)        |
| `not`       | Bitwise NOT               | `!*SP -> *SP; *SP == 0x00 -> CF;`                                                       | `0b10110110` (`0xB6`)        |
| `buf`       | Bitwise Buffer            | `*SP -> *SP; *SP == 0x00 -> CF;`                                                        | `0b10110111` (`0xB7`)        |
| `ldo OFST`  | Load from Offset          | `SP--; *(SP + OFST + 1) -> *SP;`                                                        | `0b1100OOOO` (`0xC0..=0xCF`) |
| `sto OFST`  | Store to Offset           | `*SP -> *(SP + OFST + 1); SP++;`                                                        | `0b1101OOOO` (`0xD0..=0xDF`) |
| `lda`       | Load from Address         | `**SP -> *SP;`                                                                          | `0b11100000` (`0xE0`)        |
| `sta`       | Store to Address          | `*(SP + 1) -> **SP; SP++;`                                                              | `0b11100001` (`0xE1`)        |
| `ldi`       | Load Instruction Pointer  | `SP--; IP -> *SP;`                                                                      | `0b11100010` (`0xE2`)        |
| `sti`       | Store Instruction Pointer | `*SP -> IP; SP++`                                                                       | `0b11100011` (`0xE3`)        |
| `lds`       | Load Stack Pointer        | `SP--; SP -> *SP;`                                                                      | `0b11100100` (`0xE4`)        |
| `sts`       | Store Stack Pointer       | `*SP -> SP; SP++;`                                                                      | `0b11100101` (`0xE5`)        |
| `clc`       | Clear Carry               | `0b0 -> CF;`                                                                            | `0b11101000` (`0xE8`)        |
| `sec`       | Set Carry                 | `0b1 -> CF;`                                                                            | `0b11101001` (`0xE9`)        |
| `flc`       | Flip Carry                | `!CF -> CF;`                                                                            | `0b11101010` (`0xEA`)        |
| `nop`       | No Operation              | `;`                                                                                     | `0b11101110` (`0xEE`)        |
| `pop`       | Pop                       | `SP++;`                                                                                 | `0b11101111` (`0xEF`)        |
| `phn NIMM`  | Push Negative             | `SP--; IMM -> *SP;`                                                                     | `0b1111NNNN` (`0xF0..=0xFF`) |

Negative values are represented in two's complement.

There exist a set of bijections between instruction operands and their encoding within an instruction opcode, limiting the set of values they can hold:

- `IMM <=> 0b0IIIIIII`
- `SIZE <=> 1 << 0b000000SS`
- `OFST <=> 0b0000OOOO`
- `NIMM <=> 0b1111NNNN`

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
