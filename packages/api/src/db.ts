import { invoke } from "@tauri-apps/api/core";
import type { Historys, UpdateHistory } from "./models";

export const clearData = async () => await invoke("clear_data");

export const getData = async () => await invoke<Historys>("query_data");

export const updateData = async (history: UpdateHistory) => await invoke<void>("update_data", { history });

export const deleteData = async (id: number) => await invoke<void>("delete_data", { id });
