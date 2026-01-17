import * as z from "zod";

export const RiboEvent = z.object({
  type: z.enum(["init", "update", "refresh"]),
  label: z.string(),
  payload: z.any().optional(),
});

export type RiboEvent = z.infer<typeof RiboEvent>;
