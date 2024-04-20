# CEmu

_Minimal C99 emulator for Atto-8 microcomputer_

## Overview

The minimal emulator loads a memory image file from `argv[1]` which must be exactly `0x100` bytes in size. Emulation is performed at the instruction level; that is, the emulator is built to test binaries, not to mirror the hardware. The minimal emulator partly adheres to the Atto-8 microcomputer specification as defined in [/spec/microcomputer.md](../spec/microcomputer.md).

The minimal emulator dies upon encountering an illegal opcode or a debug request, represented by unofficial opcode `0xBB`.

For best results, run with `stty -icanon -echo -nl` to disable terminal input buffering and echoing.

## Features

Execution speed is uncapped and neither the Atto-8 controller nor its display are supported. Atto-8 standard input/output is transparently passed through to the `stdin` and `stdout` file descriptors respectively.
