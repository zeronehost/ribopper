<template>
  <s-page :theme="currentTheme">
    <RouterView />
  </s-page>
</template>
<script setup lang="ts">
import { storeLoad, type Theme } from "@ribo/api";
import { computed } from "vue";
import { useSettingStore } from "@/stores/setting";

const store = useSettingStore();
const currentTheme = computed<Theme>(() => store.theme);

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
</script>