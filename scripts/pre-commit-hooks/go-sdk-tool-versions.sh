#!/usr/bin/env bash
# Pinned Go dev-tool installs for sdk/go pre-commit (not part of the published module).
# Bump package@version entries here when upgrading linters; pre-commit reinstalls when this file changes.

go_sdk_tool_packages() {
	local -n _packages="$1"

	_packages=(
		'mvdan.cc/gofumpt@v0.11.0'
		'golang.org/x/tools/cmd/goimports@v0.48.0'
		'honnef.co/go/tools/cmd/staticcheck@v0.7.0'
		'github.com/go-critic/go-critic/cmd/gocritic@v0.14.4'
		'github.com/securego/gosec/v2/cmd/gosec@v2.28.0'
	)
}
