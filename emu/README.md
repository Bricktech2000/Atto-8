# Emu

Instruction-level emulator for Atto-8 microcomputer

## Overview

The emulator loads an image file from `argv[1]`, which must be exactly `0x100` bytes in size. Emulation is performed at instruction level; that is, the emulator is built to test binaries, not to mirror the hardware. The emulator adheres to the Atto-8 microcomputer specification as defined in [/spec/microcomputer.md](../spec/microcomputer.md).

Emulation will stop upon receiving `SIGINT` (Ctrl+C) or `SIGTERM` (kill). The emulator will enter debug mode upon encountering an invalid instruction.

## Debug Mode

Unofficial instruction `0xBB` is treated as a debug request.

In debug mode, the emulator will print the current state of the machine and wait for a command to be sent to `stdin`. The following commands are supported:

- `' '` &mdash; Step one instruction.
- `'\n'` &mdash; Continue execution.

## Input

The Atto-8 microcomputer is equipped with a pair of D-pad controllers. The emulator reads input from `stdin` and maps it to the following controller buttons:

- `w` &mdash; Primary Up
- `s` &mdash; Primary Down
- `a` &mdash; Primary Left
- `d` &mdash; Primary Right
- `i` &mdash; Secondary Up
- `k` &mdash; Secondary Down
- `j` &mdash; Secondary Left
- `l` &mdash; Secondary Right
