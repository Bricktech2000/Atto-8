# Emu

_Instruction-level emulator for Atto-8 microcomputer_

## Overview

The emulator loads a memory image file from `argv[1]` which must be exactly `0x100` bytes in size. Emulation is performed at the instruction level; that is, the emulator is built to test binaries, not to mirror the hardware. The emulator adheres to the Atto-8 microcomputer specification as defined in [/spec/microcomputer.md](../spec/microcomputer.md).

Emulation will exit upon receiving `SIGINT` (Ctrl+C) or `SIGTERM` (kill). The emulator will enter debug mode upon encountering an illegal opcode.

## Standard Input/Output

The emulator sends most characters received from `stdin` to the Atto-8's standard input and sends most characters received from the Atto-8's standard output to `stdout`. The following characters are exceptions:

- `Del` &mdash; Clear standard output.
- `Tab` &mdash; Toggle displaying machine state.
- `Escape` &mdash; Forcefully enter debug mode.

## Controller

The Atto-8 microcomputer is equipped with a pair of D-pad controllers. The emulator reads input from `stdin` and maps it to the following controller buttons:

- `ArrowUp` &mdash; Primary Up
- `ArrowDown` &mdash; Primary Down
- `ArrowLeft` &mdash; Primary Left
- `ArrowRight` &mdash; Primary Right
- `PageUp` &mdash; Secondary Up
- `PageDown` &mdash; Secondary Down
- `Home` &mdash; Secondary Left
- `End` &mdash; Secondary Right

## Debug Mode

Unofficial instruction `0xBB` is treated as a debug request. Debug mode can be entered forcefully by hitting `Escape` during emulation.

In debug mode, the emulator will print the current machine state and wait for a command to be sent to `stdin`. The following commands are supported:

- `Del` &mdash; Clear standard output.
- `Tab` &mdash; Step one instruction.
- `Escape` &mdash; Continue emulation.
