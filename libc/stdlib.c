asm { @ lib/stdlib.asm }

inline void hlt(void);
inline void exit(int status) { hlt(); }
inline void abort(void) { hlt(); }
