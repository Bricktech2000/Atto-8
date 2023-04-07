# Atto-8 Microcomputer

## Overview

The Atto-8 microcomputer is a minimalist computer based on the Atto-8 microprocessor, as defined in [/spec/microprocessor.md](../spec/microprocessor.md). It equips the processor with memory, a clock, a display, and #todo input. It is designed to be a simple, easy-to-understand computer that takes full advantage of the Atto-8 microprocessor.

## Features

- 16x16 pixel display
- #todo Hz clock
- 256 bytes of RAM
- #todo input

## Display

The Atto-8 has a 16x16 pixel monochrome display. It fetches rows of pixels from addresses `0xE0..=0xFF`, and displays them from left to right, top to bottom. Refer to the following diagram, in which `[ 0xXX ]` represents a row of 8 pixels fetched from address `0xXX`:

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

#todo

## Conventions

By convention, functions are called by pushing their arguments onto the stack, pushing a return address onto the stack, and then jumping to the function's address. It is recommended that functions replace their arguments with their return values prior to returning as to mirror the behavior of instructions on the Atto-8 microprocessor.
