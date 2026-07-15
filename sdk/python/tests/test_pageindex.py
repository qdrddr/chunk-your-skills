"""Tests for skills pageindex bindings."""

from __future__ import annotations

import tempfile
from pathlib import Path

import pytest

chunk_your_skills = pytest.importorskip("chunk_your_skills")

from chunk_your_skills import (  # noqa: E402
    PageIndexConfig,
    SkillsBuilder,
    build_skills_index,
    default_page_index_config,
    get_skill_document,
    get_skill_line_content,
    get_skill_line_content_from_spec,
    get_skill_structure,
    load_skills_index_from_dir,
    md_to_tree,
    page_index_config_from_mapping,
    repair_skill_nodes,
    token_count_from_decomposed_frontmatter,
    write_skills_index,
)


def test_md_to_tree_in_memory() -> None:
    md = "# Title\n\nBody\n\n## Sub\n\nMore"
    result = md_to_tree(md, "skill.md", config=PageIndexConfig())
    assert result["doc_name"] == "skill"
    assert result["line_count"] >= 1
    assert isinstance(result["structure"], list)


def test_default_page_index_config() -> None:
    cfg = default_page_index_config()
    d = cfg.to_dict()
    assert d["if_add_node_id"] is True
    assert d["if_add_node_text"] is False


def test_page_index_config_from_mapping_partial() -> None:
    cfg = page_index_config_from_mapping({"if_add_node_text": True})
    assert cfg["if_add_node_text"] is True
    assert cfg["if_add_node_id"] is True


def test_build_write_reconstruct() -> None:
    with tempfile.TemporaryDirectory() as tmp:
        skills_dir = Path(tmp) / "skills"
        skills_dir.mkdir()
        (skills_dir / "demo.md").write_text("# Demo\n\nHello\n\n## Part\n\nWorld", encoding="utf-8")

        index = build_skills_index([str(skills_dir)])
        assert "documents" in index
        assert index["documents"]
        assert any(k.startswith("nodes/") for k in index["files"])

        catalog = Path(tmp) / "catalog"
        write_skills_index(index, str(catalog))
        doc_id = next(iter(index["documents"]))
        assert (catalog / "nodes" / "page_index.json").is_file()

        rebuilt = load_skills_index_from_dir(str(catalog))
        meta = get_skill_document(rebuilt["documents"], doc_id)
        assert meta["type"] == "md"
        structure = get_skill_structure(rebuilt["documents"], doc_id)
        assert structure
        content = get_skill_line_content_from_spec(rebuilt, doc_id, "1")
        assert content


def test_retrieve_by_node_id_after_disk_roundtrip() -> None:
    with tempfile.TemporaryDirectory() as tmp:
        skills_dir = Path(tmp) / "skills"
        skills_dir.mkdir()
        (skills_dir / "demo.md").write_text(
            "Short.\n\n# Demo\n\n## Part\n\nLonger section with more words here.",
            encoding="utf-8",
        )
        index = build_skills_index([str(skills_dir)])
        doc_id = next(iter(index["documents"]))

        catalog = Path(tmp) / "catalog"
        write_skills_index(index, str(catalog))
        rebuilt = load_skills_index_from_dir(str(catalog))
        rows = get_skill_line_content(rebuilt, doc_id, node_id_specs=["1"])
        assert rows
        assert rows[0]["node_id"] == 1
        assert rows[0]["content"] == "Short."


def test_repair_skill_nodes() -> None:
    with tempfile.TemporaryDirectory() as tmp:
        skills_dir = Path(tmp) / "skills"
        skills_dir.mkdir()
        (skills_dir / "demo.md").write_text("# Demo\n\nHello", encoding="utf-8")

        index = build_skills_index([str(skills_dir)])
        catalog = Path(tmp) / "catalog"
        write_skills_index(index, str(catalog))
        doc_id = next(iter(index["documents"]))

        repair_skill_nodes(str(catalog), doc_id)
        rebuilt = load_skills_index_from_dir(str(catalog))
        assert rebuilt["documents"]


def test_token_count_from_decomposed_frontmatter() -> None:
    content = "---\ndoc_id: d1\nnode_id: 2\ntoken_count: 42\n---\n## Body\n"
    assert token_count_from_decomposed_frontmatter(content) == 42
    assert token_count_from_decomposed_frontmatter("no frontmatter") is None


def test_build_skills_index_node_files_include_token_count() -> None:
    with tempfile.TemporaryDirectory() as tmp:
        skills_dir = Path(tmp) / "skills"
        skills_dir.mkdir()
        (skills_dir / "demo.md").write_text("# Demo\n\nHello\n\n## Part\n\nWorld", encoding="utf-8")

        index = build_skills_index([str(skills_dir)])
        assert any(
            "token_count:" in content
            for rel, content in index["files"].items()
            if rel.startswith("nodes/") and rel.endswith(".md")
        )

        doc_id = next(iter(index["documents"]))
        rows = get_skill_line_content(index, doc_id, node_id_specs=["1"])
        if rows:
            assert rows[0].get("token_count", 0) > 0


def test_get_version_matches_package() -> None:
    from chunk_your_skills import get_version

    assert get_version()


def test_skills_builder_memory_only() -> None:
    with tempfile.TemporaryDirectory() as tmp:
        skills_dir = Path(tmp) / "skills"
        skills_dir.mkdir()
        (skills_dir / "x.md").write_text("# X\n\nY", encoding="utf-8")

        builder = SkillsBuilder(memory_only=True)
        index = builder.build_from_dirs([str(skills_dir)])
        assert index["files"]
