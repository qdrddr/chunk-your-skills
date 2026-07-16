#!/usr/bin/env bash
# Retry uv sync until PyPI (or other indexes) have propagated the release.
# Usage: ./uv-sync-with-retry.sh [--group test] [other uv sync args...]
#
# Optional env:
#   UV_SYNC_REGISTRY_TARGET  wait-registry target (pypi-sdk, pypi-app-chain, ...)
#   UV_SYNC_WAIT_EVERY       run wait-registry every N failed attempts (default: 10)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

MAX_ATTEMPTS="${UV_SYNC_MAX_ATTEMPTS:-12}"
SLEEP_SECS="${UV_SYNC_RETRY_SECS:-30}"
WAIT_EVERY="${UV_SYNC_WAIT_EVERY:-10}"

maybe_wait_registry() {
	local target="${UV_SYNC_REGISTRY_TARGET:-}"
	[[ -n "$target" ]] || return 0
	echo "Re-checking registry (${target}) before retry..."
	WAIT_REGISTRY_MAX_ATTEMPTS=1 "${SCRIPT_DIR}/wait-registry.sh" "$target" || true
}

attempt=1
while [[ "$attempt" -le "$MAX_ATTEMPTS" ]]; do
	if uv sync "$@"; then
		echo "uv sync succeeded (attempt ${attempt})"
		exit 0
	fi
	if [[ "$attempt" -eq "$MAX_ATTEMPTS" ]]; then
		echo "::error::uv sync failed after ${MAX_ATTEMPTS} attempts" >&2
		exit 1
	fi
	if [[ -n "${UV_SYNC_REGISTRY_TARGET:-}" && $((attempt % WAIT_EVERY)) -eq 0 ]]; then
		maybe_wait_registry
	fi
	echo "uv sync failed (attempt ${attempt}/${MAX_ATTEMPTS}); retrying in ${SLEEP_SECS}s..."
	sleep "$SLEEP_SECS"
	attempt=$((attempt + 1))
done
