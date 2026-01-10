import { listen } from "@tauri-apps/api/event";
import type { RiboEvent } from "./models";

export const listenNotify = (callback: (data: RiboEvent) => void) =>
  listen("ribo-notify", (e) => {
    callback(e.payload as RiboEvent);
  });

export const EVENT_TYPE_INIT = "init";
export const EVENT_TYPE_UPDATE = "update";
export const EVENT_LABEL_CONFIG = "CONFIG";
export const EVENT_LABEL_RECORD = "RECORD";
export const EVENT_LABEL_ACTION = "ACTION";
export const EVENT_LABEL_ACTIONOPTION = "ACTIONOPTION";
export const EVENT_LABEL_OPTION = "OPTION";
export const EVENT_LABEL_TARGET = "TARGET";
export const EVENT_LABEL_ALL = "ALL";