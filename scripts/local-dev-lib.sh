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

	cyt_section() {
		[[ -n "${CYS_LOCAL_DEV_SHORT:-}" ]] && return 0
		echo ""
		echo "$*"
	}

	cyt_run() {
		if [[ -n "${CYS_LOCAL_DEV_SHORT:-}" ]]; then
			"$@" >/dev/null
		else
			"$@"
		fi
	}

	cyt_filter_short_logs() {
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

	cyt_cmake_make_program() {
		local candidate
		for candidate in gmake make; do
			if command -v "$candidate" >/dev/null 2>&1; then
				command -v "$candidate"
				return 0
			fi
		done
		die "missing required command: make or gmake"
	}

	cyt_npm() {
		env -u npm_config_devdir -u NODE_ENV npm "$@"
	}

	require_repo_root() {
		[[ -f "${CYS_REPO_ROOT}/Cargo.toml" ]] || die "not a repo root: ${CYS_REPO_ROOT}"
		[[ -f "${CYS_REPO_ROOT}/src/lib.rs" ]] || die "missing root Rust crate (src/lib.rs)"
		[[ -f "${CYS_REPO_ROOT}/sdk/python/pyproject.toml" ]] || die "missing sdk/python"
	}

	cyt_sync_sdk_python() {
		require_cmd uv
		cd "${CYS_REPO_ROOT}/sdk/python" || die "cd failed"
		info "uv sync sdk/python"
		cyt_run uv sync
	}

	cyt_build_rust() {
		require_cmd cargo
		cd "${CYS_REPO_ROOT}" || die "cd failed"
		info "cargo test -p chunk-your-skills"
		cyt_run env -u CARGO_TARGET_DIR cargo test -p chunk-your-skills
		info "cargo test -p chunk-your-skills --features ffi --test ffi_smoke"
		cyt_run env -u CARGO_TARGET_DIR cargo test -p chunk-your-skills --no-default-features --features ffi --test ffi_smoke
	}

	cyt_build_sdk_python() {
		require_cmd uv
		cyt_sync_sdk_python
		cd "${CYS_REPO_ROOT}/sdk/python" || die "cd failed"
		info "maturin develop --release"
		cyt_run uv run maturin develop --release
	}

	cyt_build_sdk_typescript() {
		require_cmd npm
		cd "${CYS_REPO_ROOT}/sdk/typescript" || die "cd failed"
		info "npm ci, build, test"
		cyt_run cyt_npm ci
		cyt_run cyt_npm run build
		cyt_run cyt_npm test
	}

	cyt_build_sdk_c() {
		require_cmd cmake
		require_cmd ctest
		require_cmd rustc
		cd "${CYS_REPO_ROOT}" || die "cd failed"
		local triplet make_prog
		triplet="$(rustc -vV | sed -n 's/^host: //p')"
		make_prog="$(cyt_cmake_make_program)"
		info "build C FFI (sdk/c, ${triplet})"
		cyt_run env -u CARGO_TARGET_DIR bash sdk/c/scripts/build-c-lib.sh --target "${triplet}"
		info "cmake configure + build"
		cyt_run env -u CARGO_TARGET_DIR cmake -S sdk/c -B sdk/c/build \
			-DCMAKE_BUILD_TYPE=Release \
			-DCYS_RUST_TARGET="${triplet}" \
			-DCMAKE_MAKE_PROGRAM="${make_prog}"
		cyt_run env -u CARGO_TARGET_DIR cmake --build sdk/c/build
		info "ctest sdk/c"
		cyt_run env -u CARGO_TARGET_DIR ctest --test-dir sdk/c/build --output-on-failure
	}

	cyt_build_sdk_go() {
		require_cmd go
		require_cmd rustc
		cd "${CYS_REPO_ROOT}" || die "cd failed"
		info "build C FFI (sdk/go)"
		cyt_run env -u CARGO_TARGET_DIR bash sdk/c/scripts/build-c-lib.sh --no-sync-header
		cd "${CYS_REPO_ROOT}/sdk/go" || die "cd failed"
		export CGO_ENABLED=1
		local host_triplet
		host_triplet="$(rustc -vV | sed -n 's/^host: //p')"
		export PATH="${CYS_REPO_ROOT}/target/${host_triplet}/release:${PATH}"
		info "go native ensure"
		cyt_run go run ./cmd/chunk-native-ensure -static-only
		info "go test ./..."
		cyt_run env -u CARGO_TARGET_DIR go test ./...
	}

	cyt_build_all_sdks() {
		cyt_build_sdk_python
		cyt_build_sdk_c
		cyt_build_sdk_go
		cyt_build_sdk_typescript
	}

	cyt_verify_sdk_python() {
		require_cmd uv
		cd "${CYS_REPO_ROOT}/sdk/python" || die "cd failed"
		info "verify sdk/python"
		cyt_run uv run python - "${CYS_REPO_ROOT}" <<'PY'
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

	cyt_test_sdk_python() {
		require_cmd uv
		cd "${CYS_REPO_ROOT}/sdk/python" || die "cd failed"
		info "pytest sdk/python/tests"
		cyt_run uv run pytest tests
	}

	cyt_run_all() {
		cyt_section "Core (Rust)"
		cyt_build_rust

		cyt_section "SDK: Python"
		cyt_build_sdk_python
		cyt_verify_sdk_python
		cyt_test_sdk_python

		cyt_section "SDK: C"
		cyt_build_sdk_c

		cyt_section "SDK: Go"
		cyt_build_sdk_go

		cyt_section "SDK: TypeScript"
		cyt_build_sdk_typescript
	}

fi
