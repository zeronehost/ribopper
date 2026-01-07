<template>
  <s-page :theme="currentTheme">
    <RouterView />
  </s-page>
</template>
<script setup lang="ts">
import { listenNotify, type Theme, logger, type RiboEvent } from "@ribo/api";
import { computed, onMounted, provide } from "vue";
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

listenNotify<any>((data) => {
  logger.debug("listenNotify =>", data.type);
  hookCache.forEach((cb: (event: RiboEvent<any>) => void) => {
    cb(data);
  });
});


window.addEventListener("error", (e) => {
  logger.error(e.error);
});

onMounted(() => {
  store.loadConfig();
})
</script>