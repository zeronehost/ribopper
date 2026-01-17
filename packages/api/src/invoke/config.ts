import { AppInfo, Config, UpdateApp } from "@/models";
import { invoke, Channel } from "@tauri-apps/api/core";
import { CONFIG_LOAD, CONFIG_SAVE, GET_APP_INFO, UPDATE_APP } from "./constants";

export const configLoad = async () => await invoke<Config>(CONFIG_LOAD);

export const configSave = async (config: Config) => await invoke<void>(CONFIG_SAVE, { config });

export const getAppInfo = async () => await invoke<AppInfo>(GET_APP_INFO);

export const updateApp = async (cb: (payload: UpdateApp) => void) => {
  const channel = new Channel();
  channel.onmessage(cb)
  await invoke<void>(UPDATE_APP, { channel })
};