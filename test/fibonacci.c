#include <stdio.h>

unsigned fib(unsigned n) {
  if (n < 2)
    return n;

  return fib(n - 1) + fib(n - 2);
}

const unsigned n = 13;

void main(void) {
  // for (unsigned i = 0; i <= n; i++)
  //   printf("fib(%u) = %u\r\n", i, fib(i));

  printf("fib(%u) = %u\r\n", n, fib(n));
}
