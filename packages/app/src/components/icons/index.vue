<template>
  <i class="ribo-icon" :style="{'--size': innerSize}">
    <slot />
  </i>
</template>
<script setup lang="ts">
import { computed } from "vue";

defineOptions({
  name: "RiboIcon",
});
const props = defineProps({
  size: {
    type: [String, Number],
  },
});

const innerSize = computed(() => {
  if (typeof props.size === "number") {
    return `${props.size}px`;
  }
  return props.size;
});
</script>
<style lang="scss">
.ribo-icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  font-size: var(--size, 1rem);
}
</style>