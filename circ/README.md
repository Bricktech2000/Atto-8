# Circ

_Gate-level circuit design for Atto-8 microcomputer_

## Overview

The circuit design is a [Logisim Evolution](https://github.com/logisim-evolution/logisim-evolution) project that simulates the Atto-8 microprocessor and microcomputer at the gate level. `circ.py` loads a memory image file from `argv[1]` which must be exactly `0x100` bytes in size, and a microcode image file from `argv[2]` which must be exactly `0x2000` bytes in size. Both images are then hard-coded into `atto-8.circ` before being opened in Logisim Evolution. Upon exiting Logisim Evolution, both images are cleaned up from `atto-8.circ`. The circuit design adheres to the Atto-8 microcomputer specification as defined in [/spec/microcomputer.md](../spec/microcomputer.md).
