import { Config, General, configSave, Theme, Key, Hotkey, AppInfo, getAppInfo, logger, configLoad } from "@ribo/api";
import { defineStore } from "pinia";

export const useSettingStore = defineStore("setting", {
  state: (): {
    config: Partial<Config>;
    appInfo?: AppInfo;
    _initData: Partial<Config>;
    isUpdate: boolean;
  } => ({
    config: {},
    _initData: {},
    isUpdate: false,
    appInfo: undefined,
  }),
  getters: {
    theme(): Theme {
      return this.config.theme || "auto";
    },
    max(): General["max"] | "" {
      return this.config?.general?.max ?? "";
    },
    autoStart(): boolean {
      return this.config?.general?.autoStart as boolean
    },
    hotkeys(): Hotkey {
      return this.config?.hotkey as Hotkey ?? {};
    },
    exitConfirm(): boolean {
      return this.config?.general?.exitConfirm as boolean
    }
  },
  actions: {
    toggleTheme(name: Config["theme"]) {
      this.config.theme = name;
    },
    setMax(max?: General["max"]) {
      (this.config.general as General).max = typeof max === "number" && max > 0 ? (max < 1000 ? max : 1000) : undefined;
    },
    setAutoStart(autoStart: boolean) {
      (this.config.general as General).autoStart = autoStart;
    },
    setHotkey(label: keyof Hotkey, data: Key) {
      if (!this.config.hotkey) {
        this.config.hotkey = {};
      }
      (this.config.hotkey as Hotkey)[label] = data;
    },
    setExitConfirm(exitConfirm: boolean) {
      (this.config.general as General).exitConfirm = exitConfirm;
    },

    saveConfig() {
      this.isUpdate = false;
      configSave(this.config as Config).then(() => {
        this._initData = JSON.parse(JSON.stringify(this.config));
      }).catch((e) => {
        logger.error(e);
      });
    },

    async loadConfig() {
      try {
        const config = await configLoad();
        this.config = config;
        this._initData = JSON.parse(JSON.stringify(this.config));
      } catch (e) {
        logger.error(e as Error);
      }
    },

    async getAppInfo() {
      try {
        const appInfo = await getAppInfo();
        this.appInfo = appInfo;
      } catch (e) {
        logger.error(e as Error);
      }
    },
  },
});
