# Circ

_Block-level and chip-level circuit designs for Atto-8 microcomputer_

## Overview

The circuit designs are a pair of [Logisim Evolution](https://github.com/logisim-evolution/logisim-evolution) projects that simulate the Atto-8 microprocessor and microcomputer at the block level and chip level. `circ.py` loads a memory image file from `argv[1]` which must be exactly `0x100` bytes in size, a microcode image file from `argv[2]` which must be exactly `0x2000` words in size, and a circuit file from `argv[3]`. Both images are hard-coded into the circuit file before it is launched in Logisim Evolution. The circuit designs adhere to the Atto-8 microcomputer specification as defined in [/spec/microcomputer.md](../spec/microcomputer.md).

## Conventions

Tunnels whose label begins with an `N_` are _negated_ and tunnels whose label begins with a `C_` are _clocked_. Expressed in pseudocode,

```rust
let N_TUNNEL_LABEL = !TUNNEL_LABEL; // negated tunnel
let C_TUNNEL_LABEL = CLK & TUNNEL_LABEL; // clocked tunnel
```
