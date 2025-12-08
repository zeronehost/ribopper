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
      <input />
      <s-empty v-if="isEmpty">暂时没有内容</s-empty>
      <template v-else>
        <RiboCard
          v-for="history in historys"
          :key="history.id"
          :data="history"
          :editable="isEditable(history.type)"
          deletable
          scannable
          starable
        />
      </template>
    </s-scroll-view>
  </section>
</template>
<script setup lang="ts">
import { closeWindow, type HistoryType } from "@ribo/api";
import { computed, onMounted } from "vue";
import { useRoute } from "vue-router";
import { RiboCard } from "@/components/card";
import { RiboIconClean } from "@/components/icons";
import { useDbStore } from "@/stores/db";

defineOptions({
  name: "tray_pane",
});

const route = useRoute();
const store = useDbStore();

const isEmpty = computed(() => store.total === 0);
const historys = computed(() => store.list);

const isEditable = (type: HistoryType) => type === "text";

const closeHandle = async () => {
  await closeWindow(route.name as string);
};
const cleanHandle = async () => {
  await store.clear();
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