![Atto-8 Banner](misc/assets/banner.png)

# Atto-8

_A minimalist 8-bit microcomputer with stack-based microprocessor_

## Repository Structure

- [/spec/](spec/) &mdash; Specification for Atto-8 microprocessor and microcomputer
- [/lib/](lib/) &mdash; Assembly standard library for Atto-8 microprocessor and microcomputer
- [/enc/](enc/) &mdash; Hex-to-machine-code encoder for Atto-8 microprocessor
- [/asm/](asm/) &mdash; Optimizing assembler for Atto-8 microprocessor
- [/dasm/](dasm/) &mdash; Elementary disassembler for Atto-8 microprocessor
- [/emu/](emu/) &mdash; Instruction-level emulator for Atto-8 microcomputer
- [/sim/](sim/) &mdash; Cycle-accurate simulator for Atto-8 microcomputer
- [/test/](test/) &mdash; Test programs and test framework for Atto-8 microcomputer
- [/misc/](misc/) &mdash; Miscellaneous files

## Project Status

This project is a [work in progress](TODO.md). Try it out by running any of the following commands from the [/test/](test/) directory:

```bash
# finished programs
python3 test.py asm emu draw.asm
python3 test.py asm emu random.asm
python3 test.py asm emu memory.asm
python3 test.py asm emu counter.asm
python3 test.py asm emu mushroom.asm
python3 test.py asm emu flappy.asm
python3 test.py asm emu life.asm
python3 test.py asm emu dino.asm

# work in progress
python3 test.py asm emu pong.asm
python3 test.py asm emu snake.asm
python3 test.py asm emu mandelbrot.asm

# test programs
python3 test.py asm emu ore.asm
python3 test.py asm emu multiply.asm
python3 test.py asm emu errors.asm
python3 test.py asm emu optimization.asm

# hand-assembled programs
python3 test.py enc emu nop.hex
python3 test.py enc emu addition.hex
python3 test.py enc emu checkerboard.hex
```

![Game of Life Demo](misc/assets/life.gif) ![Random Number Generator Demo](misc/assets/random.gif) ![Flappy Bird Demo](misc/assets/flappy.gif) ![Infinite Counter Demo](misc/assets/counter.gif)
