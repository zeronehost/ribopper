import { storeSetTheme } from "@ribo/api";
import { createPinia } from "pinia";

export const store = createPinia();

store.use(({ store }) => {
  store.$subscribe(({ storeId, type, events }) => {
    console.log(storeId, type, events);
    if (storeId === "setting") {
      if (type === "direct") {
        const { type: eventType, key, newValue, oldValue } = events;
        if (eventType === "set" && key === "theme" && newValue !== oldValue) {
          storeSetTheme(newValue);
        }
      }
    }
  });
});
