<template>
  <section class="setting">
    <s-navigation mode="rail" v-model.lazy="selected">
      <s-navigation-item value="general">
        <s-icon slot="icon">
          <RiboIconSetting />
        </s-icon>
        <div slot="text">
          通用
        </div>
      </s-navigation-item>
      <s-navigation-item v-if="false" value="options">
        <s-icon slot="icon">
          <RiboIconOption />
        </s-icon>
        <div slot="text">
          操作
        </div>
      </s-navigation-item>
      <s-navigation-item value="theme">
        <s-icon slot="icon">
          <RiboIconTheme />
        </s-icon>
        <div slot="text">
          主题
        </div>
      </s-navigation-item>
      <s-navigation-item value="hotkey">
        <s-icon slot="icon">
          <RiboIconKeyboard />
        </s-icon>
        <div slot="text">
          快捷键
        </div>
      </s-navigation-item>
      <s-navigation-item v-if="false" value="helper">
        <s-icon slot="icon">
          <RiboIconHelper />
        </s-icon>
        <div slot="text">
          帮助
        </div>
      </s-navigation-item>
    </s-navigation>
    <section class="content">
      <GeneralPane v-show="selected === 'general'" />
      <OptionPane v-show="selected === 'options'" />
      <ThemePane v-show="selected === 'theme'" />
      <HotkeyPane v-show="selected === 'hotkey'" />
      <HelperPane v-show="selected === 'helper'" />
    </section>
    <section class="options">
      <s-button type="elevated" @click="cancelHandle">
        <s-icon slot="start">
          <RiboIconCancel />
        </s-icon>
        取消</s-button>
      <s-button type="elevated" :disabled="isSubmit" @click="submitHandle">
        <s-icon slot="start">
          <RiboIconCheck />
        </s-icon>
        应用</s-button>
      <s-button type="elevated" @click="confirmHandle">
        <s-icon slot="start">
          <RiboIconCheck />
        </s-icon>
        确定</s-button>
    </section>
  </section>
</template>
<script setup lang="ts">
import { closeWindow } from "@ribo/api";
import { computed, ref } from "vue";
import { useRoute } from "vue-router";
import {
  RiboIconCancel,
  RiboIconCheck,
  RiboIconHelper,
  RiboIconKeyboard,
  RiboIconOption,
  RiboIconSetting,
  RiboIconTheme,
} from "@/components/icons";
import { useSettingStore } from "@/stores/setting";
import HelperPane from "./pane/helper.vue";
import HotkeyPane from "./pane/hot-key.vue";
import GeneralPane from "./pane/index.vue";
import OptionPane from "./pane/options.vue";
import ThemePane from "./pane/theme.vue";

const selected = ref("general");
const route = useRoute();
const store = useSettingStore();
const isSubmit = computed(() => !store.isUpdate);

const submitHandle = async () => {
  await store.saveConfig();
};

const confirmHandle = async () => {
  await store.saveConfig();
  await cancelHandle();
};

const cancelHandle = async () => {
  await closeWindow(route.name as string);
};
</script>
<style lang="scss">
.setting {
  height: 100vh;
  width: 100vw;
  overflow: hidden;
  display: grid;
  grid-template-areas:
  "setting content"
  "setting option";
  grid-template-columns: 80px auto;
  grid-template-rows: auto 80px;

  s-navigation {
    grid-area: setting;
    background: var(--s-color-surface-container-low, #F2F4F5);
  }
  .options {
    grid-area: option;
    display: flex;
    align-items: center;
    justify-content: end;
    width: 100%;
    gap: 1rem;
    padding: 0 1rem;
    background: var(--s-color-surface-container, #ECEEF0);

    s-button {
      border-radius: 6px;
    }
  }
  .content {
    grid-area: content;
    overflow: hidden;
    s-card {
      max-width: unset;
      margin: 1rem;
      padding: .5rem;
    }
  }
}
</style>