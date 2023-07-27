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
- 0 memory banks
- 0 interrupts
- 0 I/O ports

## Registers and Flags

| Item | Description         | Size   |
| ---- | ------------------- | ------ |
| `IP` | Instruction Pointer | 8 bits |
| `SP` | Stack Pointer       | 8 bits |
| `CF` | Carry Flag          | 1 bit  |

`IP` is a pointer to the instruction being executed in memory. Writing to `IP` through `sti` will cause the processor to jump to the specified address. `IP` is incremented after the execution of each instruction.

`SP` is a pointer to the item at top of the stack, which grows downward. Writing to `SP` through `sts` while the stack is empty will move the location of the stack in memory. Instructions increment or decrement `SP` as needed.

`CF` is a one-bit status flag shared by many instructions. As a rule of thumb:

- `CF` is set by `sec`, cleared by `clc` and flipped by `flc`.
- `CF` is set when a logical operation (`orr`, `and`, `xor`, `xnd`, `not`, `buf`) results in `0x00` and cleared otherwise.
- `CF` is set when an arithmetic operation (`add`, `sub`, `shl`, `shr`) overflows or underflows and cleared otherwise.
- `CF` is used as carry-in for arithmetic operations (`add`, `sub`, `shl`, `shr`).
- `CF` is used as the condition operand for the conditional instruction `iff`.

## Instruction Set

All instructions are 8 bits in length and most operands are sourced from the stack.

The processor assumes it can address memory of the shape `[u8; 0x100]`. The `*` operator dereferences values from said memory.

Negative values are represented in two's complement.

| Instruction | Description                   | Operation                                                             | Opcode                |
| ----------- | ----------------------------- | --------------------------------------------------------------------- | --------------------- |
| `psh X`     | Push                          | `X & 0b01111111 -> *(--SP);`                                          | `0b0XXXXXXX` (`0xXX`) |
| `add S`     | Add with Carry                | `*(SP++) + *(SP + 2 ** S) + CF -> *(SP + 2 ** S); *SP > 0xFF -> CF;`  | `0b100000SS`          |
| `sub S`     | Subtract with Carry           | `-*(SP++) + *(SP + 2 ** S) - CF -> *(SP + 2 ** S); *SP < 0x00 -> CF;` | `0b100001SS`          |
| `iff S`     | Conditional with Carry        | `CF ? *((SP++)++) : *((++SP)++ + 2 ** S) -> *(--SP);`                 | `0b100100SS`          |
| `rot S`     | Rotate                        | `(*SP << S) \| ((*SP << S) >> 8) -> *SP;`                             | `0b100101SS`          |
| `orr S`     | Bitwise OR                    | `*(SP++) \| *(SP + 2 ** S) -> *(SP + 2 ** S); *SP == 0 -> CF;`        | `0b101000SS`          |
| `and S`     | Bitwise AND                   | `*(SP++) & *(SP + 2 ** S) -> *(SP + 2 ** S); *SP == 0 -> CF;`         | `0b101001SS`          |
| `xor S`     | Bitwise XOR                   | `*(SP++) ^ *(SP + 2 ** S) -> *(SP + 2 ** S); *SP == 0 -> CF;`         | `0b101010SS`          |
| `xnd S`     | Bitwise XAND                  | `SP++; 0 -> *(SP + 2 ** S); *SP == 0 -> CF;`                          | `0b101011SS`          |
| `inc`       | Increment                     | `*SP + 1 -> *SP;`                                                     | `0b10110000`          |
| `dec`       | Decrement                     | `*SP - 1 -> *SP;`                                                     | `0b10110001`          |
| `neg`       | Negate                        | `-*SP -> *SP`                                                         | `0b10110010`          |
| `shl`       | Shift Left                    | `(*SP & 0x80) -> CF;  (*SP << 1) -> *SP;`                             | `0b10110100`          |
| `shr`       | Shift Right                   | `(*SP & 0x01) -> CF;  (*SP >> 1) -> *SP;`                             | `0b10110101`          |
| `not`       | Bitwise NOT                   | `!*SP -> *SP; *SP == 0 -> CF;`                                        | `0b10110110`          |
| `buf`       | Bitwise Buffer                | `*SP -> *SP; *SP == 0 -> CF;`                                         | `0b10110111`          |
| `ldo O`     | Load from Offset              | `*(SP + O) -> *(--SP);`                                               | `0b1100OOOO` (`0xCO`) |
| `sto O`     | Store to Offset               | `*SP++ -> *(SP + O);`                                                 | `0b1101OOOO` (`0xDO`) |
| `lda`       | Load from Address             | `*(*(SP++)) -> *(--SP);`                                              | `0b11101000` (`0xE0`) |
| `sta`       | Store to Address              | `*(SP++ + 1) -> *(*(SP++ - 1));`                                      | `0b11101001` (`0xE1`) |
| `ldi`       | Load from Instruction Pointer | `IP -> *(--SP);`                                                      | `0b11101010` (`0xE2`) |
| `sti`       | Store to Instruction Pointer  | `*(SP++) -> IP;`                                                      | `0b11101011` (`0xE3`) |
| `lds`       | Load from Stack Pointer       | `SP -> *(--SP);`                                                      | `0b11101100` (`0xE4`) |
| `sts`       | Store to Stack Pointer        | `*(SP++) -> SP;`                                                      | `0b11101101` (`0xE5`) |
| `nop`       | No Operation                  | `;`                                                                   | `0b11100000` (`0xE8`) |
| `clc`       | Clear Carry                   | `0 -> CF;`                                                            | `0b11100001` (`0xE9`) |
| `sec`       | Set Carry                     | `1 -> CF;`                                                            | `0b11100010` (`0xEA`) |
| `flc`       | Flip Carry                    | `!CF -> CF;`                                                          | `0b11100011` (`0xEB`) |
| `swp`       | Swap                          | `*(SP++) -> *SP -> *(--SP);`                                          | `0b11100100` (`0xEC`) |
| `pop`       | Pop                           | `0 -> *(SP++);`                                                       | `0b11100101` (`0xED`) |
| `phn X`     | Push Negative                 | `X \| 0b11110000 -> *(--SP);`                                         | `0b1111XXXX` (`0xFX`) |

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

## Instruction Execution

Program execution is as follows:

1. Fetch the instruction at `IP`.
2. Increment `IP`.
3. Execute the instruction.
4. Repeat.

## Conventions

The Atto-8 has no inherent endianness. With that said, instructions such as `add S` and `sub S` work best when least significant bytes are at the top of the stack, which grows downward. Consequently, it is recommended that the Atto-8 be assumed to be little-endian.
