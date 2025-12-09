<template>
  <s-page :theme="currentTheme">
    <RouterView />
  </s-page>
</template>
<script setup lang="ts">
import { listenNotify, storeLoad, type Theme } from "@ribo/api";
import { computed, onMounted } from "vue";
import { useSettingStore } from "@/stores/setting";
import { useDbStore } from "@/stores/db";
import { useRoute } from "vue-router";

const store = useSettingStore();
const currentTheme = computed<Theme>(() => store.theme);

const init = () => {
  storeLoad().then((res) => {
    if (res) {
      console.log("storeLoad =>", res);
      store.$patch({
        config: res,
        _initData: JSON.parse(JSON.stringify(res)),
        isUpdate: false,
      });
    }
  });
}

onMounted(() => {
  init();
})

const route = useRoute();
const dbStore = useDbStore();
listenNotify((data) => {
  if (data.type === "refresh" && data.label === route.name) {
    init();
  } if (data.type === "update" && data.label !== "setting") {
    dbStore.query().catch(e => {
      console.error(e);
    });
  }
});
</script>