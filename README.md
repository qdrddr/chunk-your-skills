# Chunk Your Skills

<div align="center">

[![Quick Start][quick-start-shield]](#quick-start)
[![License][license-badge-shield]][license-link]
![No Telemetry][telemetry-shield]

[![version][version-shield]][release-link]
[![discord][discord-shield]][discord-link]

![Rust][rust-tech-shield]
![Python][python-tech-shield]
![TypeScript][typescript-shield]
![Go][go-tech-shield]
![C][c-tech-shield]
![Shell][shell-shield]

</div>

Rust-first library for decomposing agent `SKILL.md` files into page-indexed nodes and recomposing
**skinny skills** — only the sections you select.

This repository is the **chunk-your-skills** SDK monorepo. The former **clear-your-tools** proxy
application layer has been removed; use [clear-your-tools](https://github.com/qdrddr/clear-your-tools)
for the MCP tool-gating proxy.

## What it does

1. **Chunk/Decompose** — parse `SKILL.md` into a tree of nodes (frontmatter, preamble, heading sections).
2. **Cache** — write `metadata.json`, `nodes/page_index.json`, and `nodes/n{id}.md` under a catalog directory.
3. **Recompose** — rebuild a skinny `SKILL.md` from selected node IDs.

## Packages C/Go/Rust/TypeScript/Python

<details closed>
<summary><strong>Published packages</strong></summary>

<div align="center">

![Windows][windows-shield]
![macOS][macos-shield]
![Linux][linux-shield]

</div>

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

**`libchunk_your_skills`** ([Release][c-link])
    </td>
    <td valign="top">

C library via CMake / `build-c-lib.sh`
    </td>
    <td valign="top">

[![GitHub sdk/c][c-version-shield]][c-link]
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

## Quick start

Install the CLI:

```bash
cargo install chunk-your-skills
```

Or build locally: `cargo build -p chunk-your-skills --release`.

Library installs:

```bash
cargo add chunk-your-skills
pip install chunk-your-skills
npm install chunk-your-skills
```

Try the bundled walkthrough — decompose a sample skill, then recompose skinny variants:

```bash
./examples/decompose.sh
export PATH="$PWD/target/release:$PATH"
./examples/recompose.sh
```

See [examples/README.md](examples/README.md) for node IDs, output paths, and CLI flags.

## SDKs

| SDK | Path | Docs |
| --- | --- | --- |
| Python | `sdk/python` | [README](sdk/python/README.md) |
| TypeScript | `sdk/typescript` | [README](sdk/typescript/README.md) |
| Go | `sdk/go` | [README](sdk/go/README.md) |
| C | `sdk/c` | [README](sdk/c/README.md) |

## Development

See [DEV.md](DEV.md) for local workflow, version sync, and publish notes.

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
[c-version-shield]: https://img.shields.io/github/v/release/qdrddr/chunk-your-skills?style=flat-square&label=sdk%2Fc&color=555&logoColor=white
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
[shell-shield]: https://img.shields.io/badge/-Shell-4EAA25?logo=gnu-bash&logoColor=white
[quick-start-shield]: https://img.shields.io/badge/Quick_Start-5_min-blue?style=for-the-badge
[telemetry-shield]: https://img.shields.io/badge/No_Telemetry-none-green?style=for-the-badge
[discord-shield]: https://img.shields.io/badge/Discord-Join-5865F2?logo=discord&logoColor=white
[discord-link]: https://discord.com/invite/FhACaAAW9C
