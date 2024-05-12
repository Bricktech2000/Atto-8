#include <stddef.h>

inline void dbg(void);
inline void trap(void);
inline void here(void);
inline void nop(void);
inline void hlt(void);
inline void stall(unsigned iters);
inline void ofst(ptrdiff_t ofst, void *ptr);
