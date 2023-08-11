# Test

_Test programs and test framework for Atto-8 microcomputer_

## Overview

End-to-end testing of the Atto-8 microprocessor and microcomputer is done through `test.py`. The script takes as argument a list of operations to perform and filenames to load. The program is a stack machine, allowing arbitrary operations to be fed into one another. Available operations are:

- `enc` &mdash; see [/enc/](../enc/)
- `asm` &mdash; see [/asm/](../asm/)
- `dasm` &mdash; see [/dasm/](../dasm/)
- `emu` &mdash; see [/emu/](../emu/)
- `mic` &mdash; see [/mic/](../mic/)
- `sim` &mdash; see [/sim/](../sim/)

## Examples

```bash
# assemble source code, emulate binary
python3 test.py flappy.asm asm emu

# assemble source code, disassemble binary, assemble disassembly, emulate resulting binary
python3 test.py life.asm asm dasm asm emu

# assemble source code, emulate binary. assemble source code, compile microcode, simulate binary with microcode
python3 test.py ctf.asm asm emu ctf.asm asm mic sim

# encode from hex, disassemble binary, assemble disassembly, compile microcode, simulate resulting binary with microcode
python3 test.py checkerboard.hex enc dasm asm mic sim
```
