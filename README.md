![Atto-8 Banner](misc/assets/banner.png)

# Atto-8

A minimalist 8-bit microcomputer with stack-based microprocessor

## Repository Structure

- [/spec/](spec/) &mdash; Specification for Atto-8 microprocessor and microcomputer
- [/lib/](lib/) &mdash; Assembly standard library for Atto-8 microprocessor and microcomputer
- [/enc/](enc/) &mdash; Hex-to-machine-code encoder for Atto-8 microprocessor
- [/asm/](asm/) &mdash; Optimizing assembler for Atto-8 microprocessor
- [/dasm/](dasm/) &mdash; Elementary disassembler for Atto-8 microprocessor
- [/emu/](emu/) &mdash; Instruction-level emulator for Atto-8 microcomputer
- [/misc/](misc/) &mdash; Miscellaneous files

## Project Status

This project is a [work in progress](TODO.md). Try it out by running any of the following commands:

```bash
# finished programs
cargo run --bin asm asm/tests/draw.asm         dasm/tests/draw.bin         && cargo run --bin dasm dasm/tests/draw.bin         dasm/tests/draw.asm         && cargo run --bin asm dasm/tests/draw.asm         emu/tests/draw.bin         && cargo run --bin emu emu/tests/draw.bin
cargo run --bin asm asm/tests/random.asm       dasm/tests/random.bin       && cargo run --bin dasm dasm/tests/random.bin       dasm/tests/random.asm       && cargo run --bin asm dasm/tests/random.asm       emu/tests/random.bin       && cargo run --bin emu emu/tests/random.bin
cargo run --bin asm asm/tests/memory.asm       dasm/tests/memory.bin       && cargo run --bin dasm dasm/tests/memory.bin       dasm/tests/memory.asm       && cargo run --bin asm dasm/tests/memory.asm       emu/tests/memory.bin       && cargo run --bin emu emu/tests/memory.bin
cargo run --bin asm asm/tests/counter.asm      dasm/tests/counter.bin      && cargo run --bin dasm dasm/tests/counter.bin      dasm/tests/counter.asm      && cargo run --bin asm dasm/tests/counter.asm      emu/tests/counter.bin      && cargo run --bin emu emu/tests/counter.bin
cargo run --bin asm asm/tests/mushroom.asm     dasm/tests/mushroom.bin     && cargo run --bin dasm dasm/tests/mushroom.bin     dasm/tests/mushroom.asm     && cargo run --bin asm dasm/tests/mushroom.asm     emu/tests/mushroom.bin     && cargo run --bin emu emu/tests/mushroom.bin
cargo run --bin asm asm/tests/flappy\ bird.asm dasm/tests/flappy\ bird.bin && cargo run --bin dasm dasm/tests/flappy\ bird.bin dasm/tests/flappy\ bird.asm && cargo run --bin asm dasm/tests/flappy\ bird.asm emu/tests/flappy\ bird.bin && cargo run --bin emu emu/tests/flappy\ bird.bin
cargo run --bin asm asm/tests/life.asm         dasm/tests/life.bin         && cargo run --bin dasm dasm/tests/life.bin         dasm/tests/life.asm         && cargo run --bin asm dasm/tests/life.asm         emu/tests/life.bin         && cargo run --bin emu emu/tests/life.bin

# work in progress
cargo run --bin asm asm/tests/pong.asm         dasm/tests/pong.bin         && cargo run --bin dasm dasm/tests/pong.bin         dasm/tests/pong.asm         && cargo run --bin asm dasm/tests/pong.asm         emu/tests/pong.bin         && cargo run --bin emu emu/tests/pong.bin
cargo run --bin asm asm/tests/snake.asm        dasm/tests/snake.bin        && cargo run --bin dasm dasm/tests/snake.bin        dasm/tests/snake.asm        && cargo run --bin asm dasm/tests/snake.asm        emu/tests/snake.bin        && cargo run --bin emu emu/tests/snake.bin

# assembler tests
cargo run --bin asm asm/tests/errors.asm       dasm/tests/errors.bin       && cargo run --bin dasm dasm/tests/errors.bin       dasm/tests/errors.asm       && cargo run --bin asm dasm/tests/errors.asm       emu/tests/errors.bin       && cargo run --bin emu emu/tests/errors.bin
cargo run --bin asm asm/tests/optimization.asm dasm/tests/optimization.bin && cargo run --bin dasm dasm/tests/optimization.bin dasm/tests/optimization.asm && cargo run --bin asm dasm/tests/optimization.asm emu/tests/optimization.bin && cargo run --bin emu emu/tests/optimization.bin

# hand-assembled programs
python3 enc/enc.py  enc/tests/addition.hex     dasm/tests/addition.bin     && cargo run --bin dasm dasm/tests/addition.bin     dasm/tests/addition.asm     && cargo run --bin asm dasm/tests/addition.asm     emu/tests/addition.bin     && cargo run --bin emu emu/tests/addition.bin
python3 enc/enc.py  enc/tests/checkerboard.hex dasm/tests/checkerboard.bin && cargo run --bin dasm dasm/tests/checkerboard.bin dasm/tests/checkerboard.asm && cargo run --bin asm dasm/tests/checkerboard.asm emu/tests/checkerboard.bin && cargo run --bin emu emu/tests/checkerboard.bin
```

![Game of Life Demo](misc/assets/life.gif) ![Random Number Generator Demo](misc/assets/random.gif) ![Flappy Bird Demo](misc/assets/flappy%20bird.gif) ![Infinite Counter Demo](misc/assets/counter.gif)
