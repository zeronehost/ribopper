import { invoke } from "@tauri-apps/api/core";
import { CLOSE_WINDOW } from "./constants";

export const closeWindow = async (label: string) => await invoke(CLOSE_WINDOW, { label });
export const WIN_LABEL_SETTING = "setting";
export const WIN_LABEL_TRAY_PANE = "tray_pane";