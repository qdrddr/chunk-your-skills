#!/usr/bin/env bash
# shellcheck shell=bash
# Resolve chunk-your-skills CLI for example scripts (not executed directly).

if [[ -z "${CYS_EXAMPLES_CLI_SOURCED:-}" ]]; then
	CYS_EXAMPLES_CLI_SOURCED=1

	CYS_EXAMPLES_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
	CYS_REPO_ROOT="$(cd "${CYS_EXAMPLES_DIR}/.." && pwd)"

	cys_resolve_cli() {
		local use_dev="${1:-0}"
		if [[ "$use_dev" == "--dev" || "$use_dev" == "1" ]]; then
			local cli
			cli="${CYS_REPO_ROOT}/target/release/chunk-your-skills"
			if [[ ! -x "$cli" ]]; then
				echo "==> building ${cli} (missing)" >&2
				(cd "$CYS_REPO_ROOT" && cargo build -p chunk-your-skills --release)
			fi
			[[ -x "$cli" ]] || {
				echo "error: failed to build ${cli}" >&2
				return 1
			}
			export CHUNK_YOUR_SKILLS_CLI="$cli"
		else
			if ! command -v chunk-your-skills >/dev/null 2>&1; then
				echo "error: chunk-your-skills not found on PATH; pass --dev to build from this repo" >&2
				return 1
			fi
			export CHUNK_YOUR_SKILLS_CLI="chunk-your-skills"
		fi
	}
fi
