#include <incl/ctype.h>

int isalnum(int c) { return isalpha(c) | isdigit(c); }

int isalpha(int c) { return (unsigned)(c | ' ') - 'a' <= 'z' - 'a'; }

int isblank(int c) { return c == ' ' | c == '\t'; }

int iscntrl(int c) { return c == '\x7f' | (unsigned)c < ' '; }

int isdigit(int c) { return (unsigned)c - '0' <= '9' - '0'; }

int isgraph(int c) { return (unsigned)c - '!' <= '~' - '!'; }

int islower(int c) { return (unsigned)c - 'a' <= 'z' - 'a'; }

int isprint(int c) { return (unsigned)c - ' ' <= '~' - ' '; }

int ispunct(int c) { return isgraph(c) & ~isalnum(c); }

int isspace(int c) { return c == ' ' | (unsigned)c - '\t' <= '\r' - '\t'; }

int isupper(int c) { return (unsigned)c - 'A' <= 'Z' - 'A'; }

int isxdigit(int c) {
  return isdigit(c) | (unsigned)(c | ' ') - 'a' <= 'f' - 'a';
}

int tolower(int c) {
  if (isupper(c))
    return c ^ ' ';
  return c;
}

int toupper(int c) {
  if (islower(c))
    return c ^ ' ';
  return c;
}
