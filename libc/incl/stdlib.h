#define size_t int // TODO should be `typeedef`
#define EXIT_SUCCESS 0
#define EXIT_FAILURE 1

#define NULL 0

int malloc(int size); // TODO should be void*
void free(int ptr);   // TODO should be void*

void exit(int status); // TODO inline
void abort(void);      // TODO inline

// int abs(int n); // TODO inline
// TODO inline int rand(void); inline void srand(unsigned int seed);
