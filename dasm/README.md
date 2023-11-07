# Dasm

_Elementary disassembler for Atto-8 microarchitecture_

## Overview

The disassembler loads a memory image from file `argv[1]` which must be exactly `0x100` bytes in size, and outputs an assembly file to `argv[2]`. Disassembly adheres to the Atto-8 microarchitecture specification as defined in [/spec/microarchitecture.md](../spec/microarchitecture.md).
