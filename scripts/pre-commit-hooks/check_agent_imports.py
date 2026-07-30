#!/usr/bin/env python3
"""Smoke-import the chunk-your-skills Python SDK."""

from __future__ import annotations


def main() -> int:
    from chunk_your_skills import build_skills_index

    assert callable(build_skills_index)
    print("chunk_your_skills import smoke check passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
