# Emu

High-level emulator for Atto-8 microcomputer

## Overview

Emulator loads an image from file `argv[1]`, which must be exactly `0x100` bytes in size. Emulation is performed at a high level; that is, the emulator is built to test binaries, not to mirror the hardware. Emulator adheres to microcomputer specification as defined in [/spec/#todo](../spec/#todo).

Emulation will stop upon receiving `SIGINT` (Ctrl+C) or `SIGTERM` (kill). Emulator will panic upon encountering an invalid instruction.

## Debugging

Unofficial instruction `0xBB` is treated as a debug request.
