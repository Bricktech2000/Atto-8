# Atto-8

A minimalist 8-bit microcomputer with stack-based microprocessor

## Repository Structure

- [/spec/](./spec/) &mdash; Specification for Atto-8 microprocessor and microcomputer
- [/lib/](./lib/) &mdash; Assembly standard library for Atto-8 microcomputer
- [/enc/](./enc/) &mdash; Hex-to-machine-code encoder for Atto-8 microprocessor
- [/asm/](./asm/) &mdash; Optimizing assembler for Atto-8 microprocessor
- [/emu/](./emu/) &mdash; High-level emulator for Atto-8 microcomputer

## Project Status

This project is a work in progress. Try it out by running any of the following commands:

```bash
# finished programs
cargo run --bin asm asm/tests/prng.asm emu/tests/prng.bin && cargo run --bin emu emu/tests/prng.bin
cargo run --bin asm asm/tests/draw.asm emu/tests/draw.bin && cargo run --bin emu emu/tests/draw.bin
cargo run --bin asm asm/tests/counter.asm emu/tests/counter.bin && cargo run --bin emu emu/tests/counter.bin
cargo run --bin asm asm/tests/mushroom.asm emu/tests/mushroom.bin && cargo run --bin emu emu/tests/mushroom.bin
cargo run --bin asm asm/tests/game\ of\ life.asm emu/tests/game\ of\ life.bin && cargo run --bin emu emu/tests/game\ of\ life.bin
cargo run --bin asm asm/tests/this\ is\ a\ test.asm emu/tests/this\ is\ a\ test.bin && cargo run --bin emu emu/tests/this\ is\ a\ test.bin

# feature tests
cargo run --bin asm asm/tests/errors.asm emu/tests/errors.bin && cargo run --bin emu emu/errors.bin
cargo run --bin asm asm/tests/optimization.asm emu/tests/optimization.bin && cargo run --bin emu emu/tests/optimization.bin

# work in progress
cargo run --bin asm asm/tests/pong.asm emu/tests/pong.bin && cargo run --bin emu emu/tests/pong.bin
```
