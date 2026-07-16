# chunk-your-skills (Go)

Go bindings for the root Rust crate via **cgo**, using `libchunk_your_skills` and `chunk_your_skills.h`.

```text
GitHub Release / build-c-lib.sh  →  libchunk_your_skills + chunk_your_skills.h
sdk/go/cgo_lib.go                →  chunkyourskills package
```

## Build native library

```bash
./sdk/c/scripts/build-c-lib.sh
go run ./sdk/go/cmd/chunk-native-ensure
```

## Usage

```go
import "github.com/qdrddr/chunk-your-skills/sdk/go/v2/chunkyourskills"

version, err := chunkyourskills.Version()
```

## Release artifacts

Downloads `chunk-your-skills-ffi-<triplet>.tar.gz` from GitHub Releases into
`$XDG_CACHE_HOME/chunk-your-skills/<version>/<triplet>/` and copies into `sdk/go/native/<triplet>/` when writable.

| Artifact | Role |
| -------- | ---- |
| `libchunk_your_skills.so` / `.dylib` / `chunk_your_skills.dll` | Shared library |
| `libchunk_your_skills.a` / `chunk_your_skills.lib` | Static library (Go links `-lchunk_your_skills`) |
| `chunk_your_skills.h` | C header |

Run `go generate` in `sdk/go` (or `go run ./cmd/chunk-native-ensure`) before tests if native artifacts are missing.
