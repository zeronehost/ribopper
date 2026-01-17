<template>
  <div class="ribo-option">
    <template v-if="options.length > 0">
      <RiboAction v-for="option in options" :key="option.id" :data="option" :selected="selected === option.id" />
    </template>
    <s-empty v-else>暂无指令</s-empty>
  </div>
</template>
<script setup lang="ts">
import { provide, type PropType, ref } from "vue";
import RiboAction from "./action.vue";
import { optionContainerKey } from "./utils";
import type { Action } from "@ribo/api";
defineOptions({
  name: 'RiboOption',
});

defineExpose({
  reset() {
    selected.value = -1;
  }
})

const props = defineProps({
  options: {
    type: Array as PropType<Action[]>,
    default: () => []
  }
});

const emits = defineEmits<{
  selected: [data: Action],
}>();

const selected = ref<number>(-1);

provide(optionContainerKey, {
  selected(data: Action) {
    selected.value = data.id;
    emits('selected', data);
  }
});
</script>
<style lang="scss">

</style>