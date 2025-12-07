import { clearData, deleteData, getData, type Historys, type UpdateHistory, updateData } from "@ribo/api";
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
    },
    async updateData(data: UpdateHistory) {
      await updateData(data);
    },
    async clear() {
      await clearData();
      this.total = 0;
      this.list = [];
    },
  },
});
