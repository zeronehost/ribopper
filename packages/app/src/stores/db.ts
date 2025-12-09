import {
  clearData,
  deleteData,
  getData,
  type Historys,
  type HistoryType,
  type UpdateHistory,
  updateData,
} from "@ribo/api";
import { defineStore } from "pinia";

export const useDbStore = defineStore("db", {
  state: (): Historys => ({
    total: 0,
    list: [],
  }),
  actions: {
    async query() {
      const data = await getData();
      this.total = data.total;
      this.list = data.list;
    },
    async delete(id: number) {
      await deleteData(id);
      await this.query();
    },
    async updateData(id: number, content: string) {
      const item = this.list.find((item) => item.id === id);
      if (item) {
        const data: UpdateHistory = {
          id,
          content,
          type: item?.type as HistoryType,
        };
        await updateData(data);
        await this.query();
      }
    },
    async clear() {
      await clearData();
      await this.query();
    },
  },
});
