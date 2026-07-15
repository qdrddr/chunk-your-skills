#!/usr/bin/env bash
# Sparse-clone chunk-your-skills at tag vX.Y.Z and render go.mod with a replace directive.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
VERSION="${CYT_RELEASE_VERSION:-1.1.0}"
TAG="v${VERSION}"
REPO="${CYT_GIT_REPO:-https://github.com/qdrddr/chunk-your-skills.git}"
STAGING="${CYT_GIT_STAGING:-${ROOT}/.staging/${VERSION}}"

if [[ ! -f "${STAGING}/sdk/go/go.mod" ]]; then
	echo "Fetching ${TAG} into ${STAGING}..." >&2
	rm -rf "$STAGING"
	git clone --depth 1 --branch "$TAG" --filter=blob:none --sparse "$REPO" "$STAGING"
	(
		cd "$STAGING"
		git sparse-checkout init --no-cone
		git sparse-checkout set \
			Cargo.toml Cargo.lock build.rs cbindgen.toml chunk_your_skills.h \
			src tests sdk/c sdk/go
	)
fi

sed "s|@CYT_GIT_STAGING@|${STAGING}|g" "${ROOT}/go.mod.in" >"${ROOT}/go.mod"

MONOREPO_ROOT="$(cd "${ROOT}/../.." && pwd)"
if [[ -f "${MONOREPO_ROOT}/sdk/go/go.mod" && -f "${MONOREPO_ROOT}/Cargo.toml" ]]; then
	rsync -a "${MONOREPO_ROOT}/sdk/go/cmd/chunk-native-ensure/" "${STAGING}/sdk/go/cmd/chunk-native-ensure/"
	echo "Overlaid monorepo chunk-native-ensure onto ${TAG} checkout" >&2
fi

echo "Prepared staging=${STAGING}" >&2
printf '%s\n' "$STAGING"
