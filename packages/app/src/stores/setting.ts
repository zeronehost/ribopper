import { type Config, type GeneralOptions, configSave, type Theme } from "@ribo/api";
import { defineStore } from "pinia";

export const useSettingStore = defineStore("setting", {
  state: (): {
    config: Partial<Config>;
    _initData: Partial<Config>;
    isUpdate: boolean;
  } => ({
    config: {},
    _initData: {},
    isUpdate: false,
  }),
  getters: {
    theme(): Theme {
      return this.config.theme || "auto";
    },
    max(): GeneralOptions["max"] | "" {
      return this.config?.general?.max ?? "";
    },
  },
  actions: {
    toggleTheme(name: Config["theme"]) {
      this.config.theme = name;
    },
    setMax(max?: GeneralOptions["max"]) {
      (this.config.general as GeneralOptions).max = typeof max === "number" && max > 0 ? max : null;
    },

    saveConfig() {
      this.isUpdate = false;
      configSave(this.config as Config).then(() => {
        this._initData = JSON.parse(JSON.stringify(this.config));
      }).catch(() => {});
    },
  },
});
