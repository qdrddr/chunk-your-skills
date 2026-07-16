"""Skills pageindex (markdown tree indexing and retrieval)."""

from __future__ import annotations

from dataclasses import dataclass
from typing import Any

from chunk_your_skills._native import ReconstructOptions as _ReconstructOptions
from chunk_your_skills._native import SkillsBuilder as _SkillsBuilder
from chunk_your_skills._native import build_page_index_for_file as _build_page_index_for_file
from chunk_your_skills._native import build_page_index_only as _build_page_index_only
from chunk_your_skills._native import build_skills_index as _build_skills_index
from chunk_your_skills._native import finalize_skill_document_json as _finalize_skill_document_json
from chunk_your_skills._native import (
    get_skill_content_retrieve_result as _get_skill_content_retrieve_result,
)
from chunk_your_skills._native import get_skill_document as _get_skill_document
from chunk_your_skills._native import get_skill_line_content as _get_skill_line_content
from chunk_your_skills._native import (
    get_skill_line_content_from_spec as _get_skill_line_content_from_spec,
)
from chunk_your_skills._native import get_skill_structure as _get_skill_structure
from chunk_your_skills._native import load_merged_skill_document_json as _load_merged_skill_document_json
from chunk_your_skills._native import load_skills_index_from_dir as _load_skills_index_from_dir
from chunk_your_skills._native import load_skills_index_from_entry as _load_skills_index_from_entry
from chunk_your_skills._native import md_to_tree as _md_to_tree
from chunk_your_skills._native import page_index_valid as _page_index_valid
from chunk_your_skills._native import parse_skill_node_ids as _parse_skill_node_ids
from chunk_your_skills._native import reconstruct_skill_markdown as _reconstruct_skill_markdown
from chunk_your_skills._native import repair_skill_nodes as _repair_skill_nodes
from chunk_your_skills._native import (
    skills_index_from_decomposed_dir as _skills_index_from_decomposed_dir,
)
from chunk_your_skills._native import (
    token_count_from_decomposed_frontmatter as _token_count_from_decomposed_frontmatter,
)
from chunk_your_skills._native import parse_frontmatter_fields as _parse_frontmatter_fields
from chunk_your_skills._native import frontmatter_field as _frontmatter_field
from chunk_your_skills._native import (
    update_skill_document_source_path as _update_skill_document_source_path,
)
from chunk_your_skills._native import write_reconstructed_skill as _write_reconstructed_skill
from chunk_your_skills._native import write_skills_index as _write_skills_index


@dataclass
class ReconstructOptions:
    keep_all_headers: bool = False

    def to_native(self) -> _ReconstructOptions:
        return _ReconstructOptions(keep_all_headers=self.keep_all_headers)


@dataclass
class PageIndexConfig:
    if_add_node_id: bool = True
    if_add_node_text: bool = False

    def to_dict(self) -> dict[str, Any]:
        return {
            "if_add_node_id": self.if_add_node_id,
            "if_add_node_text": self.if_add_node_text,
        }

    @classmethod
    def from_mapping(cls, mapping: dict[str, Any] | None) -> PageIndexConfig:
        if not mapping:
            return default_page_index_config()
        cfg = default_page_index_config()
        if "if_add_node_id" in mapping:
            cfg.if_add_node_id = bool(mapping["if_add_node_id"])
        if "if_add_node_text" in mapping:
            cfg.if_add_node_text = bool(mapping["if_add_node_text"])
        return cfg


PageIndexConfigInput = PageIndexConfig | dict[str, Any]


def default_page_index_config() -> PageIndexConfig:
    return PageIndexConfig()


def page_index_config_from_mapping(mapping: dict[str, Any] | None = None) -> dict[str, Any]:
    """Partial pageindex settings from app/YAML; Rust merges unset keys with SDK defaults."""
    return PageIndexConfig.from_mapping(mapping).to_dict()


def build_skills_index(
    skill_dirs: list[str],
    *,
    config: PageIndexConfigInput | None = None,
) -> dict[str, Any]:
    cfg = _config_dict(config)
    return _build_skills_index(skill_dirs, cfg)


def write_skills_index(index: dict[str, Any], output_dir: str) -> None:
    _write_skills_index(index, output_dir)


def load_skills_index_from_dir(catalog_dir: str) -> dict[str, Any]:
    return _load_skills_index_from_dir(catalog_dir)


def skills_index_from_decomposed_dir(dir_path: str) -> dict[str, Any]:
    return _skills_index_from_decomposed_dir(dir_path)


def repair_skill_nodes(
    entry_dir: str,
    doc_id: str,
    *,
    config: PageIndexConfigInput | None = None,
) -> None:
    cfg = _config_dict(config)
    _repair_skill_nodes(entry_dir, doc_id, cfg)


def load_merged_skill_document_json(entry_dir: str, doc_id: str) -> dict[str, Any]:
    return _load_merged_skill_document_json(entry_dir, doc_id)


def build_page_index_for_file(
    source_path: str,
    *,
    config: PageIndexConfigInput | None = None,
) -> dict[str, Any]:
    cfg = _config_dict(config)
    return _build_page_index_for_file(source_path, cfg)


def build_page_index_only(
    skill_dirs: list[str],
    *,
    config: PageIndexConfigInput | None = None,
) -> dict[str, Any]:
    cfg = _config_dict(config)
    return _build_page_index_only(skill_dirs, cfg)


def page_index_valid(entry_dir: str, content_sha256: str) -> bool:
    return _page_index_valid(entry_dir, content_sha256)


def load_skills_index_from_entry(entry_dir: str, doc_id: str) -> dict[str, Any]:
    return _load_skills_index_from_entry(entry_dir, doc_id)


