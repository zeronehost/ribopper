<template>
  <RiboOptionSection title="帮助" class="helper">
    <s-card>
      <s-icon class="logo" :src="logo"></s-icon>
      <p class="name">{{ appInfo?.name }}</p>
      <p class="version">V{{ appInfo?.version }}</p>
      <p class="description">{{ appInfo?.description }}</p>
      <!-- <p class="authors">{{ appInfo?.authors }}</p> -->
      <!-- <p class="license">{{ appInfo?.license }}</p> -->
      <!-- <p class="website"><a :href="appInfo?.website">主页</a></p> -->
      <div>
        <s-button class="update" @click="updateAppHandle">检查更新</s-button>
      </div>
    </s-card>
  </RiboOptionSection>
  <RiboDialogUpdate
    v-model="updateF"
    :value="updateInfo.progress"
    :indeterminate="updateInfo.indeterminate"
    :total="updateInfo.total"
    :downloaded="updateInfo.downloaded"
  />
</template>
<script setup lang="ts">
import { RiboOptionSection } from "@/components/section";
import { useSettingStore } from "@/stores/setting";
import { computed, ref } from "vue";
import { logger, updateApp, UpdateApp } from "@ribo/api";
import logo from "@/assets/images/logo.png";
import { RiboDialogUpdate } from "@/components/dialog";

const settingStore = useSettingStore();

const appInfo = computed(() => settingStore.appInfo);

const updateInfo = ref<UpdateApp>({
  done: false,
  indeterminate: true,
});
const updateF = ref(false);
const updateAppHandle = async () => {
  try {
    updateF.value = true;
    await updateApp((payload) => {
      updateInfo.value = payload;
    });
  } catch (e) {
    logger.error(e as Error);
  } finally {
    updateF.value = false;
  }

}
</script>
<style lang="scss">
.ribo-option-section.helper {
  s-scroll-view {
    overflow: hidden;
    padding: 1rem;
  }

  s-card {
    display: flex;
    flex-direction: column;
    max-width: unset;
    height: 100%;
    margin: 0;
    padding: 0.5rem;

    align-items: center;
    gap: 0.5rem;

    p {
      margin: 0;
    }

    .logo {
      width: 4rem;
      height: 4rem;
    }

    .name {
      font-size: 1.5rem;
      font-weight: bold;
      color: var(--s-color-primary);
    }

    .version {
      font-weight: bold;
      font-size: 1.3rem;
      color: var(--s-color-secondary);
    }

    .description {
      font-size: 1rem;
      color: var(--s-color-secondary);
    }

    .authors {
      font-size: 0.8rem;
      color: var(--s-color-tertiary);
    }

  }
}
</style>