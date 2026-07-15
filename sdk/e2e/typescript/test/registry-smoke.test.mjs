import assert from "node:assert/strict";
import test from "node:test";

import { countTokens, getVersion } from "chunk-your-skills";

test("countTokens smoke", () => {
  assert.ok(countTokens("hello world") >= 1);
});

test("getVersion smoke", () => {
  const version = getVersion();
  assert.equal(typeof version, "string");
  assert.ok(version.length > 0);
});
