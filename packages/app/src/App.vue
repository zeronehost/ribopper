<template>
  <s-page :theme="currentTheme">
    <RouterView />
  </s-page>
</template>
<script setup lang="ts">
import { listenNotify, configLoad, type Theme, logger } from "@ribo/api";
import { computed, onMounted } from "vue";
import { useRoute } from "vue-router";
import { useRecordStore } from "@/stores/record";
import { useSettingStore } from "@/stores/setting";

const store = useSettingStore();
const currentTheme = computed<Theme>(() => store.theme);

const init = () => {
  configLoad().then((res) => {
    logger.info("configLoad =>", JSON.stringify(res));
    if (res) {
      store.$patch({
        config: res,
        _initData: JSON.parse(JSON.stringify(res)),
        isUpdate: false,
      });
    }
  });
};

onMounted(() => {
  init();
});

const route = useRoute();
const recordStore = useRecordStore();
listenNotify((data) => {
  logger.debug("listenNotify =>", JSON.stringify(data), route.name);
  if (data.type === "refresh" && data.label === route.name) {
    init();
  }
  if (data.type === "update" && data.label !== "setting") {
    recordStore.getRecords().catch((e) => {
      logger.error(e);
    });
  }
});

window.addEventListener("error", (e) => {
  logger.error(e.message, `[${e.filename}:${e.lineno}${e.colno}`);
});
</script>