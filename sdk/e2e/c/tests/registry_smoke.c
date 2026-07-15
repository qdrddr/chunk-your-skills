#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "chunk_your_skills.h"

static int expect_ok(int code, const char *fn) {
    if (code == CYT_OK) {
        return 1;
    }
    const char *err = cyt_get_last_error();
    fprintf(stderr, "%s failed (%d): %s\n", fn, code, err ? err : "(no message)");
    return 0;
}

int main(void) {
    char *out = NULL;

    if (!expect_ok(cyt_get_version(&out), "cyt_get_version")) {
        return 1;
    }
    if (out == NULL || strlen(out) == 0) {
        fprintf(stderr, "expected non-empty version string\n");
        cyt_free_string(out);
        return 1;
    }
    cyt_free_string(out);

    long count = cyt_count_tokens("hello world");
    if (count < 1) {
        fprintf(stderr, "expected count_tokens >= 1, got %ld\n", count);
        return 1;
    }

    return 0;
}
