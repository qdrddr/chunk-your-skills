#!/usr/bin/env bash
# Bump version manifests, commit, push, tag, and create a GitHub Release.
#
# Usage:
#   ./scripts/publish/publish-git.sh v1.0.8
#   ./scripts/publish/publish-git.sh bump-patch
#   ./scripts/publish/publish-git.sh bump-minor
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

# shellcheck disable=SC1091
source "${SCRIPT_DIR}/../lib/shorten-paths.sh"
# shellcheck disable=SC1091
source "${SCRIPT_DIR}/version-manifests.sh"
export SHORTEN_ROOT="${ROOT}"

usage() {
	cat <<EOF
Usage: $(basename "$0") TAG | bump-patch | bump-minor

Examples:
  $(basename "$0") v1.0.8
  $(basename "$0") bump-patch
  $(basename "$0") bump-minor

Auto-bump (bump-patch / bump-minor):
  - Fetch the latest git tags and GitHub releases matching vMAJOR.MINOR.PATCH
  - Pick the highest version among both
  - bump-patch: increment PATCH, e.g. v1.0.7 -> v1.0.8
  - bump-minor: increment MINOR and reset PATCH to 0, e.g. v1.0.7 -> v1.1.0

Steps:
  1. Run scripts/publish/sync-version.sh with the semver (without the leading v)
  2. Commit all version manifest files (including search/.publish-tag)
  3. Push the current branch
  4. Force-create the monorepo git tag vX.Y.Z and push it
  5. Force-create the Go module tag sdk/go/vX.Y.Z and push it
  6. Create (or recreate) a GitHub Release for vX.Y.Z
EOF
}

validate_tag() {
	local tag="$1"
	if [[ ! "${tag}" =~ ^v[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.-]+)?(\+[0-9A-Za-z.-]+)?$ ]]; then
		echo "error: invalid tag (expected vX.Y.Z): ${tag}" >&2
		exit 1
	fi
}

require_command() {
	local cmd="$1"
	if ! command -v "${cmd}" >/dev/null 2>&1; then
		echo "error: required command not found: ${cmd}" >&2
		exit 1
	fi
}

version_files() {
	version_manifest_paths "${ROOT}"
}

read_head_crate_version() {
	git show HEAD:Cargo.toml |
		grep -E '^version[[:space:]]*=' |
		head -1 |
		sed -E 's/^version[[:space:]]*=[[:space:]]*"(.*)".*/\1/'
}

stage_version_files() {
	local file publish_tag
	publish_tag="$(version_manifest_publish_tag "${ROOT}")"

	for file in "${files[@]}"; do
		if [[ "${file}" == "${publish_tag}" ]]; then
			# Tracked for publish CI but listed in .gitignore; plain git add skips it.
			git add -f -- "${file}"
		else
			git add -- "${file}"
		fi
	done
}

assert_version_files_staged() {
	local file

	for file in "${files[@]}"; do
		if ! git diff --quiet -- "${file}"; then
			printf 'error: %s still has unstaged changes after staging version manifests\n' "${file}" | shorten_paths >&2
			exit 1
		fi
	done
}

assert_version_files_committed() {
	local file head_version

	for file in "${files[@]}"; do
		if ! git diff --quiet HEAD -- "${file}"; then
			printf 'error: %s was not committed\n' "${file}" | shorten_paths >&2
			exit 1
		fi
		if ! git diff --cached --quiet -- "${file}"; then
			printf 'error: %s is still staged but was not committed\n' "${file}" | shorten_paths >&2
			exit 1
		fi
	done

	head_version="$(read_head_crate_version)"
	if [[ "${head_version}" != "${semver}" ]]; then
		echo "error: HEAD Cargo.toml is ${head_version} but publish target is ${semver}" >&2
		exit 1
	fi
}

semver_tag_pattern='^v[0-9]+\.[0-9]+\.[0-9]+$'

collect_version_tags() {
	git fetch origin --tags --quiet 2>/dev/null || true

	git tag -l 'v[0-9]*.[0-9]*.[0-9]*' |
		grep -E "${semver_tag_pattern}" || true

	gh release list --limit 1000 --json tagName --jq '.[].tagName' |
		grep -E "${semver_tag_pattern}" || true
}

