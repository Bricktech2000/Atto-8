# Asm

_Optimizing assembler for Atto-8 microarchitecture_

## Overview

The assembler loads an assembly file from `argv[1]` and outputs a memory image file to `argv[2]` which is exactly `0x100` bytes in size. Code generation adheres to microarchitecture specification as defined in [/spec/microarchitecture.md](../spec/microarchitecture.md).

Assembly consists of the following process:

1. Preprocess then tokenize source code from file `argv[1]`.
2. Expand macro references recursively from entry point `!main`.
3. Convert tokens to IR for constant folding and optimization.
4. Compile IR to list of instructions while resolving labels.
5. Generate binary and write it to file `argv[2]`.

Labels are global by default; local labels are local to a macro. Macros are global. Macro definitions end either at the start of the next macro definition or at the end of the token stream; macro definitions may not be nested. The token stream must begin with a macro definition token so every token belongs to a macro. Tokens are to be separated by whitespace.

## Preprocessing

| Pattern                | Operation                                    |
| ---------------------- | -------------------------------------------- |
| `/# (.*)$/` and `/#$/` | Textually replace with `""`                  |
| `/@ (.*)$/`            | Textually replace with contents of file `$1` |

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
| `@DD`    | Insert `DD` into binary at current address        |
| `xXX`    | Push hexadecimal `XX` through `psh` and `phn`     |
| `add`    | Emit instruction `add 0x01`                       |
| `adS`    | Emit instruction `add S`                          |
| `sub`    | Emit instruction `sub 0x01`                       |
| `suS`    | Emit instruction `sub S`                          |
| `iff`    | Emit instruction `iff 0x01`                       |
| `ifS`    | Emit instruction `iff S`                          |
| `rot`    | Emit instruction `rot 0x01`                       |
| `roS`    | Emit instruction `rot S`                          |
| `orr`    | Emit instruction `orr 0x01`                       |
| `orS`    | Emit instruction `orr S`                          |
| `and`    | Emit instruction `and 0x01`                       |
| `anS`    | Emit instruction `and S`                          |
| `xor`    | Emit instruction `xor 0x01`                       |
| `xoS`    | Emit instruction `xor S`                          |
| `xnd`    | Emit instruction `xnd 0x01`                       |
| `xnS`    | Emit instruction `xnd S`                          |
| `inc`    | Emit instruction `inc`                            |
| `dec`    | Emit instruction `dec`                            |
| `neg`    | Emit instruction `neg`                            |
| `not`    | Emit instruction `not`                            |
| `buf`    | Emit instruction `buf`                            |
| `ldO`    | Emit instruction `ldo O`                          |
| `stO`    | Emit instruction `sto O`                          |
| `lda`    | Emit instruction `lda`                            |
| `sta`    | Emit instruction `sta`                            |
| `ldi`    | Emit instruction `ldi`                            |
| `sti`    | Emit instruction `sti`                            |
| `lds`    | Emit instruction `lds`                            |
| `sts`    | Emit instruction `sts`                            |
| `nop`    | Emit instruction `nop`                            |
| `clc`    | Emit instruction `clc`                            |
| `sec`    | Emit instruction `sec`                            |
| `flc`    | Emit instruction `flc`                            |
| `swp`    | Emit instruction `swp`                            |
| `pop`    | Emit instruction `pop`                            |
