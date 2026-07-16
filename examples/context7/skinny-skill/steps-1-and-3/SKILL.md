---
name: context7-mcp
description: >-
  This skill should be used when the user asks about libraries, frameworks,
  API references, or needs code examples. Activates for setup questions, code
  generation involving libraries, or mentions of specific frameworks like React,
  Vue, Next.js, Prisma, Supabase, etc.
---

## How to Fetch Documentation

### Step 1: Resolve the Library ID

Call `resolve-library-id` with:

- `libraryName`: The library name extracted from the user's question
- `query`: The user's full question (improves relevance ranking)

### Step 3: Fetch the Documentation

Call `query-docs` with:

- `libraryId`: The selected Context7 library ID (e.g., `/vercel/next.js`)
- `query`: The user's specific question