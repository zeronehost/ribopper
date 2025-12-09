<template>
  <RiboField :title="title">
    <!-- <RiboFieldItem title="可执行"><s-switch v-model.lazy="innerValue.execabled"></s-switch></RiboFieldItem> -->
    <RiboFieldItem title="可生成二维码"><s-switch v-model.lazy="innerValue.scannable" type="checkbox"></s-switch></RiboFieldItem>
    <RiboFieldItem title="可编辑"><s-switch v-model.lazy="innerValue.editable" type="checkbox"></s-switch></RiboFieldItem>
    <RiboFieldItem title="可删除"><s-switch v-model.lazy="innerValue.deletable" type="checkbox"></s-switch></RiboFieldItem>
    <RiboFieldItem title="可收藏"><s-switch v-model.lazy="innerValue.starable" type="checkbox"></s-switch></RiboFieldItem>
  </RiboField>
</template>
<script setup lang="ts">
import type { TypeOptions } from "@ribo/api";
import { type PropType, ref, watch } from "vue";
import { RiboField, RiboFieldItem } from "@/components/field";

defineOptions({
  name: "RiboTypeOption",
});

const props = defineProps({
  title: String,
  modelValue: {
    type: Object as PropType<TypeOptions>,
    default: () => ({}),
  },
});

const emit = defineEmits(["update:modelValue"]);

const innerValue = ref<TypeOptions>(
  Object.assign(
    {},
    {
      editable: false,
      deletable: false,
      scannable: false,
      starable: false,
    },
    props.modelValue,
  ),
);
watch(
  () => props.modelValue,
  (newVal) => {
    innerValue.value = newVal ?? {};
  },
  { deep: true },
);

watch(
  () => innerValue.value,
  (newVal) => {
    console.log("watch -> innerValue =>", newVal);
    emit("update:modelValue", newVal ?? {});
  },
  { deep: true },
);
</script>