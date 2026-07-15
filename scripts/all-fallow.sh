#!/usr/bin/env bash
# Run fallow checks via sdk/Taskfile.yml (TypeScript + e2e TypeScript).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
export PATH="${ROOT}/node_modules/.bin:${PATH}"

cd "${ROOT}/sdk"
task all-fallow
