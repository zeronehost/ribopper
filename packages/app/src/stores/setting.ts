import { type Config, type GeneralOptions, storeSave, type Theme } from "@ribo/api";
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
    typeOptions(): GeneralOptions["options"] {
      return this.config?.general?.options;
    },
    duration(): GeneralOptions["duration"] {
      return this.config?.general?.duration ?? 500;
    }
  },
  actions: {
    toggleTheme(name: Config["theme"]) {
      this.config.theme = name;
    },
    setMax(max?: GeneralOptions["max"]) {
      (this.config.general as GeneralOptions).max = typeof max === "number" && max > 0 ? max : null;
    },

    setTypeOptions(options: GeneralOptions["options"]) {
      (this.config.general as GeneralOptions).options = options;
    },
    setDuration(duration: GeneralOptions["duration"]) {
      (this.config.general as GeneralOptions).duration = duration < 500 ? 500 : duration;
    },

    saveConfig() {
      this.isUpdate = false;
      storeSave(this.config).then(() => {
        this._initData = JSON.parse(JSON.stringify(this.config));
      });
    },
  },
});
