import { defineStore } from "pinia";

export const useCacheStore = defineStore("cache", {
  state(): {dataset: Record<string, Any>} {
    return {
      dataset: {}
    }
  },
  actions: {
    set(key: string, value: Any) {
      this.dataset[key] = value;
    },
    get(key: string) {
      return this.dataset[key];
    },
    clear() {
      this.dataset = {};
    }
  }
});