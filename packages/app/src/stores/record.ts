import { getRecords, getRecord, deleteRecord, clearRecord, Record, showRecordActions, RiboEvent, logger, updateRecord } from "@ribo/api"
import { defineStore } from "pinia"
import { shallowReactive } from "vue";

export const useRecordStore = defineStore('record', {
  state: (): {
    // 数据集合
    list: Record[],
    // 搜索内容
    contentContains: string,
  } => ({
    list: [],
    contentContains: "",
  }),

  getters: {
    total(): number {
      return this.list.length
    },
  },

  actions: {
    async getAllRecords() {
      try {
        this.list = shallowReactive([]);
        this.list = await getRecords({
          contentContains: this.contentContains,
        });
      } catch (error) {
        logger.error(error as Error);
      }
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
      const flag = await clearRecord();
      if (flag) {
        this.list = shallowReactive([]);
      }
      return flag;
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