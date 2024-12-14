# Test

_Test programs and test framework for Atto-8 microcomputer_

## Overview

End-to-end testing of the Atto-8 microcomputer is carried out by ‘test.py’. The script takes as argument a list of operations to perform and filenames to load. The program is a stack machine, allowing arbitrary operations to be fed into one another. Operation execution begins only after the argument string is fully parsed and deemed well-formed. Warnings are emitted if operations return non-zero exit codes.

Available operations are as follows:

- `cc` — See [/cc/](../cc/)
- `enc` — See [/enc/](../enc/)
- `dec` — See [/dec/](../dec/)
- `asm` — See [/asm/](../asm/)
- `dasm` — See [/dasm/](../dasm/)
- `emu` — See [/emu/](../emu/)
- `cemu` — See [/cemu/](../cemu/)
- `mic` — See [/mic/](../mic/)
- `sim` — See [/sim/](../sim/)
- `circ` — See [/circ/](../circ/)
- `bf` — See [/bf/](../bf/)
- `pop` — Pop argument from the stack
- `dup` — Duplicate argument on the stack
- `pipe` — Pipe file contents to stdout

## Examples

```bash
# assemble source code, emulate binary
python3 test.py dino.asm asm emu

# assemble source code, disassemble binary
python3 test.py flappy.asm asm dasm pop

# compile to assembly with stdlib and stdio, assemble assembly, emulate binary
python3 test.py hanoi.c libc/stdlib.c libc/stdio.c libc/crt0.c cc asm emu

# assemble source code, emulate binary. assemble source code, build microcode, launch block-level circuit with binary and microcode
python3 test.py ctf.asm asm cemu ctf.asm asm mic block.circ circ

# encode from hex, disassemble binary, assemble disassembly, build microcode, simulate resulting binary with microcode
python3 test.py checkerboard.hex enc dasm asm mic sim

# assemble source code, decode to hex, encode from hex, disassemble binary, assemble disassembly, build microcode, simulate resulting binary with microcode
python3 test.py life.asm asm dec enc dasm asm mic sim

# build brainfuck microcode, simulate brainfuck source with microcode
python3 test.py squares.bf bf sim

# build brainfuck microcode, launch chip-level circuit with brainfuck source and microcode
python3 test.py fib.bf bf chip.circ circ
```
