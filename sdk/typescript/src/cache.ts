/** Rust-backed disk/memory cache for skills registries. */

import {
  configureMemoryCacheNative,
  ensureSkillsRegistryNative,
} from "./native.js";

export type CachePolicy = "auto" | "force_memory" | "force_disk";

export interface SkillEntryRef {
  entry_dir: string;
  doc_id: string;
  content_sha256: string;
  bm25_chunk_dir: string | null;
  disk_backed: boolean;
  cache_status: "hit" | "miss" | "memory_fallback";
  source_path: string;
  nodes_dir: string | null;
  document: Record<string, unknown>;
  lazy_pending?: boolean;
}

/** Filesystem path or in-memory hook/client skill payload. */
export type SkillSourceInput =
  | string
  | {
      path: string;
      content?: string;
      content_sha256?: string;
    };

export function ensureSkillsRegistry(
  sourcePaths: SkillSourceInput[],
  catalogRoot: string,
  pageindexConfig: Record<string, unknown> | null | undefined,
  pipeline: string,
  indexParamsHash: string,
  policy?: CachePolicy,
): SkillEntryRef[] {
  return ensureSkillsRegistryNative(
    sourcePaths,
    catalogRoot,
    pageindexConfig ?? undefined,
    pipeline,
    indexParamsHash,
    policy,
  ) as SkillEntryRef[];
}

export function configureMemoryCache(config: Record<string, unknown>): void {
  configureMemoryCacheNative(config);
}
