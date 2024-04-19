# Sim

_Cycle-accurate component-level simulator for Atto-8 microcomputer_

<!-- most of this document is identical to `/emu/README.md` -->

## Overview

The simulator loads a memory image file from `argv[1]` which must be exactly `0x100` bytes in size, and a microcode image file from `argv[2]` which must be exactly `0x2000` words in size. Simulation is performed at the component level; that is, the simulator is built to test microcode images by accurately mirroring the hardware. The simulator adheres to the Atto-8 microcomputer specification as defined in [/spec/microcomputer.md](../spec/microcomputer.md).

Simulation will exit upon receiving `SIGINT` (Ctrl+C) or `SIGTERM` (kill). The simulator will enter debug mode upon encountering a bus fault (unofficial control word `0xFFFC`), an illegal opcode (unofficial control word `0xFFFE`) or a microcode fault (unofficial control word `0xFFFF`).

## Standard Input/Output

The simulator sends most characters received from `stdin` to the Atto-8's standard input and sends most characters received from the Atto-8's standard output to `stdout`. The following characters are exceptions:

- `Del` &mdash; Clear standard output.
- `Tab` &mdash; Toggle displaying machine state.
- `Escape` &mdash; Forcefully enter debug mode.

## Controller

The Atto-8 microcomputer is equipped with a pair of D-pad controllers. The simulator reads input from `stdin` and maps it to the following controller buttons:

- `ArrowUp` &mdash; Primary Up
- `ArrowDown` &mdash; Primary Down
- `ArrowLeft` &mdash; Primary Left
- `ArrowRight` &mdash; Primary Right
- `PageUp` &mdash; Secondary Up
- `PageDown` &mdash; Secondary Down
- `Home` &mdash; Secondary Left
- `End` &mdash; Secondary Right

## Debug Mode

Unofficial control word `0xFFFD` is treated as a debug request. Debug mode can be entered forcefully by hitting `Escape` during emulation.

In debug mode, the simulator will print the current machine state and wait for a command to be sent to `stdin`. The following commands are supported:

- `Del` &mdash; Clear standard output.
- `Tab` &mdash; Step one instruction.
- `Escape` &mdash; Continue emulation.
