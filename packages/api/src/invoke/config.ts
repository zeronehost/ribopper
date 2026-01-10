import { AppInfo, Config } from "@/models";
import { invoke } from "@tauri-apps/api/core";
import { CONFIG_LOAD, CONFIG_SAVE, GET_APP_INFO } from "./constants";

export const configLoad = async () => await invoke<Config>(CONFIG_LOAD);

export const configSave = async (config: Config) => await invoke<void>(CONFIG_SAVE, { config });

export const getAppInfo = async () => await invoke<AppInfo>(GET_APP_INFO);