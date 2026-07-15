/** Skills pageindex (markdown tree indexing and retrieval). */

import type { SkillLineContentRow } from "./types.js";

import {
  SkillsBuilderNative,
  buildPageIndexForFileNative,
  buildPageIndexOnlyNative,
  buildSkillsIndexNative,
  finalizeSkillDocumentJsonNative,
  getSkillContentRetrieveResultNative,
  getSkillDocumentNative,
  getSkillLineContentFromSpecNative,
  getSkillLineContentNative,
  getSkillStructureNative,
  loadMergedSkillDocumentJsonNative,
  loadSkillsIndexFromDirNative,
  loadSkillsIndexFromEntryNative,
  mdToTreeNative,
  pageIndexValidNative,
  parseSkillNodeIdsNative,
  tokenCountFromDecomposedFrontmatterNative,
  reconstructSkillMarkdownNative,
  repairSkillNodesNative,
  skillsIndexFromDecomposedDirNative,
  updateSkillDocumentSourcePathNative,
  writeReconstructedSkillNative,
  writeSkillsIndexNative,
  type ReconstructOptionsNapi,
} from "./native.js";

export interface PageIndexConfig {
  ifAddNodeId?: boolean;
  ifAddNodeText?: boolean;
}

/** CamelCase SDK config or snake_case partial dict (e.g. from app YAML). */
export type PageIndexConfigInput = PageIndexConfig | Record<string, unknown>;

export interface ReconstructOptions {
  keepAllHeaders?: boolean;
}

export interface SkillsIndexDict {
  documents: Record<string, unknown>;
  files: Record<string, string>;
}

export function defaultPageIndexConfig(): PageIndexConfig {
  return {
    ifAddNodeId: true,
    ifAddNodeText: false,
  };
}

/** Partial pageindex settings from app config; Rust merges unset keys with SDK defaults. */
export function pageIndexConfigFromMapping(
  mapping?: Record<string, unknown> | null,
): Record<string, unknown> | undefined {
  if (mapping == null) return undefined;
  if (isSnakeCasePageIndexDict(mapping)) return mapping;
  return pageIndexConfigToNative(pageIndexConfigFromPartial(mapping));
}

export function pageIndexConfigFromPartial(
  partial: Partial<PageIndexConfig> & Record<string, unknown>,
): PageIndexConfig {
  const cfg = defaultPageIndexConfig();
  if (partial.ifAddNodeId !== undefined) cfg.ifAddNodeId = partial.ifAddNodeId;
  if (partial.ifAddNodeText !== undefined)
    cfg.ifAddNodeText = partial.ifAddNodeText;
  return cfg;
}

export function pageIndexConfigToNative(
  config: PageIndexConfig,
): Record<string, unknown> {
  return {
    if_add_node_id: config.ifAddNodeId ?? true,
    if_add_node_text: config.ifAddNodeText ?? false,
  };
}

function isSnakeCasePageIndexDict(config: Record<string, unknown>): boolean {
  return "if_add_node_id" in config || "if_add_node_text" in config;
}

function resolveNativeConfig(
  config?: PageIndexConfigInput,
): Record<string, unknown> | undefined {
  if (config == null) return undefined;
  if (isSnakeCasePageIndexDict(config as Record<string, unknown>)) {
    return config as Record<string, unknown>;
  }
  return pageIndexConfigToNative(config as PageIndexConfig);
}

export function buildSkillsIndex(
  skillDirs: string[],
  config?: PageIndexConfigInput,
): SkillsIndexDict {
  return buildSkillsIndexNative(
    skillDirs,
    resolveNativeConfig(config),
  ) as SkillsIndexDict;
}

export function writeSkillsIndex(
  index: SkillsIndexDict,
  outputDir: string,
): void {
  writeSkillsIndexNative(index, outputDir);
}

export function loadSkillsIndexFromDir(catalogDir: string): SkillsIndexDict {
  return loadSkillsIndexFromDirNative(catalogDir) as SkillsIndexDict;
}

export function skillsIndexFromDecomposedDir(dir: string): SkillsIndexDict {
  return skillsIndexFromDecomposedDirNative(dir) as SkillsIndexDict;
}

export function mdToTree(
  markdownContent: string,
  sourcePath: string,
  config?: PageIndexConfigInput,
): Record<string, unknown> {
  return mdToTreeNative(
    markdownContent,
    sourcePath,
    resolveNativeConfig(config),
  ) as Record<string, unknown>;
}

export function getSkillDocument(
  documents: Record<string, unknown>,
  docId: string,
): Record<string, unknown> {
  return getSkillDocumentNative(documents, docId) as Record<string, unknown>;
}

export function getSkillStructure(
  documents: Record<string, unknown>,
  docId: string,
): unknown {
  return getSkillStructureNative(documents, docId);
}

export function getSkillLineContentFromSpec(
  index: SkillsIndexDict,
  docId: string,
  lineNumSpec: string,
): SkillLineContentRow[] {
  return getSkillLineContentFromSpecNative(
    index,
    docId,
    lineNumSpec,
  ) as SkillLineContentRow[];
}

function toNativeReconstructOptions(
  options?: ReconstructOptions,
): ReconstructOptionsNapi | undefined {
  if (!options) return undefined;
  return { keepAllHeaders: options.keepAllHeaders ?? false };
}

