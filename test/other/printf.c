#include <stdio.h>

inline void w_printf(char *str) { printf(str, str); }

void main(void) {
  printf("0x%x - %u = %d\r\n", 0x21, 0x32, 0x21 - 0x32);
  printf("'%c' uses %%c\r\n", 'A');
  w_printf("fmt = (char*)%p\r\n");
  w_printf("fmt = \"%s\"");
}
