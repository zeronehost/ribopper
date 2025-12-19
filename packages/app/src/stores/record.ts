import { getRecords, deleteRecord, clearRecord, type RiboRecordWithTargets, type RiboTextRecord, type UpdateRecord } from "@ribo/api"
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
    async deleteRecord(id: number) {
      return await deleteRecord(id);
    },
    async clearRecord() {
      return await clearRecord();
    },
  }
})