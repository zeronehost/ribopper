import { invoke } from "@tauri-apps/api/core";
import { CLOSE_WINDOW } from "./constants";

export const closeWindow = async (label: string) => await invoke(CLOSE_WINDOW, { label });
