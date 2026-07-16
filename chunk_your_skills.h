/**
 * @file chunk_your_skills.h
 * @brief chunk-your-skills C FFI interface
 *
 * SKILL.md pageindex decomposition and skinny-skill recomposition.
 *
 * # Memory Management
 *
 * - Strings returned via `char**` out parameters MUST be freed with `cyt_free_string()`.
 * - Opaque handles (`CytSkillsBuilder`)
 *   MUST be freed with their matching `cyt_*_free()` function.
 * - Input C strings remain owned by the caller.
 *
 * # Thread Safety
 *
 * Error messages are stored in thread-local storage. Call `cyt_get_last_error()`
 * from the same thread that received a non-zero error code.
 *
 * # Return Conventions
 *
 * - `CYT_OK` (0) on success for status functions.
 * - Negative error codes on failure; see `cyt_get_last_error()`.
 * - JSON outputs: int return code + `char**` out param.
 * - Boolean queries: 1 true, 0 false, negative on error (or `int*` out with `CYT_OK`).
 */


#ifndef CHUNK_YOUR_SKILLS_H
#define CHUNK_YOUR_SKILLS_H

#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
namespace cyt {
#endif  // __cplusplus

/*
 Reserved node id for YAML frontmatter (`0.md`).
 */
#define CYT_NODE_ID_FRONTMATTER 0

/*
 Reserved node id for preamble body text (`1.md`).
 */
#define CYT_NODE_ID_PREAMBLE 1

/*
 First node id assigned to markdown heading sections.
 */
#define CYT_CONTENT_NODE_ID_START 2

/*
 Success return code.
 */
#define CYT_CYT_OK 0

/*
 Null pointer argument error.
 */
#define CYT_CYT_ERR_NULL_PTR -1

/*
 Invalid UTF-8 encoding error.
 */
#define CYT_CYT_ERR_INVALID_UTF8 -2

/*
 JSON parse or serialization error.
 */
#define CYT_CYT_ERR_JSON -3

/*
 Memory allocation error.
 */
#define CYT_CYT_ERR_ALLOC -4

/*
 I/O or filesystem error.
 */
#define CYT_CYT_ERR_IO -5

/*
 Invalid opaque handle.
 */
#define CYT_CYT_ERR_INVALID_HANDLE -6

/*
 Internal panic (caught at FFI boundary).
 */
#define CYT_CYT_ERR_PANIC -7

/*
 Invalid argument / value error.
 */
#define CYT_CYT_ERR_INVALID_ARG -8

/*
 Opaque skills builder handle.
 */
typedef struct CYT_CytSkillsBuilder CYT_CytSkillsBuilder;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

int cyt_path_md_ext(char **out);

int cyt_path_skills_decomposed_prefix(char **out);

int cyt_path_skills_decomposed_root(char **out);

int cyt_path_default_catalog_dir(char **out);

/*
 Ensure page index for skill sources.

 # Safety

 All string pointers must be valid null-terminated UTF-8 C strings; `out` must be non-null.
 */
int cyt_ensure_skills_registry(const char *source_paths_json,
                               const char *catalog_root,
                               const char *pageindex_config_json,
                               const char *policy,
                               char **out);

/*
 Apply in-memory cache tuning from a JSON object (`cache.memory` block).

 # Safety

 `config_json` must be a valid null-terminated UTF-8 C string when non-null.
 */
int cyt_configure_memory_cache(const char *config_json);

/*
 Get the last error message for the current thread.

 Returns NULL if no error occurred. Valid until the next `cyt_*` call on this thread.

 # Safety

 No pointer arguments; safe to call from C when linked against this library.
 */
const char *cyt_get_last_error(void);

/*
 Clear the last error for the current thread.

 # Safety

 No pointer arguments; safe to call from C when linked against this library.
 */
void cyt_clear_error(void);

/*
 Free a string allocated by `cyt_*` functions. NULL is safe.

 # Safety

 `s` must be null or a pointer previously returned by a `cyt_*` out-parameter.
 */
void cyt_free_string(char *s);

/*
 Return the library version string (caller must free with `cyt_free_string`).

 # Safety

 `out` must be a valid pointer to a `char*` that receives an allocated string.
 */
int cyt_get_version(char **out);

int cyt_build_skills_index(const char *skill_dirs_json, const char *config_json, char **out);

int cyt_write_skills_index(const char *index_json, const char *output_dir);

int cyt_load_skills_index_from_dir(const char *catalog_dir, char **out);

int cyt_repair_skill_nodes(const char *entry_dir, const char *doc_id, const char *config_json);

int cyt_skills_index_from_decomposed_dir(const char *dir, char **out);

int cyt_md_to_tree(const char *markdown_content,
                   const char *source_path,
                   const char *config_json,
                   char **out);

int cyt_get_skill_document(const char *documents_json, const char *doc_id, char **out);

int cyt_get_skill_structure(const char *documents_json, const char *doc_id, char **out);

int cyt_get_skill_line_content_from_spec(const char *index_or_docs_json,
                                         const char *doc_id,
                                         const char *line_num_spec,
                                         char **out);

int cyt_get_skill_content_retrieve_result(const char *index_or_docs_json,
                                          const char *doc_id,
                                          const char *line_num_specs_json,
                                          const char *node_id_specs_json,
                                          const char *options_json,
                                          char **out);

int cyt_reconstruct_skill_markdown(const char *index_or_docs_json,
                                   const char *doc_id,
                                   const char *line_num_specs_json,
                                   const char *node_id_specs_json,
                                   const char *options_json,
                                   char **out);

int cyt_write_reconstructed_skill(const char *catalog_dir,
                                  const char *index_or_docs_json,
                                  const char *doc_id,
                                  const char *line_num_specs_json,
                                  const char *node_id_specs_json,
                                  const char *options_json,
                                  char **out);

int cyt_get_skill_line_content(const char *index_or_docs_json,
                               const char *doc_id,
                               const char *line_num_specs_json,
                               const char *node_id_specs_json,
                               char **out);

int cyt_parse_skill_node_ids(const char *spec, char **out);

/*
 Parse ``token_count`` from decomposed markdown/JSON frontmatter when present.

 # Safety

 `content` must be a valid null-terminated UTF-8 C string, or null (returns error).
 `out` must be a valid mutable pointer to receive the token count, or null (returns error).
 */
int cyt_token_count_from_decomposed_frontmatter(const char *content, long *out);

/*
 Parse root-level YAML frontmatter keys into semantic JSON values.

 # Safety

 `content` must be a valid null-terminated UTF-8 C string, or null (returns error).
 `out` must be a valid mutable pointer to receive the JSON output string, or null (returns error).
 */
int cyt_parse_frontmatter_fields(const char *content, char **out);

/*
 Look up one semantically parsed frontmatter field by name.

 # Safety

 `content` and `key` must be valid null-terminated UTF-8 C strings, or null (returns error).
 `out` must be a valid mutable pointer to receive the JSON output string, or null (returns error).
 */
int cyt_frontmatter_field(const char *content, const char *key, char **out);

int cyt_skills_builder_new(int memory_only,
                           const char *output_dir,
                           struct CYT_CytSkillsBuilder **out);

void cyt_skills_builder_free(struct CYT_CytSkillsBuilder *builder);

int cyt_skills_builder_build_from_dirs(struct CYT_CytSkillsBuilder *builder,
                                       const char *skill_dirs_json,
                                       const char *config_json,
                                       char **out);

int cyt_skills_builder_write_catalog(struct CYT_CytSkillsBuilder *builder, char **out);

int cyt_skills_builder_to_skills_index_json(const struct CYT_CytSkillsBuilder *builder, char **out);

int cyt_skills_builder_to_skills_dict(const struct CYT_CytSkillsBuilder *builder, char **out);

int cyt_reconstruct_options_default(char **out);

int cyt_build_page_index_only(const char *skill_dirs_json, const char *config_json, char **out);

int cyt_page_index_valid(const char *entry_dir, const char *content_sha256, int *out);

int cyt_load_skills_index_from_entry(const char *entry_dir, const char *doc_id, char **out);

int cyt_load_merged_skill_document_json(const char *entry_dir, const char *doc_id, char **out);

int cyt_finalize_skill_document_json(const char *entry_dir,
                                     const char *doc_id,
                                     const char *content_sha256,
                                     const char *pipeline,
                                     const char *index_params_json,
                                     const char *built_at,
                                     const char *source_path,
                                     char **out);

int cyt_update_skill_document_source_path(const char *entry_dir,
                                          const char *doc_id,
                                          const char *source_path,
                                          char **out);

int cyt_configure_path_constants(const char *md_ext,
                                 const char *skills_decomposed_prefix,
                                 const char *skills_decomposed_root,
                                 const char *default_catalog_dir);

int cyt_to_skills_decomposed_key(const char *file_path, char **out);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#ifdef __cplusplus
}  // namespace cyt
#endif  // __cplusplus

#endif  /* CHUNK_YOUR_SKILLS_H */
