"""Library version (Rust core)."""

from __future__ import annotations

from chunk_your_skills._native import get_version as _get_version

__all__ = ["get_version"]


def get_version() -> str:
    """Return the chunk-your-skills library version string."""
    return str(_get_version())
