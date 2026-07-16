#!/usr/bin/env bash
# Recompose skinny SKILL.md files from a decomposed catalog.
#
# Run decompose first (creates the catalog under examples/context7/decomposed/):
#   ./examples/decompose.sh
#
# Node map (examples/context7/decomposed/nodes/page_index.json):
#   0 frontmatter   1 preamble        2 When to Use This Skill
#   3 How to Fetch Documentation (parent section)
#   4 Step 1: Resolve   5 Step 2: Select   6 Step 3: Fetch   7 Step 4: Use
#   8 Guidelines
#
# Node IDs are numeric (matching n0.md → 0, n4.md → 4). Parent headings are
# included automatically when you select a child node.

set -euo pipefail

CATALOG=examples/context7/decomposed
INPUT_SKILL_PATH=examples/context7/original/SKILL.md

# Full documentation-fetch workflow (nodes 4–7 under the parent section).
chunk-your-skills recompose \
	--catalog "$CATALOG" \
	--path "$INPUT_SKILL_PATH" \
	--node-id 4-7 \
	--output examples/context7/skinny-skill/fetch-workflow/SKILL.md

# Cherry-pick two steps; parent "## How to Fetch Documentation" is kept.
chunk-your-skills recompose \
	--catalog "$CATALOG" \
	--path "$INPUT_SKILL_PATH" \
	--node-id 4,6 \
	--output examples/context7/skinny-skill/steps-1-and-3/SKILL.md

# Activation-only skinny skill: preamble + when-to-use triggers.
chunk-your-skills recompose \
	--catalog "$CATALOG" \
	--path "$INPUT_SKILL_PATH" \
	--node-id 1,2 \
	--output examples/context7/skinny-skill/activation/SKILL.md

# Same activation skill, in memory from the source file (no catalog).
chunk-your-skills recompose \
	--skill "$INPUT_SKILL_PATH" \
	--node-id 1,2 \
	--output examples/context7/skinny-skill/activation/SKILL.md

# Mixed node-id ranges and lists: preamble + steps 1–3 + guidelines.
chunk-your-skills recompose \
	--skill "$INPUT_SKILL_PATH" \
	--node-id 1-3,8 \
	--output examples/context7/skinny-skill/mixed-nodes/SKILL.md

# Guidelines section only (frontmatter is always included).
chunk-your-skills recompose \
	--catalog "$CATALOG" \
	--path "$INPUT_SKILL_PATH" \
	--node-id 8 \
	--output examples/context7/skinny-skill/guidelines-only/SKILL.md

# Skeleton skill: matched sections keep body; other headings stay as stubs.
chunk-your-skills recompose \
	--catalog "$CATALOG" \
	--path "$INPUT_SKILL_PATH" \
	--node-id 4 \
	--keep-all-headers \
	--output examples/context7/skinny-skill/step1-skeleton/SKILL.md

# Default output path (omit --output): writes under
#   $CATALOG/skills/retrieve/context7/SKILL.md
chunk-your-skills recompose \
	--catalog "$CATALOG" \
	--path "$INPUT_SKILL_PATH" \
	--node-id 2,8
