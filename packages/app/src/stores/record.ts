import { getRecords, updateRecord, deleteRecord, clearRecord, type RiboRecordWithTargets, type RiboTextRecord } from "@ribo/api"
import { defineStore } from "pinia"

export const useRecordStore = defineStore('record', {
  state: (): {
    list: RiboRecordWithTargets[],
  } => ({
    list: [],
  }),

  getters: {
    total(): number {
      return this.list.length
    },
  },

  actions: {
    async getRecords() {
      this.list = await getRecords();
    },
    async updateRecord(id: number, content: string) {
      const record = this.list.find((record) => record.id === id);
      if (record) {
        const data: RiboTextRecord = {
          type: "text",
          text: content,
          id: record.id,
        };
        await updateRecord(data);
      }
    },
    async deleteRecord(id: number) {
      const deleteF = await deleteRecord(id);
      if (deleteF) {
        await this.getRecords();
      }
      return deleteF;
    },
    async clearRecord() {
      const clearF = await clearRecord();
      if (clearF) {
        await this.getRecords();
      }
      return clearF;
    }
  }
})