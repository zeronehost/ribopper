/** biome-ignore-all lint/suspicious/noExplicitAny: 忽略 */
export interface Config {
  theme: Theme;
  general: GeneralOptions;
  options: Array<Record<string, string>>;
  hotkey: RiboHotkey;
  schema: string;
}

export type Theme = "light" | "dark" | "auto";
export interface GeneralOptions {
  max?: number | null;
  autoStart: boolean;
}

export interface RiboEvent<T> {
  type: RiboEventType;
  label: string;
  payload?: T | null;
}

export type RiboEventType = "init" | "update" | "refresh";

export interface RiboHotkey {
  clear?: RiboKey,
  edit?: RiboKey,
  next?: RiboKey,
  pane?: RiboKey,
  prev?: RiboKey,
  qrcode?: RiboKey,
  delete?: RiboKey,
  copy?: RiboKey,
}

export interface RiboKey {
  altKey: boolean;
  ctrlKey: boolean;
  metaKey: boolean;
  shiftKey: boolean;
  key?: string;
}