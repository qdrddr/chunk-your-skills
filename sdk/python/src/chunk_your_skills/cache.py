"""Rust-backed disk/memory cache for skills registries."""

from __future__ import annotations

from typing import Any

import chunk_your_skills._native as _native

CachePolicy = str  # "auto" | "force_memory" | "force_disk"


def ensure_skills_registry(
    source_paths: list[str] | list[dict[str, str]],
    catalog_root: str,
    pageindex_config: dict[str, Any] | None,
    pipeline: str,
    index_params_hash: str,
    *,
    policy: CachePolicy = "auto",
) -> list[dict[str, Any]]:
    """Ensure page index (+ BM25 chunks when pipeline is bm25) for skill sources.

    Each source may be a filesystem path string or a dict with
    ``path``, ``content``, and optional ``content_sha256`` for in-memory client skills.
    """
    refs = _native.ensure_skills_registry(
        source_paths,
        catalog_root,
        pageindex_config,
        pipeline,
        index_params_hash,
        policy,
    )
    return [dict(ref) for ref in refs]


def configure_memory_cache(config: dict[str, Any]) -> None:
    """Apply in-memory cache tuning (lazy registry, LRU sizes, async disk writes)."""
    _native.configure_memory_cache(config)
