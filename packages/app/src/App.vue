<template>
  <s-page :theme="currentTheme">
    <RouterView />
  </s-page>
</template>
<script setup lang="ts">
import { listenNotify, type Theme, logger, type RiboEvent } from "@ribo/api";
import { computed, onMounted, onUnmounted, provide } from "vue";
import { useSettingStore } from "@/stores/setting";
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

const unlisten = listenNotify<any>(async (data) => {
  logger.debug("listenNotify =>", data.type);
  let arr = hookCache.values();
  let item = arr.next();
  while (!item.done) {
    await item.value(data);
    item = arr.next();
  }
});


window.addEventListener("error", (e) => {
  logger.error(e.error);
});

onMounted(() => {
  store.loadConfig();
});
onUnmounted(() => {
  unlisten.then((fn) => {
    fn()
  });
});
</script>