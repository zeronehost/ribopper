import { getRecords, deleteRecord, clearRecord, type RiboRecord } from "@ribo/api"
import { defineStore } from "pinia"
import { shallowReactive } from "vue";

export const useRecordStore = defineStore('record', {
  state: (): {
    list: RiboRecord[],
    size: number,
    index: number,
    loadEnable: boolean,
    contentContains: string,
    loading: boolean;
  } => ({
    list: [],
    size: 10,
    index: 0,
    loadEnable: true,
    contentContains: "",
    loading: false,
  }),

  getters: {
    total(): number {
      return this.list.length
    },
  },

  actions: {
    async initRecords() {
      this.list = shallowReactive([]);
      this.index = 0;
      this.size = 10;
      this.loadEnable = true;
      if (!this.loading) {
        await this.getRecords();
      }
    },
    async getRecords() {
      console.trace(`store -> getRecords => ${new Date().toLocaleTimeString()}`);
      console.count("store -> getRecords");
      if (this.loadEnable) {
        const offset = this.index * this.size;
        this.loading = true;
        const list = await getRecords({
          limit: this.size,
          offset,
          contentContains: this.contentContains,
        }).finally(() => {
          this.loading = false;
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