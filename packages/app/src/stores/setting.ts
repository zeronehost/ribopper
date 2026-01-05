import { type Config, type GeneralOptions, configSave, type Theme, type RiboKey, type RiboHotkey, logger, configLoad } from "@ribo/api";
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
    schema(): string {
      return this.config.schema as string
    },
    autoStart(): boolean {
      return this.config?.general?.autoStart as boolean
    },
    hotkeys(): RiboHotkey {
      return this.config?.hotkey as RiboHotkey ?? {};
    }
  },
  actions: {
    toggleTheme(name: Config["theme"]) {
      this.config.theme = name;
    },
    setMax(max?: GeneralOptions["max"]) {
      (this.config.general as GeneralOptions).max = typeof max === "number" && max > 0 ? (max < 1000 ? max : 1000): null;
    },
    setAutoStart(autoStart: boolean) {
      (this.config.general as GeneralOptions).autoStart = autoStart;
    },
    setHotkey(label: keyof RiboHotkey, data: RiboKey) {
      if (!this.config.hotkey) {
        this.config.hotkey = {};
      }
      (this.config.hotkey as RiboHotkey)[label] = data;
    },

    saveConfig() {
      this.isUpdate = false;
      configSave(this.config as Config).then(() => {
        this._initData = JSON.parse(JSON.stringify(this.config));
      }).catch((e) => {
        logger.error(e);
      });
    },

    loadConfig() {
      configLoad().then((config) => {
        this.config = config;
        this._initData = JSON.parse(JSON.stringify(this.config));
      }).catch((e) => {
        logger.error(e);
      })
    }
  },
});
