<template>
  <section class="tray-pane">
    <s-appbar v-if="appbar">
      <s-icon-button slot="navigation" @click="closeHandle">
        <s-icon name="close"></s-icon>
      </s-icon-button>
      <s-search slot="search" placeholder="搜索..." v-model="search" @input="searchHandle">
        <s-icon-button slot="end" v-show="search" @click="clearSearchHandle">
          <s-icon name="close">
          </s-icon>
        </s-icon-button>
      </s-search>
      <s-popup-menu slot="action">
        <s-icon-button slot="trigger">
          <s-icon name="menu"></s-icon>
        </s-icon-button>
        <s-popup-menu-item @click="cleanHandle">
          <s-icon slot="start">
            <RiboIconClean />
          </s-icon>
          清空历史记录
        </s-popup-menu-item>
      </s-popup-menu>
    </s-appbar>
    <s-empty v-if="isEmpty">暂时没有内容</s-empty>
    <RiboVirtualList v-else ref="listRef" :list #default="{ data, id, selected }" estimated-item-height="3rem">
      <RiboCard :enabled="settingStore.appInfo?.features" :data-id="id" :data :class="{ selected }"
        @option="optionHandle" />
    </RiboVirtualList>
  </section>
</template>
<script setup lang="ts">
import { closeWindow, copyRecord, logger, WIN_LABEL_TRAY_PANE } from "@ribo/api";
import { computed, nextTick, ref } from "vue";
import { useRouter } from "vue-router";
import { RiboCard } from "@/components/card";
import { RiboIconClean } from "@/components/icons";
import { useRecordStore } from "@/stores/record";
import { debounce } from "@/utils/helper";
import { Snackbar } from "sober";
import { useListenHotKey } from "@/hooks";
import { useCacheStore } from "@/stores/cache";
import { useSettingStore } from "@/stores/setting";
import { RiboVirtualList } from "@/components/scroll-view";

defineOptions({
  name: "RiboView",
});

const props = defineProps({
  appbar: {
    type: Boolean,
    default: false,
  },
  label: {
    type: String,
    required: true
  }
})

const router = useRouter();
const recordStore = useRecordStore();

const isEmpty = computed(() => recordStore.list.length === 0);

const list = computed(() => recordStore.list)

const cacheStore = useCacheStore();

const closeHandle = async () => {
  currentIndex.value = -1;
  await closeWindow(WIN_LABEL_TRAY_PANE);
};
const cleanHandle = async () => {
  await recordStore.clearRecord();
};

const optionHandle = async (option: "delete" | "edit" | "exec" | "copy" | "qrcode", id: number) => {
  if (option === "delete") {
    await recordStore.deleteRecord(id);
  } else if (option === "edit") {
    router.push({ name: "trayPaneEdit", query: { id } });
  } else if (option === "exec") {
    await recordStore.showRecordActions(id, props.label);
  } else if (option === "copy") {
    copyRecord(id)
      .then(() => {
        Snackbar.builder({
          text: "复制成功",
          duration: 1000,
          type: "success",
        });
      })
      .catch((e) => {
        logger.error(e);
        Snackbar.builder({
          text: "复制失败",
          duration: 1000,
          type: "error",
        });
      });
  } else if (option === "qrcode") {
    router.push({ name: "trayPaneQrcode", query: { id } });
  }
};

const search = computed({
  get() {
    return recordStore.contentContains ?? "";
  },
  set(val) {
    recordStore.search(val);
  }
});
const searchHandle = debounce(async () => {
  await nextTick();
  await recordStore.getAllRecords();
}, 300);
const clearSearchHandle = debounce(async () => {
  search.value = "";
  await nextTick();
  await recordStore.getAllRecords();
});


const currentIndex = computed({
  get() {
    return cacheStore.get("currentIndex") ?? -1;
  },
  set(value) {
    cacheStore.set("currentIndex", value ?? -1);
  }
});
const currentId = computed(() => list.value[currentIndex.value]?.id);

const settingStore = useSettingStore();
const listRef = ref<typeof RiboVirtualList>();
logger.debug("setting.hotkeys =>", JSON.stringify(settingStore.hotkeys));
useListenHotKey(settingStore.hotkeys, (type) => {
  logger.debug("useListenHotKey => type", type);
  const id = currentId.value;
  if (id) {
    switch (type) {
      case "edit":
        optionHandle("edit", id);
        break;
      case "copy":
        optionHandle("copy", id);
        break;
      case "qrcode":
        optionHandle("qrcode", id);
        break;
      case "delete":
        optionHandle("delete", id).then(() => {
          currentIndex.value = currentIndex.value - 1;
        });
        break;
      case "clear":
        currentIndex.value = -1;
        break;
    }
  }
  if (type === "prev") {
    listRef.value?.prev();
  } else if (type === "next") {
    listRef.value?.next();
  }
  nextTick().then(() => {
    currentIndex.value = listRef.value?.selected ?? -1;
  })
});


</script>
<style lang="scss">
section.tray-pane {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;

  s-scroll-view {
    flex: 1;
    width: 100%;
    overflow: auto;

    .selected {
      outline: 2px solid var(--s-color-primary);
    }
  }
}
</style>