#!/usr/bin/env bash
# Build a MinGW-compatible libchunk_your_skills.a import library from an MSVC chunk_your_skills.dll.
# shellcheck shell=bash
set -euo pipefail

usage() {
	cat <<'EOF'
Usage: prepare-windows-cgo.sh [LIB_DIR [NATIVE_DIR]]

Generate libchunk_your_skills.a beside chunk_your_skills.dll for Go cgo on Windows.

LIB_DIR defaults to target/<host-triplet>/release when run from repo root.
NATIVE_DIR defaults to sdk/go/native/<triplet> derived from LIB_DIR.
EOF
}

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../../.." && pwd)"
LIB_DIR="${1:-}"
NATIVE_DIR="${2:-}"

if [[ -z "$LIB_DIR" ]]; then
	LIB_DIR="${REPO_ROOT}/target/$(rustc -vV | sed -n 's/^host: //p')/release"
fi

if [[ -z "$NATIVE_DIR" ]]; then
	triplet="$(basename "$(dirname "$LIB_DIR")")"
	NATIVE_DIR="${REPO_ROOT}/sdk/go/native/${triplet}"
fi

dll="${LIB_DIR}/chunk_your_skills.dll"
import_a="${LIB_DIR}/libchunk_your_skills.a"

if [[ -f "$import_a" ]]; then
	echo "import library already exists: $import_a"
else
	[[ -f "$dll" ]] || {
		echo "error: missing DLL: $dll (build with sdk/c/scripts/build-c-lib.sh first)" >&2
		exit 1
	}

	tmpdir="$(mktemp -d)"
	trap 'rm -rf "$tmpdir"' EXIT
	cp "$dll" "${tmpdir}/chunk_your_skills.dll"
	(
		cd "$tmpdir"
		gendef chunk_your_skills.dll
		dlltool --input-def chunk_your_skills.def --dllname chunk_your_skills.dll --output-lib libchunk_your_skills.a
	)

	cp "${tmpdir}/libchunk_your_skills.a" "$import_a"
	echo "wrote $import_a"
fi

mkdir -p "$NATIVE_DIR"
cp -f "$import_a" "${NATIVE_DIR}/libchunk_your_skills.a"
echo "installed ${NATIVE_DIR}/libchunk_your_skills.a"
