"""Resolve CLI paths safely (path traversal guards for dev/audit scripts)."""

from __future__ import annotations

from pathlib import Path


def resolve_path(path: Path | str) -> Path:
    return Path(path).expanduser().resolve()


def resolve_under(base: Path, path: Path | str, *, label: str = "path") -> Path:
    base_resolved = resolve_path(base)
    resolved = resolve_path(path)
    if not resolved.is_relative_to(base_resolved):
        msg = f"{label} must stay within {base_resolved}: {resolved}"
        raise ValueError(msg)
    return resolved


def resolve_repo_root(path: Path | str) -> Path:
    root = resolve_path(path)
    markers = (
        root / "Cargo.toml",
        root / "deny.toml",
        root / "legal" / "policy.toml",
        root / "sdk" / "python" / "pyproject.toml",
    )
    if not all(marker.is_file() for marker in markers):
        msg = f"not a chunk-your-skills repository root: {root}"
        raise ValueError(msg)
    return root


def resolve_audit_path(path: Path | str, *, repo_root: Path) -> Path:
    resolved = resolve_path(path)
    cwd = Path.cwd().resolve()
    if resolved.is_relative_to(repo_root) or resolved.is_relative_to(cwd):
        return resolved
    msg = (
        f"path must stay within repository root ({repo_root}) "
        f"or current working directory ({cwd}): {resolved}"
    )
    raise ValueError(msg)


def resolve_output_path(path: Path | str, *, base: Path | None = None) -> Path:
    resolved = resolve_path(path)
    if base is not None:
        resolve_under(base, resolved, label="output path")
    return resolved


def open_output(path: Path | str, *, base: Path | None = None):
    output = resolve_output_path(path, base=base or Path.cwd())
    output.parent.mkdir(parents=True, exist_ok=True)
    return output.open("w", encoding="utf-8")
