
<template>
  <RiboOptionSection title="通用设置" class="options-panel">
    <RiboField>
      <RiboFieldItem title="历史记录数量">
        <s-text-field v-model="max" type="number" placeholder="请输入最大历史记录数据量"></s-text-field>
      </RiboFieldItem>
    </RiboField>
    <RiboTypeOption title="文本" v-model.lazy="options.text" /> 
    <RiboTypeOption title="图片" v-model="options.image" /> 
    <RiboTypeOption title="文件" v-model="options.file" /> 
    <RiboTypeOption title="路径" v-model="options.dir" /> 
  </RiboOptionSection>  
</template>
<script setup lang="ts">
import type { GeneralOptions, HistoryType, TypeOptions } from "@ribo/api";
import { computed } from "vue";
import { RiboField, RiboFieldItem } from "@/components/field";
import { RiboOptionSection } from "@/components/section";
import { useSettingStore } from "@/stores/setting";
import RiboTypeOption from "../components/type-option.vue";

const store = useSettingStore();

const max = computed({
  get() {
    return store.max;
  },
  set(value) {
    store.setMax(value as GeneralOptions["max"]);
  },
});

const options = computed<Record<HistoryType, TypeOptions>>({
  get() {
    return store.typeOptions as unknown as Record<HistoryType, TypeOptions>;
  },
  set(value) {
    store.setTypeOptions(value as Record<HistoryType, TypeOptions>);
  }
})
</script>
<style lang="scss">
  .options-panel {
    s-text-field {
      min-height: 1.5rem;
    }
  }
</style>