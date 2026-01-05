import {
  createActionOption,
  deleteAction,
  getActions,
  updateAction,
  createOption,
  deleteOption,
  updateOption,
  getOptionsByActionId,
  type Action,
  type NewAction,
  type NewOption,
  type UpdateAction,
  type UpdateOption,
} from "@ribo/api";
import { defineStore } from "pinia";

export const useActionStore = defineStore("action", {
  state: () => ({
    actions: [] as Action[],
  }),
  actions: {
    async addAction(action: NewAction) {
      await createActionOption(action);
    },
    async getActions() {
      this.actions = await getActions();
    },
    async deleteAction(id: number) {
      await deleteAction(id);
    },
    async updateAction(action: UpdateAction) {
      await updateAction(action);
    },
    async addOption(option: NewOption) {
      await createOption(option);
    },
    async getOptionsByActionId(id: number) {
      await getOptionsByActionId(id);
    },
    async deleteOption(id: number) {
      await deleteOption(id);
    },
    async updateOption(option: UpdateOption) {
      await updateOption(option);
    }
  }
});