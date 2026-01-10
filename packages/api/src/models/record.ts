import * as z from "zod";

export const RecordQuery = z.object({
  contentContains: z.string().optional(),
  targetId: z.number().optional(),
  startDate: z.date().optional(),
  endDate: z.date().optional(),
  limit: z.number().optional(),
  offset: z.number().optional(),
  orderBy: z.string().optional(),
  orderDirection: z.enum(["asc", "desc"]).optional()
});
export type RecordQuery = z.infer<typeof RecordQuery>;

export const TextRecord = z.object({
  type: z.literal("text"),
  text: z.string(),
  id: z.number(),
});
export type TextRecord = z.infer<typeof TextRecord>;

export const ImageRecord = z.object({
  type: z.literal("image"),
  image: z.instanceof(Uint8Array),
  id: z.number()
});
export type ImageRecord = z.infer<typeof ImageRecord>;

export const FileRecord = z.object({
  type: z.literal("files"),
  files: z.string().array(),
  id: z.number()
});
export type FileRecord = z.infer<typeof FileRecord>;

export const Record = z.union([TextRecord.extend({
  createdAt: z.date(),
  updatedAt: z.date()
}), ImageRecord.extend({
  createdAt: z.date(),
  updatedAt: z.date()
})
  , FileRecord.extend({
    createdAt: z.date(),
    updatedAt: z.date()
  })
]);
export type Record = z.infer<typeof Record>;

export const UpdateRecord = z.union([TextRecord, ImageRecord, FileRecord]);
export type UpdateRecord = z.infer<typeof UpdateRecord>;