# Limitations

Chunk Your Skills is a **structural** library: it splits `SKILL.md` on headings and rebuilds
markdown from explicit node IDs.

---

## Input format

### SKILL.md and headings

- Decomposition targets agent **`SKILL.md`** files with standard ATX headings (`#` through `######`).
- A heading must be followed by a space (`# Title`). Setext-style headings are not recognized.
- Headings **inside fenced code blocks** are ignored.
- Content between YAML frontmatter and the first heading becomes the **preamble** node (`1`).
- Without `---` frontmatter fences, the entire pre-heading body is treated as preamble.

### Frontmatter

- Frontmatter uses opening and closing `---` lines. An unclosed frontmatter block yields an empty
  prefix (no frontmatter node).
- YAML frontmatter is **always included** in recomposed output, even when it is not listed in
  `--node-id`.

### Section selection

- You must choose which nodes to include (`--node-id`, SDK `node_id_specs`, or line-number specs).
  There is no built-in BM25, reranker, or LLM stage in this repository.
- Selecting a child node pulls in **parent headings** automatically; sibling sections are omitted
  unless explicitly selected.
- `--keep-all-headers` preserves every heading in the document but drops body text for non-matched
  sections — useful for skeleton skills, not for hiding section titles entirely.

---

## Catalog and stability

- Node IDs are assigned in document order when you decompose. Editing the source skill (adding,
  removing, or reordering headings) changes IDs — **re-run `decompose`** before recomposing from a
  stale catalog.
- The CLI writes catalogs under `--output` (default `.catalog`). There is no automatic watch or
  invalidation.
- In-memory recompose (`--skill`) skips disk catalogs but still requires `--output`.

---

## CLI surface

- The CLI exposes `decompose` and `recompose` only. It always uses default `PageIndexConfig`
  (`if_add_node_id: true`, `if_add_node_text: false`).
- Tune page-index or cache behavior through the **Rust, Python, TypeScript, Go, or C SDKs** — not
  through CLI flags or a global config file.

---

## SDK and platform notes

| Topic | Limitation |
| --- | --- |
| **Token counting** | Removed in v1.0.8. Decomposed node files still carry an empty `token_count:` frontmatter field for compatibility. |
| **Go / C** | Require a built FFI library (`cgo` / CMake). See [sdk/go/README.md](sdk/go/README.md) and [sdk/c/README.md](sdk/c/README.md). |
| **Cache env vars** | `CHUNK_YOUR_SKILLS_CACHE_*` control memory-cache tuning in Python. |
| **Legacy cache fields** | `lru_tantivy_indexes` and `lru_tool_catalogs` remain in the memory-cache schema from an earlier monorepo layout; they are not used by the skills pageindex path. |

---

## Operational trade-offs

**Skinny skills reduce context size** when you include only the sections an agent needs for the
current task. Omitting sections saves input tokens but can hide instructions the agent would
otherwise follow — validate recomposed skills against your task before deploying them.

**Manual node selection** is predictable and auditable but does not adapt turn-by-turn.

---

## Related projects

- [examples/README.md](examples/README.md) — CLI walkthrough with the bundled context7 skill.
