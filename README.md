![Atto-8 Banner](misc/assets/banner.png)

# Atto-8

A minimalist 8-bit microcomputer with stack-based microprocessor

## Repository Structure

- [/spec/](spec/) &mdash; Specification for Atto-8 microprocessor and microcomputer
- [/lib/](lib/) &mdash; Assembly standard library for Atto-8 microprocessor and microcomputer
- [/enc/](enc/) &mdash; Hex-to-machine-code encoder for Atto-8 microprocessor
- [/asm/](asm/) &mdash; Optimizing assembler for Atto-8 microprocessor
- [/emu/](emu/) &mdash; Instruction-level emulator for Atto-8 microcomputer
- [/misc/](misc/) &mdash; Miscellaneous files

## Project Status

This project is a [work in progress](TODO.md). Try it out by running any of the following commands:

```bash
# finished programs
cargo run --bin asm asm/tests/draw.asm emu/tests/draw.bin && cargo run --bin emu emu/tests/draw.bin
cargo run --bin asm asm/tests/random.asm emu/tests/random.bin && cargo run --bin emu emu/tests/random.bin
cargo run --bin asm asm/tests/memory.asm emu/tests/memory.bin && cargo run --bin emu emu/tests/memory.bin
cargo run --bin asm asm/tests/counter.asm emu/tests/counter.bin && cargo run --bin emu emu/tests/counter.bin
cargo run --bin asm asm/tests/mushroom.asm emu/tests/mushroom.bin && cargo run --bin emu emu/tests/mushroom.bin
cargo run --bin asm asm/tests/flappy\ bird.asm emu/tests/flappy\ bird.bin && cargo run --bin emu emu/tests/flappy\ bird.bin
cargo run --bin asm asm/tests/game\ of\ life.asm emu/tests/game\ of\ life.bin && cargo run --bin emu emu/tests/game\ of\ life.bin

# work in progress
cargo run --bin asm asm/tests/pong.asm emu/tests/pong.bin && cargo run --bin emu emu/tests/pong.bin
cargo run --bin asm asm/tests/snake.asm emu/tests/snake.bin && cargo run --bin emu emu/tests/snake.bin

# assembler tests
cargo run --bin asm asm/tests/errors.asm emu/tests/errors.bin && cargo run --bin emu emu/errors.bin
cargo run --bin asm asm/tests/optimization.asm emu/tests/optimization.bin && cargo run --bin emu emu/tests/optimization.bin

# hand-assembled programs
python3 enc/enc.py enc/tests/addition.hex emu/tests/addition.bin && cargo run --bin emu emu/tests/addition.bin
python3 enc/enc.py enc/tests/checkerboard.hex emu/tests/checkerboard.bin && cargo run --bin emu emu/tests/checkerboard.bin
```

![Game of Life Demo](misc/assets/game%20of%20life.gif) ![Random Demo](misc/assets/random.gif) ![Flappy Bird Demo](misc/assets/flappy%20bird.gif) ![Counter Demo](misc/assets/counter.gif)
