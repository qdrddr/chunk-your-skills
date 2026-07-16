#!/usr/bin/env bash
# Render gitignored manifests from .in templates using CHUNK_YOUR_SKILLS_RELEASE_VERSION.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERSION="${CHUNK_YOUR_SKILLS_RELEASE_VERSION:-}"

if [[ -z "$VERSION" ]]; then
	if [[ -n "${TAG:-}" ]]; then
		# shellcheck source=parse-version.sh
		eval "$("${ROOT}/scripts/parse-version.sh")"
	else
		echo "CHUNK_YOUR_SKILLS_RELEASE_VERSION or TAG must be set" >&2
		exit 1
	fi
fi

render() {
	local src="$1"
	local dst="$2"
	mkdir -p "$(dirname "$dst")"
	sed "s/@CHUNK_YOUR_SKILLS_RELEASE_VERSION@/${VERSION}/g" "$src" >"$dst"
	echo "rendered ${dst}"
}

render_rust_cargo() {
	local dst="${ROOT}/rust/Cargo.toml"
	if [[ "${CHUNK_YOUR_SKILLS_E2E_USE_WORKSPACE:-}" == "1" ]]; then
		cat >"$dst" <<'EOF'
[package]
name = "chunk-your-skills-registry-e2e"
version = "0.0.0"
edition = "2024"
publish = false

[dependencies]
chunk-your-skills = { path = "../../.." }
serde_json = "1"
EOF
		echo "rendered ${dst} (workspace path=../../..)"
		return 0
	fi
	render "${ROOT}/rust/Cargo.toml.in" "$dst"
}

render_python_pyproject() {
	local dst="${ROOT}/python/pyproject.toml"
	if [[ "${CHUNK_YOUR_SKILLS_E2E_USE_WORKSPACE:-}" == "1" ]]; then
		cat >"$dst" <<'EOF'
[project]
name = "chunk-your-skills-registry-e2e"
version = "0.0.0"
requires-python = ">=3.13,<4.0"
dependencies = ["chunk-your-skills"]

[dependency-groups]
test = ["pytest>=8.0"]

[tool.uv.sources]
chunk-your-skills = { path = "../../python", editable = true }

[tool.pytest.ini_options]
testpaths = ["tests"]
EOF
		echo "rendered ${dst} (workspace path=../../python)"
		return 0
	fi
	render "${ROOT}/python/pyproject.toml.in" "$dst"
}

render_typescript_package() {
	local dst="${ROOT}/typescript/package.json"
	if [[ "${CHUNK_YOUR_SKILLS_E2E_USE_WORKSPACE:-}" == "1" ]]; then
		cat >"$dst" <<'EOF'
{
  "name": "chunk-your-skills-registry-e2e",
  "private": true,
  "type": "module",
  "scripts": {
    "test": "node test/run.mjs"
  },
  "devDependencies": {
    "chunk-your-skills": "file:../../typescript"
  }
}
EOF
		echo "rendered ${dst} (workspace file:../../typescript)"
		return 0
	fi
	render "${ROOT}/typescript/package.json.in" "$dst"
}

go_module_path() {
	local ver="$1"
	local staging="${CHUNK_YOUR_SKILLS_E2E_STAGING:-}"
	if [[ -n "$staging" && -f "${staging}/sdk/go/go.mod" ]]; then
		awk '/^module / { print $2; exit }' "${staging}/sdk/go/go.mod"
		return 0
	fi
	local major="${ver%%.*}"
	local base="github.com/qdrddr/chunk-your-skills/sdk/go"
	if [[ "$major" -ge 2 ]]; then
		echo "${base}/v${major}"
	else
		echo "$base"
	fi
}

go_require_version() {
	local ver="$1"
	local module_path="$2"
	local major="${ver%%.*}"
	if [[ "$major" -ge 2 && "$module_path" != */v* ]]; then
		echo "v${ver}+incompatible"
	else
		echo "v${ver}"
	fi
}

render_go_mod() {
	local src="$1"
	local dst="$2"
	local staging="${CHUNK_YOUR_SKILLS_E2E_STAGING:-${TMPDIR:-/tmp}/chunk-your-skills-e2e-${VERSION}}"
	local module_path require_version
	module_path="$(go_module_path "$VERSION")"
	require_version="$(go_require_version "$VERSION" "$module_path")"
	sed -e "s/@CHUNK_YOUR_SKILLS_RELEASE_VERSION@/${VERSION}/g" \
		-e "s|@CHUNK_YOUR_SKILLS_E2E_STAGING@|${staging}|g" \
		-e "s|@CHUNK_YOUR_SKILLS_GO_MODULE_PATH@|${module_path}|g" \
		-e "s/@CHUNK_YOUR_SKILLS_GO_REQUIRE_VERSION@/${require_version}/g" \
		"$src" >"$dst"
	echo "rendered ${dst} (staging=${staging}, module=${module_path}, require=${require_version})"
}

render_go_test() {
	local src="${ROOT}/go/registry_smoke_test.go.in"
	local dst="${ROOT}/go/registry_smoke_test.go"
	local staging="${CHUNK_YOUR_SKILLS_E2E_STAGING:-${TMPDIR:-/tmp}/chunk-your-skills-e2e-${VERSION}}"
	local module_path
	module_path="$(go_module_path "$VERSION")"
	sed -e "s|@CHUNK_YOUR_SKILLS_GO_MODULE_PATH@|${module_path}|g" \
		"$src" >"$dst"
	echo "rendered ${dst} (module=${module_path})"
}

render_rust_cargo
render_python_pyproject
render_typescript_package
render_go_mod "${ROOT}/go/go.mod.in" "${ROOT}/go/go.mod"
render_go_test