latest_version_tag() {
	local -a versions=()

	mapfile -t versions < <(collect_version_tags | sort -uV)
	if ((${#versions[@]} == 0)); then
		echo "error: no vMAJOR.MINOR.PATCH tags or releases found; pass an explicit tag" >&2
		exit 1
	fi

	printf '%s\n' "${versions[-1]}"
}

resolve_bump_tag() {
	local bump_kind="$1"
	local latest major minor patch semver

	latest="$(latest_version_tag)"
	semver="${latest#v}"
	IFS='.' read -r major minor patch <<<"${semver}"

	case "${bump_kind}" in
	bump-patch)
		patch=$((patch + 1))
		;;
	bump-minor)
		minor=$((minor + 1))
		patch=0
		;;
	*)
		echo "error: unknown bump kind: ${bump_kind}" >&2
		exit 1
		;;
	esac

	printf 'v%s.%s.%s\n' "${major}" "${minor}" "${patch}"
}

previous_tag() {
	local tag="$1"
	git tag -l 'v[0-9]*.[0-9]*.[0-9]*' --sort=-version:refname |
		while read -r candidate; do
			if [[ "${candidate}" != "${tag}" ]]; then
				printf '%s\n' "${candidate}"
				return 0
			fi
		done
}

release_notes() {
	local tag="$1"
	local prev_tag repo

	prev_tag="$(previous_tag "${tag}")"
	repo="$(gh repo view --json nameWithOwner --jq .nameWithOwner)"

	if [[ -n "${prev_tag}" ]]; then
		printf '**Full Changelog**: https://github.com/%s/compare/%s...%s\n' \
			"${repo}" "${prev_tag}" "${tag}"
	else
		printf 'Release %s\n' "${tag}"
	fi
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
	usage
	exit 0
fi

if [[ $# -ne 1 ]]; then
	usage >&2
	exit 1
fi

require_command git
require_command gh

arg="$1"
case "${arg}" in
bump-patch | bump-minor)
	tag="$(resolve_bump_tag "${arg}")"
	echo "${arg} resolved next tag: ${tag}"
	;;
*)
	tag="${arg}"
	;;
esac

validate_tag "${tag}"
semver="${tag#v}"

cd "${ROOT}"

if ! git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
	echo "error: not inside a git repository" >&2
	exit 1
fi

branch="$(git branch --show-current)"
if [[ -z "${branch}" ]]; then
	echo "error: detached HEAD; checkout a branch before publishing" >&2
	exit 1
fi

mapfile -t files < <(version_files)

"${SCRIPT_DIR}/sync-version.sh" "${semver}"

stage_version_files
assert_version_files_staged
if git diff --cached --quiet; then
	head_version="$(read_head_crate_version)"
	if [[ "${head_version}" != "${semver}" ]]; then
		echo "error: version manifests are not staged at ${semver} (HEAD is ${head_version})" >&2
		exit 1
	fi
	echo "version manifests already at ${semver}; skipping commit"
else
	# sync-version already ran above; skip the hook so commit does not re-run it
	# and abort while the index is locked.
	SKIP=sync-version git commit -m "version bump to ${tag}"
	assert_version_files_committed
fi

git push origin HEAD

git tag -f "${tag}"
git push -f origin "${tag}"

go_tag="sdk/go/${tag}"
git tag -f "${go_tag}"
git push -f origin "${go_tag}"

notes="$(release_notes "${tag}")"
if gh release view "${tag}" >/dev/null 2>&1; then
	gh release delete "${tag}" -y
fi
gh release create "${tag}" \
	--title "${tag}" \
	--notes "${notes}" \
	--prerelease

cat <<EOF | shorten_paths
published ${tag}:
  branch: ${branch}
  commit: $(git rev-parse --short HEAD)
  go module tag: ${go_tag}
  release: https://github.com/$(gh repo view --json nameWithOwner --jq .nameWithOwner)/releases/tag/${tag}
EOF
