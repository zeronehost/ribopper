import { logger, Hotkey } from "@ribo/api";
import { onBeforeUnmount, onMounted } from "vue";

export const useListenHotKey = (hotkey: Hotkey, handle: (label: Omit<keyof Hotkey, "clear" | "pane">) => void) => {
  logger.debug("hotkey listener mounted");
  const listenHotkeyHandle = (e: Event) => {
    if (e instanceof KeyboardEvent) {
      if (
        hotkey.edit &&
        e.key === hotkey.edit.key &&
        e.ctrlKey === hotkey.edit.ctrl &&
        e.altKey === hotkey.edit.alt &&
        e.shiftKey === hotkey.edit.shift &&
        e.metaKey === hotkey.edit.meta
      ) {
        e.preventDefault();
        handle("edit");
      } else if (
        hotkey.clear &&
        e.key === hotkey.clear.key &&
        e.ctrlKey === hotkey.clear.ctrl &&
        e.altKey === hotkey.clear.alt &&
        e.shiftKey === hotkey.clear.shift &&
        e.metaKey === hotkey.clear.meta
      ) {

        e.preventDefault();
        handle("clear");
      } else if (
        hotkey.next &&
        e.key === hotkey.next.key &&
        e.ctrlKey === hotkey.next.ctrl &&
        e.altKey === hotkey.next.alt &&
        e.shiftKey === hotkey.next.shift &&
        e.metaKey === hotkey.next.meta
      ) {
        e.preventDefault();
        handle("next");
      } else if (
        hotkey.prev &&
        e.key === hotkey.prev.key &&
        e.ctrlKey === hotkey.prev.ctrl &&
        e.altKey === hotkey.prev.alt &&
        e.shiftKey === hotkey.prev.shift &&
        e.metaKey === hotkey.prev.meta
      ) {
        e.preventDefault();
        handle("prev");
      } else if (
        hotkey.qrcode &&
        e.key === hotkey.qrcode.key &&
        e.ctrlKey === hotkey.qrcode.ctrl &&
        e.altKey === hotkey.qrcode.alt &&
        e.shiftKey === hotkey.qrcode.shift &&
        e.metaKey === hotkey.qrcode.meta
      ) {
        e.preventDefault();
        handle("qrcode");
      } else if (
        hotkey.copy &&
        e.key === hotkey.copy.key &&
        e.ctrlKey === hotkey.copy.ctrl &&
        e.altKey === hotkey.copy.alt &&
        e.shiftKey === hotkey.copy.shift &&
        e.metaKey === hotkey.copy.meta
      ) {
        e.preventDefault();
        handle("copy");
      } else if (
        hotkey.delete &&
        e.key === hotkey.delete.key &&
        e.ctrlKey === hotkey.delete.ctrl &&
        e.altKey === hotkey.delete.alt &&
        e.shiftKey === hotkey.delete.shift &&
        e.metaKey === hotkey.delete.meta
      ) {
        e.preventDefault();
        handle("delete");
      }
    }
  };

  onMounted(() => {
    logger.info("listen hotkey");
    document.addEventListener("keydown", listenHotkeyHandle);
  });
  onBeforeUnmount(() => {
    logger.info("unlisten hotkey");
    document.removeEventListener("keydown", listenHotkeyHandle);
  });
}