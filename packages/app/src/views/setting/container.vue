<template>
  <s-page class="setting" :theme="currentTheme">
    <s-menu @click="gotoHandle">
      <s-menu-item data-name="general" :checked="currentRoute === 'general'">
        <s-icon slot="start">
          <RiboIconSetting />
        </s-icon>
        通用
      </s-menu-item>
      <s-menu-item data-name="options" :checked="currentRoute === 'options'">
        <s-icon slot="start">
          <RiboIconOption />
        </s-icon>
        操作
      </s-menu-item>
      <s-menu-item data-name="theme" :checked="currentRoute === 'theme'">
        <s-icon slot="start">
          <RiboIconTheme />
        </s-icon>
        主题
      </s-menu-item>
      <s-menu-item data-name="hotkey" :checked="currentRoute === 'hotkey'">
        <s-icon slot="start">
          <RiboIconKeyboard />
        </s-icon>
        快捷键
      </s-menu-item>
      <s-menu-item data-name="helper" :checked="currentRoute === 'helper'">
        <s-icon slot="start">
          <RiboIconHelper />
        </s-icon>
        帮助
      </s-menu-item>
    </s-menu>
    <section class="container">
      <router-view />
    </section>
  </s-page>
</template>
<script setup lang="ts">
import { RiboIconHelper, RiboIconKeyboard, RiboIconOption, RiboIconSetting, RiboIconTheme } from '@/components/icons';
import { useSettingStore } from '@/stores/setting';
import { computed } from 'vue';
import { useRoute, useRouter } from 'vue-router';

const route = useRoute();
const currentRoute = computed(() => {
  return route.name;
});

const store = useSettingStore();
const currentTheme = computed<"light" | "dark" | "auto">(() => store.theme);

const router = useRouter();

const gotoHandle = (e: MouseEvent) => {
  const name = (e.target as HTMLElement)?.dataset.name;
  if (name) {
    router.push({ name });
  }
}
</script>
<style lang="scss">
s-page.setting {
  display: flex;

  section.container {
    flex: 1;
  }
}
</style>