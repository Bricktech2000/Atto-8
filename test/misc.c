#pragma once

#define STDIO "stdio.h"

#include STDIO
#include "display.h"
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#undef STDIO
#undef STDIO
#undef UNDEFINED

// #define EMPTY
// #include EMPTY<file.h>
// EMPTY #include <file.h>

#// null directive

#define ADDITION                                                               \
  1 + TWO //                                     line continuation
#define TWO 2

/* /* multi // line
   comment */

char *str_lit_concat(void) {
  return "string literal "
         "concatenation";
}

inline void bar(void) memset(DISPLAY_BUFFER, 0xff, DISPLAY_BUFFER_LEN);
inline void baz(void) { memset(DISPLAY_BUFFER, 0x00, DISPLAY_BUFFER_LEN); }

inline void delay(int iterations);
void sort(size_t len, void *arr);
int foo(int n, char *test);

void main(void) {
  // return 0;
  // return !5;
  // return +-+-5;
  // return 1 + 2;
  // return ~2 + (3 + 4) * 5;
  // return 1 || 0 && 2;
  // return 2 == 4 >= 2;
  // return 2 > 1 == 4 >= 2;
  // return 6 * 10 + 4;
  // ADDITION == 3;

  // sort(strlen("Atto-8"), "Atto-8");
  // puts("Atto-8");

  if (true)
    free(malloc(10));
  putc(foo(42, "test") + '\000' + '\x00' + 000 + 0b0 + 0B0 + 0x0 + 0X0);
  puts("***ok\r\n" + (unsigned)0b0011);

  asm { !display_buffer sts }
  while (1) {
    bar();
    delay(0x1F);
    baz();
    delay(0x1F);
  }
}

int foo(int n, char *test) {
  { return n; }
  foo(n + 1, test);
}
