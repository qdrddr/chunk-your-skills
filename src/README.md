# chunk-your-skills (Rust)

Rust library and CLI for SKILL.md pageindex and skinny-skill recomposition. Published on [crates.io](https://crates.io/crates/chunk-your-skills).

```bash
cargo build
cargo test
```

```toml
# Cargo.toml
[dependencies]
chunk-your-skills = "1"
```

```rust
use chunk_your_skills::{PageIndexConfig, build_skills_index};

let index = build_skills_index(&["/path/to/skills"], &PageIndexConfig::default())?;
println!("{:?}", index.documents.keys().collect::<Vec<_>>());
```

The crate root is `../Cargo.toml`; this `src/` tree is the `chunk_your_skills` library.
