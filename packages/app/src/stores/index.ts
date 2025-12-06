import { createPinia } from "pinia";
import { useSettingStore } from "./setting";

export const store = createPinia();

// store.use(({ store }) => {
//   store.$subscribe(({ storeId, type, events }) => {
//     console.log("storeId =>", storeId);
//     console.log("type =>", type);
//     console.log("events =>", events);
//     console.log("store =>", store);
//     if (storeId === "setting" && type === "direct" &&( events.key !== "isUpdate" || events.key !== "_initData")) {
//       store.isUpdate = store
//     }
//   });
// });
export const useSubscribe = {
  install() {
    const settingStore = useSettingStore();
    settingStore.$subscribe((mutation, state) => {
      if (mutation.type === "direct") {
        console.log(state);
        // TODO 比较两对象是否相等
      }
    });
  },
};
