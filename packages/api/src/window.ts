import { invoke } from "@tauri-apps/api/core";

export const closeWindow = async (label: string) => await invoke("close_window", { label });
