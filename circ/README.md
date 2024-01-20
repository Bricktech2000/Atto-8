# Circ

_Block-level circuit design for Atto-8 microcomputer_

## Overview

The circuit design is a [Logisim Evolution](https://github.com/logisim-evolution/logisim-evolution) project that simulates the Atto-8 microprocessor and microcomputer at the gate level. `circ.py` loads a memory image file from `argv[1]` which must be exactly `0x100` bytes in size, a microcode image file from `argv[2]` which must be exactly `0x2000` words in size, and a circuit file from `argv[3]`. Both images are hard-coded into the circuit file before launching it in Logisim Evolution. The circuit design adheres to the Atto-8 microcomputer specification as defined in [/spec/microcomputer.md](../spec/microcomputer.md).
