# Enc

_Hex-to-opcode encoder for Atto-8 microarchitecture_

## Overview

The encoder encodes hexadecimal instructions (plain text characters in the range `/[0-9A-F]/`) from `argv[1]`, and outputs a memory image file to `argv[2]` which is exactly `0x100` bytes in size. It ignores both whitespace and comments of the forms `/# .*$/` and `/#$/`.
