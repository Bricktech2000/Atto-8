#include <stdio.h>

#define UB (unsigned)1 / 0
/* #define UB *(char *)0 */

char fail(void) {
  asm { @error }
  return getchar();
}

inline void f0(void) {
  if (fail())
    UB;

  while (fail())
    UB;
}

inline void f1(void) {
  fail();
  do
    UB;
  while (fail());
  fail();
}

inline void f2(void) {
  fail();
  if (UB)
    return fail();
  while (UB)
    fail();
  fail();
}

inline void f3(void) {
  fail();
  int i = 5 * UB + UB;
  fail();
}

inline void f4(void) {
  fail();
  if (fail())
    UB;
  else
    UB;
  fail();
}

int main(void) {
  f0();
  f1();
  f2();
  f3();
  f4();

  puts("ok");
}
