<template>
  <RiboOptionSection class="theme" title="主题设置">
    <s-card>
      <div slot="headline" class="header">内置主题</div>
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
            <RiboIconAuto />
          </s-icon>
          <span>给随系统</span>
        </s-segmented-button-item>
        <s-segmented-button-item value="custom" disabled>
          <s-icon slot="start">
            <RiboIconCustom />
          </s-icon>
          <span>自定义</span>
        </s-segmented-button-item>
      </s-segmented-button>
    </s-card>
    <s-card v-if="currentTheme === 'custom'">
      <span>建设中...</span>
    </s-card>
  </RiboOptionSection>
</template>
<script setup lang="ts">
import { computed } from "vue";
import { RiboIconAuto, RiboIconCustom } from "@/components/icons";
import { RiboOptionSection } from "@/components/section";
import { useSettingStore } from "@/stores/setting";
import { type Theme } from "@ribo/api";

const store = useSettingStore();

const currentTheme = computed({
  get() {
    return store.theme;
  },
  set(value: Theme) {
    store.toggleTheme(value);
  },
});
</script>
<style lang="scss">
.theme {
  s-card {
    display: block;
    max-width: unset;
    margin: 1rem;
    padding: .5rem;
    .header {
      font-size: 1rem;
      font-weight: bold;
      margin: 0;
      margin-bottom: 1rem;
    }
  }
}
</style>