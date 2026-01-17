import * as z from "zod";

export const AppInfo = z.object({
  name: z.string(),
  version: z.string(),
  description: z.string(),
  authors: z.string(),
  license: z.string(),
  website: z.string(),
  features: z.object({
    action: z.boolean().optional(),
    image: z.boolean().optional(),
  })
});

export type AppInfo = z.infer<typeof AppInfo>;

export const UpdateApp = z.object({
  done: z.boolean(),
  indeterminate: z.boolean(),
  total: z.number().optional(),
  downloaded: z.number().optional(),
  progress: z.number().optional(),
});

export type UpdateApp = z.infer<typeof UpdateApp>;