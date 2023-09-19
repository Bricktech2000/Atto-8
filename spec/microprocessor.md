# Atto-8 Microprocessor

## Overview

The Atto-8 microprocessor is a minimalist stack-based processor implementing the Atto-8 microarchitecture as defined in [/spec/microarchitecture.md](../spec/microarchitecture.md). It is designed to keep logic IC count to a minimum while still being a complete implementation of the Atto-8 microarchitecture, only consisting of one full adder, one NAND gate and a few latches. It is intended to be used as a learning tool for students and hobbyists, and as a basis for more complex processors.

![Atto-8 Microprocessor Diagram](../misc/assets/microprocessor.png)

## Components

The major components of the Atto-8 microprocessor are stateful. Stateful components are as follows:

| Component | Name                | Size   | Description                                                    |
| --------- | ------------------- | ------ | -------------------------------------------------------------- |
| `IP`      | Instruction Pointer | 8 bits | See [/spec/microarchitecture.md](../spec/microarchitecture.md) |
| `SP`      | Stack Pointer       | 8 bits | See [/spec/microarchitecture.md](../spec/microarchitecture.md) |
| `CF`      | Carry Flag          | 1 bit  | See [/spec/microarchitecture.md](../spec/microarchitecture.md) |
| `IL`      | Instruction Latch   | 8 bits | Stores the opcode for the instruction currently being executed |
| `SC`      | Step Counter        | 5 bits | Counts microcode steps within an instruction                   |
| `AL`      | Address Latch       | 8 bits | Latches a value from `DATA` and outputs to `ADDR`              |
| `XL`      | X Latch             | 8 bits | Latches a value from `DATA` and produces derivations           |
| `YL`      | Y Latch             | 8 bits | Latches a value from `DATA` and produces derivations           |
| `ZL`      | Z Latch             | 8 bits | Latches a value from `DATA` and produces derivations           |

Derivations on the Atto-8 microprocessor are stateless components that derive their output continuously from other components. Derivations are as follows:

| Component   | Name                               | Size    | Description                                                 |
| ----------- | ---------------------------------- | ------- | ----------------------------------------------------------- |
| `CTRL`      | Control Word Derivation            | 16 bits | Turns the output of `MIC` into control signals              |
| `SIZE_DATA` | Size-to-Data Derivation            | 1 bit   | Computed using `SIZE_AND_CIN` and `SUM_DATA`                |
| `OFST_DATA` | Offset-to-Data Derivation          | 1 bit   | Computed using `OFST_AND_CF` and `SUM_DATA` and `NAND_DATA` |
| `SET_CIN`   | Set-to-Carry-In Derivation         | 1 bit   | Computed using `SIZE_AND_CIN` and `SUM_DATA`                |
| `COUT_CF`   | Carry-Out-to-Carry-Flag Derivation | 1 bit   | Computed using `OFST_AND_CF` and `SUM_DATA` and `NAND_DATA` |
| `ZERO_CF`   | Zero-to-Carry-Flag Derivation      | 1 bit   | Computed using `OFST_AND_CF` and `SUM_DATA` and `NAND_DATA` |
| `ONES_DATA` | Ones-to-Data Derivation            | 1 bit   | Computed using all control signals ending in `_DATA`        |
| `ONES`      | Ones Derivation                    | 8 bits  | Outputs `0xFF` to `DATA` when `DATA` is floating            |
| `SIZE`      | Size Derivation                    | 8 bits  | Computes the `SIZE` operand from `IL`                       |
| `OFST`      | Offset Derivation                  | 8 bits  | Computes the `OFST` operand from `IL`                       |
| `SUM`       | Sum Derivation                     | 8 bits  | Computes the sum of `XL` and `YL`                           |
| `NAND`      | Not-And Derivation                 | 8 bits  | Computes the not-and of `YL` and `ZL`                       |
| `CIN`       | `SUM` Carry-In Derivation          | 1 bit   | Outputs to `SUM` carry in                                   |
| `COUT`      | `SUM` Carry-Out Derivation         | 1 bit   | Computes `SUM` carry out                                    |
| `ZERO`      | `NAND` Is-Zero Derivation          | 1 bit   | Computes `NAND` is-zero flag                                |
| `MIC`       | Microcode Derivation               | 16 bits | Computes microcode step from `IL`, `CF` and `SC`            |

## Control Word

The control word is a 16-bit natural number output from `MIC`, the microcode ROM. Control signals are bit-mapped into the control word as follows, where `0x0` represents the least significant bit:

