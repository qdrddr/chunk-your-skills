import { readFileSync, writeFileSync } from "node:fs";
import { resolve, sep } from "node:path";

import { mdToTree } from "chunk-your-skills";

const SAFE_PATH = /^[\w.\-+]+(?:\/[\w.\-+]+)*$/;

/** @param {string} file @param {string} label */
function resolveWithinCwd(file, label) {
  if (!file || file.includes("\0") || !SAFE_PATH.test(file)) {
    throw new Error(
      `${label} must be a simple relative path under the current directory`,
    );
  }
  const base = resolve(process.cwd());
  const resolved = resolve(base, file);
  if (resolved !== base && !resolved.startsWith(`${base}${sep}`)) {
    throw new Error(`${label} must stay within ${base}: ${resolved}`);
  }
  return resolved;
}

/** @returns {{ file?: string, output?: string }} */
export function parseTestArgs() {
  const args = process.argv.slice(2);
  const fileIdx = args.indexOf("--file");
  const outIdx = args.indexOf("--output");
  return {
    file: fileIdx >= 0 ? args[fileIdx + 1] : undefined,
    output: outIdx >= 0 ? args[outIdx + 1] : undefined,
  };
}

/** @param {string} file */
export function loadSnapshot(file) {
  const safePath = resolveWithinCwd(file, "--file");
  return readFileSync(safePath, "utf8");
}

/** @param {string} markdown */
export function extractSnapshotParts(markdown) {
  return { markdown };
}

/** @param {string} markdown */
export function catalogDictFromSnapshot(markdown) {
  const tree = mdToTree(markdown, "skill.md", {});
  return {
    json: [{ file_path: "skill/tree.json", content: tree }],
    md: [{ file_path: "skill/body.md", content: markdown }],
  };
}

/** @param {{ json?: unknown[], md?: unknown[] }} catalog @param {string | undefined} outputFile */
export function writeOutput(catalog, outputFile) {
  if (outputFile) {
    const resolved = resolveWithinCwd(outputFile, "--output");
    writeFileSync(resolved, JSON.stringify(catalog, null, 2));
  }
}
