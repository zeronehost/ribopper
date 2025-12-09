import { invoke } from "@tauri-apps/api/core";

export const copyData = async (id: number) => await invoke("copy_data", { id })