| Bit   | Control Signal | Name                            |
| ----- | -------------- | ------------------------------- |
| `0xF` | `CLR_SC`       | Clear to Step Counter           |
| `0xE` | `DATA_IL`      | Data Bus to Instruction Latch   |
| `0xD` | `SIZE_AND_CIN` | Size and Carry In               |
| `0xC` | `OFST_AND_CF`  | Offset and Carry Flag           |
| `0xB` | `IP_DATA`      | Instruction Pointer to Data Bus |
| `0xA` | `DATA_IP`      | Data Bus to Instruction Pointer |
| `0x9` | `SP_DATA`      | Stack Pointer to Data Bus       |
| `0x8` | `DATA_SP`      | Data Bus to Stack Pointer       |
| `0x7` | `DATA_AL`      | Data Bus to Address Latch       |
| `0x6` | `MEM_DATA`     | Data Bus to Memory              |
| `0x5` | `DATA_MEM`     | Memory to Data Bus              |
| `0x4` | `DATA_XL`      | Data Bus to X Latch             |
| `0x3` | `DATA_YL`      | Data Bus to Y Latch             |
| `0x2` | `DATA_ZL`      | Data Bus to Z Latch             |
| `0x1` | `SUM_DATA`     | Sum to Data Bus                 |
| `0x0` | `NAND_DATA`    | Not-And to Data Bus             |

The control signals `SIZE_AND_CIN` and `OFST_AND_CF` are not directly used as control signals; rather, they produce derivations, which are then used as control signals. This mechanism allows the control word to be no more than 16 bits wide, reducing hardware complexity.

It is worth noting that:

- Pointers (`IP` and `SP`) are registers that can both read and write to `DATA`.
- `SC` increments every clock cycle and may only be reset to `0x00`, through `CLR_SC`.
- Latches (`IL`, `AL`, `XL`, `YL`, `ZL`) can only read from and cannot write to `DATA`.
- `SUM` can only output `XL + YL` to `DATA`, and `NAND` can only output `~(YL & ZL)` to `DATA`.
- The value of `CF` can only be set to the value of `ZERO` or to the value of `COUT`.

It follows that:

- `SP++`, `SP--` and `IP++` are non-trivial operations, requiring the use of `XL`, `YL` and `SUM`.
- Reads from `XL`, `YL` and `ZL` are non-trivial operations, requiring the use of `SUM` and `NAND`.

These design decisions greatly simplify the hardware complexity of the Atto-8 microprocessor, at the cost of performance and microcode complexity.

## Instruction Set

The instruction set of the Atto-8 microprocessor adheres to the Atto-8 microarchitecture as defined in [/spec/microarchitecture.md](../spec/microarchitecture.md). Instruction clock cycle counts are detailed below.

| Instruction | Clocks                          |
| ----------- | ------------------------------- |
| `psh IMM`   | `10`                            |
| `add SIZE`  | `14 + SIZE`                     |
| `sub SIZE`  | `14 + SIZE`                     |
| `iff SIZE`  | `13 + SIZE`                     |
| `rot SIZE`  | `18 + SIZE + *SP * (18 + SIZE)` |
| `orr SIZE`  | `14 + SIZE`                     |
| `and SIZE`  | `11 + SIZE`                     |
| `xor SIZE`  | `22 + SIZE`                     |
| `xnd SIZE`  | `8 + SIZE`                      |
| `inc`       | `6`                             |
| `dec`       | `8`                             |
| `neg`       | `11`                            |
| `shl`       | `9`                             |
| `shr`       | `16`                            |
| `not`       | `8`                             |
| `buf`       | `9`                             |
| `ldo OFST`  | `12 + OFST`                     |
| `sto OFST`  | `11 + OFST`                     |
| `lda`       | `9`                             |
| `sta`       | `15`                            |
| `ldi`       | `9`                             |
| `sti`       | `6`                             |
| `lds`       | `10`                            |
| `sts`       | `5`                             |
| `nop`       | `3`                             |
| `clc`       | `6`                             |
| `sec`       | `6`                             |
| `flc`       | `6`                             |
| `swp`       | `15`                            |
| `pop`       | `5`                             |
| `phn IMM`   | `10`                            |

The `rot SIZE` instruction requires `18 + SIZE` clock cycles to execute, plus another `18 + SIZE` for every bit rotated. Consequently, `rot` can be used as a stall instruction.

Memory reads and writes around `SP` must be idempotent, and a memory read from an address around `SP` must yield the last value written to that address. That is, stack memory is expected to behave like "normal" memory. If this expectation is not fulfilled, the behavior of instructions accessing the stack is undefined.
