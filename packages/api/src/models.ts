export interface Config {
  theme?: Theme;
  general?: GeneralOptions;
  options: Array<Record<string, string>>;
  hotkey: Array<Record<string, Array<string>>>;
}

export type Theme = "light" | "dark" | "auto";
export interface GeneralOptions {
  max?: number | null;
  options?: Record<HistoryType, Partial<TypeOptions>>;
}

export interface TypeOptions {
  editable: boolean;
  deletable: boolean;
  scannable: boolean;
  starable: boolean;
}

export interface History {
  id: number;
  content: string;
  type: HistoryType;
  createdAt?: Date;
  updatedAt?: Date;
}

export type UpdateHistory = Omit<History, "createdAt" | "updatedAt">;

export type HistoryType = "text" | "image" | "file" | "dir";

export interface Historys {
  list: History[];
  total: number;
}
