// TODO pointer
#define void_p int

#define size_t int // TODO should be `typeedef`
#define EXIT_SUCCESS 0
#define EXIT_FAILURE 1

#define NULL 0

void_p malloc(size_t size);
void free(void_p ptr);

inline void exit(int status);
inline void abort(void);

inline int abs(int n);
// TODO inline int rand(void);
// TODO inline void srand(unsigned int seed);
