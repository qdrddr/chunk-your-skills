import assert from "node:assert/strict";
import test from "node:test";

import { getVersion } from "chunk-your-skills";

test("getVersion smoke", () => {
  const version = getVersion();
  assert.equal(typeof version, "string");
  assert.ok(version.length > 0);
});
