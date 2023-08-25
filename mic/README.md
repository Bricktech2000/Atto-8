# Mic

_Microcode builder for Atto-8 microprocessor_

## Overview

The microcode builder is a tool for generating microcode for the Atto-8 microprocessor. It outputs a microcode image file to `argv[1]` which is exactly `0x2000` bytes in size. The microcode builder adheres to the Atto-8 microprocessor specification as defined in [/spec/microprocessor.md](../spec/microprocessor.md).

Unofficial instruction `0xBB` is mapped to unofficial control word `0xFFFD` for debug requests.

## Conventions

The fetch cycle assumes that `YL` is set to `0x00`, which allows it to be significantly shorter. Consequently, every instruction must reset `YL` to `0x00` before clearing `SC` and fetching the next instruction.
