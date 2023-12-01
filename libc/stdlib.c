asm { @ lib/stdlib.asm }

inline void exit(int status) asm { !hlt }
inline void abort(void) asm { !hlt }
