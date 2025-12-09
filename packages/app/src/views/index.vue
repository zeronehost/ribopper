<template>
  <section class="tray-pane">
    <s-appbar>
      <s-icon-button slot="navigation" @click="closeHandle">
        <s-icon name="close"></s-icon>
      </s-icon-button>
      <s-search slot="search" placeholder="搜索...">
        <template slot="start"></template>
         <s-icon-button slot="end">
           <s-icon name="search"></s-icon>
         </s-icon-button>
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
        <RiboCard
          v-for="history in historys"
          :key="history.id"
          :data="history"
          :editable="editabled(history.type)"
          :deletable="deletabled(history.type)"
          :scannable="scannabled(history.type)"
          :starable="starabled(history.type)"
          @edit="editHandle"
          @delete="deleteHandle"
          @scan="scanHandle"
          @star="starHandle"
          @copy="copyHandle"
          @exec="execHandle"
        />
      </template>
    </s-scroll-view>
  </section>
</template>
<script setup lang="ts">
import { closeWindow, type HistoryType, type TypeOptions } from "@ribo/api";
import { computed, onMounted } from "vue";
import { useRoute } from "vue-router";
import { RiboCard } from "@/components/card";
import { RiboIconClean } from "@/components/icons";
import { useDbStore } from "@/stores/db";
import { useSettingStore } from "@/stores/setting";

defineOptions({
  name: "tray_pane",
});

const route = useRoute();
const store = useDbStore();
const settingStore = useSettingStore();

const isEmpty = computed(() => store.total === 0);
const historys = computed(() => store.list);

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
const starHandle = (id: number) => {
  console.log("starHandle =>", id);
};
const copyHandle = (id: number) => {
  console.log("copyHandle =>", id);
};
const execHandle = (id: number) => {
  console.log("execHandle =>", id);
};

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
    // display: flex;
    // flex-direction: column;
    overflow: auto;
  }
}
</style>