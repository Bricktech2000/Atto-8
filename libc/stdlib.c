// clang-format off

asm {
  #include "../lib/stdlib.asm"
}

// clang-format on

inline void exit(int status) {
  asm { !hlt }
}

inline void abort(void) {
  asm { !hlt }
}
