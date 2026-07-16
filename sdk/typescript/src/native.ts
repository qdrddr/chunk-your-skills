import { createRequire } from "node:module";

const require = createRequire(import.meta.url);

const native = require("../native.cjs") as typeof import("../native.d.ts");

export const getVersionNative = native.getVersion;

// Skills pageindex
export const buildSkillsIndexNative = native.buildSkillsIndexNapi;
export const writeSkillsIndexNative = native.writeSkillsIndexNapi;
export const loadSkillsIndexFromDirNative = native.loadSkillsIndexFromDirNapi;
export const skillsIndexFromDecomposedDirNative =
  native.skillsIndexFromDecomposedDirNapi;
export const repairSkillNodesNative = native.repairSkillNodes;
export const buildPageIndexOnlyNative = native.buildPageIndexOnly;
export const buildPageIndexForFileNative = native.buildPageIndexForFile;
export const pageIndexValidNative = native.pageIndexValid;
export const loadSkillsIndexFromEntryNative = native.loadSkillsIndexFromEntry;
export const loadMergedSkillDocumentJsonNative =
  native.loadMergedSkillDocumentJson;
export const finalizeSkillDocumentJsonNative = native.finalizeSkillDocumentJson;
export const updateSkillDocumentSourcePathNative =
  native.updateSkillDocumentSourcePath;
export const mdToTreeNative = native.mdToTreeNapi;
export const getSkillDocumentNative = native.getSkillDocumentNapi;
export const getSkillStructureNative = native.getSkillStructureNapi;
export const getSkillLineContentFromSpecNative =
  native.getSkillLineContentFromSpecNapi;
export const getSkillLineContentNative = native.getSkillLineContentNapi;
export const getSkillContentRetrieveResultNative =
  native.getSkillContentRetrieveResultNapi;
export const reconstructSkillMarkdownNative =
  native.reconstructSkillMarkdownNapi;
export const writeReconstructedSkillNative = native.writeReconstructedSkillNapi;
export const parseSkillNodeIdsNative = native.parseSkillNodeIdsNapi;
export const tokenCountFromDecomposedFrontmatterNative =
  native.tokenCountFromDecomposedFrontmatterNapi;
export const parseFrontmatterFieldsNative = native.parseFrontmatterFieldsNapi;
export const frontmatterFieldNative = native.frontmatterFieldNapi;
export const SkillsBuilderNative = native.SkillsBuilderNapi;

// Cache
export const ensureSkillsRegistryNative = native.ensureSkillsRegistry;
export const configureMemoryCacheNative = native.configureMemoryCache;

export type { ReconstructOptionsNapi } from "../native.d.ts";
