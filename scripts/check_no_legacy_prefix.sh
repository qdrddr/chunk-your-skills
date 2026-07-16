#!/usr/bin/env bash
# Fail if legacy three-letter project prefixes remain in tracked source.
set -euo pipefail

root="$(git rev-parse --show-toplevel)"
cd "$root"

python3 - <<'PY'
from __future__ import annotations

import pathlib
import re
import sys

root = pathlib.Path(".")
skip_dirs = {"target", "node_modules", ".git", ".venv", "dist", ".gitnexus", "graphify-out", ".code-review-graph", ".codebase-memory", ".cursor", ".debug"}
skip_files = {pathlib.Path("scripts/check_no_legacy_prefix.sh")}

c, y, t = (chr(99), chr(121), chr(116))
C, Y, T = (chr(67), chr(89), chr(84))
legacy_word = re.compile(rf"\b{c}{y}{t}\b")
legacy_fn = re.compile(rf"{c}{y}{t}_")
legacy_macro = re.compile(rf"{C}{Y}{T}_")
legacy_camel = re.compile(rf"C{y}{t}[A-Z]")

patterns = [
    (legacy_word, "word"),
    (legacy_fn, "fn prefix"),
    (legacy_macro, "macro prefix"),
    (legacy_camel, "CamelCase type"),
]

violations: list[str] = []
for path in sorted(root.rglob("*")):
    if not path.is_file():
        continue
    if path in skip_files:
        continue
    if any(part in skip_dirs for part in path.parts):
        continue
    try:
        text = path.read_text(encoding="utf-8")
    except (UnicodeDecodeError, OSError):
        continue
    for pattern, label in patterns:
        for match in pattern.finditer(text):
            line = text.count("\n", 0, match.start()) + 1
            violations.append(f"{path.relative_to(root)}:{line}: legacy {label}")

if violations:
    print("ERROR: legacy naming still present:", file=sys.stderr)
    for line in violations:
        print(line, file=sys.stderr)
    sys.exit(1)

print("check_no_legacy_prefix: OK")
PY
