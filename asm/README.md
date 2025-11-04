# Asm

_Optimizing assembler for Atto‑8 microarchitecture_

## Overview

The assembler loads an assembly file from `argv[1]` and outputs a memory image file to `argv[2]` which is exactly `0x100` bytes in size. Code generation adheres to Atto‑8 microarchitecture specification as defined in [/spec/microarchitecture.md](../spec/microarchitecture.md).

Assembly consists of the following process:

1. Preprocess then tokenize source code from file `argv[1]`.
2. Expand macro references recursively from entry point `!main`.
3. Convert tokens to IR for constant folding and optimization.
4. Compile IR to list of instructions while resolving labels.
5. Generate binary and write it to file `argv[2]`.

Labels are global by default; local labels are local to a macro. Macros are global. Macro definitions end either at the start of the next macro definition or at the end of the token stream; macro definitions may not be nested. The token stream must begin with a macro definition token so every token belongs to a macro. Tokens are to be separated by whitespace; after preprocessing, all whitespace is considered equivalent.

## Preprocessing

| Pattern                | Operation                                    |
| ---------------------- | -------------------------------------------- |
| `/# (.*)$/` and `/#$/` | Textually replace with `""`                  |
| `/@ (.*)$/`            | Textually replace with contents of file `$1` |

## Optimization

Assembler optimizations assume the carry flag is always clear, and may leave the carry flag in an unspecified state. Consequently, program behavior may be altered during the optimization stage. Instructions annotated with the `@dyn` directive are guaranteed to be left unaltered. Instructions `clc`, `sec` and `flc` are guaranteed to be left unaltered.

## Tokens

| Token    | Operation                                         |
| -------- | ------------------------------------------------- |
| `label:` | Define label `label` at current address           |
| `label.` | Define local label `label` at current address     |
| `:label` | Push address of label `label`                     |
| `.label` | Push address of local label `label`               |
| `macro!` | Define start of macro `macro`                     |
| `!macro` | Token-wise replace with contents of macro `macro` |
| `@error` | Emit error and terminate compilation              |
| `@const` | Assert that preceding expression is constant      |
| `@data`  | Insert preceding expression into binary           |
| `@dyn`   | Inhibit optimization of preceding instruction     |
| `@org`   | Set location counter to preceding expression      |
| `@DD`    | Insert `DD` into binary; shorhand for `xDD @data` |
| `xXX`    | Push hexadecimal `XX` through `psh` and `phn`     |
| `add`    | Emit instruction `add 0x01`                       |
| `adS`    | Emit instruction `add S`                          |
| `sub`    | Emit instruction `sub 0x01`                       |
| `suS`    | Emit instruction `sub S`                          |
| `iff`    | Emit instruction `iff 0x01`                       |
| `ifS`    | Emit instruction `iff S`                          |
| `swp`    | Emit instruction `swp 0x01`                       |
| `swS`    | Emit instruction `swp S`                          |
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
| `pop`    | Emit instruction `pop`                            |

## Conventions

By convention, functions are called by pushing their arguments onto the stack in reverse order, pushing a return address onto the stack, and then jumping to the function’s address. It is recommended that functions replace their arguments with their return values prior to returning as to mirror the behavior of instructions on the Atto‑8 microarchitecture.

The state of the carry flag is generally unspecified after a macro or function begins and before it returns, and macros and functions are not required to preserve the state of the carry flag.
