#define STDIO "stdio.h"

#include STDIO
#include <stdlib.h>
#include <string.h>

// #define EMPTY
// #include EMPTY<file.h>
// EMPTY #include <file.h>

#define ADDITION 1 + TWO
#define TWO 2

#define DISPLAY_BUFFER 0xE0
#define DISPLAY_BUFFER_LEN 0x20

inline void bar(void) {
  return memset(DISPLAY_BUFFER, 0xFF, DISPLAY_BUFFER_LEN);
}
inline void baz(void) {
  return memset(DISPLAY_BUFFER, 0x00, DISPLAY_BUFFER_LEN);
}

inline void delay(int iterations);
int foo(void);

int main(void) {
  // return 0;

  // return 1 + 2;

  // return ~2 + (3 + 4) * 5;

  // return 1 || 0 && 2;

  // return 2 == 4 >= 2;

  // return 2 > 1 == 4 >= 2;

  // return 6 * 10 + 4;

  free(malloc(2));

  puts("***ok\r\n" + 3);

  putc(foo() + (char)1);

  while (1) {
    bar();
    delay(0x1F);
    baz();
    delay(0x1F);
  }

  ADDITION == 3;
}

int foo(void) {
  return 42;
  foo();
}
