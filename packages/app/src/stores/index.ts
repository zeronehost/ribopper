import { createPinia } from "pinia";
import { isEqual } from "@/utils/types";
import { useSettingStore } from "./setting";

export const store = createPinia();
export const useSubscribe = {
  install() {
    const settingStore = useSettingStore();
    settingStore.$subscribe((mutation, state) => {
      if (mutation.type === "direct") {
        state.isUpdate = !isEqual(state.config, state._initData);
      }
    });
  },
};
