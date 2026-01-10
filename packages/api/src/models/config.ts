import * as z from "zod";

export const General = z.object({
  max: z.number().int().min(1).max(1000).optional(),
  autoStart: z.boolean(),
  exitConfirm: z.boolean()
});
export type General = z.infer<typeof General>;

export const Key = z.object({
  alt: z.boolean(),
  ctrl: z.boolean(),
  meta: z.boolean(),
  shift: z.boolean(),
  key: z.string(),
});
export type Key = z.infer<typeof Key>;

export const Hotkey = z.object({
  clear: Key,
  edit: Key,
  next: Key,
  pane: Key,
  prev: Key,
  qrcode: Key,
  delete: Key,
  copy: Key,
}).partial();
export type Hotkey = z.infer<typeof Hotkey>;

export const Theme = z.enum(["light", "dark", "auto"]);
export type Theme = z.infer<typeof Theme>;

export const Config = z.object({
  theme: Theme,
  general: General,
  hotkey: Hotkey,
});
export type Config = z.infer<typeof Config>;
