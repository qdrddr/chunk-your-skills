/** TypeScript SDK for chunk-your-skills (Rust-backed SKILL.md pageindex). */

export { getVersion } from "./core.js";
export {
  configureTokenizerDefaults,
  countJsonTokens,
  countTokens,
  countTokensBatch,
} from "./tokens.js";
export {
  configureMemoryCache,
  ensureSkillsRegistry,
  type CachePolicy,
  type SkillEntryRef,
  type SkillSourceInput,
} from "./cache.js";
export {
  SkillsBuilder,
  buildSkillsIndex,
  buildPageIndexOnly,
  buildPageIndexForFile,
  pageIndexValid,
  repairSkillNodes,
  loadSkillsIndexFromEntry,
  loadMergedSkillDocumentJson,
  finalizeSkillDocumentJson,
  updateSkillDocumentSourcePath,
  defaultPageIndexConfig,
  getSkillContentRetrieveResult,
  getSkillDocument,
  getSkillLineContent,
  getSkillLineContentFromSpec,
  getSkillStructure,
  loadSkillsIndexFromDir,
  mdToTree,
  pageIndexConfigFromMapping,
  pageIndexConfigFromPartial,
  pageIndexConfigToNative,
  parseSkillNodeIds,
  tokenCountFromDecomposedFrontmatter,
  parseFrontmatterFields,
  frontmatterField,
  reconstructSkillMarkdown,
  skillsIndexFromDecomposedDir,
  writeReconstructedSkill,
  writeSkillsIndex,
  type PageIndexConfig,
  type PageIndexConfigInput,
  type ReconstructOptions,
  type SkillsIndexDict,
} from "./pageindex.js";
export type { SkillLineContentRow } from "./types.js";
