#include <stdio.h>

#include "examples/common.h"

int main(void) {
    const char *md = "# Title\n\nBody\n\n## Sub\n\nMore";
    char *tree_out = NULL;
    if (!cyt_example_ok(cyt_md_to_tree(md, "skill.md", "{}", &tree_out),
                        "cyt_md_to_tree")) {
        return 1;
    }
    char *tree = cyt_example_take(&tree_out);
    if (tree == NULL || strstr(tree, "Title") == NULL) {
        fprintf(stderr, "md_to_tree missing content\n");
        cyt_example_free(tree);
        return 1;
    }
    cyt_example_free(tree);

    char *node_ids_out = NULL;
    if (!cyt_example_ok(cyt_parse_skill_node_ids("8-10", &node_ids_out),
                        "cyt_parse_skill_node_ids")) {
        return 1;
    }
    char *node_ids = cyt_example_take(&node_ids_out);
    if (node_ids == NULL || strstr(node_ids, "8") == NULL) {
        fprintf(stderr, "unexpected node id list\n");
        cyt_example_free(node_ids);
        return 1;
    }
    cyt_example_free(node_ids);

    printf("skills: pageindex ok\n");
    return 0;
}
