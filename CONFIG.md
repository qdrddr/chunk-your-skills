# Configuration

**Chunk Your Skills** decomposes agent `SKILL.md` files into page-indexed nodes and recomposes
**skinny skills** from selected sections. Configuration is passed through the CLI flags, SDK
constructs, and (for Python) optional environment variables — there is no global `config.yaml`.

---

## How it works

```text
SKILL.md
    │
    ▼
Decompose  ──► metadata.json, nodes/page_index.json, nodes/n{id}.md
    │
    ▼
Recompose  ──► skinny SKILL.md (selected node IDs + frontmatter)
```

1. **Decompose** — parse YAML frontmatter, preamble, and `#` heading sections into a catalog.
2. **Select** — choose node IDs (manually, from `page_index.json`, or via an external ranker).
3. **Recompose** — rebuild markdown from matched nodes; parent headings are included automatically.

See [examples/README.md](examples/README.md) for a full walkthrough.

---

## CLI

Install: `cargo install chunk-your-skills` or build from source (`cargo build -p chunk-your-skills --release`).

### `decompose`

Write a catalog directory from one skill file.

| Flag | Required | Default | Description |
| --- | --- | --- | --- |
| `--skill` | yes | — | Path to `SKILL.md` |
| `--output` | no | `.catalog` | Catalog root directory |

```bash
chunk-your-skills decompose \
  --skill examples/context7/original/SKILL.md \
  --output examples/context7/decomposed/
```

Catalog layout:

```text
{catalog}/
├── metadata.json
└── nodes/
    ├── page_index.json   # section tree + node IDs
    ├── n0.md               # frontmatter
    ├── n1.md               # preamble
    └── n2.md …               # one file per heading section
```

### `recompose`

Build a skinny skill from node IDs. Provide exactly one source: `--catalog` or `--skill`.

| Flag | Required | Default | Description |
| --- | --- | --- | --- |
| `--catalog` | one of source | — | Decomposed catalog from `decompose` |
| `--skill` | one of source | — | Index in memory (no catalog on disk) |
| `--doc-id` | with `--catalog` | auto | Document id from `page_index.json` |
| `--path` | with `--catalog` | auto | Original skill path from `page_index.json` |
| `--node-id` | yes | — | Comma-separated IDs and ranges (`1-3,5,8`) |
| `--output` | with `--skill` | catalog default | Output `SKILL.md` path |
| `--keep-all-headers` | no | `false` | Emit every heading; omit body for non-matched sections |

```bash
# From catalog
chunk-your-skills recompose \
  --catalog examples/context7/decomposed \
  --path examples/context7/original/SKILL.md \
  --node-id 4-7 \
  --output examples/context7/skinny-skill/fetch-workflow/SKILL.md

# In memory (requires --output)
chunk-your-skills recompose \
  --skill examples/context7/original/SKILL.md \
  --node-id 1,2 \
  --output /tmp/activation/SKILL.md
```

When `--output` is omitted with `--catalog`, the CLI writes under
`{catalog}/skills/retrieve/{doc_id}/SKILL.md`.

---

## Page index options

`PageIndexConfig` controls how the page index JSON is built. Defaults apply everywhere unless you
pass overrides through an SDK.

| Field | Default | Description |
| --- | --- | --- |
| `if_add_node_id` | `true` | Include `node_id` on each structure node |
| `if_add_node_text` | `false` | Embed full section text in the structure tree |

### Rust

```rust
use chunk_your_skills::{PageIndexConfig, build_skills_index};

let config = PageIndexConfig {
    if_add_node_id: true,
    if_add_node_text: false,
};
let index = build_skills_index(&["/path/to/skills"], &config)?;
```

### Python

```python
from chunk_your_skills import PageIndexConfig, build_skills_index

config = PageIndexConfig(if_add_node_id=True, if_add_node_text=False)
index = build_skills_index(["/path/to/skills"], config)
```

JSON/dict form (also accepted by FFI and cache APIs):

```json
{
  "if_add_node_id": true,
  "if_add_node_text": false
}
```

### TypeScript

```typescript
import { buildSkillsIndex, defaultPageIndexConfig } from "chunk-your-skills";

const config = { ...defaultPageIndexConfig(), ifAddNodeText: true };
const index = buildSkillsIndex(["/path/to/skills"], config);
```

The CLI always uses `PageIndexConfig::default()`.

---

## Recompose options

`ReconstructOptions` applies to `reconstruct_skill_markdown`, `write_reconstructed_skill`, and the
CLI `--keep-all-headers` flag.

| Field | Default | Description |
| --- | --- | --- |
| `keep_all_headers` | `false` | When `true`, every document heading appears in output; non-matched sections keep the heading line only |

Frontmatter (node `0`) is always included. Selecting a child node automatically includes ancestor
headings.

---

## Path defaults

SDK hosts can override catalog path conventions via `PathConfig` (Rust `configure_paths`):

| Field | Default |
| --- | --- |
| `md_ext` | `.md` |
| `skills_decomposed_prefix` | `skills/decomposed/` |
| `skills_decomposed_root` | `skills/decomposed` |
| `default_catalog_dir` | `.catalog` |

---

## Memory cache (Python / registry)

When building or refreshing skill registries at scale, tune in-memory caching.

### Environment variables

| Variable | Default | Description |
| --- | --- | --- |
| `CYT_CACHE_LAZY_REGISTRY` | `false` | Defer registry loading until first use |
| `CYT_CACHE_ASYNC_DISK` | `true` | Write catalog files asynchronously |
| `CYT_CACHE_LRU_CHUNK_BODIES` | `512` | LRU capacity for chunk bodies |
| `CYT_CACHE_LRU_MERGED_DOCUMENTS` | `128` | LRU capacity for merged documents |
| `CYT_CACHE_LRU_SKILLS_INDEX` | `64` | LRU capacity for skills indexes |
| `CYT_CACHE_LRU_TANTIVY_INDEXES` | `32` | Reserved (legacy field) |
| `CYT_CACHE_LRU_TOOL_CATALOGS` | `128` | Reserved (legacy field) |

The `CYT_` prefix is historical; these variables apply to chunk-your-skills cache behavior only.

### Python API

```python
from chunk_your_skills import configure_memory_cache, ensure_skills_registry

configure_memory_cache({
    "lazy_registry": False,
    "async_disk_writes": True,
    "lru": {"skills_index": 32, "chunk_bodies": 256},
})

refs = ensure_skills_registry(
    ["/path/to/skills"],
    catalog_root="/path/to/.catalog",
    pageindex_config={"if_add_node_id": True},
    policy="auto",  # "auto" | "force_memory" | "force_disk"
)
```

`ensure_skills_registry` accepts filesystem paths or in-memory dicts with `path`, `content`, and
optional `content_sha256`.

---

## Node ID reference

Reserved IDs (see [examples/README.md](examples/README.md)):

| ID | Section |
| --- | --- |
| 0 | YAML frontmatter |
| 1 | Preamble (body before first heading) |
| 2+ | Heading sections (`#` … `######`) |

Node IDs match filenames (`n4.md` → `4`). IDs are stable for a given decomposed catalog; re-run
`decompose` after editing the source skill to refresh the catalog.

---

## Development

See [DEV.md](DEV.md) for local workflow, version sync, and publish notes.

## License

Apache-2.0 — see [LICENSE](LICENSE).