def finalize_skill_document_json(
    entry_dir: str,
    doc_id: str,
    *,
    pipeline: str,
    index_params: dict[str, Any],
    source_path: str,
) -> dict[str, Any]:
    return _finalize_skill_document_json(
        entry_dir,
        doc_id,
        pipeline=pipeline,
        index_params=index_params,
        source_path=source_path,
    )


def update_skill_document_source_path(
    entry_dir: str,
    doc_id: str,
    source_path: str,
) -> dict[str, Any]:
    return _update_skill_document_source_path(entry_dir, doc_id, source_path)


def md_to_tree(
    markdown_content: str,
    source_path: str,
    *,
    config: PageIndexConfigInput | None = None,
) -> dict[str, Any]:
    cfg = _config_dict(config)
    return _md_to_tree(markdown_content, source_path, cfg)


def get_skill_document(documents: dict[str, Any], doc_id: str) -> dict[str, Any]:
    return _get_skill_document(documents, doc_id)


def get_skill_structure(documents: dict[str, Any], doc_id: str) -> list[Any] | dict[str, Any]:
    return _get_skill_structure(documents, doc_id)


def get_skill_line_content_from_spec(
    index: dict[str, Any],
    doc_id: str,
    line_num_spec: str,
) -> list[dict[str, Any]]:
    return _get_skill_line_content_from_spec(index, doc_id, line_num_spec)


def get_skill_line_content(
    index: dict[str, Any],
    doc_id: str,
    *,
    line_num_specs: list[str] | None = None,
    node_id_specs: list[str] | None = None,
) -> list[dict[str, Any]]:
    return _get_skill_line_content(
        index,
        doc_id,
        line_num_specs=line_num_specs,
        node_id_specs=node_id_specs,
    )


def get_skill_content_retrieve_result(
    index: dict[str, Any],
    doc_id: str,
    *,
    line_num_specs: list[str] | None = None,
    node_id_specs: list[str] | None = None,
    options: ReconstructOptions | dict[str, Any] | None = None,
) -> dict[str, Any]:
    return _get_skill_content_retrieve_result(
        index,
        doc_id,
        line_num_specs=line_num_specs,
        node_id_specs=node_id_specs,
        options=_reconstruct_options_native(options),
    )


def reconstruct_skill_markdown(
    index: dict[str, Any],
    doc_id: str,
    *,
    line_num_specs: list[str] | None = None,
    node_id_specs: list[str] | None = None,
    options: ReconstructOptions | dict[str, Any] | None = None,
) -> dict[str, Any]:
    return _reconstruct_skill_markdown(
        index,
        doc_id,
        line_num_specs=line_num_specs,
        node_id_specs=node_id_specs,
        options=_reconstruct_options_native(options),
    )


def write_reconstructed_skill(
    catalog_dir: str,
    index: dict[str, Any],
    doc_id: str,
    *,
    line_num_specs: list[str] | None = None,
    node_id_specs: list[str] | None = None,
    options: ReconstructOptions | dict[str, Any] | None = None,
) -> str:
    return _write_reconstructed_skill(
        catalog_dir,
        index,
        doc_id,
        line_num_specs=line_num_specs,
        node_id_specs=node_id_specs,
        options=_reconstruct_options_native(options),
    )


def parse_skill_node_ids(spec: str) -> list[int]:
    return _parse_skill_node_ids(spec)


def token_count_from_decomposed_frontmatter(content: str) -> int | None:
    """Parse ``token_count`` from decomposed markdown/JSON frontmatter when present."""
    return _token_count_from_decomposed_frontmatter(content)


def parse_frontmatter_fields(content: str) -> list[dict[str, Any]] | None:
    """Parse root-level YAML frontmatter keys into semantic values."""
    return _parse_frontmatter_fields(content)


def frontmatter_field(content: str, key: str) -> Any:
    """Look up one semantically parsed frontmatter field by name."""
    return _frontmatter_field(content, key)


class SkillsBuilder:
    def __init__(self, *, memory_only: bool = True, output_dir: str | None = None) -> None:
        self._inner = _SkillsBuilder(memory_only=memory_only, output_dir=output_dir)

    def build_from_dirs(
        self,
        skill_dirs: list[str],
        *,
        config: PageIndexConfigInput | None = None,
    ) -> dict[str, Any]:
        cfg = _config_dict(config)
        return self._inner.build_from_dirs(skill_dirs, cfg)

    def build_from_file(
        self,
        source_path: str,
        *,
        config: PageIndexConfigInput | None = None,
    ) -> dict[str, Any]:
        cfg = _config_dict(config)
        return self._inner.build_from_file(source_path, cfg)

    def write_catalog(self) -> dict[str, Any]:
        return self._inner.write_catalog()

    def to_skills_index_json(self) -> dict[str, Any]:
        return self._inner.to_skills_index_json()

    def to_skills_dict(self) -> dict[str, Any]:
        return self._inner.to_skills_dict()


def _config_dict(config: PageIndexConfigInput | None) -> dict[str, Any] | None:
    if config is None:
        return None
    if isinstance(config, PageIndexConfig):
        return config.to_dict()
    if isinstance(config, dict) and _is_snake_case_pageindex_dict(config):
        return config
    if isinstance(config, dict):
        return PageIndexConfig.from_mapping(config).to_dict()
    return config


def _is_snake_case_pageindex_dict(config: dict[str, Any]) -> bool:
    return any(key in config for key in ("if_add_node_id", "if_add_node_text"))


def _reconstruct_options_native(
    options: ReconstructOptions | dict[str, Any] | None,
) -> _ReconstructOptions | None:
    if options is None:
        return None
    if isinstance(options, ReconstructOptions):
        return options.to_native()
    return _ReconstructOptions(
        keep_all_headers=bool(options.get("keep_all_headers", options.get("keepAllHeaders", False))),
    )
