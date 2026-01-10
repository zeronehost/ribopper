import * as z from "zod";

export const Option = z.object({
  id: z.number(),
  name: z.string("请输入指令名称"),
  command: z.string("请输入指令内容"),
  out: z.enum(["ingore", "replace", "append"]),
  actionId: z.number(),
  description: z.string("请输入指令描述"),
  updatedAt: z.date(),
  createdAt: z.date(),
});

export type Option = z.infer<typeof Option>;

export const Action = z.object({
  id: z.number(),
  name: z.string("请输入操作名称"),
  pattern: z.string("请输入匹配模式"),
  description: z.string("请输入操作描述"),
  options: Option.array(),
  updatedAt: z.date(),
  createdAt: z.date(),
});

export type Action = z.infer<typeof Action>;

export const NewOption = Option.omit({
  id: true,
  createdAt: true,
  updatedAt: true,
});

export type NewOption = z.infer<typeof NewOption>;

export const NewAction = Action.omit({
  id: true,
  createdAt: true,
  updatedAt: true,
  options: true,
}).extend({
  options: NewOption.array(),
});

export type NewAction = z.infer<typeof NewAction>;

export const UpdateAction = Action.omit({
  options: true,
  createdAt: true,
  updatedAt: true,
});
export type UpdateAction = z.infer<typeof UpdateAction>;

export const UpdateOption = Option.omit({
  createdAt: true,
  updatedAt: true,
})
export type UpdateOption = z.infer<typeof UpdateOption>;