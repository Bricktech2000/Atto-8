#define size_t unsigned // TODO should be `typedef`
#define EXIT_SUCCESS 0
#define EXIT_FAILURE 1

#define NULL 0

void *malloc(size_t size);
void free(void *ptr);
void *realloc(void *ptr, size_t new_size);

inline void exit(int status);
inline void abort(void);

inline int abs(int n);
int rand(void);
void srand(unsigned seed);
