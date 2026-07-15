#include <stdio.h>

#include "examples/common.h"

int main(void) {
    char *out = NULL;
    const char *skill_dirs = "[]";
    const char *config = "{}";

    if (!cyt_example_ok(cyt_build_skills_index(skill_dirs, config, &out),
                        "cyt_build_skills_index")) {
        return 1;
    }

    char *json = cyt_example_take(&out);
    if (json == NULL || strstr(json, "\"files\"") == NULL) {
        fprintf(stderr, "unexpected skills index JSON\n");
        cyt_example_free(json);
        return 1;
    }
    cyt_example_free(json);

    printf("basic: build_skills_index ok\n");
    return 0;
}
