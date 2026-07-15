import assert from "node:assert/strict";
import test from "node:test";

import {
  catalogDictFromSnapshot,
  extractSnapshotParts,
  loadSnapshot,
  parseTestArgs,
  resolveSnapshotPath,
  writeOutput,
} from "./example-snapshot.mjs";

const { file: exampleFile, output: outputFile } = parseTestArgs();

test(
  "decompose skill markdown from example file",
  {
    skip: exampleFile
      ? false
      : "pass --file to run against a local SKILL.md snapshot",
  },
  () => {
    if (!exampleFile) {
      return;
    }
    const snapshotPath = resolveSnapshotPath(exampleFile);
    const markdown = loadSnapshot(snapshotPath);
    extractSnapshotParts(markdown);

    const catalog = catalogDictFromSnapshot(markdown);
    const jsonChunks = catalog.json ?? [];
    const mdChunks = catalog.md ?? [];

    assert.ok(jsonChunks.length > 0, "mdToTree produced no json chunks");
    assert.ok(mdChunks.length > 0, "snapshot produced no markdown chunks");
    assert.ok(
      jsonChunks.some(
        (/** @type {{ file_path?: string }} */ entry) =>
          typeof entry.file_path === "string" &&
          entry.file_path.endsWith(".json"),
      ),
      "expected tree json chunk",
    );

    writeOutput(catalog, outputFile);
  },
);
