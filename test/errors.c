/* #error test error in file __FILE__ on line __LINE__ */

/* #foobar */

/* int main(void) { return 0x; } */

/* int main(void) { return 0b1234; } */

/* int main( { */
/*   return 0; } */

/* int main(void) { return 5 + ; } */

/* 123 */

/* int main(void) { 1 ? 2; } */

/* int main(void) { */
/*   if (1 */
/*     return 0; */
/* } */

/* void main(void) */

/* void main(void) { */

/* asm("parentheses") */

/* asm { */

/* #include no_quotes */

/* #include "file" trailing */

/* char *main(void) { */
/*   "unclosed; } */

/* char main(void) { return 'abc'; } */

/* char *main(void) { return "\X1F"; } */

/* char *main(void) { return "\xyz"; } */
