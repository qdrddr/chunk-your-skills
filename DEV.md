# Development guide

Rust-first monorepo for decomposing agent `SKILL.md` files into page-indexed nodes and
recomposing **skinny skills**. See [README.md](README.md) for package overview and quick start.

## Repository layout

```text
Cargo.toml, src/              # Rust core + CLI + FFI (chunk-your-skills)
sdk/python/                   # PyPI chunk-your-skills (chunk_your_skills)
sdk/typescript/               # npm chunk-your-skills
sdk/go/                       # Go module (chunkyourskills, cgo)
sdk/c/                        # C FFI + CMake (libchunk_your_skills)
sdk/e2e/                      # Published-package smoke tests
examples/                     # decompose/recompose demos (context7 skill)
scripts/                      # local-dev, sync-version, publish helpers
```

## Prerequisites

- **Rust** (stable) — core crate, FFI, and native bindings
- **uv** — Python SDK (`sdk/python`)
- **Node.js 22+** and **npm** — TypeScript SDK (`sdk/typescript`)
- **Go** — Go SDK (`sdk/go`, requires cgo)
- **CMake**, **make** (or **gmake**), **ctest** — C SDK (`sdk/c`)

Optional: **prek** / **pre-commit** for local hooks (see `.pre-commit-config.yaml`).

## Local workflow

```bash
# Full check (Rust + all SDKs)
./scripts/local-dev.sh all

# Rust only (unit tests + FFI smoke)
./scripts/local-dev.sh core-rust
cargo test -p chunk-your-skills --all-features

# Python SDK (editable install + verify + pytest)
./scripts/local-dev.sh sdk-python
cd sdk/python && uv run pytest

# TypeScript SDK (npm ci, build, test)
./scripts/local-dev.sh sdk-typescript

# C + Go (builds FFI first)
./scripts/local-dev.sh sdk-c
./scripts/local-dev.sh sdk-go

# CI-equivalent smoke (Rust + Python build/verify/pytest)
./scripts/local-dev.sh ci

# Quieter output
./scripts/local-dev.sh --silent all
```

## CLI examples

See [examples/README.md](examples/README.md) for `decompose.sh` / `recompose.sh`, node IDs, and
skinny-skill output paths.

## Version sync

Version source of truth: root `Cargo.toml`.

```bash
./scripts/sync-version.sh          # read version from Cargo.toml
./scripts/sync-version.sh 1.0.10   # set and propagate to all manifests
```

Propagates to `Cargo.lock`, `sdk/python/pyproject.toml`, `sdk/typescript/package.json`,
`sdk/c/CMakeLists.txt`, and `sdk/go/moduleversion/version.go`.

## Publish (maintainers)

Tag `vX.Y.Z` triggers GitHub workflows:

1. `publish-crates.yml` → [crates.io `chunk-your-skills`](https://crates.io/crates/chunk-your-skills)
2. In parallel on tag push:
   - `publish-c-ffi.yml` → GitHub Release assets (`libchunk_your_skills`)
3. After crates publish succeeds:
   - `publish-pypi-sdk.yml`, `publish-npm-sdk.yml` (parallel)
   - `e2e-published-crates.yml`, `e2e-published-go-c.yml`
4. After PyPI/npm publish:
   - `e2e-published-pypi-sdk.yml`, `e2e-published-npm.yml`

Manual release helper:

```bash
./scripts/publish-git.sh bump-patch   # or bump-minor, or v1.0.10
```

Individual registry scripts (when needed): `scripts/publish-crates.sh`, `publish-pypi.sh`,
`publish-npm.sh`.

## Published-package E2E (local)

```bash
# All targets at workspace version (from Cargo.toml)
./sdk/e2e/scripts/run-local.sh

# One target, skip registry polling
./sdk/e2e/scripts/run-local.sh --skip-wait python

# Go/C against unreleased local work
./sdk/e2e/scripts/run-local.sh --workspace --skip-wait go c
```

See [sdk/e2e/README.md](sdk/e2e/README.md) for harness details.

## FFI header sync

```bash
cargo build -p chunk-your-skills --no-default-features --features ffi
cp chunk_your_skills.h sdk/c/include/
```

Or: `bash sdk/c/scripts/build-c-lib.sh` (builds the shared library and syncs the header by default).
