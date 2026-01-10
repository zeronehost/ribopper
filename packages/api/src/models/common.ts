import * as z from "zod";

export const AppInfo = z.object({
  name: z.string(),
  version: z.string(),
  description: z.string(),
  authors: z.string(),
  license: z.string(),
  website: z.string()
});

export type AppInfo = z.infer<typeof AppInfo>;