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
    logger.info("init => configLoad called");
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
  logger.debug("listenNotify =>", data.type);
  if (data.type === "refresh" && typeof route.name === "string" && route.name.includes(data.label)) {
    init();
  }
  if (data.type === "update" && data.label !== "setting") {
    recordStore.getRecords().catch((e) => {
      logger.error(e);
    });
  }
});

window.addEventListener("error", (e) => {
  logger.error(e.error);
});
</script>