#include <stdio.h>

unsigned fib(unsigned n) {
  if (n == 0)
    return n;

  if (n == 1)
    return n;

  return fib(n - 1) + fib(n - 2);
}

#define N 13

void main(void) {
  // for (unsigned i = 0; i < 14; i++)
  //   printf("fib(%u) = %u\r\n", i, fib(i));

  printf("fib(%u) = %u\r\n", N, fib(N));
}
