# Dec

_Opcode-to-hex decoder for Atto-8 microarchitecture_

## Overview

The decoder loads a memory image from file `argv[1]` which must be exactly `0x100` bytes in size, and outputs a plain text file containing hexadecimal instructions (ASCII characters in the range `/[0-9A-F]/`) to `argv[2]`.
