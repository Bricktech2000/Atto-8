# Asm

_Optimizing assembler for Atto-8 microprocessor_

## Overview

The assembler loads an assembly file from `argv[1]` and outputs a memory image file to `argv[2]` which is exactly `0x100` bytes in size. Code generation adheres to microprocessor specification as defined in [/spec/microprocessor.md](../spec/microprocessor.md).

Assembly consists of the following process:

1. Preprocess then tokenize source code from file `argv[1]`.
2. Expand macro references recursively from entry point `!main`.
3. Convert tokens to IR for constant folding and optimization.
4. Compile IR to list of instructions while resolving labels.
5. Generate binary and write it to file `argv[2]`.

Labels are global by default; local labels are local to a macro. Macros are global. Macro definitions end either at the start of the next macro definition or at the end of the token stream; macro definitions may not be nested. The token stream must begin with a macro definition token so every token belongs to a macro. Tokens are to be separated by whitespace.

## Preprocessing

| Pattern      | Operation                                    |
| ------------ | -------------------------------------------- |
| `/#( .*)?$/` | Textually replace with `""`                  |
| `/@( .*)?$/` | Textually replace with contents of file `$1` |

## Tokens

| Token    | Operation                                         |
| -------- | ------------------------------------------------- |
| `label:` | Define label `label` at current address           |
| `label.` | Define local label `label` at current address     |
| `:label` | Push address of label `label`                     |
| `.label` | Push address of local label `label`               |
| `macro!` | Define start of macro `macro`                     |
| `!macro` | Token-wise replace with contents of macro `macro` |
| `@const` | Assert that preceding expression is constant      |
| `@dyn`   | Inhibit optimization of preceding instruction     |
| `@org`   | Set location counter to preceding expression      |
| `@err`   | Emit error and terminate compilation              |
| `dDD`    | Insert `DD` in binary at current address          |
| `xXX`    | Push `XX` through `psh` and `phn` instructions    |
| `add`    | Instruction `add 0x01`                            |
| `adS`    | Instruction `add S`                               |
| `sub`    | Instruction `sub 0x01`                            |
| `suS`    | Instruction `sub S`                               |
| `iff`    | Instruction `iff 0x01`                            |
| `ifS`    | Instruction `iff S`                               |
| `rot`    | Instruction `rot 0x01`                            |
| `roS`    | Instruction `rot S`                               |
| `orr`    | Instruction `orr 0x01`                            |
| `orS`    | Instruction `orr S`                               |
| `and`    | Instruction `and 0x01`                            |
| `anS`    | Instruction `and S`                               |
| `xor`    | Instruction `xor 0x01`                            |
| `xoS`    | Instruction `xor S`                               |
| `xnd`    | Instruction `xnd 0x01`                            |
| `xnS`    | Instruction `xnd S`                               |
| `inc`    | Instruction `inc`                                 |
| `dec`    | Instruction `dec`                                 |
| `neg`    | Instruction `neg`                                 |
| `not`    | Instruction `not`                                 |
| `buf`    | Instruction `buf`                                 |
| `ldO`    | Instruction `ldo O`                               |
| `stO`    | Instruction `sto O`                               |
| `lda`    | Instruction `lda`                                 |
| `sta`    | Instruction `sta`                                 |
| `ldi`    | Instruction `ldi`                                 |
| `sti`    | Instruction `sti`                                 |
| `lds`    | Instruction `lds`                                 |
| `sts`    | Instruction `sts`                                 |
| `nop`    | Instruction `nop`                                 |
| `clc`    | Instruction `clc`                                 |
| `sec`    | Instruction `sec`                                 |
| `flc`    | Instruction `flc`                                 |
| `swp`    | Instruction `swp`                                 |
| `pop`    | Instruction `pop`                                 |
