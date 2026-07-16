#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "chunk_your_skills.h"

static int expect_ok(int code, const char *fn) {
    if (code == CHUNK_YOUR_SKILLS_OK) {
        return 1;
    }
    const char *err = chunk_your_skills_get_last_error();
    fprintf(stderr, "%s failed (%d): %s\n", fn, code, err ? err : "(no message)");
    return 0;
}

int main(void) {
    char *out = NULL;

    if (!expect_ok(chunk_your_skills_get_version(&out), "chunk_your_skills_get_version")) {
        return 1;
    }
    if (out == NULL || strlen(out) == 0) {
        fprintf(stderr, "expected non-empty version string\n");
        chunk_your_skills_free_string(out);
        return 1;
    }
    chunk_your_skills_free_string(out);

    return 0;
}
