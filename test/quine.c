// constructive quine. uses `CRLF` line endings to ensure the carriage is
// returned. assumes `'\r'`, `'\n'` and `'"'` to occupy codepoints 13, 10
// and 34, respectively

// clang-format off

#include<stdio.h>
char*s="#include<stdio.h>%c%cchar*s=%c%s%c;int main(void){printf(s,13,10,34,s,34);}";int main(void){printf(s,13,10,34,s,34);}
