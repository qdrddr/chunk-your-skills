# chunk-your-skills (Python)

Python bindings for the [chunk-your-skills](https://crates.io/crates/chunk-your-skills) Rust crate —
SKILL.md pageindex and skinny-skill recomposition.

```bash
cd sdk/python
uv sync
uv run maturin develop --release
```

```python
from chunk_your_skills import PageIndexConfig, build_skills_index

index = build_skills_index(["/path/to/skills"], PageIndexConfig())
print(index["documents"])
```

The native extension is built from the root crate via `manifest-path = "../../Cargo.toml"` in `pyproject.toml`.
