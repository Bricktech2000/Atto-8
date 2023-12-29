#define ptrdiff_t int // TODO should be `typedef`
#define size_t int    // TODO should be `typedef`

#define NULL 0

void strcat(char *dst, const char *src); // TODO should return `char*`
char *strchr(const char *str, int chr);
size_t strlen(const char *str);
void strcpy(char *dst, const char *src); // TODO should return `char*`
int strcmp(const char *str1, const char *str2);
char *strend(const char *str); // TODO nonstandard

void *memchr(void *ptr, int chr, size_t len);
void memset(void *ptr, int chr, size_t len); // TODO should return `void*`
void memcpy(void *dst, const void *src,
            size_t len); // TODO should return `void*`
int memcmp(const void *ptr1, const void *ptr2, size_t len);
void memswp(void *ptr1, void *ptr2, size_t len);     // TODO nonstandard
void memxor(void *dst, const void *src, size_t len); // TODO nonstandard
void memmove(void *dst, const void *src, size_t len);
