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

inline void bar(void) memset(DISPLAY_BUFFER, 0xFF, DISPLAY_BUFFER_LEN);
inline void baz(void) { memset(DISPLAY_BUFFER, 0x00, DISPLAY_BUFFER_LEN); }

inline void delay(int iterations);
void sort(size_t len, void_p arr);
int foo(void);

int main(void) {
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
  putc(foo() + (char)1);
  puts("***ok\r\n" + 0b0011);

  while (1) {
    bar();
    delay(0x1F);
    baz();
    delay(0x1F);
  }
}

int foo(void) {
  return 42;
  foo();
}
