import { type Config, type GeneralOptions, storeSave, type Theme } from "@ribo/api";
import { defineStore } from "pinia";

export const useSettingStore = defineStore("setting", {
  state: (): {
    config: Partial<Config>;
    _initData: Config;
    isUpdate: boolean;
  } => ({
    config: {
      options: [],
      hotkey: [],
    },
    _initData: {
      options: [],
      hotkey: [],
    },
    isUpdate: false,
  }),
  getters: {
    theme(): Theme {
      return this.config.theme || "auto";
    },
    max(): GeneralOptions["max"] | "" {
      return this.config?.general?.max ?? "";
    },
    typeOptions(): GeneralOptions["options"] {
      return this.config?.general?.options;
    },
  },
  actions: {
    toggleTheme(name: Config["theme"]) {
      this.config.theme = name;
    },
    setMax(max?: GeneralOptions["max"]) {
      if (!this.config.general) {
        this.config.general = {};
      }
      this.config.general.max = typeof max === "number" && max > 0 ? max : null;
    },

    setTypeOptions(options: GeneralOptions["options"]) {
      if (!this.config.general) {
        this.config.general = {};
      }
      this.config.general.options = options;
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
