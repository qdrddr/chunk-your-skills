#!/usr/bin/env bash
# Shared version manifest paths for sync-version.sh and publish-git.sh.
# shellcheck shell=bash

version_manifest_paths() {
	local root="$1"
	printf '%s\n' \
		"${root}/Cargo.toml" \
		"${root}/Cargo.lock" \
		"${root}/sdk/python/pyproject.toml" \
		"${root}/sdk/python/uv.lock" \
		"${root}/sdk/typescript/package.json" \
		"${root}/sdk/typescript/package-lock.json" \
		"${root}/sdk/c/CMakeLists.txt" \
		"${root}/sdk/go/moduleversion/version.go" \
		"${root}/search/.publish-tag"
}

version_manifest_relative_labels() {
	cat <<'EOF'
  - Cargo.toml (root crate)
  - Cargo.lock (chunk-your-skills)
  - sdk/python/pyproject.toml
  - sdk/python/uv.lock (chunk-your-skills)
  - sdk/typescript/package.json
  - sdk/typescript/package-lock.json
  - sdk/c/CMakeLists.txt (project VERSION)
  - sdk/go/moduleversion/version.go (Version)
  - search/.publish-tag (tag=vX.Y.Z)
EOF
}
