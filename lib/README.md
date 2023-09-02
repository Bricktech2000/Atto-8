# Lib

_Assembly standard library for Atto-8 microcomputer_

## Overview

This library provides utilities and convenience macros for the Atto-8 microcomputer. It is to be assembled with the Atto-8 assembler located in [/asm/](../asm/).

- [/lib/core.asm](core.asm) &mdash; Core instruction-like macros such as `!call` and `!hlt`
- [/lib/types.asm](types.asm) &mdash; Type definitions and operations such as fixed-point arithmetic and integer multiplication
- [/lib/string.asm](string.asm) &mdash; String operations inspired by the C header `string.h` such as `strlen` and `memcpy`
- [/lib/stdlib.asm](stdlib.asm) &mdash; Standard library functions inspired by the C header `stdlib.h` such as `malloc` and `rand`
- [/lib/stdio.asm](stdio.asm) &mdash; Standard input/output functions inspired by the C header `stdio.h` such as `getc` and `puts`
- [/lib/display.asm](display.asm) &mdash; Display utilities such as pixel manipulation macros and text rendering functions

## Conventions

Files in the standard library do not import their dependencies; they are to be imported by the user.

Macros and labels ending in `.min` are hand-optimized for speed or size. Their API is often different from their non-minified counterparts and may be unintuitive and contain undocumented behavior. Their non-minified counterparts are more readable and are preferred unless speed or size is a concern.

Macros ending in `.dyn` may perform superfluous operations at runtime if their arguments are not constants. Their non-dynamic counterparts leverage the `@const` directive to perform these operations at assembly time. Macros ending in `.dyn` should only be used when the assembler fails to resolve a constant expression in their non-dynamic counterparts.

Macros ending in `.def` are containers for function definitions. Most standard library utilities are provided as macros but some are provided as functions. Invoking a macro ending in `.def` will define a label of the same name at the current address followed by the function implementation. These functions are meant to be called with the `!call` macro.
