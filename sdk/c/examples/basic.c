#include <stdio.h>

#include "examples/common.h"

int main(void) {
    char *out = NULL;
    const char *skill_dirs = "[]";
    const char *config = "{}";

    if (!chunk_your_skills_example_ok(
            chunk_your_skills_build_skills_index(skill_dirs, config, &out),
            "chunk_your_skills_build_skills_index")) {
        return 1;
    }

    char *json = chunk_your_skills_example_take(&out);
    if (json == NULL || strstr(json, "\"files\"") == NULL) {
        fprintf(stderr, "unexpected skills index JSON\n");
        chunk_your_skills_example_free(json);
        return 1;
    }
    chunk_your_skills_example_free(json);

    printf("basic: build_skills_index ok\n");
    return 0;
}
