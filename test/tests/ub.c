#include <stdio.h>

#define UB (unsigned)1 / 0
/* #define UB *(char *)0 */

inline char fail(void) asm { !trap }

inline char ok(char c) asm { !putc x00 }

inline void f0(void) {
  if (ok('0'))
    UB;

  while (ok('1'))
    UB;
}

inline void f1(void) {
  do
    UB;
  while (fail());

  fail();
}

inline void f2(void) {
  do
    ok('2');
  while (UB);

  fail();
}

inline void f3(void) {
  if (UB)
    return fail();

  while (UB)
    fail();

  fail();
}

inline void f4(void) {
  ok('3');
  int i = 5 * UB + UB;
  fail();
}

inline void f5(void) {
  if (ok('4'))
    UB;
  else {
    while (1)
      break;
    ok('5');
    UB;
    fail();
  }

  fail();
}

inline void f6(void) {
  if (ok('6'))
    UB;
  else {
    ok('7');
    return;
  }

  fail();
}

void main(void) {
  f0();
  f1();
  f2();
  f3();
  f4();
  f5();
  f6();

  ok('8');
}
