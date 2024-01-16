# BF

_Brainfuck architecture frontend for Atto-8 microprocessor_

## Overview

The Atto-8 microprocessor, specified in [/spec/microprocessor.md](../spec/microprocessor.md), is a highly flexible general-purpose platform for building processors. This frontend aims to demonstrate the Atto-8 microprocessor's high degree of flexibility by implementing an architecture based on the Brainfuck programming language.

- [/bf/bf-mic.rs](bf-mic.rs) &mdash; Builds a microcode image implementing the frontend's architecture, described below.
- [/bf/bf-pad.py](bf-pad.py) &mdash; Strips no-ops from a Brainfuck source file and pads it using null bytes to a length of `0x100` bytes.

## Components

| Component | Name                | Size   |
| --------- | ------------------- | ------ |
| `IP`      | Instruction Pointer | 8 bits |
| `HP`      | Head Pointer        | 8 bits |

`IP` is a pointer to the next instruction to be executed and can be indirectly written to through instructions `[` and `]`. `IP` is incremented before the execution of every instruction.

`HP` is a read-write head that can be moved through instructions `>` and `<` and used through instructions `+`, `-`, `.` and `,`.

## Instruction Set

The architecture assumes memory of the shape `[u8; 0x100]`, from which the `*` operator dereferences values. Any instruction not present in the table below is ignored and treated as a no-op.

| Instruction | Name       | Operation                               | Opcode  |
| ----------- | ---------- | --------------------------------------- | ------- |
| `>`         | Move Right | `HP--;`                                 | `b'>'`  |
| `<`         | Move Left  | `HP++;`                                 | `b'<'`  |
| `+`         | Increment  | `(*HP)++;`                              | `b'+'`  |
| `-`         | Decrement  | `(*HP)--`                               | `b'-'`  |
| `.`         | Output     | `*HP -> *0x00;`                         | `b'.'`  |
| `,`         | Input      | `*0x00 -> *HP;`                         | `b','`  |
| `[`         | Loop Begin | `if (*HP == 0x00) matching(']') -> IP;` | `b'['`  |
| `]`         | Loop End   | `if (*HP != 0x00) matching('[') -> IP;` | `b']'`  |
| (EOF)       | Halt       | `while (1);`                            | `b'\0'` |

Unofficial opcode `b'#'` is mapped to unofficial control word `0xFFFD` for debug requests.

## Initial State

The frontend is initialized with the following state:

| Commponent | Value  |
| ---------- | ------ |
| `IP`       | `0x01` |
| `HP`       | `0xFE` |

This implies that:

- Execution begins at address `0x01`.
- The start of the tape is located at address `0xFE`.

## Implementation Constraints

Writing beyond the start of the tape will corrupt the frontend's state, resulting in undefined behavior. Writing too far into the tape will corrupt program memory, resulting in undefined behavior. Brackets `[` and `]` must be balanced; failure to meet this condition will result in undefined behavior.

Address `0x00` is assumed to be a memory-mapped standard input/output device. That is, `.` outputs by writing a byte to address `0x00` and `,` inputs by reading a byte from address `0x00`.

When used as part of the Atto-8 microcomputer, the frontend implements non-blocking input which will return `0x00` if no input is currently available. This is different from an `EOF` condition, as input may become available at a later time. To send a newline to the Atto-8 microcomputer, the full `CRLF` sequence is required.

Cells are 8-bit unsigned integers and wrap on overflow and underflow.
