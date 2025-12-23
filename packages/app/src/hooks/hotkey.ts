import type { RiboHotkey } from "@ribo/api";
import { onBeforeUnmount, onMounted } from "vue";

import { useSettingStore } from "@/stores/setting";

export const useListenHotKey = (handle: (label: Omit<keyof RiboHotkey, "clear" | "pane">) => void) => {
  const settingStore = useSettingStore();
  const hotkey = settingStore.hotkeys;
  const listenHotkeyHandle = (e: Event) => {
    if (e instanceof KeyboardEvent) {
      if (
        hotkey.edit &&
        e.key === hotkey.edit.key &&
        e.ctrlKey === hotkey.edit.ctrlKey &&
        e.altKey === hotkey.edit.altKey &&
        e.shiftKey === hotkey.edit.shiftKey &&
        e.metaKey === hotkey.edit.metaKey
      ) {
        e.preventDefault();
        handle("edit");
      } else if (
        hotkey.clear &&
        e.key === hotkey.clear.key &&
        e.ctrlKey === hotkey.clear.ctrlKey &&
        e.altKey === hotkey.clear.altKey &&
        e.shiftKey === hotkey.clear.shiftKey &&
        e.metaKey === hotkey.clear.metaKey
      ) {

        e.preventDefault();
        handle("clear");
      } else if (
        hotkey.next &&
        e.key === hotkey.next.key &&
        e.ctrlKey === hotkey.next.ctrlKey &&
        e.altKey === hotkey.next.altKey &&
        e.shiftKey === hotkey.next.shiftKey &&
        e.metaKey === hotkey.next.metaKey
      ) {
        e.preventDefault();
        handle("next");
      } else if (
        hotkey.prev &&
        e.key === hotkey.prev.key &&
        e.ctrlKey === hotkey.prev.ctrlKey &&
        e.altKey === hotkey.prev.altKey &&
        e.shiftKey === hotkey.prev.shiftKey &&
        e.metaKey === hotkey.prev.metaKey
      ) {
        e.preventDefault();
        handle("prev");
      } else if (
        hotkey.qrcode &&
        e.key === hotkey.qrcode.key &&
        e.ctrlKey === hotkey.qrcode.ctrlKey &&
        e.altKey === hotkey.qrcode.altKey &&
        e.shiftKey === hotkey.qrcode.shiftKey &&
        e.metaKey === hotkey.qrcode.metaKey
      ) {
        e.preventDefault();
        handle("qrcode");
      } else if (
        hotkey.copy &&
        e.key === hotkey.copy.key &&
        e.ctrlKey === hotkey.copy.ctrlKey &&
        e.altKey === hotkey.copy.altKey &&
        e.shiftKey === hotkey.copy.shiftKey &&
        e.metaKey === hotkey.copy.metaKey
      ) {
        e.preventDefault();
        handle("copy");
      } else if (
        hotkey.delete &&
        e.key === hotkey.delete.key &&
        e.ctrlKey === hotkey.delete.ctrlKey &&
        e.altKey === hotkey.delete.altKey &&
        e.shiftKey === hotkey.delete.shiftKey &&
        e.metaKey === hotkey.delete.metaKey
      ) {
        e.preventDefault();
        handle("delete");
      }
    }
  };

  onMounted(() => {
    console.log("listen hotkey");
    document.addEventListener("keydown", listenHotkeyHandle);
  });
  onBeforeUnmount(() => {
    console.log("unlisten hotkey");
    document.removeEventListener("keydown", listenHotkeyHandle);
  });
}