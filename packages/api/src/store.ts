import { invoke } from "@tauri-apps/api/core";
import type { Config } from "./models";

export const storeLoad = async () => await invoke<Config | null>("store_load");

export const storeSave = async (config: Partial<Config>) => await invoke<void>("store_save", { config });
