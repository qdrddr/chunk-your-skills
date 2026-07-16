# Migration from clear-your-tools / cyt-indexer-sdk

This repository was split from [clear-your-tools](https://github.com/qdrddr/clear-your-tools).
The proxy application (`cyt` CLI, `src/cyt/`, hooks, pruners) remains in clear-your-tools.
**chunk-your-skills** is the skills-only SDK and CLI.

## Package renames

| Before | After |
| ------ | ----- |
| `cyt-indexer` (crate) | `chunk-your-skills` |
| `cyt-indexer-sdk` (PyPI/npm) | `chunk-your-skills` |
| `cyt_indexer` (Python module) | `chunk_your_skills` |
| `cytindexer` (Go package) | `chunkyourskills` |
| `libcyt_indexer` / `cyt_indexer.h` | `libchunk_your_skills` / `chunk_your_skills.h` |
| `clear-your-tools` (this repo) | removed — use clear-your-tools repo for the proxy |

## What moved vs removed

**Moved here (skills-only):**

- Pageindex: `build_skills_index`, `SkillsBuilder`, `md_to_tree`, reconstruct/retrieve skill content
- BM25 cohesion chunking for skill nodes
- Skills registry cache (`ensure_skills_registry`)

**Removed from this repo (stay in clear-your-tools):**

- MCP tool catalog build/retrieve/prune
- Proxy (`cyt proxy`, `cyt launch`)
- Tool policies, rerank/LLM pruners
- Agent hooks and stats

## Python

```bash
# Before
pip install cyt-indexer-sdk
from cyt_indexer import build_skills_index

# After
pip install chunk-your-skills
from chunk_your_skills import build_skills_index
```

`pyproject.toml` maturin config:

```toml
module-name = "chunk_your_skills._native"
manifest-path = "../../Cargo.toml"
```

## TypeScript

```bash
# Before
npm install cyt-indexer-sdk

# After
npm install chunk-your-skills
```

Native binary: `chunk-your-skills.*.node` (was `cyt-indexer-sdk.*.node`).

## Rust

```toml
# Before
cyt-indexer = "1.1.0"

# After
chunk-your-skills = "1.1.0"
```

CLI binary: `chunk-your-skills` (was `cyt-indexer`).

## Go

```go
// Before
import "github.com/qdrddr/clear-your-tools/sdk/go/cytindexer"

// After
import "github.com/qdrddr/chunk-your-skills/sdk/go/chunkyourskills"
```

Build C lib first: `./sdk/c/scripts/build-c-lib.sh`, then `go run ./cmd/chunk-native-ensure`.

## C

```cmake
# Before
find_package(CYT REQUIRED)
target_link_libraries(myapp PRIVATE CYT::cyt_indexer)

# After
find_package(CYS REQUIRED)
target_link_libraries(myapp PRIVATE CYS::chunk_your_skills)
```

Header: `#include "chunk_your_skills.h"`.

## Version sync

```bash
./scripts/sync-version.sh
```

Reads semver from root `Cargo.toml` and propagates to Python, npm, C CMake, and Go module version.

## Still need the proxy?

Install clear-your-tools from its repository or PyPI:

```bash
uv tool install 'clear-your-tools[all]'
```

Point it at a local chunk-your-skills Python wheel if you are developing both projects together.
