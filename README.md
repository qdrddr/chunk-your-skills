# Chunk Your Skills

Rust-first library for decomposing agent `SKILL.md` files into page-indexed nodes and recomposing
**skinny skills** — only the sections you select.

This repository is the **chunk-your-skills** SDK monorepo. The former **clear-your-tools** proxy
application layer has been removed; use [clear-your-tools](https://github.com/qdrddr/clear-your-tools)
for the MCP tool-gating proxy.

## What it does

1. **Decompose** — parse `SKILL.md` into a tree of nodes (frontmatter, preamble, heading sections).
2. **Cache** — write `metadata.json`, `nodes/page_index.json`, and `nodes/n{id}.md` under a catalog directory.
3. **Recompose** — rebuild a skinny `SKILL.md` from selected node IDs.

## Packages

| Language | Package | Import |
| -------- | ------- | ------ |
| Rust | [`chunk-your-skills`](https://crates.io/crates/chunk-your-skills) | `chunk_your_skills` |
| Python | [`chunk-your-skills`](https://pypi.org/project/chunk-your-skills/) | `chunk_your_skills` |
| TypeScript | [`chunk-your-skills`](https://www.npmjs.com/package/chunk-your-skills) | `chunk-your-skills` |
| C | `libchunk_your_skills` | `chunk_your_skills.h` |
| Go | [`sdk/go`](https://pkg.go.dev/github.com/qdrddr/chunk-your-skills/sdk/go) | `chunkyourskills` |

## Quick start (CLI)

```bash
cargo build -p chunk-your-skills --release

# Decompose one skill file into .catalog/
./target/release/chunk-your-skills decompose --skill path/to/SKILL.md --output .catalog

# Recompose skinny skill from node IDs (catalog from decompose)
./target/release/chunk-your-skills recompose \
  --catalog .catalog \
  --path path/to/SKILL.md \
  --node-id 1-3,5,8 \
  --output skinny/SKILL.md

# Recompose in memory directly from a skill file (no catalog)
./target/release/chunk-your-skills recompose \
  --skill path/to/SKILL.md \
  --node-id 1,2 \
  --output skinny/SKILL.md
```

## Quick start (Python)

```bash
cd sdk/python
uv sync
uv run maturin develop --release
uv run python -c "from chunk_your_skills import build_skills_index; print(build_skills_index)"
```

## Development

```bash
./scripts/local-dev.sh all          # Rust + all SDKs
./scripts/local-dev.sh sdk-python   # Python only
./scripts/sync-version.sh           # propagate semver from root Cargo.toml
```

## Layout

```text
Cargo.toml, src/              # Rust core + CLI + FFI (root crate)
sdk/python/                   # PyO3 bindings (chunk_your_skills)
sdk/typescript/               # napi-rs bindings
sdk/c/                        # C FFI + CMake
sdk/go/                       # cgo bindings (chunkyourskills)
```

## Migration

See [MIGRATION.md](MIGRATION.md) for adopting chunk-your-skills from clear-your-tools / cyt-indexer-sdk.

## License

Apache-2.0 — see [LICENSE](LICENSE).
