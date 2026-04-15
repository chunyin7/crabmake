#include "check.h"
#include "math.h"
#include "util/strutil.h"
#include <stdio.h>
#include <string.h>

static void test_math(void) {
    CHECK_EQ(add(10, 3), 13);
    CHECK_EQ(subtract(10, 3), 7);
    CHECK_EQ(multiply(10, 3), 30);
    CHECK_EQ(factorial(0), 1);
    CHECK_EQ(factorial(1), 1);
    CHECK_EQ(factorial(5), 120);
    CHECK_EQ(factorial(7), 5040);
}

static void test_strutil(void) {
    CHECK_EQ(str_len(""), 0);
    CHECK_EQ(str_len("crabmake"), 8);

    CHECK(str_eq("foo", "foo"));
    CHECK(!str_eq("foo", "bar"));
    CHECK(!str_eq("foo", "foobar"));
    CHECK(str_eq("", ""));

    char buf[16];
    strcpy(buf, "crabmake");
    str_reverse(buf);
    CHECK(str_eq(buf, "ekambarc"));
}

int main(void) {
    test_math();
    test_strutil();
    printf("all tests passed\n");
    return 0;
}
