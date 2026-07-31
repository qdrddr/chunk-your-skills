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
scripts/                      # local/dev, publish, pre-commit-hooks, legal, deps
```

## Prerequisites

- **Rust** (stable) — core crate, FFI, and native bindings
- **uv** — Python SDK (`sdk/python`)
- **Node.js 22+** and **npm** — TypeScript SDK (`sdk/typescript`)
- **Go** — Go SDK (`sdk/go`, requires cgo)
- **CMake**, **make** (or **gmake**), **ctest** — C SDK (`sdk/c`)

Optional: **prek** / **pre-commit** for local hooks (see `.pre-commit-config.yaml`). The
`check-no-legacy-prefix` hook runs `./scripts/pre-commit-hooks/check_no_legacy_prefix.sh` to block legacy naming
regressions.

If you use a local Cursor rules file named for the old prefix, rename it to
`chunk-your-skills-injection.mdc` under `.cursor/rules/` (gitignored).

## Local workflow

```bash
# Full check (Rust + all SDKs)
./scripts/local/dev/workflow.sh all

# Rust only (unit tests + FFI smoke)
./scripts/local/dev/workflow.sh core-rust
cargo test -p chunk-your-skills --all-features

# Python SDK (editable install + verify + pytest)
./scripts/local/dev/workflow.sh sdk-python
cd sdk/python && uv run pytest

# TypeScript SDK (npm ci, build, test)
./scripts/local/dev/workflow.sh sdk-typescript

# C + Go (builds FFI first)
./scripts/local/dev/workflow.sh sdk-c
./scripts/local/dev/workflow.sh sdk-go

# CI-equivalent smoke (Rust + Python build/verify/pytest)
./scripts/local/dev/workflow.sh ci

# Quieter output
./scripts/local/dev/workflow.sh --silent all
```

## CLI examples

See [examples/README.md](examples/README.md) for `decompose.sh` / `recompose.sh`, node IDs, and
skinny-skill output paths.

## Version sync

Version source of truth: root `Cargo.toml`.

```bash
./scripts/publish/sync-version.sh          # read version from Cargo.toml
./scripts/publish/sync-version.sh 1.0.10   # set and propagate to all manifests
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
./scripts/publish/publish-git.sh bump-patch   # or bump-minor, or v1.0.10
```

Individual registry scripts (when needed): `scripts/publish/publish-crates.sh`, `publish-pypi.sh`,
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

## Cargo.lock and monorepo patches

`Cargo.lock` in this repo must **not** contain `[[patch.unused]]` stanzas. They are not
dependencies of `chunk-your-skills`; Cargo adds them when this checkout inherits unrelated
`[patch.crates-io]` entries from the parent **clear-your-tools** monorepo
(`../.cargo/config.toml`), typically patches for `chunk-your-tools` worktrees.

**Fix (do once, not via strip scripts):**

1. Remove any `[[patch.unused]]` blocks from `Cargo.lock` (delete the lines; do not commit them).
2. Ensure `clear-your-tools/.cargo/config.toml` does **not** define `[patch.crates-io]` at the
   monorepo root. Cross-crate worktree patches belong on the indexer workspace that needs them,
   not on every nested checkout.
3. Use normal `cargo generate-lockfile` / `cargo build` — they stay clean when parent config is
   correct.

This repo's `.cargo/config.toml` only overrides `chunk-your-skills` to `path = "."` when nested,
so local edits use this tree instead of a versioned worktree path.

`./scripts/deps/verify-pins.sh` fails if `[[patch.unused]]` appears in `Cargo.lock`. Rust
lock validation runs in an isolated copy of the repo so parent `clear-your-tools` patch
config does not affect the check (see `run_cargo_metadata_locked_isolated` in
`scripts/deps/verify-pins.sh`).
