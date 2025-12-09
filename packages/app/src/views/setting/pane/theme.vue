<template>
  <RiboOptionSection class="theme" title="主题设置">
    <s-card>
      <div slot="headline">内置主题</div>
      <s-segmented-button v-model.lazy="currentTheme" mode="fixed">
        <s-segmented-button-item value="light">
          <s-icon slot="start" name="light_mode"></s-icon>
          <span>浅色</span>
        </s-segmented-button-item>
        <s-segmented-button-item value="dark">
          <s-icon slot="start" name="dark_mode"></s-icon>
          <span>深色</span>
        </s-segmented-button-item>
        <s-segmented-button-item value="auto">
          <s-icon slot="start">
            <RiboIconBrightnessAuto />
          </s-icon>
          <span>给随系统</span>
        </s-segmented-button-item>
      </s-segmented-button>
    </s-card>
  </RiboOptionSection>
</template>
<script setup lang="ts">
import { computed } from "vue";
import {
  RiboIconBrightnessAuto,
} from "@/components/icons";
import { RiboOptionSection } from "@/components/section";
import { useSettingStore } from "@/stores/setting";

const store = useSettingStore();

const currentTheme = computed({
  get() {
    return store.theme;
  },
  set(value: "light" | "dark" | "auto") {
    store.toggleTheme(value);
  },
});
</script>
<style lang="scss">
.theme {
  s-card {
    display: block;
  }
}
</style>