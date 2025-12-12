
<template>
  <RiboOptionSection title="通用设置" class="options-panel">
    <RiboField>
      <RiboFieldItem title="历史记录数量">
        <s-text-field v-model="max" type="number" placeholder="请输入最大历史记录数据量"></s-text-field>
      </RiboFieldItem>
      <!-- <RiboFieldItem title="查询剪贴板间隔时间">
        <s-text-field v-model="duration" type="number" placeholder="请输入查询剪贴板间隔时间"></s-text-field>
      </RiboFieldItem> -->
    </RiboField>
  </RiboOptionSection>  
</template>
<script setup lang="ts">
import type { GeneralOptions } from "@ribo/api";
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
    store.setMax(value as GeneralOptions["max"]);
  },
});

</script>
<style lang="scss">
  .options-panel {
    s-text-field {
      min-height: 1.5rem;
    }
  }
</style>