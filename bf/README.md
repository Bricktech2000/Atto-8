# BF

_Brainfuck architecture front-end for Atto-8 microprocessor_

## Overview

The Atto-8 microprocessor, specified in [/spec/microprocessor.md](../spec/microprocessor.md), is a highly flexible general-purpose platform for building processors. This front-end aims to demonstrate the Atto-8 microprocessor's high degree of flexibility by implementing an architecture based on the Brainfuck programming language.

- [/bf/bf-mic.rs](bf-mic.rs) &mdash; Builds a microcode image implementing the front-end's architecture, described below.
- [/bf/bf-pad.py](bf-pad.py) &mdash; Strips no-ops from a Brainfuck source file and pads it using null bytes to a length of `0x100` bytes.

## Components

| Component | Name                | Size   |
| --------- | ------------------- | ------ |
| `IP`      | Instruction Pointer | 8 bits |
| `HP`      | Head Pointer        | 8 bits |

`IP` is a pointer to the current instruction and can be indirectly written to through instructions `[` and `]`. `IP` is incremented after the execution of every instruction.

`HP` is a read-write head that can be moved through instructions `>` and `<` and used through instructions `+`, `-`, `.` and `,`.

## Instruction Set

The architecture assumes memory of the shape `[u8; 0x100]`, from which the `*` operator dereferences values. Any instruction not present in the table below is ignored and treated as a no-op.

| Instruction | Name       | Operation                              | Opcode  |
| ----------- | ---------- | -------------------------------------- | ------- |
| `>`         | Move Right | `HP++;`                                | `b'>'`  |
| `<`         | Move Left  | `HP--;`                                | `b'<'`  |
| `+`         | Increment  | `(*HP)++;`                             | `b'+'`  |
| `-`         | Decrement  | `(*HP)--`                              | `b'-'`  |
| `.`         | Output     | `*HP -> *0x00;`                        | `b'.'`  |
| `,`         | Input      | `*0x00 -> *HP;`                        | `b','`  |
| `[`         | Loop Begin | `if (*HP == 0x00) IP = matching(']');` | `b'['`  |
| `]`         | Loop End   | `if (*HP != 0x00) IP = matching('[');` | `b']'`  |
| (EOF)       | Halt       | `while (1);`                           | `b'\0'` |

Unofficial opcode `b'#'` is mapped to unofficial control word `0xFFFD` for debug requests.

## Constraints

Writing beyond the start of the tape will corrup the front-end's state, resulting in undefined behavior. Writing too far into the tape will corrupt program memory, resulting in undefined behavior.

Every program must begin with instructions `>>`; failure to do so will result in undefined behavior. Every `[` must be matched with a corresponding `]`, and vice-versa; failure to do so will result in undefined behavior.

Address `0x00` is assumed to be a memory-mapped standard input/output device. That is, `.` outputs by writing a byte to address `0x00` and `,` inputs by reading a byte from address `0x00`.

Addresses `0x01` and `0xFF` are reserved for the front-end. Address `0x01` stores the current nesting level for walking between `[` and `]`, and address `0xFF` stores the current walking state.
