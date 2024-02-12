#include "incl/core.h"
#include <incl/stdlib.h>

asm { @ lib/stdlib.asm }

inline void exit(int status) { hlt(); }
inline void abort(void) { hlt(); }