export function getSkillLineContent(
  index: SkillsIndexDict,
  docId: string,
  opts?: {
    lineNumSpecs?: string[];
    nodeIdSpecs?: string[];
  },
): SkillLineContentRow[] {
  return getSkillLineContentNative(
    index,
    docId,
    opts?.lineNumSpecs,
    opts?.nodeIdSpecs,
  ) as SkillLineContentRow[];
}

export function getSkillContentRetrieveResult(
  index: SkillsIndexDict,
  docId: string,
  opts?: {
    lineNumSpecs?: string[];
    nodeIdSpecs?: string[];
    options?: ReconstructOptions;
  },
): Record<string, unknown> {
  return getSkillContentRetrieveResultNative(
    index,
    docId,
    opts?.lineNumSpecs,
    opts?.nodeIdSpecs,
    toNativeReconstructOptions(opts?.options),
  ) as Record<string, unknown>;
}

export function reconstructSkillMarkdown(
  index: SkillsIndexDict,
  docId: string,
  opts?: {
    lineNumSpecs?: string[];
    nodeIdSpecs?: string[];
    options?: ReconstructOptions;
  },
): {
  markdown: string;
  matched_node_ids: number[];
  node_ids: number[];
  output_rel_path: string;
} {
  return reconstructSkillMarkdownNative(
    index,
    docId,
    opts?.lineNumSpecs,
    opts?.nodeIdSpecs,
    toNativeReconstructOptions(opts?.options),
  ) as {
    markdown: string;
    matched_node_ids: number[];
    node_ids: number[];
    output_rel_path: string;
  };
}

export function writeReconstructedSkill(
  catalogDir: string,
  index: SkillsIndexDict,
  docId: string,
  opts?: {
    lineNumSpecs?: string[];
    nodeIdSpecs?: string[];
    options?: ReconstructOptions;
  },
): string {
  return writeReconstructedSkillNative(
    catalogDir,
    index,
    docId,
    opts?.lineNumSpecs,
    opts?.nodeIdSpecs,
    toNativeReconstructOptions(opts?.options),
  );
}

export function parseSkillNodeIds(spec: string): number[] {
  return parseSkillNodeIdsNative(spec);
}

export function tokenCountFromDecomposedFrontmatter(
  content: string,
): number | null {
  return tokenCountFromDecomposedFrontmatterNative(content);
}

export function repairSkillNodes(
  entryDir: string,
  docId: string,
  config?: PageIndexConfigInput,
): void {
  repairSkillNodesNative(entryDir, docId, resolveNativeConfig(config));
}

export function buildPageIndexOnly(
  skillDirs: string[],
  config?: PageIndexConfigInput,
): SkillsIndexDict {
  return buildPageIndexOnlyNative(
    skillDirs,
    resolveNativeConfig(config),
  ) as SkillsIndexDict;
}

export function buildPageIndexForFile(
  sourcePath: string,
  config?: PageIndexConfigInput,
): SkillsIndexDict {
  return buildPageIndexForFileNative(
    sourcePath,
    resolveNativeConfig(config),
  ) as SkillsIndexDict;
}

export function pageIndexValid(
  entryDir: string,
  contentSha256: string,
): boolean {
  return pageIndexValidNative(entryDir, contentSha256);
}

export function loadSkillsIndexFromEntry(
  entryDir: string,
  docId: string,
): SkillsIndexDict {
  return loadSkillsIndexFromEntryNative(entryDir, docId) as SkillsIndexDict;
}

export function loadMergedSkillDocumentJson(
  entryDir: string,
  docId: string,
): Record<string, unknown> {
  return loadMergedSkillDocumentJsonNative(entryDir, docId) as Record<
    string,
    unknown
  >;
}

export function finalizeSkillDocumentJson(
  entryDir: string,
  docId: string,
  options: {
    pipeline: string;
    indexParams: Record<string, unknown>;
    sourcePath: string;
  },
): Record<string, unknown> {
  return finalizeSkillDocumentJsonNative(
    entryDir,
    docId,
    options.pipeline,
    options.indexParams,
    options.sourcePath,
  ) as Record<string, unknown>;
}

export function updateSkillDocumentSourcePath(
  entryDir: string,
  docId: string,
  sourcePath: string,
): Record<string, unknown> {
  return updateSkillDocumentSourcePathNative(
    entryDir,
    docId,
    sourcePath,
  ) as Record<string, unknown>;
}

export class SkillsBuilder {
  private inner: InstanceType<typeof SkillsBuilderNative>;

  constructor(options?: { memoryOnly?: boolean; outputDir?: string }) {
    this.inner = new SkillsBuilderNative(
      options?.memoryOnly ?? true,
      options?.outputDir,
    );
  }

  buildFromDirs(
    skillDirs: string[],
    config?: PageIndexConfigInput,
  ): SkillsIndexDict {
    return this.inner.buildFromDirs(
      skillDirs,
      resolveNativeConfig(config),
    ) as SkillsIndexDict;
  }

  buildFromFile(
    sourcePath: string,
    config?: PageIndexConfigInput,
  ): SkillsIndexDict {
    return this.inner.buildFromFile(
      sourcePath,
      resolveNativeConfig(config),
    ) as SkillsIndexDict;
  }

  writeCatalog(): SkillsIndexDict {
    return this.inner.writeCatalog() as SkillsIndexDict;
  }

  toSkillsIndexJson(): Record<string, unknown> {
    return this.inner.toSkillsIndexJson() as Record<string, unknown>;
  }

  toSkillsDict(): Record<string, unknown> {
    return this.inner.toSkillsDict() as Record<string, unknown>;
  }
}
