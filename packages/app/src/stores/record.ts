import { getRecords, getRecord, deleteRecord, clearRecord, Record, showRecordActions, RiboEvent, logger, updateRecord } from "@ribo/api"
import { defineStore } from "pinia"
import { shallowReactive } from "vue";

export const useRecordStore = defineStore('record', {
  state: (): {
    // 数据集合
    list: Record[],
    // 页大小
    size: number,
    // 当前页码
    index: number,
    // 是否正在加载
    loading: boolean,
    // 是否全部加载完成
    finished: boolean,
    // 刷新
    refreshing: boolean,
    // 搜索内容
    contentContains: string,
    // 当前操作的 id
    currentId: number,
  } => ({
    list: [],
    size: 10,
    index: 0,
    loading: false,
    finished: false,
    refreshing: false,
    contentContains: "",
    currentId: -1,
  }),

  getters: {
    total(): number {
      return this.list.length
    },
  },

  actions: {
    // async loadRecords(action: RiboEvent["action"]) {
    //   if (action === "CLEAR" || action === "CREATE") {
    //     this.list = shallowReactive([]);
    //     this.index = 0;
    //     this.size = 10;
    //     this.loadEnable = true;
    //     if (!this.loading) {
    //       await this.getRecords();
    //     }
    //   } else if (action === "UPDATE" || action === "DELETE") {
    //     // const i = this.index > 0 ? this.index : 1;
    //     const index = this.list.findIndex((record) => record.id === this.currentId);
    //     console.log("loadRecords =>", index);
    //     const currentPage = Math.floor(index / this.size);
    //     console.log("loadRecords => 1", index, currentPage);
    //     if (currentPage > this.index) return;
    //     // const max_len = (currentPage + 1) * this.size;
    //     const min_len = currentPage * this.size;
    //     console.log("loadRecords => 2", index, currentPage, min_len);
    //     if (index < 0) return;
    //     console.log("loadRecords => 3", index, currentPage, min_len);
    //     this.list = this.list.slice(0, min_len);
    //     this.index = currentPage;
    //     await this.getRecords();
    //   }
    // },
    async init() {
      this.finished = false;
      this.loading = true;
      this.index = 0;
      await this.getRecords();
    },
    async load(action: RiboEvent["action"]) {
      if (action === "DELETE" || action === "UPDATE") {
        const index = this.list.findIndex((record) => record.id === this.currentId);
        if (index >= 0) {
          const page = Math.floor(index / this.size);
          this.list = this.list.slice(0, page * this.size);
          this.index = page;
          await this.getRecords();
          this.finished = false;
        }
      } else if (action === "CLEAR" || action === "CREATE") {
        await this.init();
      }
    },
    async getRecords() {
      if (this.finished) return;
      if (this.refreshing) {
        this.list = shallowReactive([]);
        this.refreshing = false;
      }
      try {
        const data = await getRecords({
          limit: this.size,
          offset: this.index * this.size,
          contentContains: this.contentContains,
        });
        if (data.length === 0) {
          this.finished = true;
        } else {
          this.list = [...this.list, ...data];
          this.index += 1;
        }
      } catch (error) {
        logger.error(error as Error);
      }
      this.loading = false;
    },
    search(contentContains: string) {
      this.contentContains = contentContains;
    },
    async deleteRecord(id: number) {
      this.currentId = id;
      return await deleteRecord(id);
    },
    async clearRecord() {
      return await clearRecord();
    },
    async showRecordActions(id: number, label: string) {
      await showRecordActions(id, label);
    },
    async getRecord(id: number) {
      try {
        return await getRecord(id)
      } catch (error) {
        logger.error(error as Error);
        throw error;
      }
    },
    async updateRecord(id: number, text: string) {
      this.currentId = id;
      try {
        return updateRecord({
          id,
          text,
          type: "text"
        });
      } catch (error) {
        logger.error(error as Error);
        throw error;
      }
    }
  }
})