import { listen } from "@tauri-apps/api/event";
import type { RiboEvent } from "./models";

export const listenNotify = (callback: (data: RiboEvent) => void) => {
  listen("ribo-notify", (e) => {
    console.log(e);
    callback(e.payload as RiboEvent);
  });
};
