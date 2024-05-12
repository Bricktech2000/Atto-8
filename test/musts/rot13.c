#include <stdio.h>

void main(void) {
  while (1) {
    char c = getchar();
    unsigned l = (c | ' ') - 'a';

    if (l <= 13)
      putchar(c + 13);
    else if (l - 13 <= 13)
      putchar(c - 13);
    else
      putchar(c);
  }
}
