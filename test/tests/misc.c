#pragma once

#define STDIO "stdio.h"

#include STDIO
#include "display.h"
#include <iso646.h>
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

inline void impure(void) asm {}
inline void bar(void) memset(DISPLAY_BUFFER, 0xff, DISPLAY_BUFFER_LEN);
inline void baz(void) { memset(DISPLAY_BUFFER, 0x00, DISPLAY_BUFFER_LEN); }

inline void delay(int iterations);
int foo(int n, char *test);

void main(void) {
  // return 0;
  // return !5;
  // return +-+-5;
  // return 1 + 2;
  // return compl 2 + (3 + 4) * 5;
  // return 1 or 0 and 2;
  // return 2 == 4 >= 2;
  // return 2 > 1 == 4 >= 2;
  // return 6 * 10 + 4;
  // ADDITION == 3;

  // char *str = malloc(5);
  // strcpy(str, "Atto");
  // // malloc(1);
  // char *new_str = realloc(str, 7);
  // strcat(new_str, "-8");
  // puts(new_str);

  if (0)
    ;

  do
    puts("in file " __FILE__ "\n");
  while (!main);

  unsigned offset = 0b0011;
  putchar((&*foo)(1, "/-\\|") + '\000' + '\x00' + 000 + 0b0 + 0B0 + 0x0 + 0X0);
  if (offset + 1)
    puts("***ok\n" + offset);
  13 * ~offset + (impure(), 5);

  asm { !display_buffer sts }
  while (impure(), (bar(), 1)) {
    delay(0x1F);
    baz(), delay(0x1F);
    continue;
    break;
  }
}

int foo(int n, char *test) {
  { return n[test]; }
  foo(n + 1, test);
}
