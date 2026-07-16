#include <stdio.h>

#include "examples/common.h"

int main(void) {
    const char *md = "# Title\n\nBody\n\n## Sub\n\nMore";
    char *tree_out = NULL;
    if (!chunk_your_skills_example_ok(
            chunk_your_skills_md_to_tree(md, "skill.md", "{}", &tree_out),
            "chunk_your_skills_md_to_tree")) {
        return 1;
    }
    char *tree = chunk_your_skills_example_take(&tree_out);
    if (tree == NULL || strstr(tree, "Title") == NULL) {
        fprintf(stderr, "md_to_tree missing content\n");
        chunk_your_skills_example_free(tree);
        return 1;
    }
    chunk_your_skills_example_free(tree);

    char *node_ids_out = NULL;
    if (!chunk_your_skills_example_ok(
            chunk_your_skills_parse_skill_node_ids("8-10", &node_ids_out),
            "chunk_your_skills_parse_skill_node_ids")) {
        return 1;
    }
    char *node_ids = chunk_your_skills_example_take(&node_ids_out);
    if (node_ids == NULL || strstr(node_ids, "8") == NULL) {
        fprintf(stderr, "unexpected node id list\n");
        chunk_your_skills_example_free(node_ids);
        return 1;
    }
    chunk_your_skills_example_free(node_ids);

    printf("skills: pageindex ok\n");
    return 0;
}
