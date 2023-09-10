#define CORE "lib/core.asm"

asm {
  #include CORE
  #include <lib/stdio.asm>
}

#include <stdio.h>

// #define EMPTY
// #include EMPTY<file.h>
// EMPTY #include <file.h>

#define ADDITION 1 + TWO
#define TWO 2

int foo() {
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

  ADDITION == 3;
  return foo() + (char)1;
  return 2 > 1 == 4 >= 2;
}
