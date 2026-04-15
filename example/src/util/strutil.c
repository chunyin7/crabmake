#include "util/strutil.h"

size_t str_len(const char *s) {
    size_t n = 0;
    while (s[n] != '\0') n++;
    return n;
}

int str_eq(const char *a, const char *b) {
    while (*a && *b) {
        if (*a != *b) return 0;
        a++;
        b++;
    }
    return *a == *b;
}

void str_reverse(char *s) {
    size_t n = str_len(s);
    for (size_t i = 0; i < n / 2; i++) {
        char tmp = s[i];
        s[i] = s[n - 1 - i];
        s[n - 1 - i] = tmp;
    }
}
