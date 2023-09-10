# Test

_Test programs and test framework for Atto-8 microcomputer_

## Overview

End-to-end testing of the Atto-8 microcomputer is carried out by `test.py`. The script takes as argument a list of operations to perform and filenames to load. The program is a stack machine, allowing arbitrary operations to be fed into one another. Operation execution begins only after the argument string is fully parsed and deemed well-formed. Warnings are emitted if operations return non-zero exit codes.

Available operations are as follows:

- `cc` &mdash; See [/cc/](../cc/)
- `enc` &mdash; See [/enc/](../enc/)
- `dec` &mdash; See [/dec/](../dec/)
- `asm` &mdash; See [/asm/](../asm/)
- `dasm` &mdash; See [/dasm/](../dasm/)
- `emu` &mdash; See [/emu/](../emu/)
- `mic` &mdash; See [/mic/](../mic/)
- `sim` &mdash; See [/sim/](../sim/)
- `circ` &mdash; See [/circ/](../circ/)
- `pop` &mdash; Pop argument from the stack
- `dup` &mdash; Duplicate argument on the stack
- `pipe` &mdash; Pipe file contents to stdout

## Examples

```bash
# assemble source code, emulate binary
python3 test.py dino.asm asm emu

# assemble source code, disassemble binary
python3 test.py flappy.asm asm dasm pop

# assemble source code, emulate binary. assemble source code, build microcode, simulate binary with microcode
python3 test.py ctf.asm asm emu ctf.asm asm mic sim

# encode from hex, disassemble binary, assemble disassembly, build microcode, launch circuit with resulting binary and microcode
python3 test.py checkerboard.hex enc dasm asm mic circ

# assemble source code, decode to hex, encode from hex, disassemble binary, assemble disassembly, build microcode, simulate resulting binary with microcode
python3 test.py life.asm asm dec enc dasm asm mic sim
```
