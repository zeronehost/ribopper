<template>
  <s-page :theme="currentTheme">
    <RouterView />
  </s-page>
</template>
<script setup lang="ts">
import { listenNotify, configLoad, type Theme, logger, type RiboEvent } from "@ribo/api";
import { computed, onMounted, provide } from "vue";
// import { useRecordStore } from "@/stores/record";
import { useSettingStore } from "@/stores/setting";
// import { useActionStore } from "@/stores/action";
import { rootContextKey } from "@/utils/types";


const store = useSettingStore();
const currentTheme = computed<Theme>(() => store.theme);
const hookCache = new Set<(event: RiboEvent<any>) => void>();

provide(rootContextKey, {
  register<T>(cb: (event: RiboEvent<T>) => void) {
    hookCache.add(cb);
  },
  unregister<T>(cb: (event: RiboEvent<T>) => void) {
    hookCache.delete(cb);
  }
})

// const updateConfigCb = () => {
//   configLoad().then((res) => {
//     logger.info("init => configLoad called");
//     if (res) {
//       store.$patch({
//         config: res,
//         _initData: JSON.parse(JSON.stringify(res)),
//         isUpdate: false,
//       });
//     }
//   });
// };


console.log("listenNotify =>")
listenNotify<any>((data) => {
  logger.debug("listenNotify =>", data.type);
  console.log("listenNotify =>", data);
  // if (data.type === EVENT_TYPE_UPDATE && data.label === EVENT_LABEL_RECORD) {
  //   updateRecordCb();
  // } else if (data.type === EVENT_TYPE_UPDATE && data.label === EVENT_LABEL_CONFIG) {
  //   updateConfigCb();
  // } else if (data.type === EVENT_TYPE_UPDATE && data.label === EVENT_LABEL_ACTION || data.label === EVENT_LABEL_ACTIONOPTION) {
  //   updateActionCb();
  // }
  // hookCache
  hookCache.forEach((cb: (event: RiboEvent<any>) => void) => {
    console.log(data);
    cb(data);
  });
});

// const recordStore = useRecordStore();
// const updateRecordCb = () => {
//   recordStore.getRecords().catch((e) => {
//     logger.error(e);
//   });
// }

// const actionStore = useActionStore();
// const updateActionCb = () => {
//   actionStore.getActions().catch((e) => {
//     logger.error(e);
//   })
// }

window.addEventListener("error", (e) => {
  logger.error(e.error);
});
// onMounted(() => {
//   updateConfigCb();
//   updateActionCb();
//   updateRecordCb();
// });
</script>