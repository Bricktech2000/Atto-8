#include <stdio.h>

void hanoi(unsigned n, char dst, char via, char src) {
  if (n != 0) {
    hanoi(n - 1, via, dst, src); // `n - 1` disks from `src` to `via`
    printf("#%u | %c -> %c\n", n, src, dst); // `1` disk from `src` to `dst`
    hanoi(n - 1, dst, src, via); // `n - 1` disks from `via` to `dst`
  }
}

const unsigned n = 5;

void main(void) { hanoi(n, 'C', 'B', 'A'); }
