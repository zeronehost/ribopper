import { getRecords, getRecord, deleteRecord, clearRecord, Record, showRecordActions, logger, updateRecord } from "@ribo/api"
import { defineStore } from "pinia"
import { shallowReactive } from "vue";

export const useRecordStore = defineStore('record', {
  state: (): {
    // 数据集合
    list: Record[],
    // 搜索内容
    contentContains: string,
    // 当前页码
    // index: number,
    // 每页条数
    size: number,
    // 是否正在加载
    loading: boolean,
    // 是否加载完成
    finished: boolean,
  } => ({
    list: [],
    contentContains: "",
    // 分页配置
    // index: 0,
    size: 50,
    loading: false,
    finished: false,
  }),

  getters: {
    total(): number {
      return this.list.length
    },
  },

  actions: {
    async reset() {
      this.list = shallowReactive([]);
      this.finished = false;
      await this.getRecords();
    },
    async getRecords() {
      if (this.loading || this.finished) return;
      this.loading = true;
      try {
        const list = await getRecords({
          contentContains: this.contentContains,
          offset: this.total,
          limit: this.size,
        });
        if (list.length < this.size) {
          this.finished = true;
        }
        this.list.push(...list);
      } catch (error) {
        logger.error(error as Error);
      }
      this.loading = false;
    },
    search(contentContains: string) {
      this.contentContains = contentContains;
    },
    async deleteRecord(id: number) {
      const flag = await deleteRecord(id);
      if (flag) {
        const index = this.list.findIndex((record) => record.id === id);
        if (index >= 0) {
          this.list.splice(index, 1);
        }
      }
    },
    async clearRecord() {
      try {
        await clearRecord();
      } catch (error) {
        logger.error(error as Error);
        throw error;
      }
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
      try {
        const flag = await updateRecord({
          id,
          text,
          type: "text"
        });
        if (flag) {
          const index = this.list.findIndex((record) => record.id === id);
          if (index >= 0 && this.list[index]?.type === "text") {
            this.list[index].text = text;
          }
        }
        return flag;
      } catch (error) {
        logger.error(error as Error);
        throw error;
      }
    }
  }
})