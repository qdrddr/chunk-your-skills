#include <stdio.h>

#include "examples/common.h"

int main(void) {
    char *out = NULL;
    int code = cyt_md_to_tree(NULL, "skill.md", "{}", &out);
    if (code == CYT_CYT_OK) {
        fprintf(stderr, "expected failure for NULL markdown_content\n");
        cyt_example_free(cyt_example_take(&out));
        return 1;
    }

    const char *err = cyt_get_last_error();
    if (err == NULL || err[0] == '\0') {
        fprintf(stderr, "expected thread-local error message\n");
        return 1;
    }

    cyt_clear_error();
    if (cyt_get_last_error() != NULL) {
        fprintf(stderr, "cyt_clear_error did not clear message\n");
        return 1;
    }

    code = cyt_md_to_tree("not markdown only", "skill.md", "not-json", &out);
    if (code == CYT_CYT_OK) {
        fprintf(stderr, "expected config parse failure\n");
        cyt_example_free(cyt_example_take(&out));
        return 1;
    }

    printf("error_handling: failure paths ok\n");
    return 0;
}
