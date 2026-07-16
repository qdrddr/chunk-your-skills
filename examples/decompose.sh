#!/usr/bin/env bash
# Decompose SKILL.md into a page-indexed catalog.
#
# Usage:
#   ./examples/decompose.sh [--dev]
#
#   --dev  Build target/release/chunk-your-skills if missing (repo checkout).
#          Default: use chunk-your-skills from PATH (cargo install / crates.io).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=examples/_cli.sh
source "${SCRIPT_DIR}/_cli.sh"

USE_DEV=0
if [[ "${1:-}" == "--dev" ]]; then
	USE_DEV=1
	shift
fi
if [[ $# -gt 0 ]]; then
	echo "usage: $0 [--dev]" >&2
	exit 1
fi

cys_resolve_cli "$USE_DEV"

"${CHUNK_YOUR_SKILLS_CLI}" decompose \
	--skill examples/context7/original/SKILL.md \
	--output examples/context7/decomposed/
