#include <incl/ctype.h>

int isalnum(int c) { return isalpha(c) | isdigit(c); }

int isalpha(int c) { return (unsigned)(c | ' ') - 'a' <= 'z' - 'a'; }

int isblank(int c) { return c == ' ' | c == '\t'; }

int iscntrl(int c) { return (unsigned)c < ' '; }

int isdigit(int c) { return (unsigned)c - '0' <= '9' - '0'; }

int isgraph(int c) { return (unsigned)c - '!' < '~' - '!'; }

int islower(int c) { return (unsigned)c - 'a' <= 'z' - 'a'; }

int isprint(int c) { return (unsigned)c - ' ' < '~' - ' '; }

int ispunct(int c) { return isgraph(c) & ~isalnum(c); }

int isspace(int c) asm {
  // clang-format off
  x00
    x01 !'\s' xo4 iff
    x01 !'\f' xo4 iff
    x01 !'\n' xo4 iff
    x01 !'\r' xo4 iff
    x01 !'\t' xo4 iff
    x01 !'\v' xo4 iff
  st1
  // clang-format on
}

int isupper(int c) { return (unsigned)c - 'A' <= 'Z' - 'A'; }

int isxdigit(int c) {
  return isdigit(c) | (unsigned)(c | ' ') - 'a' <= 'f' - 'a';
}

int tolower(int c) {
  if (isupper(c))
    return c ^ ' ';
  else
    return c;
}

int toupper(int c) {
  if (islower(c))
    return c ^ ' ';
  else
    return c;
}
