#include <stdio.h>

unsigned fib(unsigned n) {
  if (n < 2)
    return n;

  return fib(n - 1) + fib(n - 2);
}

void print_fib(unsigned n) {
  if (n != 0)
    print_fib(n - 1);

  printf("%u ", fib(n));
}

const unsigned n = 13;

void main(void) {
  print_fib(n);
  putchar('\n');
}
