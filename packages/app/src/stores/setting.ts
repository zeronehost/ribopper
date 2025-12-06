import { type Config, storeSave } from "@ribo/api";
import { defineStore } from "pinia";

export const useSettingStore = defineStore("setting", {
  state: (): {
    config: Config;
    _initData: Config;
    isUpdate: boolean;
  } => ({
    config: {
      theme: "auto",
    },
    _initData: {
      theme: "auto",
    },
    isUpdate: false,
  }),
  getters: {
    theme(): Config["theme"] {
      return this.config.theme || "auto";
    },
  },
  actions: {
    toggleTheme(name: Config["theme"]) {
      this.config.theme = name;
    },

    saveConfig() {
      this.isUpdate = false;
      storeSave(this.config).then(() => {
        this._initData = JSON.parse(JSON.stringify(this.config));
      });
      // TODO: save to local storage
    },
  },
});
