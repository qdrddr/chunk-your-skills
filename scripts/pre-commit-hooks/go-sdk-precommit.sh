#!/usr/bin/env bash
# Run Go SDK pre-commit tools scoped to sdk/go.
#
# Pre-commit / prek-loop hooks (see .pre-commit-config.yaml, prek-hook-groups.yaml go):
#   fumpt      -> go-fumpt
#   imports    -> go-imports
#   tidy       -> go-mod-tidy-repo
#   vet        -> go-vet-repo-mod
#   staticcheck-> go-staticcheck-repo-mod
#   critic     -> go-critic
#   sec        -> go-sec-repo-mod
#   build      -> go-build-repo-mod
#   test       -> go-test-repo-mod
#   lint       -> vet + staticcheck + critic + sec (local convenience)
#   format     -> fumpt + imports on all sdk/go/*.go (local convenience)
set -euo pipefail

ROOT="$(cd "$(git rev-parse --show-toplevel 2>/dev/null || pwd)" && pwd -P)"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GO_DIR="${ROOT}/sdk/go"
DEV_BIN="${GO_DIR}/.dev-bin"
VERSIONS_FILE="${SCRIPT_DIR}/go-sdk-tool-versions.sh"

# shellcheck source=scripts/pre-commit-hooks/go-sdk-tool-versions.sh
source "${VERSIONS_FILE}"

GO_SDK_TOOL_PACKAGES=()
go_sdk_tool_packages GO_SDK_TOOL_PACKAGES

cd "$GO_DIR"
export CGO_ENABLED=1
host_triplet="$(rustc -vV | sed -n 's/^host: //p')"
export PATH="${ROOT}/target/${host_triplet}/release:${PATH}"

ensure_go_sdk_tools() {
	mkdir -p "${DEV_BIN}"
	local stamp="${DEV_BIN}/.install-stamp"
	if [[ ! -f "${stamp}" || "${VERSIONS_FILE}" -nt "${stamp}" ]]; then
		local pkg
		for pkg in "${GO_SDK_TOOL_PACKAGES[@]}"; do
			GOBIN="${DEV_BIN}" go install "${pkg}"
		done
		touch "${stamp}"
	fi
	export PATH="${DEV_BIN}:${PATH}"
}

rel_paths() {
	local out=()
	local f
	for f in "$@"; do
		out+=("${f#sdk/go/}")
	done
	printf '%s\n' "${out[@]}"
}

all_go_sources() {
	find . -name '*.go' -not -path './.dev-bin/*' | sort
}

tool=${1:?usage: go-sdk-precommit.sh TOOL [args...]}
shift

case "$tool" in
fumpt | imports | format | staticcheck | critic | sec | lint)
	ensure_go_sdk_tools
	;;
esac

case "$tool" in
fumpt)
	mapfile -t files < <(rel_paths "$@")
	if ((${#files[@]} == 0)); then
		mapfile -t files < <(all_go_sources)
	fi
	if ((${#files[@]})); then
		gofumpt -l -w "${files[@]}"
	fi
	;;
imports)
	mapfile -t files < <(rel_paths "$@")
	if ((${#files[@]} == 0)); then
		mapfile -t files < <(all_go_sources)
	fi
	if ((${#files[@]})); then
		goimports -w "${files[@]}"
	fi
	;;
format)
	mapfile -t files < <(all_go_sources)
	if ((${#files[@]})); then
		gofumpt -l -w "${files[@]}"
		goimports -w "${files[@]}"
	fi
	;;
tidy)
	go mod tidy
	;;
vet)
	go vet ./...
	;;
staticcheck)
	staticcheck ./...
	;;
critic)
	gocritic check ./...
	;;
sec)
	gosec ./...
	;;
lint)
	go vet ./...
	staticcheck ./...
	gocritic check ./...
	gosec ./...
	;;
build)
	go build ./...
	;;
test)
	env -u CARGO_TARGET_DIR "${ROOT}/sdk/c/scripts/build-c-lib.sh" --no-sync-header
	go run ./cmd/chunk-native-ensure -static-only
	env -u CARGO_TARGET_DIR go test ./...
	;;
*)
	echo "unknown tool: $tool" >&2
	exit 1
	;;
esac
