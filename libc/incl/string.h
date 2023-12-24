// TODO pointer
#define char_p int
#define c_char_p int
#define void_p int
#define c_void_p int

#define ptrdiff_t int // TODO should be `typedef`
#define size_t int    // TODO should be `typedef`

#define NULL 0

void strcat(char_p dst, c_char_p src); // TODO should return char*
char_p strchr(c_char_p str, int chr);
size_t strlen(c_char_p str);
void strcpy(char_p dst, c_char_p src); // TODO should return char*
int strcmp(c_char_p str1, c_char_p str2);
char_p strend(c_char_p str); // TODO nonstandard

void_p memchr(void_p ptr, int chr, size_t len);
void memset(void_p ptr, int chr, size_t len);      // TODO should return void*
void memcpy(void_p dst, c_void_p src, size_t len); // TODO should return void*
int memcmp(c_void_p ptr1, c_void_p ptr2, size_t len);
void memswp(void_p ptr1, void_p ptr2, size_t len); // TODO nonstandard
void memxor(void_p dst, c_void_p src, size_t len); // TODO nonstandard
void memmove(void_p dst, c_void_p src, size_t len);
