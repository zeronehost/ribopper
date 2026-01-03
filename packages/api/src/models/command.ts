export interface Option {
  name: string;
  description: string;
  command: string;
  cwd: string;
}

export interface Action {
  id: number;
  name: string;
  description: string;
  pattern: string;
  options: Option[];
  createdAt: Date;
  updatedAt: Date;
}