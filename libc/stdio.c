#include <incl/stdio.h>

asm { @ lib/stdio.asm }

inline char block_getc(void);
inline void putc(char c);
int getchar(void) { return block_getc(); }
inline int putchar(int c) { return putc(c), c; }
