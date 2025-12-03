import { invoke } from "@tauri-apps/api/core";

export const clearData = async () => await invoke("clear_data");
