# Dasm

Elementary disassembler for Atto-8 microprocessor

## Overview

The disassembler loads a memory image from file `argv[1]`, which must be exactly `0x100` bytes in size, and outputs an assembly file to `argv[2]`. Disassembly adheres to microprocessor specification as defined in [/spec/microprocessor.md](../spec/microprocessor.md).
