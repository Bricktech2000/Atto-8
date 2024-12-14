// constructive quine. assumes '\n' and '"' to occupy codepoints 10 and 34,
// respectively

// clang-format off

#include<stdio.h>
char*s="#include<stdio.h>%cchar*s=%c%s%c;int main(void){printf(s,10,34,s,34);}";int main(void){printf(s,10,34,s,34);}
