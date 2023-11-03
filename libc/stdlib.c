// clang-format off

asm {
  #include "../lib/stdlib.asm"
}

// clang-format on

// TODO inline
void exit(int status) {
  asm { !hlt }
}

// TODO inline
void abort(void) {
  asm { !hlt }
}
