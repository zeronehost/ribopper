import { getRecords, deleteRecord, clearRecord, type RiboRecord } from "@ribo/api"
import { defineStore } from "pinia"

export const useRecordStore = defineStore('record', {
  state: (): {
    list: RiboRecord[],
    size: number,
    index: number,
    loadEnable: boolean,
    contentContains: string,
  } => ({
    list: [],
    size: 10,
    index: 0,
    loadEnable: true,
    contentContains: ""
  }),

  getters: {
    total(): number {
      return this.list.length
    },
  },

  actions: {
    reset() {
      this.list = [];
      this.index = 0;
      this.size = 10;
      this.loadEnable = true;
    },
    async getRecords() {
      if (this.loadEnable) {
        const offset = this.index * this.size;
        const list = await getRecords({
          limit: this.size,
          offset,
          contentContains: this.contentContains,
        });
        if (list.length >= this.size) {
          this.index += 1;
        } else {
          this.loadEnable = false;
        }
        this.list.push(...list);
      }
    },
    search(contentContains: string) {
      this.contentContains = contentContains;
    },
    async deleteRecord(id: number) {
      return await deleteRecord(id);
    },
    async clearRecord() {
      return await clearRecord();
    },
  }
})