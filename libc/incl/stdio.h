#define stdin asm { !stdin }   // TODO should be `const inline FILE *stdin;`
#define stdout asm { !stdout } // TODO should be `const inline FILE *stdout;`

#define NULL 0
#define EOF -1

char fgetc(int stream); // TODO should be FILE*
char getc();
void fgets(int stream, int buf); // TODO should be FILE*, char*
void gets(int buf);              // TODO should be char*

void fputc(int stream, char c); // TODO should be FILE*
void putc(char c);
void fputs(int stream, int buf); // TODO should be FILE*, char*
void puts(int buf);              // TODO should be char*
