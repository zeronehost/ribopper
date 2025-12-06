<template>
  <section class="ribo-option-section">
    <s-appbar>
      <span slot="headline">{{title}}</span>
    </s-appbar>
    <s-scroll-view>
      <slot />
    </s-scroll-view>
  </section>
</template>
<script setup lang="ts">
defineProps<{
  title: string;
}>();
defineOptions({
  name: "RiboOptionSection",
});
</script>
<style lang="scss">
section {
  &.ribo-option-section {
    height: 100%;
    width: 100%;
    display: flex;
    flex-direction: column;
    s-scroll-view {
      flex: 1;
      display: flex;
      flex-direction: column;
    }
  }
}
</style>