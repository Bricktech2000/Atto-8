#pragma once

#define STDIO "stdio.h"

#include STDIO
#include <stdlib.h>
#include <string.h>

#undef STDIO
#undef STDIO
#undef UNDEFINED

// #define EMPTY
// #include EMPTY<file.h>
// EMPTY #include <file.h>

// #error test error in file __FILE__ on line __LINE__

// #invalid directive

#// null directive

#define ADDITION 1 + TWO
#define TWO 2

#define DISPLAY_BUFFER (void *)0xE0
#define DISPLAY_BUFFER_LEN 0x20

inline void bar(void) memset(DISPLAY_BUFFER, 0xFF, DISPLAY_BUFFER_LEN);
inline void baz(void) { memset(DISPLAY_BUFFER, 0x00, DISPLAY_BUFFER_LEN); }

inline void delay(int iterations);
void sort(size_t len, void *arr);
int foo(int n, char *test);

void main(void) {
  // return 0;
  // return 1 + 2;
  // return ~2 + (3 + 4) * 5;
  // return 1 || 0 && 2;
  // return 2 == 4 >= 2;
  // return 2 > 1 == 4 >= 2;
  // return 6 * 10 + 4;
  // ADDITION == 3;

  // sort(strlen("Atto-8"), "Atto-8");
  // puts("Atto-8");

  free(malloc(10));
  putc(foo(42, "test") + (char)1);
  puts("***ok\r\n" + (unsigned)0b0011);

  asm { xE0 sts } // `!display_buffer`
  while (1) {
    bar();
    delay(0x1F);
    baz();
    delay(0x1F);
  }
}

int foo(int n, char *test) {
  return n;
  foo(n + 1, test);
}
