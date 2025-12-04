<template>
  <RouterView />
</template>
<script setup lang="ts">
import { initListener, storeLoad } from "@ribo/api";
import { useRoute } from "vue-router";
import { useSettingStore } from "@/stores/setting";

const route = useRoute();
const store = useSettingStore();

storeLoad().then((data: Any) => {
  store.$state = data as Any;
});

initListener((event: string, data: Any) => {
  if (route.path.startsWith("/setting")) {
    return;
  }
  if (event === "update") {
    if (data.key === "theme") {
      store.toggleTheme(data.value as "light" | "dark" | "auto");
    }
  }
});
</script>