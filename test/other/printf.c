#include <stdio.h>

inline void w_printf(char *str) { printf(str, str); }

void main(void) {
  printf("0x%x - %u = %d\n", 0x21, 0x32, 0x21 - 0x32);
  printf("'%c' uses %%c\n", 'A');
  w_printf("fmt = (char*)%p\n");
  w_printf("fmt = \"%s\""), putchar('\n');
}
