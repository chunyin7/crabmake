#ifndef STRUTIL_H
#define STRUTIL_H

#include <stddef.h>

size_t str_len(const char *s);
int str_eq(const char *a, const char *b);
void str_reverse(char *s);

#endif
