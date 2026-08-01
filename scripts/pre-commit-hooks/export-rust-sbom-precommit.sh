#!/usr/bin/env bash
# Pre-commit wrapper for export-rust-sbom.sh.
#
# Regenerates SBOM artifacts and stages them so prek/pre-commit can include the
# updates in the same commit instead of failing with "files were modified".

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
CDX_FILE="${ROOT}/chunk-your-skills.cdx.json"
SNYK_FILE="${ROOT}/chunk-your-skills.snyk.json"

run_cmd() {
	if command -v rtk >/dev/null 2>&1; then
		rtk "$@"
	else
		"$@"
	fi
}

cd "${ROOT}"
bash "${ROOT}/scripts/deps/export-rust-sbom.sh"

for file in "${CDX_FILE}" "${SNYK_FILE}"; do
	[[ -f "${file}" ]] || {
		echo "error: missing ${file} after export-rust-sbom" >&2
		exit 1
	}
done

run_cmd git add -- "${CDX_FILE}" "${SNYK_FILE}"
