#define STDIO "stdio.h"

#include STDIO
#include <stdlib.h>

// #define EMPTY
// #include EMPTY<file.h>
// EMPTY #include <file.h>

#define ADDITION 1 + TWO
#define TWO 2

int foo() {
  free(malloc(10));

  return 42;
  foo();
}

int main() {
  // return 0;

  // return 1 + 2;

  // return ~2 + (3 + 4) * 5;

  // return 1 || 0 && 2;

  // return 2 == 4 >= 2;

  // return 2 > 1 == 4 >= 2;

  // return 6 * 10 + 4;

  puts("***ok\r\n" + 3);

  ADDITION == 3;
  return foo() + (char)1;
  return 2 > 1 == 4 >= 2;
}
