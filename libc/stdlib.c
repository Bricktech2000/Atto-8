#include <incl/stdlib.h>
#include <incl/string.h>

asm { @ lib/stdlib.asm }

void *realloc(void *ptr, size_t new_size) {
  // `malloc` stores block header immediately before block data
  size_t size = *((size_t *)ptr - (unsigned)1);

  free(ptr);
  void *new_ptr = malloc(new_size);

  // `malloc` and `free` are guaranteed not to overwrite block data
  if (new_ptr != ptr)
    // technically, `restrict` violated if previous block gets coalesced.
    // in practice, `memcpy` still works because it copies data backwards
    memcpy(new_ptr, ptr, size);

  return new_ptr;
}

// clang-format off

asm { exit! !hlt }
asm { abort! !hlt }
