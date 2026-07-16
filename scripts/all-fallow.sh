#!/usr/bin/env bash
# Run fallow checks via sdk/Taskfile.yml (TypeScript + e2e TypeScript).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
export PATH="${ROOT}/node_modules/.bin:${PATH}"

E2E_TS="${ROOT}/sdk/e2e/typescript"
if [[ ! -d "${E2E_TS}/node_modules" ]]; then
	if [[ ! -f "${E2E_TS}/package.json" ]]; then
		if [[ -z "${CHUNK_YOUR_SKILLS_RELEASE_VERSION:-}" ]]; then
			CHUNK_YOUR_SKILLS_RELEASE_VERSION="$(
				awk -F'"' '/^version = / { print $2; exit }' "${ROOT}/Cargo.toml"
			)"
			export CHUNK_YOUR_SKILLS_RELEASE_VERSION
		fi
		export CHUNK_YOUR_SKILLS_E2E_USE_WORKSPACE=1
		"${ROOT}/sdk/e2e/scripts/render-manifests.sh"
	fi
	(
		cd "${E2E_TS}"
		npm install
	)
fi

cd "${ROOT}/sdk"
task all-fallow
