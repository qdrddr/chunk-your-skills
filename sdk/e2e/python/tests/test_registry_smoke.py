"""Smoke tests for chunk-your-skills installed from PyPI."""

from __future__ import annotations

import tempfile
from pathlib import Path

from chunk_your_skills import build_skills_index, count_tokens


def test_build_skills_index_from_registry() -> None:
    with tempfile.TemporaryDirectory() as tmp:
        skills_dir = Path(tmp) / "skills-src"
        skills_dir.mkdir()
        (skills_dir / "create-hook.md").write_text(
            "# Create Hook\n\nIntro\n\n## Usage\n\nRun the hook.\n",
            encoding="utf-8",
        )
        index = build_skills_index([str(skills_dir)])
        assert index["documents"]
        assert any(k.startswith("nodes/") and k.endswith(".md") for k in index["files"])


def test_count_tokens_smoke() -> None:
    assert count_tokens("hello world") >= 1
