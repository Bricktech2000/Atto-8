// clang-format off

// import macros referenced by C compiler, such as
// `!call`, `!ret`, `!jmp`, `!mul`, `!div`, `!mod`
asm { @ lib/core.asm }
asm { @ lib/types.asm }

// import crt0 bootstrap routines
asm { @ libc/crt0.asm }
