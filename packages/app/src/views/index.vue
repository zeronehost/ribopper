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
    <s-scroll-view>
      <s-empty v-if="isEmpty">暂时没有内容</s-empty>
      <template v-else>
        <RiboCard v-for="history in historys" :key="history.id" :data="history" :editable="editabled(history.type)"
          :deletable="deletabled(history.type)" :scannable="scannabled(history.type)"
          :is-star="history.favorites"
          :starable="starabled(history.type)" @edit="editHandle" @delete="deleteHandle" @scan="scanHandle"
          @star="starHandle" @copy="copyHandle" @exec="execHandle" />
      </template>
    </s-scroll-view>
  </section>
</template>
<script setup lang="ts">
import { closeWindow, type HistoryType, type TypeOptions, copyData } from "@ribo/api";
import { computed, onMounted, ref } from "vue";
import { useRoute } from "vue-router";
import { RiboCard } from "@/components/card";
import { RiboIconClean } from "@/components/icons";
import { useDbStore } from "@/stores/db";
import { useSettingStore } from "@/stores/setting";
import { Snackbar } from "sober";
import { debounce } from "@/utils/helper";

defineOptions({
  name: "tray_pane",
});

const route = useRoute();
const store = useDbStore();
const settingStore = useSettingStore();
const searchReg = ref();

const isEmpty = computed(() => store.total === 0);
const historys = computed(() => {
  if (searchReg.value) {
    return store.list.filter((item) => searchReg.value.test(item.content))
  }
  return store.list
});

const options = computed(() => settingStore.typeOptions);
const enable = (type: HistoryType, option: keyof TypeOptions) => {
  return options.value?.[type][option] || false;
};
const editabled = (type: HistoryType) => enable(type, "editable");
const deletabled = (type: HistoryType) => enable(type, "deletable");
const scannabled = (type: HistoryType) => enable(type, "scannable");
const starabled = (type: HistoryType) => enable(type, "starable");

const closeHandle = async () => {
  await closeWindow(route.name as string);
};
const cleanHandle = async () => {
  await store.clear();
};

const editHandle = async (id: number, content: string) => {
  await store.updateData(id, content);
};
const deleteHandle = async (id: number) => {
  await store.delete(id);
};
const scanHandle = (id: number) => {
  console.log("scanHandle =>", id);
};
const starHandle = async (id: number) => {
  await store.ToggleFavorites(id);
};
const copyHandle = (id: number) => {
  console.log("复制数据 =>", id);
  copyData(id).then(() => {
    Snackbar.builder({
      text: "复制成功",
      duration: 1000,
      type: "success",
    });
  }).catch((e) => {
    console.error(e);
    Snackbar.builder({
      text: "复制失败",
      duration: 1000,
      type: "error",
    });
  });
};
const execHandle = (id: number) => {
  console.log("execHandle =>", id);
};

const search = ref("");
const fn = debounce(() => {
  searchReg.value = new RegExp(search.value, "i");
}, 300);
const clearSearchHandle = debounce(() => {
  search.value = "";
  searchReg.value = undefined;
});
const searchHandle = fn

onMounted(async () => {
  await store.query();
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
  }
}
</style>