#!/usr/bin/env bash
# shellcheck shell=bash
# Shared helpers for chunk-your-skills local development.
# Not meant to be executed directly.

if [[ -z "${CYS_LOCAL_DEV_LIB_SOURCED:-}" ]]; then
	CYS_LOCAL_DEV_LIB_SOURCED=1

	CYS_REPO_ROOT="${CYS_REPO_ROOT:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}"
	export CYS_REPO_ROOT

	die() {
		echo "error: $*" >&2
		exit 1
	}

	info() {
		[[ -n "${CYS_LOCAL_DEV_SHORT:-}" ]] && return 0
		echo "==> $*"
	}

	cys_section() {
		[[ -n "${CYS_LOCAL_DEV_SHORT:-}" ]] && return 0
		echo ""
		echo "$*"
	}

	cys_run() {
		if [[ -n "${CYS_LOCAL_DEV_SHORT:-}" ]]; then
			"$@" >/dev/null
		else
			"$@"
		fi
	}

	cys_filter_short_logs() {
		awk '
			/^==>/ { next }
			/^OK:/ { next }
			/^  / { next }
			/error:/ { print; next }
			/warning:/ { print; next }
			/FAILED/ { print; next }
			/not ok / { print; next }
		'
	}

	require_cmd() {
		command -v "$1" >/dev/null 2>&1 || die "missing required command: $1"
	}

	cys_cmake_make_program() {
		local candidate
		for candidate in gmake make; do
			if command -v "$candidate" >/dev/null 2>&1; then
				command -v "$candidate"
				return 0
			fi
		done
		die "missing required command: make or gmake"
	}

	cys_npm() {
		env -u npm_config_devdir -u NODE_ENV npm "$@"
	}

	require_repo_root() {
		[[ -f "${CYS_REPO_ROOT}/Cargo.toml" ]] || die "not a repo root: ${CYS_REPO_ROOT}"
		[[ -f "${CYS_REPO_ROOT}/src/lib.rs" ]] || die "missing root Rust crate (src/lib.rs)"
		[[ -f "${CYS_REPO_ROOT}/sdk/python/pyproject.toml" ]] || die "missing sdk/python"
	}

	cys_sync_sdk_python() {
		require_cmd uv
		cd "${CYS_REPO_ROOT}/sdk/python" || die "cd failed"
		info "uv sync sdk/python"
		cys_run uv sync
	}

	cys_build_rust() {
		require_cmd cargo
		cd "${CYS_REPO_ROOT}" || die "cd failed"
		info "cargo test -p chunk-your-skills"
		cys_run env -u CARGO_TARGET_DIR cargo test -p chunk-your-skills
		info "cargo test -p chunk-your-skills --features ffi --test ffi_smoke"
		cys_run env -u CARGO_TARGET_DIR cargo test -p chunk-your-skills --no-default-features --features ffi --test ffi_smoke
	}

	cys_build_sdk_python() {
		require_cmd uv
		cys_sync_sdk_python
		cd "${CYS_REPO_ROOT}/sdk/python" || die "cd failed"
		info "maturin develop --release"
		cys_run uv run maturin develop --release
	}

	cys_build_sdk_typescript() {
		require_cmd npm
		cd "${CYS_REPO_ROOT}/sdk/typescript" || die "cd failed"
		info "npm ci, build, test"
		cys_run cys_npm ci
		cys_run cys_npm run build
		cys_run cys_npm test
	}

	cys_build_sdk_c() {
		require_cmd cmake
		require_cmd ctest
		require_cmd rustc
		cd "${CYS_REPO_ROOT}" || die "cd failed"
		local triplet make_prog
		triplet="$(rustc -vV | sed -n 's/^host: //p')"
		make_prog="$(cys_cmake_make_program)"
		info "build C FFI (sdk/c, ${triplet})"
		cys_run env -u CARGO_TARGET_DIR bash sdk/c/scripts/build-c-lib.sh --target "${triplet}"
		info "cmake configure + build"
		cys_run env -u CARGO_TARGET_DIR cmake -S sdk/c -B sdk/c/build \
			-DCMAKE_BUILD_TYPE=Release \
			-DCYS_RUST_TARGET="${triplet}" \
			-DCMAKE_MAKE_PROGRAM="${make_prog}"
		cys_run env -u CARGO_TARGET_DIR cmake --build sdk/c/build
		info "ctest sdk/c"
		cys_run env -u CARGO_TARGET_DIR ctest --test-dir sdk/c/build --output-on-failure
	}

	cys_build_sdk_go() {
		require_cmd go
		require_cmd rustc
		cd "${CYS_REPO_ROOT}" || die "cd failed"
		info "build C FFI (sdk/go)"
		cys_run env -u CARGO_TARGET_DIR bash sdk/c/scripts/build-c-lib.sh --no-sync-header
		cd "${CYS_REPO_ROOT}/sdk/go" || die "cd failed"
		export CGO_ENABLED=1
		local host_triplet
		host_triplet="$(rustc -vV | sed -n 's/^host: //p')"
		export PATH="${CYS_REPO_ROOT}/target/${host_triplet}/release:${PATH}"
		info "go native ensure"
		cys_run go run ./cmd/chunk-native-ensure -static-only
		info "go test ./..."
		cys_run env -u CARGO_TARGET_DIR go test ./...
	}

	cys_build_all_sdks() {
		cys_build_sdk_python
		cys_build_sdk_c
		cys_build_sdk_go
		cys_build_sdk_typescript
	}

	cys_verify_sdk_python() {
		require_cmd uv
		cd "${CYS_REPO_ROOT}/sdk/python" || die "cd failed"
		info "verify sdk/python"
		cys_run uv run python - "${CYS_REPO_ROOT}" <<'PY'
import json
import sys
from importlib import metadata
from pathlib import Path

root = Path(sys.argv[1]).resolve()
sdk_root = (root / "sdk" / "python").resolve()

try:
    dist = metadata.distribution("chunk-your-skills")
except metadata.PackageNotFoundError:
    sys.exit("chunk-your-skills is not installed; run: ./scripts/local-dev.sh sdk-python")

install_kind = "editable"
try:
    direct = json.loads(dist.read_text("direct_url.json"))
    url = str(direct.get("url", "")).replace("\\", "/")
    if "sdk/python" not in url:
        sys.exit(
            "chunk-your-skills direct_url.json does not point at sdk/python:\n" + url
        )
except FileNotFoundError:
    import chunk_your_skills

    pkg_dir = Path(chunk_your_skills.__file__).resolve()
    if sdk_root not in pkg_dir.parents:
        sys.exit(
            "chunk-your-skills is not loaded from sdk/python\n"
            f"  package file: {pkg_dir}\n"
            f"  expected under: {sdk_root}\n"
            "Run ./scripts/local-dev.sh sdk-python"
        )
    install_kind = "path"

from chunk_your_skills._native import build_skills_index

if not callable(build_skills_index):
    sys.exit("chunk_your_skills._native.build_skills_index is not callable (rebuild with sdk-python)")

print("OK: local chunk-your-skills (sdk/python)")
print(f"  sdk root: {sdk_root}")
print(f"  install: {install_kind}")
PY
	}

	cys_test_sdk_python() {
		require_cmd uv
		cd "${CYS_REPO_ROOT}/sdk/python" || die "cd failed"
		info "pytest sdk/python/tests"
		cys_run uv run pytest tests
	}

	cys_run_all() {
		cys_section "Core (Rust)"
		cys_build_rust

		cys_section "SDK: Python"
		cys_build_sdk_python
		cys_verify_sdk_python
		cys_test_sdk_python

		cys_section "SDK: C"
		cys_build_sdk_c

		cys_section "SDK: Go"
		cys_build_sdk_go

		cys_section "SDK: TypeScript"
		cys_build_sdk_typescript
	}

fi
