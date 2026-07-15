#!/usr/bin/env bash
# Build a MinGW-compatible libchunk_your_skills.a import library from an MSVC chunk_your_skills.dll.
# shellcheck shell=bash
set -euo pipefail

usage() {
	cat <<'EOF'
Usage: prepare-windows-cgo.sh [LIB_DIR]

Generate libchunk_your_skills.a beside chunk_your_skills.dll for Go cgo (-lchunk_your_skills).

LIB_DIR defaults to sdk/go/native/<host-triplet> when run from repo root.
EOF
}

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
LIB_DIR="${1:-}"

if [[ -z "$LIB_DIR" ]]; then
	LIB_DIR="${REPO_ROOT}/sdk/go/native/x86_64-pc-windows-msvc"
fi

dll="${LIB_DIR}/chunk_your_skills.dll"
import_a="${LIB_DIR}/libchunk_your_skills.a"
native_dir="${REPO_ROOT}/sdk/go/native/x86_64-pc-windows-msvc"

if [[ -f "$import_a" ]]; then
	echo "import library already exists: $import_a"
	if [[ -w "$native_dir" ]]; then
		cp -f "$import_a" "${native_dir}/libchunk_your_skills.a"
	fi
	exit 0
fi

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
if [[ -w "$native_dir" ]]; then
	cp -f "$import_a" "${native_dir}/libchunk_your_skills.a"
fi
