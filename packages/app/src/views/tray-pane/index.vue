<template>
  <section class="tray-pane">
    <s-appbar>
      <s-icon-button slot="navigation" @click="closeHandle">
        <s-icon name="close"></s-icon>
      </s-icon-button>
      <s-search slot="search" placeholder="搜索..." v-model="search" @input="searchHandle">
        <s-icon slot="end" name="close" v-show="search" @click="clearSearchHandle">
        </s-icon>
      </s-search>
      <s-icon-button slot="action" @click="cleanHandle">
        <s-icon>
          <RiboIconClean />
        </s-icon>
      </s-icon-button>
    </s-appbar>
    <!-- <s-tab v-if="hasFavorites" mode="fixed" v-model.lazy="selected">
      <s-tab-item value="all"><span slot="text">全部</span></s-tab-item>
      <s-tab-item value="favorites"><span slot="text">仅收藏</span></s-tab-item>
    </s-tab> -->
    <!-- <s-scroll-view>
      <s-empty v-if="isEmpty">暂时没有内容</s-empty>
      <template v-else>
        <RiboCard :class="{ selected: currentId === record.id }" v-for="record in list" :key="record.id" :data="record"
          @option="optionHandle" />
      </template>
    </s-scroll-view> -->
    <RiboScrollView @load="scrollHandle">
      <s-empty v-if="isEmpty">暂时没有内容</s-empty>
      <template v-else>
        <RiboCard :class="{ selected: currentId === record.id }" v-for="record in list" :key="record.id" :data="record"
          @option="optionHandle" />
      </template>
    </RiboScrollView>
  </section>
</template>
<script setup lang="ts">
import { closeWindow, copyRecord, EVENT_LABEL_ALL, EVENT_LABEL_RECORD, EVENT_LABEL_TARGET, EVENT_TYPE_INIT, EVENT_TYPE_UPDATE, logger, WIN_LABEL_TRAY_PANE, type RiboEvent } from "@ribo/api";
import { computed, inject, onMounted, onUnmounted, ref } from "vue";
import { useRouter } from "vue-router";
import { RiboCard } from "@/components/card";
import { RiboIconClean } from "@/components/icons";
import { useRecordStore } from "@/stores/record";
import { debounce } from "@/utils/helper";
import { Snackbar } from "sober";
import { useListenHotKey } from "@/hooks";
import { useCacheStore } from "@/stores/cache";
import { useSettingStore } from "@/stores/setting";
import { rootContextKey } from "@/utils/types";
import { RiboScrollView } from "@/components/scroll-view";

defineOptions({
  name: "tray_pane",
});

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

const scrollHandle = () => {
  recordStore.getRecords();
}


const search = computed({
  get() {
    return recordStore.contentContains ?? "";
  },
  set(val) {
    recordStore.search(val);
  }
});
const searchHandle = debounce(() => {
  recordStore.reset();
  recordStore.getRecords();
}, 300);
const clearSearchHandle = debounce(() => {
  search.value = "";
  recordStore.reset();
  recordStore.getRecords();
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

const setting = useSettingStore();
logger.debug("setting.hotkeys =>", JSON.stringify(setting.hotkeys));
useListenHotKey(setting.hotkeys, (type) => {
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
        // cleanHandle().then(() => {
        // });
        break;
    }
  }
  switch (type) {
    case "prev":
      currentIndex.value = currentIndex.value <= 0 ? recordStore.total - 1 : currentIndex.value - 1;
      document.querySelector(".selected")!.scrollIntoView({ behavior: "auto", block: "center" });
      break;
    case "next":
      currentIndex.value = currentIndex.value >= recordStore.total - 1 ? 0 : currentIndex.value + 1;
      document.querySelector(".selected")!.scrollIntoView({ behavior: "auto", block: "center" });
      break;
  }
});

const context = inject(rootContextKey);

const loadRecords = (event: RiboEvent<void>) => {
  if (
    (event.type === EVENT_TYPE_INIT || event.type === EVENT_TYPE_UPDATE)
    && (
      event.label === EVENT_LABEL_RECORD
      || event.label === EVENT_LABEL_TARGET
      || event.label === EVENT_LABEL_ALL
    )) {
      recordStore.reset();
      recordStore.getRecords();
  }
};

onMounted(() => {
  context?.register(loadRecords);
  recordStore.reset();
  recordStore.getRecords();
});
onUnmounted(() => {
  context?.unregister(loadRecords);
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