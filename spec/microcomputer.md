# Atto-8 Microcomputer

## Overview

The Atto-8 microcomputer is a minimalist computer based on the Atto-8 microprocessor, as defined in [/spec/microprocessor.md](../spec/microprocessor.md). It equips the processor with a clock, memory, a display, and a pair of D-pad controllers. It is designed to be a simple system that takes full advantage of the Atto-8 microprocessor. It is intended to be used as a learning tool for students and hobbyists, and as a basis for more complex computers.

## Features

- 100kHz clock
- 256 bytes of memory
- 16x16 pixel display
- Two D-pad controllers

## Display

The Atto-8 is equipped with a 16x16 pixel monochrome display. It fetches rows of pixels from addresses `0xE0..=0xFF`, the _display buffer_, and displays them from left to right, top to bottom. Refer to the following diagram, in which `[ 0xXX ]` represents a row of 8 pixels fetched from address `0xXX`:

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

## Input

The Atto-8 is equipped with a pair of memory-mapped 4-button D-pad controllers. The _input buffer_ is a byte located at address `0x00`, the lower 4 bits of which represent the state of the buttons on the primary controller, and the upper 4 bits of which represent the state of the buttons on the secondary controller. It is bit-mapped as follows, where `0` represents the least significant bit:

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

Upon a button state change, the microcomputer will set or clear the corresponding bit in the input buffer, without affecting the other bits. The input buffer behaves as any other memory region and can therefore both be read from and written to by a program.

On boot up, input is disabled and the Atto-8 will not write to the input buffer. To enable input permanently, the program must write `0x00` to the input buffer.

## Conventions

By convention, functions are called by pushing their arguments onto the stack, pushing a return address onto the stack, and then jumping to the function's address. It is recommended that functions replace their arguments with their return values prior to returning as to mirror the behavior of instructions on the Atto-8 microprocessor.
