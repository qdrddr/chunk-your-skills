#!/usr/bin/env bash
# update pyproject.toml version first

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
export ROOT
version="$(
	grep -E '^version[[:space:]]*=' "${ROOT}/sdk/python/pyproject.toml" |
		head -1 |
		sed -E 's/^version[[:space:]]*=[[:space:]]*"(.*)".*/\1/'
)"
export version
export tag="v${version}"

oco -n
git checkout main
git pull origin main
git tag "${tag}"
git push origin "${tag}"

git tag "sdk/go/v${version}"
git push origin "sdk/go/v${version}"

cd sdk/python
# uv version 1.2.0
# Build for your current platform
uv run maturin build --release --out ../../dist
# Source distribution
uv build --sdist --out-dir ../../dist
# Publish (needs PyPI token or ~/.pypirc)
uv publish --publish-url https://upload.pypi.org/legacy/ ../../dist/*
