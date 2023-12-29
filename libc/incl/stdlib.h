#define size_t int // TODO should be `typedef`
#define EXIT_SUCCESS 0
#define EXIT_FAILURE 1

#define NULL 0

void *malloc(size_t size);
void free(void *ptr);

inline void exit(int status);
inline void abort(void);

inline int abs(int n);
inline int rand(int seed); // TODO should take in `void`
