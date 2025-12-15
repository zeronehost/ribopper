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
    <s-tab v-if="hasFavorites" mode="fixed" v-model.lazy="selected">
      <s-tab-item value="all"><span slot="text">全部</span></s-tab-item>
      <s-tab-item value="favorites"><span slot="text">仅收藏</span></s-tab-item>
    </s-tab>
    <s-scroll-view>
      <s-empty v-if="isEmpty">暂时没有内容</s-empty>
      <template v-else>
        <RiboCard v-for="history in historys" :key="history.id" :data="history"
          @edit="editHandle" @delete="deleteHandle" @scan="scanHandle"
          @favorites="favoritesHandle" @copy="copyHandle" @exec="execHandle" />
      </template>
    </s-scroll-view>
  </section>
</template>
<script setup lang="ts">
import { closeWindow, copyData } from "@ribo/api";
import { Snackbar } from "sober";
import { computed, onMounted, ref } from "vue";
import { useRoute } from "vue-router";
import { RiboCard } from "@/components/card";
import { RiboIconClean } from "@/components/icons";
import { useDbStore } from "@/stores/db";
import { debounce } from "@/utils/helper";

defineOptions({
  name: "tray_pane",
});

const route = useRoute();
const store = useDbStore();
const searchReg = ref();
const isEmpty = computed(() => store.total === 0);
const hasFavorites = computed(() => store.favorites.length > 0);
const selected = ref("all");

const list = computed(() => selected.value === "favorites" ? store.favorites : store.list)
const historys = computed(() => {
  if (searchReg.value) {
    return list.value.filter((item) => searchReg.value.test(item.content));
  }
  return list.value;
});

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
const favoritesHandle = async (id: number) => {
  await store.ToggleFavorites(id);
};
const copyHandle = (id: number) => {
  console.log("复制数据 =>", id);
  copyData(id)
    .then(() => {
      Snackbar.builder({
        text: "复制成功",
        duration: 1000,
        type: "success",
      });
    })
    .catch((e) => {
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
const searchHandle = fn;

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