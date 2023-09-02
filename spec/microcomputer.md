# Atto-8 Microcomputer

## Overview

The Atto-8 microcomputer is a minimalist computer based on the Atto-8 microprocessor as defined in [/spec/microprocessor.md](../spec/microprocessor.md). It equips the processor with a clock, memory, standard input/output, a display, and a pair of D-pad controllers. It is designed to be a simple system that takes full advantage of the Atto-8 microprocessor. It is intended to be used as a learning tool for students and hobbyists, and as a basis for more complex computers.

## Features

- 1 MHz clock
- 256 bytes of memory
- Standard input/output
- 16x16 pixel display
- Two D-pad controllers

## Standard Input/Output

The Atto-8 microcomputer supports standard input/output through the _input buffer_, a byte located at address `0x00`. Standard input and standard output are buffered in hardware; reading from the input buffer will return the next byte from `stdin`, and writing to the input buffer will send the byte to `stdout`. If `stdin` is empty, reading from the input buffer will fall back to returning controller states. Reads and writes to to address `0x00` are intercepted by the microcomputer, and, therefore, the input buffer does not behave like other memory regions.

On startup, `stdin` is initialized with the contents of memory address `0x00`. That is, the first read from address `0x00` will return the byte located at address `0x00`, and subsequent reads will return either bytes from standard input or controller states.

## Display

The Atto-8 microcomputer is equipped with a 16x16 pixel monochrome display. It fetches rows of pixels from addresses `0xE0..0x100`, the _display buffer_, and displays them from left to right, top to bottom. Refer to the following diagram, in which `[ 0xXX ]` represents a row of 8 pixels fetched from address `0xXX`:

```
[      0xE0      ] [      0xE1      ]
[      0xE2      ] [      0xE3      ]
[      0xE4      ] [      0xE5      ]
[      0xE6      ] [      0xE7      ]
[      0xE8      ] [      0xE9      ]
[      0xEA      ] [      0xEB      ]
[      0xEC      ] [      0xED      ]
[      0xEE      ] [      0xEF      ]
[      0xF0      ] [      0xF1      ]
[      0xF2      ] [      0xF3      ]
[      0xF4      ] [      0xF5      ]
[      0xF6      ] [      0xF7      ]
[      0xF8      ] [      0xF9      ]
[      0xFA      ] [      0xFB      ]
[      0xFC      ] [      0xFD      ]
[      0xFE      ] [      0xFF      ]
```

The display buffer behaves as any other memory region and can therefore both be read from and written to by a program.

## Controller

The Atto-8 microcomputer is equipped with a pair of memory-mapped 4-button D-pad controllers. If `stdin` is empty, reading from the _input buffer_, a byte located at address `0x00`, will fall back to returning controller states. In that event, the lower 4 bits of the input buffer represent the state of the buttons on the primary controller, and its upper 4 bits represent the state of the buttons on the secondary controller. It is bit-mapped as follows, where `0` represents the least significant bit:

```
7 6 5 4 3 2 1 0
| | | | | | | |
r l d u R L D U
| | | | | | | |
| | | | | | | +--- Primary Up
| | | | | | +----- Primary Down
| | | | | +------- Primary Left
| | | | +--------- Primary Right
| | | +----------- Secondary Up
| | +------------- Secondary Down
| +--------------- Secondary Left
+----------------- Secondary Right

Primary  Secondary
   U         u
 L + R     l + r
   D         d
```
