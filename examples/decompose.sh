#!/usr/bin/env bash
# cargo clean && cargo build --release --bin chunk-your-skills

target/release/chunk-your-skills decompose --skill examples/context7/original/SKILL.md --output examples/context7/decomposed/
