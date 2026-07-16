# Registry end-to-end tests

Smoke tests that install **only published packages** from public registries—or, for Go/C, a **sparse GitHub tag
checkout**—not the active monorepo tree (unless `--workspace` is set).

| Harness | Source | Package |
| ------- | ------ | ------- |
| [`rust/`](rust/) | [crates.io](https://crates.io/crates/chunk-your-skills) | `chunk-your-skills` |
| [`python/`](python/) | [PyPI](https://pypi.org/project/chunk-your-skills/) | `chunk-your-skills` |
| [`typescript/`](typescript/) | [npm](https://www.npmjs.com/package/chunk-your-skills) | `chunk-your-skills` |
| [`go/`](go/) | [GitHub tag](https://github.com/qdrddr/chunk-your-skills/tags) | `github.com/qdrddr/chunk-your-skills/sdk/go` |
| [`c/`](c/) | [GitHub tag](https://github.com/qdrddr/chunk-your-skills/tags) | `sdk/c` + `libchunk_your_skills` built from tagged root crate |

## Local run

```bash
# Workspace version from root Cargo.toml, all targets
./sdk/e2e/scripts/run-local.sh

# Explicit version (packages must already be published; tag must exist for go/c)
./sdk/e2e/scripts/run-local.sh 1.1.0

# One target, skip registry polling
./sdk/e2e/scripts/run-local.sh --skip-wait python

# Go/C against current monorepo (unreleased local work)
./sdk/e2e/scripts/run-local.sh --workspace --skip-wait go c
```

Targets: `rust`, `python`, `typescript`, `go`, `c`, `all` (default).

Go and C harnesses clone tag `vX.Y.Z` into `CHUNK_YOUR_SKILLS_E2E_STAGING`, build
`libchunk_your_skills` from the tagged root crate, then run isolated tests.

See [`.github/workflows/`](../../.github/workflows/) for CI wiring (`e2e-published-*.yml` after publish workflows).
