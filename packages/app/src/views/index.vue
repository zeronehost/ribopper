<template>
  <s-page class="tray-pane" :theme="currentTheme">
    <s-appbar>
      <s-icon-button slot="navigation" @click="closeHandle">
        <s-icon name="close"></s-icon>
      </s-icon-button>
      <s-search slot="search" placeholder="请输入">
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
      <RiboCard :id="1" content="aaaa" editable deletable scannable starable />
      <s-empty>暂时没有内容</s-empty>
    </s-scroll-view>
  </s-page>
</template>
<script setup lang="ts">
import { clearData, closeWindow } from "@ribo/api";
import { computed } from "vue";
import { useRoute } from "vue-router";
import { RiboCard } from "@/components/card";
import { RiboIconClean } from "@/components/icons";
import { useSettingStore } from "@/stores/setting";

defineOptions({
  name: "tray_pane",
});

const store = useSettingStore();
const currentTheme = computed<"light" | "dark" | "auto">(() => store.theme);

const route = useRoute();

const closeHandle = async () => {
  await closeWindow(route.name as string);
};
const cleanHandle = async () => {
  await clearData();
};
</script>
<style lang="scss">
s-page.tray-pane {
  display: flex;
  flex-direction: column;
  height: 100vh;

  s-scroll-view {
    flex: 1;
    width: 100%;
    display: flex;
    flex-direction: column
  }
}
</style>