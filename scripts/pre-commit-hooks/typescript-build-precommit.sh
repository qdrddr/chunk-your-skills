#!/usr/bin/env bash
# Pre-commit wrapper for the TypeScript SDK build.
#
# Ensures npm devDependencies (including @types/node) are present before tsc runs.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TS_DIR="${ROOT}/sdk/typescript"

run_npm() {
	if command -v rtk >/dev/null 2>&1; then
		rtk env -u npm_config_devdir -u NODE_ENV npm "$@"
	else
		env -u npm_config_devdir -u NODE_ENV npm "$@"
	fi
}

cd "${TS_DIR}"

if [[ ! -d node_modules/@types/node ]]; then
	run_npm ci
fi

run_npm run build
