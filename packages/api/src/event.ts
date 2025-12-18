import { listen } from "@tauri-apps/api/event";
import type { RiboEvent } from "./models";

export const listenNotify = <T>(callback: (data: RiboEvent<T>) => void) => {
  listen("ribo-notify", (e) => {
    console.log(e);
    callback(e.payload as RiboEvent<T>);
  }).catch(e => {console.log(e)});
};
