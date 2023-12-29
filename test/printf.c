#include <stdio.h>

void main(void) {
  printf("0x%x - %u = %d\r\n", 0x21, 0x32, 0x21 - 0x32);
  printf("'%c' uses %%c\r\n", 'A');
  printf("fmt = (char*)%p\r\n", "fmt = (char*)%p\r\n");
  printf("fmt = \"%s\"", "fmt = \"%s\"");

  return;
}
