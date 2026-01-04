export interface Action {
  id: number;
  pattern: string;
  description: string;
  options: Option[];
  updatedAt: Date,
  createdAt: Date,
}

export interface Option {
  id: number;
  command: string;
  description: string;
  updatedAt: Date,
  createdAt: Date,
}

export type NewOption = Omit<Option, "id" | "createdAt" | "updatedAt">;

export type NewAction = Omit<Action, "id" | "createdAt" | "updatedAt" | "options"> & {
  options: NewOption[]
};

export type UpdateAction = Omit<Action, "options" | "createdAt" | "updatedAt">;

export type UpdateOption = Omit<Option, "createdAt" | "updatedAt">;