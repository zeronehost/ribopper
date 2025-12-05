import { invoke } from "@tauri-apps/api/core";
// import { listen } from "@tauri-apps/api/event";

export const storeSetTheme = async (theme: string) => {
  await invoke("store_set_theme", { theme });
};

export const storeLoad = async () => await invoke("store_load");

// export const initListener = <T>(cb: (event: string, payload: T) => void) => {
//   listen<{ event: string; data: { key: string; value: T } }>("ribo-store", (e) => {
//     cb(e.payload.event, e.payload.data as T);
//   });
// };
