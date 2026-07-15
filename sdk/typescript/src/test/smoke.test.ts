import assert from "node:assert/strict";
import test from "node:test";

import { getVersion } from "../index.js";
import { countTokens } from "../tokens.js";
import { mdToTree } from "../pageindex.js";
import { isJsonRecord } from "../types.js";

test("getVersion returns a non-empty semver-like string", () => {
  const version = getVersion();
  assert.match(version, /^\d+\.\d+\.\d+/);
});

test("countTokens returns a positive count for non-empty text", () => {
  const count = countTokens("hello world");
  assert.ok(count > 0);
});

test("mdToTree parses markdown into tree JSON", () => {
  const tree = mdToTree("# Title\n\nBody", "skill.md", {});
  assert.ok(isJsonRecord(tree));
});
