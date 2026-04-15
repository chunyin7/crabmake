#ifndef CHECK_H
#define CHECK_H

#include <stdio.h>
#include <stdlib.h>

#define CHECK(cond)                                                           \
    do {                                                                      \
        if (!(cond)) {                                                        \
            fprintf(stderr, "FAIL %s:%d: %s\n", __FILE__, __LINE__, #cond);   \
            exit(1);                                                          \
        }                                                                     \
    } while (0)

#define CHECK_EQ(a, b)                                                        \
    do {                                                                      \
        long _a = (long)(a);                                                  \
        long _b = (long)(b);                                                  \
        if (_a != _b) {                                                       \
            fprintf(stderr, "FAIL %s:%d: %s (%ld) != %s (%ld)\n",             \
                    __FILE__, __LINE__, #a, _a, #b, _b);                      \
            exit(1);                                                          \
        }                                                                     \
    } while (0)

#endif
