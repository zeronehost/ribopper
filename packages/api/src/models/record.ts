
export interface RecordQuery {
  contentContains?: string;
  targetId?: number;
  startDate?: Date,
  endDate?: Date,
  limit?: number,
  offset?: number,
  orderBy?: string,
  orderDirection?: "asc" | "desc",
}

export interface RecordWithTargetsBase {
  targets: string[],
  targetCount: number,
}


export interface RiboTextRecord {
  type: "text",
  text: string,
  id: number,
}
export interface RiboImageRecord {
  type: "image",
  image: Uint8Array,
  id: number,
}
export interface RiboFileRecord {
  type: "files",
  files: string,
  id: number,
}

export type RiboRecord = (
  RiboTextRecord |
  RiboImageRecord |
  RiboFileRecord
) & {
  createdAt: Date,
  updatedAt: Date,
}

// export type RiboRecordWithTargets = RiboRecord & RecordWithTargetsBase;

export type UpdateRecord = RiboTextRecord | RiboImageRecord | RiboFileRecord;