import * as z from "zod";

export const RiboEvent = z.object({
  type: z.enum(["init", "update", "refresh"]),
  label: z.string(),
  action: z.enum([
    "CREATE",
    "UPDATE",
    "READ",
    "DELETE",
    "CLEAR",
    "SAVE",
  ]),
});

export type RiboEvent = z.infer<typeof RiboEvent>;
