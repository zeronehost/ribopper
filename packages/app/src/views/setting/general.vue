
<template>
  <RiboOptionSection title="通用设置" class="options-panel">
    <RiboField>
      <RiboFieldItem title="历史记录数量" tip="控制保留最近剪贴板历史记录的数量，最大支持设置 1000 条，默认无限制">
        <s-text-field v-model="max" type="number" max="1000" min="1" placeholder="请输入最大历史记录数据量"></s-text-field>
      </RiboFieldItem>
      <s-divider></s-divider>
      <RiboFieldItem title="开机启动" tip="控制是否开机自启">
        <s-switch v-model.lazy="autoStart" type="checkbox"></s-switch>
      </RiboFieldItem>
      <s-divider></s-divider>
      <RiboFieldItem title="退出确认" tip="控制是否在关闭窗口或退出应用程序之前显示确认对话框">
        <s-switch v-model.lazy="exitConfirm" type="checkbox"></s-switch>
      </RiboFieldItem>
    </RiboField>
  </RiboOptionSection>  
</template>
<script setup lang="ts">
import { General } from "@ribo/api";
import { computed } from "vue";
import { RiboField, RiboFieldItem } from "@/components/field";
import { RiboOptionSection } from "@/components/section";
import { useSettingStore } from "@/stores/setting";

const store = useSettingStore();

const max = computed({
  get() {
    return store.max;
  },
  set(value) {
    store.setMax(value as General["max"]);
  },
});

const autoStart = computed({
  get() {
    return store.autoStart;
  },
  set(value) {
    store.setAutoStart(value as General["autoStart"])
  }
});

const exitConfirm = computed({
  get() {
    return store.exitConfirm;
  },
  set(value) {
    store.setExitConfirm(value as General["exitConfirm"])
  }
})

</script>