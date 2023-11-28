// TODO pointer
#define FILE_p int
#define char_p int
#define c_char_p int

#define stdin 0  // TODO should be `inline const FILE_p stdin;`
#define stdout 0 // TODO should be `inline const FILE_p stdout;`
#define stderr 0 // TODO should be `inline const FILE_p stdout;`

#define NULL 0
#define EOF -1

// TODO shoud be `int fgetc(FILE*);`
inline char fgetc(FILE_p stream);
// TODO shoud be `int getchar(void);`
inline char getc(void);
// TODO shoud be `char* fgets(char* buf, int size, FILE* stream);`
void fgets(FILE_p stream, char_p buf);
// TODO should be `char* gets(char* buf);`
void gets(char_p buf);

// TODO should be `int fputc(int c, FILE* stream);`
inline void fputc(FILE_p stream, char c);
// TODO should be `int putchar(int c);`
inline void putc(char c);
// TODO should be `int fputs(const char* buf, FILE* stream);`
void fputs(FILE_p stream, c_char_p buf);
// TODO should be `int puts(const char* buf);`
void puts(c_char_p buf);
