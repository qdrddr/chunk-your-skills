# CLI examples

Decompose a `SKILL.md` into page-indexed nodes and recompose **skinny skills** from selected
sections. Scripts: [`decompose.sh`](decompose.sh), [`recompose.sh`](recompose.sh). Sample skill:
[`context7/original/SKILL.md`](context7/original/SKILL.md).

## Prerequisites

Build the CLI from the repo root:

```bash
cargo build -p chunk-your-skills --release
```

`decompose.sh` calls `./target/release/chunk-your-skills` directly. `recompose.sh` expects
`chunk-your-skills` on your `PATH` — add the release dir for local runs:

```bash
export PATH="$PWD/target/release:$PATH"
```

Or install globally: `cargo install --path . --bin chunk-your-skills`.

## Decompose

```bash
./examples/decompose.sh
```

This parses `examples/context7/original/SKILL.md` and writes a catalog under
`examples/context7/decomposed/`:

```text
decomposed/
├── metadata.json
└── nodes/
    ├── page_index.json   # section tree + node IDs
    ├── n0.md               # frontmatter
    ├── n1.md               # preamble
    └── …                   # one file per heading section
```

Browse `nodes/page_index.json` or individual `n{id}.md` files to see what each node contains
before recomposing.

## Recompose

Run decompose first, then:

```bash
./examples/recompose.sh
```

The script builds several skinny skills under `examples/context7/skinny-skill/` to show common
patterns:

| Output | Node IDs | Notes |
| --- | --- | --- |
| `fetch-workflow/SKILL.md` | `4-7` | Full documentation-fetch workflow (catalog mode) |
| `steps-1-and-3/SKILL.md` | `4,6` | Cherry-pick steps; parent heading kept automatically |
| `activation/SKILL.md` | `1,2` | Preamble + when-to-use triggers |
| `mixed-nodes/SKILL.md` | `1-3,8` | In-memory recompose (no catalog) |
| `guidelines-only/SKILL.md` | `8` | Single section (frontmatter always included) |
| `step1-skeleton/SKILL.md` | `4` + `--keep-all-headers` | Matched section full; other headings as stubs |

The last example omits `--output`; the CLI writes to the default path under the catalog
(`$CATALOG/skills/retrieve/context7/SKILL.md`).

### Node map

From [`context7/decomposed/nodes/page_index.json`](context7/decomposed/nodes/page_index.json):

| ID | Section |
| --- | --- |
| 0 | Frontmatter |
| 1 | Preamble |
| 2 | When to Use This Skill |
| 3 | How to Fetch Documentation (parent) |
| 4 | Step 1: Resolve the Library ID |
| 5 | Step 2: Select the Best Match |
| 6 | Step 3: Fetch the Documentation |
| 7 | Step 4: Use the Documentation |
| 8 | Guidelines |

Node IDs match filenames (`n4.md` → `4`). Selecting a child node includes its parent headings
automatically.

### Manual CLI

Equivalent one-liners (from repo root):

```bash
# Decompose
./target/release/chunk-your-skills decompose \
  --skill examples/context7/original/SKILL.md \
  --output examples/context7/decomposed/

# Recompose from catalog
chunk-your-skills recompose \
  --catalog examples/context7/decomposed \
  --path examples/context7/original/SKILL.md \
  --node-id 1,2 \
  --output examples/context7/skinny-skill/activation/SKILL.md

# Recompose in memory (no catalog)
chunk-your-skills recompose \
  --skill examples/context7/original/SKILL.md \
  --node-id 4-7 \
  --output examples/context7/skinny-skill/fetch-workflow/SKILL.md
```

`--node-id` accepts comma-separated IDs and ranges (`1-3,5,8`).

## See also

- [go-git-smoke/README.md](go-git-smoke/README.md) — Go SDK smoke test (git tag + GitHub Release FFI;
unrelated to the CLI scripts above)
