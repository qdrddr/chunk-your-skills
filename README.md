# Chunk Your Skills

<div align="center">

[![License][license-badge-shield]][license-link]
[![version][version-shield]][release-link]

![Rust][rust-tech-shield]
![Python][python-tech-shield]
![TypeScript][typescript-shield]
![Go][go-tech-shield]
![C][c-tech-shield]

</div>

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

<details open>
<summary><strong>Published packages</strong></summary>

<table border="0">
  <tr>
    <td valign="top">

**`chunk-your-skills`** ([crates.io][rust-link])
    </td>
    <td valign="top">

Rust library and CLI
    </td>
    <td valign="top">

[![crates.io chunk-your-skills][rust-version-shield]][rust-link]

[![crates.io downloads][rust-downloads-shield]][rust-link]
    </td>
  </tr>
  <tr>
    <td valign="top">

**`chunk-your-skills`** ([PyPI][pypi-link])
    </td>
    <td valign="top">

Python SDK (`import chunk_your_skills`)
    </td>
    <td valign="top">

[![PyPI chunk-your-skills][pypi-version-shield]][pypi-link]

[![PyPI downloads][pypi-downloads-shield]][pypi-link]
    </td>
  </tr>
  <tr>
    <td valign="top">

**`chunk-your-skills`** ([npm][npm-link])
    </td>
    <td valign="top">

TypeScript SDK
    </td>
    <td valign="top">

[![npm chunk-your-skills][npm-version-shield]][npm-link]

[![npm downloads][npm-downloads-shield]][npm-link]
    </td>
  </tr>
  <tr>
    <td valign="top">

**`libchunk_your_skills`** ([GitHub Release][c-link])
    </td>
    <td valign="top">

C library via CMake / `build-c-lib.sh`
    </td>
    <td valign="top">

[![GitHub libchunk_your_skills][c-version-shield]][c-link]
    </td>
  </tr>
  <tr>
    <td valign="top">

**`sdk/go`** ([pkg.go.dev][go-link])
    </td>
    <td valign="top">

Go SDK via cgo (`import chunkyourskills`)
    </td>
    <td valign="top">

[![pkg.go.dev sdk/go][go-version-shield]][go-link]
    </td>
  </tr>
</table>

</details>

## Quick start (CLI)

```bash
cargo install where chunk-your-skills

# Or local build for developers
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

## SDKs

| SDK | Path | Docs |
| --- | --- | --- |
| Python | `sdk/python` | [README](sdk/python/README.md) |
| TypeScript | `sdk/typescript` | [README](sdk/typescript/README.md) |
| Go | `sdk/go` | [README](sdk/go/README.md) |
| C | `sdk/c` | [README](sdk/c/README.md) |

## Supported platforms

<div align="center">

[![Windows][windows-shield]](#supported-platforms)
[![macOS][macos-shield]](#supported-platforms)
[![Linux][linux-shield]](#supported-platforms)

</div>

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

## License

Apache-2.0 — see [LICENSE](LICENSE).

[license-badge-shield]: https://img.shields.io/badge/License-Apache_2.0-yellow?style=for-the-badge
[license-link]: LICENSE
[version-shield]: https://img.shields.io/github/v/release/qdrddr/chunk-your-skills?style=flat-square&label=version&color=4385BE&logoColor=white
[release-link]: https://github.com/qdrddr/chunk-your-skills/releases
[rust-version-shield]: https://img.shields.io/crates/v/chunk-your-skills?logo=rust&color=e6522c&logoColor=white
[rust-downloads-shield]: https://img.shields.io/crates/d/chunk-your-skills?logo=rust&color=e6522c&logoColor=white
[rust-link]: https://crates.io/crates/chunk-your-skills
[pypi-version-shield]: https://img.shields.io/pypi/v/chunk-your-skills?logo=pypi&logoColor=white&color=2E8B57
[pypi-downloads-shield]: https://img.shields.io/pypi/dm/chunk-your-skills?logo=pypi&logoColor=white&color=2E8B57
[pypi-link]: https://pypi.org/project/chunk-your-skills/
[npm-version-shield]: https://img.shields.io/npm/v/chunk-your-skills?logo=npm&color=3178C6&logoColor=white
[npm-downloads-shield]: https://img.shields.io/npm/dm/chunk-your-skills?logo=npm&color=3178C6&logoColor=white
[npm-link]: https://www.npmjs.com/package/chunk-your-skills
[c-version-shield]: https://img.shields.io/github/v/release/qdrddr/chunk-your-skills?style=flat-square&label=libchunk_your_skills&color=555&logoColor=white
[c-link]: https://github.com/qdrddr/chunk-your-skills/releases
[go-version-shield]: https://pkg.go.dev/badge/github.com/qdrddr/chunk-your-skills/sdk/go
[go-link]: https://pkg.go.dev/github.com/qdrddr/chunk-your-skills/sdk/go
[rust-tech-shield]: https://img.shields.io/badge/-Rust-e6522c?logo=rust&logoColor=white
[python-tech-shield]: https://img.shields.io/badge/-Python-3776AB?logo=python&logoColor=white
[typescript-shield]: https://img.shields.io/badge/-TypeScript-3178C6?logo=typescript&logoColor=white
[go-tech-shield]: https://img.shields.io/badge/-Go-00ADD8?logo=go&logoColor=white
[c-tech-shield]: https://img.shields.io/badge/-C-A8B9CC?logo=c&logoColor=white
[windows-shield]: https://img.shields.io/badge/Windows-supported-0078D6?logo=windows&logoColor=white
[macos-shield]: https://img.shields.io/badge/macOS-supported-000000?logo=apple&logoColor=white
[linux-shield]: https://img.shields.io/badge/Linux-supported-FCC624?logo=linux&logoColor=black
