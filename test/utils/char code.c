#include <stdio.h>

char tolower(char c) {
  if ((unsigned)c - 'A' <= 'Z' - 'A')
    return c + ' ';
  else
    return c;
}

char toupper(char c) {
  if ((unsigned)c - 'a' <= 'z' - 'a')
    return c - ' ';
  else
    return c;
}

void main(void) {
  while (1)
    // printf(" %u", getchar());
    printf(" @%x", getchar());
  // printf("%c", tolower(getchar()));
  // printf("%c", toupper(getchar()));
}
