import { defineStore } from "pinia";

export const useSettingStore = defineStore("setting", {
  state: (): {
    theme: "light" | "dark" | "auto";
  } => ({
    theme: "auto",
  }),
  actions: {
    toggleTheme(name: "light" | "dark" | "auto") {
      this.theme = name;
    },
  },
});
