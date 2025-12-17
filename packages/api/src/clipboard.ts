import { invoke } from "@tauri-apps/api/core";

export const getRecords = async () => await invoke("get_records", { query: {} });
