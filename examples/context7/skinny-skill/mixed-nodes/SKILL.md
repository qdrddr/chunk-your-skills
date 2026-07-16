---
name: context7-mcp
description: >-
  This skill should be used when the user asks about libraries, frameworks,
  API references, or needs code examples. Activates for setup questions, code
  generation involving libraries, or mentions of specific frameworks like React,
  Vue, Next.js, Prisma, Supabase, etc.
---

When the user asks about libraries, frameworks, or needs code examples, use Context7 to fetch
current documentation instead of relying on training data.

## When to Use This Skill

Activate this skill when the user:

- Asks setup or configuration questions ("How do I configure Next.js middleware?")
- Requests code involving libraries ("Write a Prisma query for...")
- Needs API references ("What are the Supabase auth methods?")
- Mentions specific frameworks (React, Vue, Svelte, Express, Tailwind, etc.)

## How to Fetch Documentation

## Guidelines

- **Be specific**: Pass the user's full question as the query for better results
- **Version awareness**: When users mention versions ("Next.js 15", "React 19"), use
  version-specific library IDs if available from the resolution step
- **Prefer official sources**: When multiple matches exist, prefer official/primary packages over community forks