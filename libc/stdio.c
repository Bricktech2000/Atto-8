#include <incl/stdio.h>

asm { @ lib/stdio.asm }

// clang-format off

asm { putchar! !putc }
asm { getchar! !getc }
