#include <stdio.h>

unsigned collatz(unsigned n) {
  printf("%u ", n);

  if (n == 1)
    return 0;

  if (n % 2)
    return 1 + collatz(3 * n + 1);
  else
    return 1 + collatz(n / 2);
}

const unsigned n = 11;

void main(void) { printf("(%u)\n", collatz(n)); }
