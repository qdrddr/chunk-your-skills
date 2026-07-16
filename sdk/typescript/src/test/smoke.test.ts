import assert from "node:assert/strict";
import test from "node:test";

import { getVersion } from "../index.js";
import {
  mdToTree,
  parseFrontmatterFields,
  frontmatterField,
} from "../pageindex.js";
import { isJsonRecord } from "../types.js";

test("getVersion returns a non-empty semver-like string", () => {
  const version = getVersion();
  assert.match(version, /^\d+\.\d+\.\d+/);
});

test("mdToTree parses markdown into tree JSON", () => {
  const tree = mdToTree("# Title\n\nBody", "skill.md", {});
  assert.ok(isJsonRecord(tree));
});

test("parseFrontmatterFields folds YAML description", () => {
  const frontmatter =
    "---\nname: demo\ndescription: >-\n  Line one.\n  Line two.\n---";
  const fields = parseFrontmatterFields(frontmatter);
  assert.ok(fields);
  assert.deepEqual(fields[0], { name: "demo" });
  assert.deepEqual(fields[1], { description: "Line one. Line two." });
  assert.equal(
    frontmatterField(frontmatter, "description"),
    "Line one. Line two.",
  );
});
