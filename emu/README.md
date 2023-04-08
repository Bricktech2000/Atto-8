# Emu

High-level emulator for Atto-8 microcomputer

## Overview

Emulator loads a memory image from file `argv[1]`, which must be exactly `0x100` bytes in size. Emulation is performed at a high level; that is, the emulator is built to test binaries, not to mirror the hardware. The emulator adheres to the Atto-8 microcomputer specification as defined in [/spec/microcomputer.md](../spec/microcomputer.md).

Emulation will stop upon receiving `SIGINT` (Ctrl+C) or `SIGTERM` (kill). Emulator will enter debug mode upon encountering an invalid instruction.

## Debug Mode

Unofficial instruction `0xBB` is treated as a debug request.

In debug mode, the emulator will print the current state of the machine and wait for a command to be sent to `stdin`. The following commands are supported:

- `' '` &mdash; Step one instruction.
- `'\n'` &mdash; Continue execution.
