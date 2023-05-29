# Test

_Test programs and test framework for Atto-8 microcomputer_

## Overview

End-to-end testing of the Atto-8 microprocessor and microcomputer is done through `test.py`. The script takes as arguments a list of operations to perform along with the path to a file to load. The output of one operation is fed into the next operation. Available operations are:

- `enc` &mdash; see [/enc/](../enc/)
- `asm` &mdash; see [/asm/](../asm/)
- `dasm` &mdash; see [/dasm/](../dasm/)
- `emu` &mdash; see [/emu/](../emu/)

## Examples

```bash
# assemble source code, disassemble binary, assemble disassembly, emulate resulting binary
python3 test.py asm dasm asm emu life.asm

# encode from hex, disassemble binary, assemble disassembly, emulate resulting binary
python3 test.py enc dasm asm emu addition.hex
```
