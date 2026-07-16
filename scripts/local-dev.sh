#!/usr/bin/env bash
# Local monorepo workflow: Rust core → SDK artifacts (no app layer).
#
# Usage:
#   ./scripts/local-dev.sh [--short|--silent] <command> [args...]
#
# Commands:
#   core-rust | rust     cargo test -p chunk-your-skills (+ ffi smoke)
#   sdk-python           maturin develop --release + verify sdk/python
#   sdk-verify           verify sdk/python install + native import
#   sdk-typescript       npm ci, build, test (sdk/typescript)
#   sdk-c                cmake build + ctest (sdk/c)
#   sdk-go               build C FFI + go test (sdk/go)
#   sdk-all              all SDK targets above
#   ci                   rust + sdk smoke checks
#   all                  core-rust → all SDKs
#
# Examples:
#   ./scripts/local-dev.sh all
#   ./scripts/local-dev.sh --silent sdk-go
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# shellcheck disable=SC1091
source "${SCRIPT_DIR}/local-dev-lib.sh"
# shellcheck disable=SC1091
source "${SCRIPT_DIR}/shorten-paths.sh"
export SHORTEN_ROOT="${CYS_REPO_ROOT}"

CYS_LOCAL_DEV_SHORT="${CYS_LOCAL_DEV_SHORT:-}"
LOCAL_DEV_ARGS=()
while (($#)); do
	case "$1" in
	--short | --silent)
		CYS_LOCAL_DEV_SHORT=1
		shift
		;;
	*)
		LOCAL_DEV_ARGS+=("$1")
		shift
		;;
	esac
done
export CYS_LOCAL_DEV_SHORT

usage() {
	sed -n '2,22p' "$0" | sed 's/^# \{0,1\}//'
}

_cys_local_dev_main() {
	local cmd="${1:-}"
	shift || true

	case "${cmd}" in
	core-rust | rust)
		require_repo_root
		cys_build_rust
		;;
	sdk-python)
		require_repo_root
		cys_build_sdk_python
		cys_verify_sdk_python
		;;
	sdk-verify)
		require_repo_root
		cys_verify_sdk_python
		;;
	sdk-typescript)
		require_repo_root
		cys_build_sdk_typescript
		;;
	sdk-c)
		require_repo_root
		cys_build_sdk_c
		;;
	sdk-go)
		require_repo_root
		cys_build_sdk_go
		;;
	sdk-all)
		require_repo_root
		cys_section "SDK: Python"
		cys_build_sdk_python
		cys_verify_sdk_python
		cys_section "SDK: C"
		cys_build_sdk_c
		cys_section "SDK: Go"
		cys_build_sdk_go
		cys_section "SDK: TypeScript"
		cys_build_sdk_typescript
		;;
	all)
		require_repo_root
		cys_run_all
		info "all done"
		;;
	ci)
		require_repo_root
		cys_section "CI"
		cys_build_rust
		cys_build_sdk_python
		cys_verify_sdk_python
		cys_test_sdk_python
		;;
	"" | -h | --help | help)
		usage
		;;
	*)
		if [[ -n "${CYS_LOCAL_DEV_SHORT:-}" ]]; then
			die "unknown command: ${cmd}"
		fi
		echo "unknown command: ${cmd}" >&2
		echo >&2
		usage >&2
		return 1
		;;
	esac
}

if [[ -n "${CYS_LOCAL_DEV_SHORT}" ]]; then
	_cys_local_dev_main "${LOCAL_DEV_ARGS[@]}" 2>&1 | shorten_paths | cys_filter_short_logs
else
	_cys_local_dev_main "${LOCAL_DEV_ARGS[@]}" 2>&1 | shorten_paths
fi
exit "${PIPESTATUS[0]}"
