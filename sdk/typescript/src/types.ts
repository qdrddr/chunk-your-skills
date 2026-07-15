export type JsonRecord = Record<string, unknown>;

export function isJsonRecord(value: unknown): value is JsonRecord {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

/** One node/chunk row from skill line-content retrieval. */
export interface SkillLineContentRow {
  line_num?: number;
  node_id?: number;
  chunk_id?: number;
  content: string;
  token_count?: number;
}
