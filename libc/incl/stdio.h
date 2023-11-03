// TODO pointer
#define FILE_p int
#define char_p int

#define stdin 0  // TODO should be `inline const FILE_p stdin;`
#define stdout 0 // TODO should be `inline const FILE_p stdout;`

#define NULL 0
#define EOF -1

char fgetc(FILE_p stream);
char getc();
void fgets(FILE_p stream, char_p buf); // TODO should take in size
void gets(char_p buf);

void fputc(FILE_p stream, char c);
void putc(char c);
void fputs(FILE_p stream, char_p buf);
void puts(char_p buf);
