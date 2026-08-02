#!/usr/bin/env bash
# Pre-commit wrapper for PSScriptAnalyzer format.
#
# Formats tracked PowerShell files and stages them so prek does not fail with
# "files were modified by this hook".

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PWSH="${PRE_COMMIT_PWSH_PATH:-pwsh}"

run_cmd() {
	if command -v rtk >/dev/null 2>&1; then
		rtk "$@"
	else
		"$@"
	fi
}

if (("$#" > 0)); then
	files=("$@")
else
	mapfile -t files < <(
		cd "${ROOT}"
		git ls-files -z -- '*.ps1' '*.psm1' '*.psd1' | tr '\0' '\n' | sed '/^$/d'
	)
fi

((${#files[@]})) || exit 0

command -v "${PWSH}" >/dev/null 2>&1 || {
	echo "error: ${PWSH} not found (install PowerShell)" >&2
	exit 1
}

for file in "${files[@]}"; do
	abs="${ROOT}/${file}"
	[[ -f "${abs}" ]] || continue
	"${PWSH}" -NoProfile -Command "
		if (-not (Get-InstalledModule PSScriptAnalyzer -ErrorAction SilentlyContinue)) {
			Install-Module -Name PSScriptAnalyzer -Scope CurrentUser -Force -AllowClobber
		}
		\$content = (Get-Content -Raw -LiteralPath '${abs}').Trim()
		Invoke-Formatter -ScriptDefinition \$content | Out-File -LiteralPath '${abs}'
	"
done

run_cmd git add -- "${files[@]}"
