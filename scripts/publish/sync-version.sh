#!/usr/bin/env bash
# Propagate a single semver to all package manifests and lockfiles.
#
# Usage:
#   ./scripts/publish/sync-version.sh [VERSION]
#
# If VERSION is omitted, read it from root Cargo.toml.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

# shellcheck disable=SC1091
source "${SCRIPT_DIR}/../lib/shorten-paths.sh"
# shellcheck disable=SC1091
source "${SCRIPT_DIR}/version-manifests.sh"
export SHORTEN_ROOT="${ROOT}"
CARGO_TOML="${ROOT}/Cargo.toml"
CARGO_LOCK="${ROOT}/Cargo.lock"
SDK_PYPROJECT="${ROOT}/sdk/python/pyproject.toml"
SDK_UV_LOCK="${ROOT}/sdk/python/uv.lock"
PACKAGE_JSON="${ROOT}/sdk/typescript/package.json"
PACKAGE_LOCK="${ROOT}/sdk/typescript/package-lock.json"
C_CMAKE="${ROOT}/sdk/c/CMakeLists.txt"
GO_VERSION="${ROOT}/sdk/go/moduleversion/version.go"
TAG_FILE="${ROOT}/search/.publish-tag"

usage() {
	cat <<EOF
Usage: $(basename "$0") [VERSION]

Propagate VERSION to all manifests and lockfiles:
$(version_manifest_relative_labels)

If VERSION is omitted, read it from ${CARGO_TOML}.
EOF
}

read_crate_version() {
	grep -E '^version[[:space:]]*=' "${CARGO_TOML}" |
		head -1 |
		sed -E 's/^version[[:space:]]*=[[:space:]]*"(.*)".*/\1/'
}

validate_version() {
	local version="$1"
	if [[ ! "${version}" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.-]+)?(\+[0-9A-Za-z.-]+)?$ ]]; then
		echo "error: invalid semver: ${version}" >&2
		exit 1
	fi
}

replace_file_if_changed() {
	local tmp="$1"
	local file="$2"
	if cmp -s "${tmp}" "${file}"; then
		rm -f "${tmp}"
		return 1
	fi
	mv "${tmp}" "${file}"
}

update_toml_version() {
	local file="$1"
	local version="$2"
	local tmp
	tmp="$(mktemp)"
	awk -v version="${version}" '
    !done && /^version[[:space:]]*=/ {
      print "version = \"" version "\""
      done=1
      next
    }
    { print }
  ' "${file}" >"${tmp}"
	replace_file_if_changed "${tmp}" "${file}" || true
}

update_cargo_lock_version() {
	local version="$1"
	local tmp
	tmp="$(mktemp)"
	awk -v version="${version}" '
    /^\[\[package\]\]/ { in_package=1 }
    /^\[\[/ && $0 !~ /^\[\[package\]\]/ { in_package=0 }
    in_package && /^name = "chunk-your-skills"$/ { found=1 }
    found && /^version = / {
      print "version = \"" version "\""
      found=0
      next
    }
    { print }
  ' "${CARGO_LOCK}" >"${tmp}"
	replace_file_if_changed "${tmp}" "${CARGO_LOCK}" || true
}

update_package_json_version() {
	local version="$1"
	local tmp
	tmp="$(mktemp)"
	awk -v version="${version}" '
    !done && /^  "version": "/ {
      print "  \"version\": \"" version "\","
      done=1
      next
    }
    { print }
  ' "${PACKAGE_JSON}" >"${tmp}"
	replace_file_if_changed "${tmp}" "${PACKAGE_JSON}" || true
}

update_package_lock_version() {
	local version="$1"
	local tmp
	tmp="$(mktemp)"
	awk -v version="${version}" '
    BEGIN { root_done=0; pkg_done=0 }
    !root_done && /^  "version": "/ {
      print "  \"version\": \"" version "\","
      root_done=1
      next
    }
    !pkg_done && /^      "version": "/ {
      print "      \"version\": \"" version "\","
      pkg_done=1
      next
    }
    { print }
  ' "${PACKAGE_LOCK}" >"${tmp}"
	replace_file_if_changed "${tmp}" "${PACKAGE_LOCK}" || true
}

update_cmake_project_version() {
	local version="$1"
	local tmp
	tmp="$(mktemp)"
	awk -v version="${version}" '
    /^project\(chunk-your-skills-c VERSION / {
      print "project(chunk-your-skills-c VERSION " version " LANGUAGES C)"
      next
    }
    { print }
  ' "${C_CMAKE}" >"${tmp}"
	replace_file_if_changed "${tmp}" "${C_CMAKE}" || true
}

update_go_module_version() {
	local version="$1"
	local tmp
	tmp="$(mktemp)"
	awk -v version="${version}" '
    /^const Version = "/ {
      print "const Version = \"" version "\""
      next
    }
    { print }
  ' "${GO_VERSION}" >"${tmp}"
	replace_file_if_changed "${tmp}" "${GO_VERSION}" || true
}

update_uv_lock_version() {
	local version="$1"
	local tmp
	tmp="$(mktemp)"
	awk -v version="${version}" '
    /^name = "chunk-your-skills"$/ { found=1 }
    found && /^version = / {
      print "version = \"" version "\""
      found=0
      next
    }
    { print }
  ' "${SDK_UV_LOCK}" >"${tmp}"
	replace_file_if_changed "${tmp}" "${SDK_UV_LOCK}" || true
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
	usage
	exit 0
fi

if [[ $# -gt 1 ]]; then
	usage >&2
	exit 1
fi

if [[ $# -eq 1 ]]; then
	version="$1"
else
	version="$(read_crate_version)"
	if [[ -z "${version}" ]]; then
		printf 'error: could not read version from %s\n' "${CARGO_TOML}" | shorten_paths >&2
		exit 1
	fi
fi

validate_version "${version}"

mapfile -t manifest_files < <(version_manifest_paths "${ROOT}")
for file in "${manifest_files[@]}"; do
	if [[ "${file}" == "${TAG_FILE}" ]]; then
		continue
	fi
	if [[ ! -f "${file}" ]]; then
		printf 'error: missing %s\n' "${file}" | shorten_paths >&2
		exit 1
	fi
done

tag="v${version}"

update_toml_version "${CARGO_TOML}" "${version}"
update_cargo_lock_version "${version}"
update_toml_version "${SDK_PYPROJECT}" "${version}"
update_uv_lock_version "${version}"
update_package_json_version "${version}"
update_package_lock_version "${version}"
update_cmake_project_version "${version}"
update_go_module_version "${version}"
mkdir -p "$(dirname "${TAG_FILE}")"
printf 'tag=%s\n' "${tag}" >"${TAG_FILE}"

cat <<EOF | shorten_paths
synced version ${version} to:
  ${CARGO_TOML}
  ${CARGO_LOCK} (chunk-your-skills)
  ${SDK_PYPROJECT}
  ${SDK_UV_LOCK} (chunk-your-skills)
  ${PACKAGE_JSON}
  ${PACKAGE_LOCK}
  ${C_CMAKE} (project VERSION)
  ${GO_VERSION} (Version)
  ${TAG_FILE} (tag=${tag})
EOF
