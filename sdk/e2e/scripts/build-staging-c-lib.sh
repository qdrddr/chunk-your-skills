#!/usr/bin/env bash
# Build libchunk_your_skills in CYT_E2E_STAGING for Go/C E2E harnesses.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
STAGING="${CYT_E2E_STAGING:?run prepare-release-checkout.sh first}"
TRIPLET="${CYT_RUST_TARGET:-$("${ROOT}/scripts/host-rust-target.sh")}"
PROFILE="${CYT_C_LIB_PROFILE:-release}"

release_flag=(--release)
if [[ "$PROFILE" == "debug" ]]; then
	release_flag=()
fi

if [[ ! -f "${STAGING}/Cargo.toml" ]]; then
	echo "::error::missing Cargo.toml in CYT_E2E_STAGING=${STAGING}" >&2
	exit 1
fi

export CARGO_TARGET_DIR="${STAGING}/target"

rustup target add "$TRIPLET" >/dev/null 2>&1 || true

host="$("${ROOT}/scripts/host-rust-target.sh")"
target_args=(--target "$TRIPLET")
artifact_dir="${CARGO_TARGET_DIR}/${TRIPLET}/${PROFILE}"

echo "Building chunk-your-skills ffi in ${STAGING} for ${TRIPLET}/${PROFILE}" >&2
(
	cd "$STAGING"
	cargo clean -p chunk-your-skills --target "$TRIPLET" >/dev/null 2>&1 || true
	cargo build -p chunk-your-skills --no-default-features --features ffi \
		"${target_args[@]}" "${release_flag[@]}"
)

if [[ "$TRIPLET" == "$host" && ! -f "${artifact_dir}/libchunk_your_skills.dylib" && ! -f "${artifact_dir}/libchunk_your_skills.so" && ! -f "${artifact_dir}/chunk_your_skills.dll" ]]; then
	host_dir="${CARGO_TARGET_DIR}/${PROFILE}"
	mkdir -p "$artifact_dir"
	shopt -s nullglob
	for artifact in "${host_dir}/libchunk_your_skills.dylib" "${host_dir}/libchunk_your_skills.so" \
		"${host_dir}/chunk_your_skills.dll" "${host_dir}/chunk_your_skills.dll.lib" \
		"${host_dir}/libchunk_your_skills.a" "${host_dir}/chunk_your_skills.lib"; do
		if [[ -f "$artifact" ]]; then
			cp -f "$artifact" "${artifact_dir}/$(basename "$artifact")"
		fi
	done
	shopt -u nullglob
fi

header_src="${STAGING}/chunk_your_skills.h"
header_dst="${STAGING}/sdk/c/include/chunk_your_skills.h"
[[ -f "$header_src" ]] || {
	echo "::error::missing generated header: ${header_src}" >&2
	exit 1
}
mkdir -p "$(dirname "$header_dst")"
cp "$header_src" "$header_dst"
echo "Synced header -> ${header_dst}" >&2

shared=""
case "${TRIPLET}" in
*-pc-windows-msvc) shared="${artifact_dir}/chunk_your_skills.dll" ;;
*-apple-darwin) shared="${artifact_dir}/libchunk_your_skills.dylib" ;;
*) shared="${artifact_dir}/libchunk_your_skills.so" ;;
esac
[[ -f "$shared" ]] || {
	echo "::error::missing shared library: ${shared}" >&2
	exit 1
}
