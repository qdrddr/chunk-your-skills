# Changelog

All notable changes to **chunk-your-skills** are documented here. Version numbers follow the root
`Cargo.toml` and are synced to Python, npm, Go, and C manifests via `scripts/sync-version.sh`.

---

## 2.0.0

Breaking rename to remove legacy three-letter prefixes that collided with other projects:

- C FFI exports use `chunk_your_skills_*` functions and `CHUNK_YOUR_SKILLS_*` constants.
- Public env vars use `CHUNK_YOUR_SKILLS_*` (for example `CHUNK_YOUR_SKILLS_RELEASE_VERSION`, `CHUNK_YOUR_SKILLS_CACHE_*`).
- Internal dev tooling keeps `CYS_*` where already established (CMake `CYS::chunk_your_skills`).
- Added `scripts/check_no_legacy_prefix.sh` and a pre-commit hook to prevent regressions.

## 1.0.9

- Granular GitHub Actions E2E workflows for published crates, PyPI, npm, Go, and C packages.
- CLI examples and [examples/README.md](examples/README.md) walkthrough for decompose/recompose.
- Rust SDK section in [src/README.md](src/README.md).

## 1.0.8

- Removed token counting from all SDKs and FFI (C, Python, TypeScript, Go).
- Added `scripts/publish-git.sh` release helper.
- Updated example decomposed node files and E2E fixtures accordingly.

## 1.0.7

- Frontmatter helpers: `parse_frontmatter_fields`, `frontmatter_field`, and related SDK exports.
- Markdown linting configuration updates.

## 1.0.6

- GitHub Actions workflow path fixes for cross-platform native binding builds.

## 1.0.5

- Windows Go cgo build improvements (`prepare-windows-cgo.sh`).

## 1.0.4 – 1.0.2

- Multi-language SDK publish pipeline (crates.io, PyPI, npm, GitHub Release C/Go assets).
- Version sync across Rust, Python, TypeScript, Go, and CMake manifests.
- Renamed release artifact env var to `CHUNK_YOUR_SKILLS_RELEASE_VERSION`.

## 1.0.1

- Initial publish scripts for crates.io, npm, and PyPI.

## 1.0.0

A focused **chunk-your-skills** SDK:

- Rust core + CLI for `SKILL.md` pageindex, catalog materialization, and skinny-skill recomposition.
- Python (`chunk_your_skills`), TypeScript, Go (cgo), and C (FFI) bindings over the same Rust crate.
